use fastly::{http::Method, Body, Error, ObjectStore, Request, Response};
use http::HeaderMap;
use http_range::HttpRange;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Metadata {
    #[serde(rename = "ETag")]
    etag: String,
    #[serde(rename = "Last-Modified")]
    last_modified: String,
    #[serde(rename = "Content-Type")]
    content_type: Option<String>,
}

pub fn get(store_name: &str, request: Request) -> Result<Option<Response>, Error> {
    let method = request.get_method();
    let is_head_request = method == Method::HEAD;
    // static files should only respond on HEAD and GET requests
    if !is_head_request && method != Method::GET {
        return Ok(None);
    }

    // if path ends in / or does not have an extension
    // then append /index.html to the end so we can serve a page
    let mut path = request.get_path().to_string();
    if path.ends_with('/') {
        path += "index.html"
    } else if !path.contains('.') {
        path += "/index.html"
    }

    let metadata_path = format!("{}__metadata__", path);

    let store = ObjectStore::open(store_name).map(|store| store.expect("ObjectStore exists"))?;

    let metadata = store.lookup(&metadata_path)?;
    if metadata.is_none() {
        return Ok(None);
    }
    let metadata = metadata.expect("Metadata is valid");
    let metadata: Metadata = serde_json::from_str(&metadata.into_string())?;
    let response = check_preconditions(request, &metadata)?;
    if let (Some(response), _) = response {
        return Ok(Some(response));
    }
    let request = response.1;
    
    let item = store.lookup(&path)?;

    match item {
        None => return Ok(None),
        Some(item) => {
            let mut headers = HeaderMap::new();
            headers.insert(http::header::ETAG, metadata.etag.parse()?);
            headers.insert(http::header::LAST_MODIFIED, metadata.last_modified.parse()?);

            headers.insert(http::header::ACCEPT_RANGES, "bytes".parse()?);

            if let Some(content_type) = metadata.content_type {
                headers.insert(http::header::CONTENT_TYPE, content_type.parse()?);
            }
            let range = request.get_header_str("range");

            match range {
                Some(range) => {
                    let item_buffer = item.into_bytes();
                    let total = item_buffer.len();
                    match HttpRange::parse(range, total.try_into()?) {
                        Ok(subranges) => {
                            if subranges.len() == 1 {
                                let start: usize = subranges[0].start.try_into()?;
                                let end: usize = subranges[0].length.try_into()?;
                                let end: usize = start + end;
                                headers.insert(
                                    http::header::CONTENT_RANGE,
                                    format!("bytes {}-{}/{}", start, end, total).parse()?,
                                );
                                headers.insert(
                                    http::header::CONTENT_LENGTH,
                                    (end - start + 1).to_string().parse()?,
                                );
                                let mut response = Response::from_status(206);
                                for (name, value) in headers {
                                    response.set_header(name.expect("name is a HeaderName"), value);
                                }
                                if is_head_request {
                                    return Ok(Some(response));
                                } else {
                                    let body = &item_buffer[start..end];
                                    response.set_body(body);
                                    return Ok(Some(response));
                                }
                            } else {
                                let mut body = fastly::Body::new();
                                let boundary = "\n--3d6b6a416f9b5\n".as_bytes();
                                let mime = headers.get("content-type");
                                let mime_type = match mime {
                                    Some(mime) => {
                                        let value = format!("Content-Type: {}\n", mime.to_str()?);
                                        Some(value.as_bytes().to_owned())
                                    }
                                    None => None,
                                };
                                headers.insert(
                                    http::header::CONTENT_TYPE,
                                    "multipart/byteranges; boundary=3d6b6a416f9b5".parse()?,
                                );
                                let mut length = boundary.len();
                                for range in subranges {
                                    let start: usize = range.start.try_into()?;
                                    let end: usize = range.length.try_into()?;
                                    let end: usize = start + end - 1;
                                    body.write_bytes(boundary);
                                    length += boundary.len();
                                    if let Some(ref mime_type) = mime_type {
                                        body.write_bytes(&mime_type);
                                        length += mime_type.len();
                                    }
                                    let range = format!("Content-Range: bytes {}-{}/{}\n\n", start, end, total)
                                        .as_bytes()
                                        .to_owned();
                                    body.write_bytes(&range);
                                    length += range.len();
                                    let buffer = &item_buffer[start..end];
                                    body.write_bytes(buffer);
                                    length += buffer.len();
                                }
                                body.write_bytes(boundary);
                                length += boundary.len();
                                headers.insert(
                                    http::header::CONTENT_LENGTH,
                                    length.to_string().parse()?,
                                );
                                let mut response = Response::from_status(206);
                                for (name, value) in headers {
                                    response.set_header(name.expect("name is a HeaderName"), value);
                                }
                                if is_head_request {
                                    return Ok(Some(response));
                                } else {
                                    response.set_body(body);
                                    return Ok(Some(response));
                                }
                            }
                        }
                        Err(err) => match err {
                            http_range::HttpRangeParseError::InvalidRange => {
                                headers.insert(
                                    http::header::CONTENT_LENGTH,
                                    total.to_string().parse()?,
                                );
                                return non_range_response(
                                    is_head_request,
                                    headers,
                                    fastly::Body::from(item_buffer),
                                );
                            }
                            http_range::HttpRangeParseError::NoOverlap => {
                                headers.insert(
                                    http::header::CONTENT_RANGE,
                                    format!("bytes */{}", total).parse()?,
                                );
                                let mut response = Response::from_status(416);
                                for (name, value) in headers {
                                    response.set_header(name.expect("name is a HeaderName"), value);
                                }
                                return Ok(Some(response));
                            }
                        },
                    };
                }
                None => {
                    return non_range_response(is_head_request, headers, item);
                }
            }
        }
    }
}

fn non_range_response(
    is_head_request: bool,
    headers: HeaderMap,
    item: Body,
) -> Result<Option<Response>, Error> {
    let mut response = Response::from_status(200);
    for (name, value) in headers {
        response.set_header(name.expect("name is a HeaderName"), value)
    }
    if !is_head_request {
        response.set_body(item);
    }
    return Ok(Some(response));
}

fn check_preconditions(
    mut request: Request,
    metadata: &Metadata,
) -> Result<(Option<Response>, Request), Error> {
    // https://httpwg.org/specs/rfc9110.html#rfc.section.13.2.2
    // A recipient cache or origin server MUST evaluate the request preconditions defined by this specification in the following order:
    // 1. When recipient is the origin server and If-Match is present, evaluate the If-Match precondition:
    // - if true, continue to step 3
    // - if false, respond 412 (Precondition Failed) unless it can be determined that the state-changing request has already succeeded (see Section 13.1.1)
    let mut header = request.get_header("if-match");
    if let Some(header) = header {
        if !if_match(metadata, header.to_str()?) {
            return Ok((Some(Response::from_status(412)), request));
        }
        // } else {
        //     // 2. When recipient is the origin server, If-Match is not present, and If-Unmodified-Since is present, evaluate the If-Unmodified-Since precondition:
        //     // - if true, continue to step 3
        //     // - if false, respond 412 (Precondition Failed) unless it can be determined that the state-changing request has already succeeded (see Section 13.1.4)
        //     header = request.get_header("if-unmodified-since");
        //     if let Some(header) = header {
        //         if !ifUnmodifiedSince(metadata, header.to_str()?) {
        //             return Ok((Some(Response::from_status(412)), request));
        //         }
        //     }
    }

    // 3. When If-None-Match is present, evaluate the If-None-Match precondition:
    // - if true, continue to step 5
    // - if false for GET/HEAD, respond 304 (Not Modified)
    // - if false for other methods, respond 412 (Precondition Failed)
    header = request.get_header("if-none-match");
    let method = request.get_method();
    let get = "GET";
    let head = "HEAD";
    if let Some(header) = header {
        if !if_none_match(metadata, header.to_str()?) {
            if method == get || method == head {
                let mut response = Response::from_status(304);
                response.set_header(
                    http::header::ETAG,
                    metadata.etag.parse::<http::HeaderValue>()?,
                );
                response.set_header(
                    http::header::LAST_MODIFIED,
                    metadata.last_modified.parse::<http::HeaderValue>()?,
                );

                response.set_header(
                    http::header::ACCEPT_RANGES,
                    "bytes".parse::<http::HeaderValue>()?,
                );

                if let Some(content_type) = &metadata.content_type {
                    response.set_header(
                        http::header::CONTENT_TYPE,
                        content_type.parse::<http::HeaderValue>()?,
                    );
                }
                return Ok((Some(response), request));
            }
            return Ok((Some(Response::from_status(412)), request));
        }
    } else {
        // 4. When the method is GET or HEAD, If-None-Match is not present, and If-Modified-Since is present, evaluate the If-Modified-Since precondition:
        // - if true, continue to step 5
        // - if false, respond 304 (Not Modified)
        if method == get || method == head {
            header = request.get_header("if-modified-since");
            if let Some(header) = header {
                if !if_modified_since(metadata, header.to_str()?) {
                    let mut response = Response::from_status(304);
                    response.set_header(
                        http::header::ETAG,
                        metadata.etag.parse::<http::HeaderValue>()?,
                    );
                    response.set_header(
                        http::header::LAST_MODIFIED,
                        metadata.last_modified.parse::<http::HeaderValue>()?,
                    );

                    response.set_header(
                        http::header::ACCEPT_RANGES,
                        "bytes".parse::<http::HeaderValue>()?,
                    );

                    if let Some(content_type) = &metadata.content_type {
                        response.set_header(
                            http::header::CONTENT_TYPE,
                            content_type.parse::<http::HeaderValue>()?,
                        );
                    }
                    return Ok((Some(response), request));
                }
            }
        }
    }

    // 5. When the method is GET and both Range and If-Range are present, evaluate the If-Range precondition:
    // - if true and the Range is applicable to the selected representation, respond 206 (Partial Content)
    // - otherwise, ignore the Range header field and respond 200 (OK)
    if method == get {
        if request.contains_header("range") {
            header = request.get_header("if-range");
            if let Some(header) = header {
                if !if_range(metadata, header.to_str()?) {
                    // We delete the range headers so that the `get` function will return the full body
                    request.remove_header("range");
                }
            }
        }
    }

    // 6. Otherwise,
    // - perform the requested method and respond according to its success or failure.
    return Ok((None, request));
}

fn is_weak(etag: &str) -> bool {
    return etag.starts_with("W/\"");
}

fn is_strong(etag: &str) -> bool {
    return etag.starts_with("\"");
}

fn opaque_tag(etag: &str) -> &str {
    if is_weak(etag) {
        return &etag[2..];
    }
    return etag;
}
fn weak_match(a: &str, b: &str) -> bool {
    // https://httpwg.org/specs/rfc9110.html#entity.tag.comparison
    // two entity tags are equivalent if their opaque-tags match character-by-character, regardless of either or both being tagged as "weak".
    return opaque_tag(a) == opaque_tag(b);
}

fn strong_match(a: &str, b: &str) -> bool {
    // https://httpwg.org/specs/rfc9110.html#entity.tag.comparison
    // two entity tags are equivalent if both are not weak and their opaque-tags match character-by-character.
    return is_strong(a) && is_strong(b) && a == b;
}

fn split_list(value: &str) -> Vec<&str> {
    return value.split(",").into_iter().map(|s| s.trim()).collect();
}

// https://httpwg.org/specs/rfc9110.html#field.if-match
fn if_match(validation_fields: &Metadata, header: &str) -> bool {
    // Optimisation for this library as we know there is an etag
    // if validation_fields.etag.is_none() {
    //     return true;
    // }

    // 1. If the field value is "*", the condition is true if the origin server has a current representation for the target resource.
    if header == "*" {
        // Optimisation for this library as we know there is an etag
        // if validation_fields.etag.is_some() {
        return true;
        // }
    } else {
        // 2. If the field value is a list of entity tags, the condition is true if any of the listed tags match the entity tag of the selected representation.
        // An origin server MUST use the strong comparison function when comparing entity tags for If-Match (Section 8.8.3.2),
        // since the client intends this precondition to prevent the method from being applied if there have been any changes to the representation data.
        if split_list(header)
            .into_iter()
            .any(|etag| {
                strong_match(etag, &validation_fields.etag)
            })
        {
            return true;
        }
    }

    // 3. Otherwise, the condition is false.
    return false;
}

// https://httpwg.org/specs/rfc9110.html#field.if-none-match
fn if_none_match(validation_fields: &Metadata, header: &str) -> bool {
    // 1. If the field value is "*", the condition is false if the origin server has a current representation for the target resource.
    if header == "*" {
        // Optimisation for this library as we know there is an etag
        // if validation_fields.etag.is_some() {
        return false;
        // }
    } else {
        // 2. If the field value is a list of entity tags, the condition is false if one of the listed tags matches the entity tag of the selected representation.
        // A recipient MUST use the weak comparison function when comparing entity tags for If-None-Match (Section 8.8.3.2), since weak entity tags can be used for cache validation even if there have been changes to the representation data.
        if split_list(header)
            .iter()
            .any(|etag| weak_match(etag, &validation_fields.etag))
        {
            return false;
        }
    }

    // 3. Otherwise, the condition is true.
    return true;
}

// https://httpwg.org/specs/rfc9110.html#field.if-modified-since
fn if_modified_since(validation_fields: &Metadata, header: &str) -> bool {
    // A recipient MUST ignore the If-Modified-Since header field if the received field value is not a valid HTTP-date, the field value has more than one member, or if the request method is neither GET nor HEAD.
    let date = httpdate::parse_http_date(header);
    if date.is_err() {
        return true;
    }

    // 1. If the selected representation's last modification date is earlier or equal to the date provided in the field value, the condition is false.
    if httpdate::parse_http_date(&validation_fields.last_modified).expect("validation_fields.last_modified is valid HTTP-date") <= date.expect("date is valid HTTP-date") {
        return false;
    }
    // 2. Otherwise, the condition is true.
    return true;
}

// https://httpwg.org/specs/rfc9110.html#field.if-unmodified-since
// fn ifUnmodifiedSince(validation_fields: &Metadata, header: &str) -> bool {
//     // A recipient MUST ignore the If-Unmodified-Since header field if the received field value is not a valid HTTP-date (including when the field value appears to be a list of dates).
//     let date = httpdate::parse_http_date(header);
//     if date.is_err() {
//         return true;
//     }

//     // 1. If the selected representation's last modification date is earlier than or equal to the date provided in the field value, the condition is true.
//     if (httpdate::parse_http_date(&validation_fields.last_modified).expect("validation_fields.last_modified is valid HTTP-date") <= date.expect("date is valid HTTP-date")) {
//         return true;
//     }
//     // 2. Otherwise, the condition is false.
//     return false;
// }

// https://httpwg.org/specs/rfc9110.html#field.if-range
fn if_range(validation_fields: &Metadata, header: &str) -> bool {
    let date = httpdate::parse_http_date(header);
    if let Ok(date) = date {
        // To evaluate a received If-Range header field containing an HTTP-date:
        // 1. If the HTTP-date validator provided is not a strong validator in the sense defined by Section 8.8.2.2, the condition is false.
        // 2. If the HTTP-date validator provided exactly matches the Last-Modified field value for the selected representation, the condition is true.
        if httpdate::parse_http_date(&validation_fields.last_modified).expect("validation_fields.last_modified is valid HTTP-date") == date {
            return true;
        }
        // 3. Otherwise, the condition is false.
        return false;
    } else {
        // To evaluate a received If-Range header field containing an entity-tag:
        // 1. If the entity-tag validator provided exactly matches the ETag field value for the selected representation using the strong comparison function (Section 8.8.3.2), the condition is true.
        if strong_match(header, &validation_fields.etag) {
            return true;
        }
        // 2. Otherwise, the condition is false.
        return false;
    }
}

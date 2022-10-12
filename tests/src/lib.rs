#[allow(dead_code)]
fn get_host() -> String {
    std::env::var("TEST_HOST").expect("Expected TEST_HOST environment variable to exist. E.G. `TEST_HOST='https://example.com' cargo nextest run`")
}

#[allow(dead_code)]
#[derive(PartialEq)]
enum Method {
    HEAD,
    GET,
    PUT,
    POST,
    DELETE,
    PATCH,
}

async fn get_original_response(url:&str, method: &Method) -> reqwest::Response {
    let client = reqwest::Client::new();
    let res = match method {
        Method::HEAD => client.head(url),
        Method::GET => client.get(url),
        Method::PUT => client.put(url),
        Method::POST => client.post(url),
        Method::DELETE => client.delete(url),
        Method::PATCH => client.patch(url),
    }
    .send()
    .await
    .unwrap();
    assert_eq!(res.status(), 200);
    return res;
}

#[allow(dead_code)]
async fn when_if_match_exists_and_evaluation_of_if_match_is_false_return_412_or_2xx(url: &str, method: &Method, expected_status: u16) {
    let client = reqwest::Client::new();

    let res = match method {
        Method::HEAD => client.head(url),
        Method::GET => client.get(url),
        Method::PUT => client.put(url),
        Method::POST => client.post(url),
        Method::DELETE => client.delete(url),
        Method::PATCH => client.patch(url),
    }
    .header("if-match", "")
    .send()
    .await
    .unwrap();
    assert_eq!(res.status(), expected_status);
    assert_eq!(res.bytes().await.unwrap(), bytes::Bytes::new());
}

// TODO: These all can be tested with If-Match:*
#[allow(dead_code)]
async fn when_if_match_and_evaluation_of_if_match_is_true_perform_the_requested_method_and_respond_according_to_its_success_or_failure (url: &str, method: &Method){
    let client = reqwest::Client::new();

    let res = get_original_response(url, method).await;
    let body = res.bytes().await.unwrap();

    let res = match method {
        Method::HEAD => client.head(url),
        Method::GET => client.get(url),
        Method::PUT => client.put(url),
        Method::POST => client.post(url),
        Method::DELETE => client.delete(url),
        Method::PATCH => client.patch(url),
    }
    .header("if-match", "*")
    .send()
    .await
    .unwrap();
    assert_eq!(res.status(), 200);
    assert_eq!(res.bytes().await.unwrap(), body);
}
#[allow(dead_code)]
async fn when_if_match_and_if_unmodified_since_and_evaluation_of_if_match_is_true_do_not_use_if_unmodified_since_perform_the_requested_method_and_respond_according_to_its_success_or_failure (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_modified_since_and_evaluation_of_if_match_is_true_and_method_is_get_or_head_do_not_use_if_modified_since_perform_the_requested_method_and_respond_according_to_its_success_or_failure (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_modified_since_and_evaluation_of_if_match_is_true_and_method_is_not_get_or_head_do_not_use_if_modified_since_perform_the_requested_method_and_respond_according_to_its_success_or_failure (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_none_match_and_evaluation_of_if_match_is_true_and_evaluation_of_if_none_match_is_false_and_method_is_get_or_head_return_304 (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_none_match_and_evaluation_of_if_match_is_true_and_evaluation_of_if_none_match_is_false_and_method_is_not_get_or_head_return_412 (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_unmodified_since_and_if_modified_since_and_evaluation_of_if_match_is_true_do_not_use_if_unmodified_since (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_unmodified_since_and_if_modified_since_and_evaluation_of_if_match_is_true_and_method_is_not_get_or_head_do_not_use_if_unmodified_since_and_do_not_use_if_modified_since (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_unmodified_since_and_if_modified_since_and_evaluation_of_if_match_is_true_and_method_is_get_or_head_do_not_use_if_unmodified_since_and_evaluation_of_if_modified_since_is_false_return_304 (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_unmodified_since_and_if_modified_since_and_evaluation_of_if_match_is_true_and_method_is_get_or_head_do_not_use_if_unmodified_since_and_evaluation_of_if_modified_since_is_true_perform_the_requested_method_and_respond_according_to_its_success_or_failure (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_unmodified_since_and_if_none_match_and_evaluation_of_if_match_is_true_do_not_use_if_unmodified_since_and_evaluation_of_if_none_match_is_false_and_method_is_get_or_head_return_304 (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_unmodified_since_and_if_none_match_and_evaluation_of_if_match_is_true_do_not_use_if_unmodified_since_and_evaluation_of_if_none_match_is_false_and_method_is_not_get_or_head_return_412 (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_unmodified_since_and_if_none_match_and_evaluation_of_if_match_is_true_do_not_use_if_unmodified_since_and_evaluation_of_if_none_match_is_true_perform_the_requested_method_and_respond_according_to_its_success_or_failure (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_unmodified_since_and_if_range_and_evaluation_of_if_match_is_true_do_not_use_if_unmodified_since_and_do_not_use_if_range_perform_the_requested_method_and_respond_according_to_its_success_or_failure (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_unmodified_since_and_if_range_and_range_and_evaluation_of_if_match_is_true_do_not_use_if_unmodified_since_and_evaluation_of_if_range_is_true_return_206 (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_unmodified_since_and_if_range_and_range_and_evaluation_of_if_match_is_true_do_not_use_if_unmodified_since_and_evaluation_of_if_range_is_false_return_200 (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_modified_since_and_if_none_match_and_evaluation_of_if_match_is_true_and_evaluation_of_if_none_match_is_false_and_method_is_get_or_head_return_304 (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_modified_since_and_if_none_match_and_evaluation_of_if_match_is_true_and_evaluation_of_if_none_match_is_false_and_method_is_not_get_or_head_return_412 (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_modified_since_and_if_none_match_and_evaluation_of_if_match_is_true_and_evaluation_of_if_none_match_is_true_and_method_is_not_get_or_head_do_not_use_if_modified_since_perform_the_requested_method_and_respond_according_to_its_success_or_failure (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_modified_since_and_if_none_match_and_evaluation_of_if_match_is_true_and_evaluation_of_if_none_match_is_true_and_method_is_get_or_head_and_evaluation_of_if_modified_since_is_false_return_304 (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_modified_since_and_if_none_match_and_evaluation_of_if_match_is_true_and_evaluation_of_if_none_match_is_true_and_method_is_get_or_head_and_evaluation_of_if_modified_since_is_true_perform_the_requested_method_and_respond_according_to_its_success_or_failure (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_modified_since_and_if_range_and_evaluation_of_if_match_is_true_and_method_is_not_get_or_head_do_not_use_if_modified_since_do_not_use_if_range_perform_the_requested_method_and_respond_according_to_its_success_or_failure (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_modified_since_and_if_range_and_evaluation_of_if_match_is_true_and_method_is_get_or_head_and_evaluation_of_if_modified_since_is_false_return_304 (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_modified_since_and_if_range_and_evaluation_of_if_match_is_true_and_method_is_get_or_head_and_evaluation_of_if_modified_since_is_true_do_not_use_if_range_perform_the_requested_method_and_respond_according_to_its_success_or_failure (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_modified_since_and_if_range_and_range_and_evaluation_of_if_match_is_true_and_method_is_not_get_and_evaluation_of_if_modified_since_is_true_do_not_use_if_range_perform_the_requested_method_and_respond_according_to_its_success_or_failure (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_modified_since_and_if_range_and_range_and_evaluation_of_if_match_is_true_and_method_is_get_and_evaluation_of_if_modified_since_is_true_and_evaluation_of_if_range_is_true_return_206 (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_modified_since_and_if_range_and_range_and_evaluation_of_if_match_is_true_and_method_is_get_and_evaluation_of_if_modified_since_is_true_and_evaluation_of_if_range_is_false_return_200 (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_none_match_and_if_range_and_evaluation_of_if_match_is_true_and_evaluation_of_if_none_match_is_false_and_method_is_get_or_head_return_304 (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_none_match_and_if_range_and_evaluation_of_if_match_is_true_and_evaluation_of_if_none_match_is_false_and_method_not_is_get_or_head_return_412 (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_none_match_and_if_range_and_evaluation_of_if_match_is_true_and_evaluation_of_if_none_match_is_true_do_not_use_if_range_perform_the_requested_method_and_respond_according_to_its_success_or_failure (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_none_match_and_if_range_and_range_and_evaluation_of_if_match_is_true_and_evaluation_of_if_none_match_is_true_and_method_is_not_get_do_not_use_if_range_perform_the_requested_method_and_respond_according_to_its_success_or_failure (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_none_match_and_if_range_and_range_and_evaluation_of_if_match_is_true_and_evaluation_of_if_none_match_is_true_and_method_is_get_and_evaluation_of_if_range_is_true_return_206 (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_none_match_and_if_range_and_range_and_evaluation_of_if_match_is_true_and_evaluation_of_if_none_match_is_true_and_method_is_get_and_evaluation_of_if_range_is_false_return_200 (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_unmodified_since_and_if_modified_since_and_if_none_match_and_evaluation_of_if_match_is_true_do_not_use_if_unmodified_since_and_evaluation_of_if_none_match_is_false_and_method_is_get_or_head_return_304 (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_unmodified_since_and_if_modified_since_and_if_none_match_and_evaluation_of_if_match_is_true_do_not_use_if_unmodified_since_and_evaluation_of_if_none_match_is_false_and_method_is_not_get_or_head_return_412 (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_unmodified_since_and_if_modified_since_and_if_none_match_and_evaluation_of_if_match_is_true_do_not_use_if_unmodified_since_and_evaluation_of_if_none_match_is_true_do_not_use_if_modified_since_perform_the_requested_method_and_respond_according_to_its_success_or_failure (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_unmodified_since_and_if_modified_since_and_if_range_and_evaluation_of_if_match_is_true_do_not_use_if_unmodified_since_and_method_get_or_head_and_evaluation_of_if_modified_since_is_false_return_304 (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_unmodified_since_and_if_modified_since_and_if_range_and_evaluation_of_if_match_is_true_do_not_use_if_unmodified_since_and_method_get_or_head_and_evaluation_of_if_modified_since_is_true_do_not_use_if_range_perform_the_requested_method_and_respond_according_to_its_success_or_failure (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_unmodified_since_and_if_modified_since_and_if_range_and_evaluation_of_if_match_is_true_do_not_use_if_unmodified_since_and_method_not_get_or_head_do_not_use_if_modified_since_is_true_do_not_use_if_range_perform_the_requested_method_and_respond_according_to_its_success_or_failure (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_unmodified_since_and_if_modified_since_and_if_range_and_range_and_evaluation_of_if_match_is_true_do_not_use_if_unmodified_since_and_method_not_get_and_evaluation_of_if_modified_since_is_true_do_not_use_if_range_perform_the_requested_method_and_respond_according_to_its_success_or_failure (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_unmodified_since_and_if_modified_since_and_if_range_and_range_and_evaluation_of_if_match_is_true_do_not_use_if_unmodified_since_and_method_get_and_evaluation_of_if_modified_since_is_true_and_evaluation_of_if_range_is_true_return_206 (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_unmodified_since_and_if_modified_since_and_if_range_and_range_and_evaluation_of_if_match_is_true_do_not_use_if_unmodified_since_and_method_get_and_evaluation_of_if_modified_since_is_true_and_evaluation_of_if_range_is_false_return_200 (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_unmodified_since_and_if_none_match_and_if_range_and_evaluation_of_if_match_is_true_do_not_use_if_unmodified_since_and_evaluation_of_if_none_match_is_false_and_method_is_get_or_head_return_304 (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_unmodified_since_and_if_none_match_and_if_range_and_evaluation_of_if_match_is_true_do_not_use_if_unmodified_since_and_evaluation_of_if_none_match_is_false_and_method_is_not_get_or_head_return_412 (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_unmodified_since_and_if_none_match_and_if_range_and_evaluation_of_if_match_is_true_do_not_use_if_unmodified_since_and_evaluation_of_if_none_match_is_true_do_not_use_if_range_perform_the_requested_method_and_respond_according_to_its_success_or_failure (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_unmodified_since_and_if_none_match_and_if_range_and_range_and_evaluation_of_if_match_is_true_do_not_use_if_unmodified_since_and_evaluation_of_if_none_match_is_true_and_method_is_not_get_do_not_use_if_range_perform_the_requested_method_and_respond_according_to_its_success_or_failure (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_unmodified_since_and_if_none_match_and_if_range_and_range_and_evaluation_of_if_match_is_true_do_not_use_if_unmodified_since_and_evaluation_of_if_none_match_is_true_and_method_is_get_and_evaluation_of_if_range_is_true_return_206 (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_unmodified_since_and_if_none_match_and_if_range_and_range_and_evaluation_of_if_match_is_true_do_not_use_if_unmodified_since_and_evaluation_of_if_none_match_is_true_and_method_is_get_and_evaluation_of_if_range_is_false_return_200 (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_modified_since_and_if_none_match_and_if_range_and_evaluation_of_if_match_is_true_and_evaluation_of_if_none_match_is_false_and_method_is_get_or_head_return_304 (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_modified_since_and_if_none_match_and_if_range_and_evaluation_of_if_match_is_true_and_evaluation_of_if_none_match_is_false_and_method_not_is_get_or_head_return_412 (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_modified_since_and_if_none_match_and_if_range_and_evaluation_of_if_match_is_true_and_evaluation_of_if_none_match_is_true_do_not_use_if_modified_since_do_not_use_if_range_perform_the_requested_method_and_respond_according_to_its_success_or_failure (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_modified_since_and_if_none_match_and_if_range_and_range_and_evaluation_of_if_match_is_true_and_evaluation_of_if_none_match_is_true_do_not_use_if_modified_since_and_method_not_get_do_not_use_if_range_perform_the_requested_method_and_respond_according_to_its_success_or_failure (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_modified_since_and_if_none_match_and_if_range_and_range_and_evaluation_of_if_match_is_true_and_evaluation_of_if_none_match_is_true_do_not_use_if_modified_since_and_method_get_and_evaluation_of_if_range_is_true_return_206 (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_modified_since_and_if_none_match_and_if_range_and_range_and_evaluation_of_if_match_is_true_and_evaluation_of_if_none_match_is_true_do_not_use_if_modified_since_and_method_get_and_evaluation_of_if_range_is_false_return_200 (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_unmodified_since_and_if_modified_since_and_if_none_match_and_if_range_and_evaluation_of_if_match_is_true_do_not_use_if_unmodified_since_and_evaluation_of_if_none_match_is_false_and_method_is_get_or_head_return_304 (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_unmodified_since_and_if_modified_since_and_if_none_match_and_if_range_and_evaluation_of_if_match_is_true_do_not_use_if_unmodified_since_and_evaluation_of_if_none_match_is_false_and_method_is_not_get_or_head_return_412 (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_unmodified_since_and_if_modified_since_and_if_none_match_and_if_range_and_evaluation_of_if_match_is_true_do_not_use_if_unmodified_since_and_evaluation_of_if_none_match_is_true_do_not_use_if_modified_since_and_method_not_get_do_not_use_if_range_perform_the_requested_method_and_respond_according_to_its_success_or_failure (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_unmodified_since_and_if_modified_since_and_if_none_match_and_if_range_and_evaluation_of_if_match_is_true_do_not_use_if_unmodified_since_and_evaluation_of_if_none_match_is_true_do_not_use_if_modified_since_and_method_get_and_evaluation_of_if_range_is_true_return_206 (url: &str, method: &Method){}
#[allow(dead_code)]
async fn when_if_match_and_if_unmodified_since_and_if_modified_since_and_if_none_match_and_if_range_and_evaluation_of_if_match_is_true_do_not_use_if_unmodified_since_and_evaluation_of_if_none_match_is_true_do_not_use_if_modified_since_and_method_get_and_evaluation_of_if_range_is_false_return_200 (url: &str, method: &Method){}

#[allow(dead_code)]
async fn when_if_unmodified_since_and_evaluation_of_if_unmodified_since_is_false_return_412_or_2xx(url: &str, method: &Method, expected_status: u16) {
    let client = reqwest::Client::new();
    let res = get_original_response(url, method).await;

    let last_modified = res.headers().get("last-modified").unwrap().to_owned();
    let before_last_modified = httpdate::fmt_http_date(
        httpdate::parse_http_date(last_modified.to_str().unwrap())
            .unwrap()
            .checked_sub(std::time::Duration::from_secs(60))
            .unwrap(),
    );
    let body = res.bytes().await.unwrap();

    let res = match method {
        Method::HEAD => client.head(url),
        Method::GET => client.get(url),
        Method::PUT => client.put(url),
        Method::POST => client.post(url),
        Method::DELETE => client.delete(url),
        Method::PATCH => client.patch(url),
    }
    .header("if-unmodified-since", &before_last_modified)
    .send()
    .await
    .unwrap();
    assert_eq!(res.status(), expected_status);
    assert_eq!(res.bytes().await.unwrap(), body);

    let res = match method {
        Method::HEAD => client.head(url),
        Method::GET => client.get(url),
        Method::PUT => client.put(url),
        Method::POST => client.post(url),
        Method::DELETE => client.delete(url),
        Method::PATCH => client.patch(url),
    }
    .header("if-unmodified-since", "carrot")
    .send()
    .await
    .unwrap();
    assert_eq!(res.status(), expected_status);
    assert_eq!(res.bytes().await.unwrap(), body);
}

#[allow(dead_code)]
async fn when_if_modified_since_and_method_is_get_or_head_and_evaluation_of_if_modified_since_is_false_return_304(url: &str, method: &Method) {
    let client = reqwest::Client::new();

    let res = get_original_response(url, method).await;

    let last_modified = res.headers().get("last-modified").unwrap().to_owned();
    let after_last_modified = httpdate::fmt_http_date(
        httpdate::parse_http_date(last_modified.to_str().unwrap())
            .unwrap()
            .checked_add(std::time::Duration::from_secs(60))
            .unwrap(),
    );

    let res = match method {
        Method::HEAD => client.head(url),
        Method::GET => client.get(url),
        Method::PUT => unreachable!(),
        Method::POST => unreachable!(),
        Method::DELETE => unreachable!(),
        Method::PATCH => unreachable!(),
    }
    .header("if-modified-since", &last_modified)
    .send()
    .await
    .unwrap();
    assert_eq!(res.status(), 304);
    assert_eq!(res.bytes().await.unwrap(), bytes::Bytes::new());

    let res = match method {
        Method::HEAD => client.head(url),
        Method::GET => client.get(url),
        Method::PUT => unreachable!(),
        Method::POST => unreachable!(),
        Method::DELETE => unreachable!(),
        Method::PATCH => unreachable!(),
    }
    .header("if-modified-since", &after_last_modified)
    .send()
    .await
    .unwrap();
    assert_eq!(res.status(), 304);
    assert_eq!(res.bytes().await.unwrap(), bytes::Bytes::new());
}

#[allow(dead_code)]
async fn when_if_modified_since_and_method_is_get_or_head_and_evaluation_of_if_modified_since_is_true_perform_the_requested_method_and_respond_according_to_its_success_or_failure(url: &str, method: &Method, expected_status: u16) {
    let client = reqwest::Client::new();

    let res = get_original_response(url, method).await;

    let last_modified = res.headers().get("last-modified").unwrap().to_owned();
    let before_last_modified = httpdate::fmt_http_date(
        httpdate::parse_http_date(last_modified.to_str().unwrap())
            .unwrap()
            .checked_sub(std::time::Duration::from_secs(60))
            .unwrap(),
    );
    let body = res.bytes().await.unwrap();

    let res = match method {
        Method::HEAD => client.head(url),
        Method::GET => client.get(url),
        Method::PUT => unreachable!(),
        Method::POST => unreachable!(),
        Method::DELETE => unreachable!(),
        Method::PATCH => unreachable!(),
    }
    .header("if-modified-since", &before_last_modified)
    .send()
    .await
    .unwrap();
    assert_eq!(res.status(), expected_status);
    assert_eq!(res.bytes().await.unwrap(), body);

    let res = match method {
        Method::HEAD => client.head(url),
        Method::GET => client.get(url),
        Method::PUT => unreachable!(),
        Method::POST => unreachable!(),
        Method::DELETE => unreachable!(),
        Method::PATCH => unreachable!(),
    }
    .header("if-modified-since", "carrot")
    .send()
    .await
    .unwrap();
    assert_eq!(res.status(), expected_status);
    assert_eq!(res.bytes().await.unwrap(), body);
}

#[allow(dead_code)]
async fn when_if_modified_since_and_method_is_not_get_or_head_do_not_use_if_modified_since_perform_the_requested_method_and_respond_according_to_its_success_or_failure(url: &str, method: &Method, expected_status: u16) {
    let client = reqwest::Client::new();

    let res = get_original_response(url, method).await;

    let last_modified = res.headers().get("last-modified").unwrap().to_owned();
    let before_last_modified = httpdate::fmt_http_date(
        httpdate::parse_http_date(last_modified.to_str().unwrap())
            .unwrap()
            .checked_sub(std::time::Duration::from_secs(60))
            .unwrap(),
    );
    let after_last_modified = httpdate::fmt_http_date(
        httpdate::parse_http_date(last_modified.to_str().unwrap())
            .unwrap()
            .checked_add(std::time::Duration::from_secs(60))
            .unwrap(),
    );
    let body = res.bytes().await.unwrap();

    let res = match method {
        Method::HEAD => unimplemented!(),
        Method::GET => unimplemented!(),
        Method::PUT => client.put(url),
        Method::POST => client.post(url),
        Method::DELETE => client.delete(url),
        Method::PATCH => client.patch(url),
    }
    .header("if-modified-since", &before_last_modified)
    .send()
    .await
    .unwrap();
    assert_eq!(res.status(), expected_status);
    assert_eq!(res.bytes().await.unwrap(), body);

    let res = match method {
        Method::HEAD => unimplemented!(),
        Method::GET => unimplemented!(),
        Method::PUT => client.put(url),
        Method::POST => client.post(url),
        Method::DELETE => client.delete(url),
        Method::PATCH => client.patch(url),
    }
    .header("if-modified-since", &last_modified)
    .send()
    .await
    .unwrap();
    assert_eq!(res.status(), expected_status);
    assert_eq!(res.bytes().await.unwrap(), bytes::Bytes::new());

    let res = match method {
        Method::HEAD => unimplemented!(),
        Method::GET => unimplemented!(),
        Method::PUT => client.put(url),
        Method::POST => client.post(url),
        Method::DELETE => client.delete(url),
        Method::PATCH => client.patch(url),
    }
    .header("if-modified-since", "carrot")
    .send()
    .await
    .unwrap();
    assert_eq!(res.status(), expected_status);
    assert_eq!(res.bytes().await.unwrap(), body);

    let res = match method {
        Method::HEAD => unimplemented!(),
        Method::GET => unimplemented!(),
        Method::PUT => client.put(url),
        Method::POST => client.post(url),
        Method::DELETE => client.delete(url),
        Method::PATCH => client.patch(url),
    }
    .header("if-modified-since", &after_last_modified)
    .send()
    .await
    .unwrap();
    assert_eq!(res.status(), expected_status);
    assert_eq!(res.bytes().await.unwrap(), bytes::Bytes::new());
}

#[allow(dead_code)]
async fn if_none_match(url: &str, method: &Method) {
    let client = reqwest::Client::new();

    let res = get_original_response(url, method).await;

    let etag = res.headers().get("etag").unwrap().to_owned();
    let body = res.bytes().await.unwrap();

    let res = match method {
        Method::HEAD => client.head(url),
        Method::GET => client.get(url),
        Method::PUT => client.put(url),
        Method::POST => client.post(url),
        Method::DELETE => client.delete(url),
        Method::PATCH => client.patch(url),
    }
    .header("if-none-match", &etag)
    .send()
    .await
    .unwrap();
    assert_eq!(res.status(), 304);
    assert_eq!(res.bytes().await.unwrap(), bytes::Bytes::new());

    let res = match method {
        Method::HEAD => client.head(url),
        Method::GET => client.get(url),
        Method::PUT => client.put(url),
        Method::POST => client.post(url),
        Method::DELETE => client.delete(url),
        Method::PATCH => client.patch(url),
    }
    .header("if-none-match", "")
    .send()
    .await
    .unwrap();
    assert_eq!(res.status(), 200);
    assert_eq!(res.bytes().await.unwrap(), body);
}

#[allow(dead_code)]
async fn range(url: &str, method: &Method) {
    let client = reqwest::Client::new();

    let res = get_original_response(url, method).await;

    let body = res.bytes().await.unwrap();

    let res = match method {
        Method::HEAD => client.head(url),
        Method::GET => client.get(url),
        Method::PUT => client.put(url),
        Method::POST => client.post(url),
        Method::DELETE => client.delete(url),
        Method::PATCH => client.patch(url),
    }
        .header("range", "1-10,20-30")
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    assert_eq!(res.bytes().await.unwrap(), body);

    let res = match method {
        Method::HEAD => client.head(url),
        Method::GET => client.get(url),
        Method::PUT => client.put(url),
        Method::POST => client.post(url),
        Method::DELETE => client.delete(url),
        Method::PATCH => client.patch(url),
    }
        .header("range", "bytes=1-10,20-30")
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 206);
    match method {
        Method::HEAD => assert_eq!(res.bytes().await.unwrap(), bytes::Bytes::from(&b""[..])),
        Method::GET => assert_eq!(res.bytes().await.unwrap(), bytes::Bytes::from(&b"\n--3d6b6a416f9b5\nContent-Type: text/html\nContent-Range: bytes 1-10/679\n\n!doctype \n--3d6b6a416f9b5\nContent-Type: text/html\nContent-Range: bytes 20-30/679\n\nl>\n<head>\n--3d6b6a416f9b5"[..])),
        Method::PUT => unimplemented!(),
        Method::POST => unimplemented!(),
        Method::DELETE => unimplemented!(),
        Method::PATCH => unimplemented!(),
    }

    let res = match method {
        Method::HEAD => client.head(url),
        Method::GET => client.get(url),
        Method::PUT => client.put(url),
        Method::POST => client.post(url),
        Method::DELETE => client.delete(url),
        Method::PATCH => client.patch(url),
    }
        .header("range", "bytes=100-10")
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 416);
    assert_eq!(res.bytes().await.unwrap(), bytes::Bytes::new());
}

#[allow(dead_code)]
async fn if_range(url: &str, method: &Method, expected_status: u16) {
    let client = reqwest::Client::new();

    let res = get_original_response(url, method).await;

    let etag = res.headers().get("etag").unwrap().to_owned();
    let last_modified = res.headers().get("last-modified").unwrap().to_owned();
    let before_last_modified = httpdate::fmt_http_date(
        httpdate::parse_http_date(last_modified.to_str().unwrap())
            .unwrap()
            .checked_sub(std::time::Duration::from_secs(60))
            .unwrap(),
    );
    let after_last_modified = httpdate::fmt_http_date(
        httpdate::parse_http_date(last_modified.to_str().unwrap())
            .unwrap()
            .checked_add(std::time::Duration::from_secs(60))
            .unwrap(),
    );
    let body = res.bytes().await.unwrap();

    let res = match method {
        Method::HEAD => client.head(url),
        Method::GET => client.get(url),
        Method::PUT => client.put(url),
        Method::POST => client.post(url),
        Method::DELETE => client.delete(url),
        Method::PATCH => client.patch(url),
    }
        .header("range", "bytes=1-10,20-30")
        .header("if-range", &etag)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), expected_status);
    assert_eq!(res.bytes().await.unwrap(), body);

    let res = match method {
        Method::HEAD => client.head(url),
        Method::GET => client.get(url),
        Method::PUT => client.put(url),
        Method::POST => client.post(url),
        Method::DELETE => client.delete(url),
        Method::PATCH => client.patch(url),
    }
        .header("range", "bytes=1-10,20-30")
        .header("if-range", &before_last_modified)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), expected_status);
    assert_eq!(res.bytes().await.unwrap(), body);
    let res = match method {
        Method::HEAD => client.head(url),
        Method::GET => client.get(url),
        Method::PUT => client.put(url),
        Method::POST => client.post(url),
        Method::DELETE => client.delete(url),
        Method::PATCH => client.patch(url),
    }
        .header("range", "bytes=1-10,20-30")
        .header("if-range", &last_modified)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 206);
    if method == &Method::GET {
        assert_eq!(res.bytes().await.unwrap(), bytes::Bytes::from(&b"\n--3d6b6a416f9b5\nContent-Type: text/html\nContent-Range: bytes 1-10/679\n\n!doctype \n--3d6b6a416f9b5\nContent-Type: text/html\nContent-Range: bytes 20-30/679\n\nl>\n<head>\n--3d6b6a416f9b5"[..]));
    } else {
        assert_eq!(res.bytes().await.unwrap(), bytes::Bytes::from(&b""[..]));
    }
    let res = match method {
        Method::HEAD => client.head(url),
        Method::GET => client.get(url),
        Method::PUT => client.put(url),
        Method::POST => client.post(url),
        Method::DELETE => client.delete(url),
        Method::PATCH => client.patch(url),
    }
        .header("range", "bytes=1-10,20-30")
        .header("if-range", &after_last_modified)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), expected_status);
    assert_eq!(res.bytes().await.unwrap(), body);
}

#[tokio::test]
async fn test_get_root_path_if_match() {
    let url = format!("{}/", get_host());
    when_if_match_exists_and_evaluation_of_if_match_is_false_return_412_or_2xx(&url, &Method::GET, 412).await;
}

#[tokio::test]
async fn test_method_get_path_root_when_if_match_and_evaluation_of_if_match_is_true_perform_the_requested_method_and_respond_according_to_its_success_or_failure() {
    let url = format!("{}/", get_host());
    when_if_match_and_evaluation_of_if_match_is_true_perform_the_requested_method_and_respond_according_to_its_success_or_failure (&url, &Method::GET).await;
}



#[tokio::test]
async fn test_get_root_path_if_unmodified_since() {
    let host = get_host();
    let url = format!("{}/", host);
    when_if_unmodified_since_and_evaluation_of_if_unmodified_since_is_false_return_412_or_2xx(&url, &Method::GET, 200).await;
}

#[tokio::test]
async fn test_get_root_path_if_none_match() {
    let host = get_host();
    let url = format!("{}/", host);
    if_none_match(&url, &Method::GET).await;
}

#[tokio::test]
async fn test_get_root_path_if_modified_since() {
    let host = get_host();
    let url = format!("{}/", host);
    when_if_modified_since_and_method_is_get_or_head_and_evaluation_of_if_modified_since_is_false_return_304(&url, &Method::GET).await;
    when_if_modified_since_and_method_is_get_or_head_and_evaluation_of_if_modified_since_is_true_perform_the_requested_method_and_respond_according_to_its_success_or_failure(&url, &Method::GET, 200).await;
    // NOTE: Skipped as this library only handles GET and HEAD
    // when_if_modified_since_and_method_is_not_get_or_head_do_not_use_if_modified_since_perform_the_requested_method_and_respond_according_to_its_success_or_failure(&url, &Method::PUT, 200).await;
}

#[tokio::test]
async fn test_get_root_path_range() {
    let host = get_host();
    let url = format!("{}/", host);
    range(&url, &Method::GET).await;
}

#[tokio::test]
async fn test_get_root_path_if_range() {
    let host = get_host();
    let url = format!("{}/", host);
    if_range(&url, &Method::GET, 200).await;
}

#[tokio::test]
async fn test_head_root_path_if_match() {
    let host = get_host();
    let url = format!("{}/", host);
    when_if_match_exists_and_evaluation_of_if_match_is_false_return_412_or_2xx(&url, &Method::HEAD, 412).await;
}

#[tokio::test]
async fn test_head_root_path_if_unmodified_since() {
    let host = get_host();
    let url = format!("{}/", host);
    when_if_unmodified_since_and_evaluation_of_if_unmodified_since_is_false_return_412_or_2xx(&url, &Method::HEAD, 200).await;
}

#[tokio::test]
async fn test_head_root_path_if_none_match() {
    let host = get_host();
    let url = format!("{}/", host);
    if_none_match(&url, &Method::HEAD).await;
}

#[tokio::test]
async fn test_head_root_path_if_modified_since() {
    let host = get_host();
    let url = format!("{}/", host);
    when_if_modified_since_and_method_is_get_or_head_and_evaluation_of_if_modified_since_is_false_return_304(&url, &Method::HEAD).await;
    when_if_modified_since_and_method_is_get_or_head_and_evaluation_of_if_modified_since_is_true_perform_the_requested_method_and_respond_according_to_its_success_or_failure(&url, &Method::HEAD, 200).await;
    // NOTE: Skipped as this library only handles GET and HEAD
    // when_if_modified_since_and_method_is_not_get_or_head_do_not_use_if_modified_since_perform_the_requested_method_and_respond_according_to_its_success_or_failure(&url, &Method::PUT, 200).await;
}

#[tokio::test]
async fn test_head_root_path_range() {
    let host = get_host();
    let url = format!("{}/", host);
    range(&url, &Method::HEAD).await;
}

#[tokio::test]
async fn test_head_root_path_if_range() {
    let host = get_host();
    let url = format!("{}/", host);
    if_range(&url, &Method::HEAD, 206).await;
}

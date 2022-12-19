use c_at_e_file_server::get;
use fastly::{Error, Request, Response};

#[fastly::main]
fn main(request: Request) -> Result<Response, Error> {
    if let Ok(fastly_service_version) = std::env::var("FASTLY_SERVICE_VERSION") {
        println!("FASTLY_SERVICE_VERSION: {}", fastly_service_version);
    }
    let mut response = get("site", request)?;
    return match response {
        Some(response) => {
            // Enable Dynamic Compression -- https://developer.fastly.com/learning/concepts/compression/#dynamic-compression
            response.set_header("x-compress-hint", "on");
            Ok(response)
        },
        None => Ok(Response::from_status(404)),
    };
}

use crate::url_store::DynamoUrlStore;
/**
 * https://awslabs.github.io/aws-lambda-rust-runtime/lambda_http/
 */
use lambda_http::http::{HeaderValue, Method, StatusCode};
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use std::env;

mod url;
mod url_store;

fn response(code: StatusCode, headers: Vec<(&str, &str)>, body: &str) -> Response<String> {
    let mut builder = Response::builder()
        .status(code)
        .header("content-type", "text/plain");

    for (header, value) in headers {
        builder = builder.header(header, value);
    }

    builder.body(String::from(body)).unwrap()
}

/**
 * 짧은 URL을 받으면 긴 URL로 리다이렉트시키는 함수
 */
async fn handle_get(event: Request) -> Response<String> {
    let path = event.uri().path();
    let short_url = if path.starts_with("/") {
        &path[1..]
    } else {
        path
    };
    let store = DynamoUrlStore::new().await;
    println!("short_url = {}", short_url);
    match url_store::lengthen(&store, short_url).await {
        Some(long_url) => {
            println!("long_url={}", long_url);
            response(
                StatusCode::FOUND,
                vec![("Location", &long_url)],
                &format!("Redirect to {}", long_url),
            )
        }
        None => response(StatusCode::NOT_FOUND, vec![], "URL Not Found"),
    }
}

async fn create_short_url(long_url: &str) -> Response<String> {
    match HeaderValue::from_str(long_url.trim()) {
        Ok(long_url) => {
            let mut store = DynamoUrlStore::new().await;
            let long_url = long_url.to_str().unwrap();
            println!("long-url={}", long_url);
            let short_url = url_store::shorten(&mut store, long_url).await;
            response(StatusCode::CREATED, vec![], &short_url)
        }
        Err(_) => response(StatusCode::BAD_REQUEST, vec![], "Bad URL"),
    }
}

/**
 * 긴 URL을 받아서 짧은 URL을 만들어주는 함수
 */
async fn handle_post(event: Request) -> Response<String> {
    let auth_header_value = event
        .headers()
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("no-api-key");

    let api_key = env::var("API_KEY").unwrap_or("very-secure-api-key123".to_owned());

    let valid_key: bool = auth_header_value == format!("Bearer {}", api_key);

    match (valid_key, event.body()) {
        (false, _) => response(
            StatusCode::UNAUTHORIZED,
            vec![],
            "Valid API_KEY is required",
        ),
        (true, Body::Text(long_url)) => create_short_url(long_url).await,
        _ => response(StatusCode::BAD_REQUEST, vec![], "Request body required"),
    }
}

// This is the main body for the function.
// Write your code inside it.
// There are some code example in the following URLs:
// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples

async fn function_handler(event: Request) -> Result<Response<String>, Error> {
    let resp = match event.method() {
        &Method::GET => handle_get(event).await,
        &Method::POST => handle_post(event).await,
        _ => response(StatusCode::NOT_FOUND, vec![], "Not Found"),
    };
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}

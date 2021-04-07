use crate::database::DatabaseConnection;
use tide::{http::mime, log, Request, Response};

pub async fn handler(request: Request<DatabaseConnection>) -> tide::Result {
    log::debug!("Endpoint Found: {}", request.url().to_string());
    let body = include_str!("../../pages/index.html");
    let response = Response::builder(200)
        .body(body)
        // .header("custom-header", "value")
        .content_type(mime::HTML)
        .build();

    Ok(response)
}

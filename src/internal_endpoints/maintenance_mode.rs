use tide::{log, Request, Response, StatusCode};
use crate::database::DatabaseConnection;

pub async fn handler(request: Request<DatabaseConnection>) -> tide::Result {
    log::debug!("Request: {:#?}", request);
    // TODO
    Ok(Response::new(StatusCode::Accepted))
}
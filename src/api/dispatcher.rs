use crate::database::DatabaseConnection;
use tide::{log, Request, Response, StatusCode};

pub async fn handler(mut request: Request<DatabaseConnection>) -> tide::Result {
    // TODO: Verify authentication
    // TODO: Implement r2d2_sqlite pooling via `request.state()`
    let endpoint = request.param("endpoint")?;
    match endpoint {
        "show_users" => {
            log::info!("Found: {}", endpoint);
            let request_body = request.body_string().await?;
            crate::api::show_users::handler(request.state(), &request_body)
        }
        _ => {
            log::warn!("Not found: {}", endpoint);
            Ok(Response::new(StatusCode::NotFound))
        }
    }
}

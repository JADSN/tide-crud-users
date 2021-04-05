use tide::{log, Request, Response, StatusCode};

use crate::{
    api::{add_user::AddUser, show_users::ShowUsers},
    database::DatabaseConnection,
    endpoint::Handler,
};

pub async fn handler(mut request: Request<DatabaseConnection>) -> tide::Result {
    // TODO: Verify authentication
    let endpoint = request.param("endpoint")?;
    match endpoint {
        "show_users" => {
            log::info!("Found: {}", endpoint);
            let request_body = request.body_string().await?;
            ShowUsers::handler(ShowUsers, request.state(), &request_body)
        }
        "add_user" => {
            log::info!("Found: {}", endpoint);
            let request_body = request.body_string().await?;
            AddUser::handler(AddUser, request.state(), &request_body)
        }
        _ => {
            log::warn!("Not found: {}", endpoint);
            Ok(Response::new(StatusCode::NotFound))
        }
    }
}

// pub struct Dispatcher;

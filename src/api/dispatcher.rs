use tide::{log, Request, Response, StatusCode};

use crate::{
    api::{
        add_user::AddUser, delete_user::DeleteUser, export_users::ExportUsers,
        show_departments::ShowDepartments, show_permissions::ShowPermissions,
        show_statuses::ShowStatuses, show_user::ShowUser, show_users::ShowUsers,
        update_user::UpdateUser,
    },
    database::DatabaseConnection,
    endpoint::Handler,
};

pub async fn handler(mut request: Request<DatabaseConnection>) -> tide::Result {
    // TODO: Verify authentication
    let endpoint = request.param("endpoint")?;
    match endpoint {
        "show_users" => {
            log::info!("Endpoint found: {}", endpoint);
            let request_body = request.body_string().await?;
            ShowUsers::handler(ShowUsers, request.state(), &request_body)
        }
        "export_users" => {
            log::info!("Endpoint found: {}", endpoint);
            let request_body = request.body_string().await?;
            ExportUsers::handler(ExportUsers, request.state(), &request_body)
        }
        "show_user" => {
            log::info!("Endpoint found: {}", endpoint);
            let request_body = request.body_string().await?;
            ShowUser::handler(ShowUser, request.state(), &request_body)
        }
        "add_user" => {
            log::info!("Endpoint found: {}", endpoint);
            let request_body = request.body_string().await?;
            AddUser::handler(AddUser, request.state(), &request_body)
        }
        "update_user" => {
            log::info!("Endpoint found: {}", endpoint);
            let request_body = request.body_string().await?;
            UpdateUser::handler(UpdateUser, request.state(), &request_body)
        }
        "delete_user" => {
            log::info!("Endpoint found: {}", endpoint);
            let request_body = request.body_string().await?;
            DeleteUser::handler(DeleteUser, request.state(), &request_body)
        }
        "show_departments" => {
            log::info!("Endpoint found: {}", endpoint);
            let request_body = request.body_string().await?;
            ShowDepartments::handler(ShowDepartments, request.state(), &request_body)
        }
        "show_permissions" => {
            log::info!("Endpoint found: {}", endpoint);
            let request_body = request.body_string().await?;
            ShowPermissions::handler(ShowPermissions, request.state(), &request_body)
        }
        "show_statuses" => {
            log::info!("Endpoint found: {}", endpoint);
            let request_body = request.body_string().await?;
            ShowStatuses::handler(ShowStatuses, request.state(), &request_body)
        }
        _ => {
            log::warn!("Not found: {}", endpoint);
            Ok(Response::new(StatusCode::NotFound))
        }
    }
}

// pub struct Dispatcher;

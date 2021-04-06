mod model;
mod outcome;
mod view;

use crate::{
    api::{MvpError, MvpResult},
    database::DatabaseConnection,
    endpoint::{Endpoint, Handler, Name, Presenter},
};

use outcome::InternalMessage;

// Endpoint definition
#[derive(Debug)]
pub struct UpdateUser;

impl Endpoint for UpdateUser {}

impl Name for UpdateUser {
    fn name(&self) -> &'static str {
        module_path!()
            .split("::")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
    }
}

impl Presenter<UpdateUser, DatabaseConnection, InternalMessage, MvpError, MvpResult> for UpdateUser {}

impl Handler<UpdateUser, DatabaseConnection, InternalMessage, MvpError, MvpResult> for UpdateUser {}

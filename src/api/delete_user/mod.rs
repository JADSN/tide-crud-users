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
pub struct DeleteUser;

impl Endpoint for DeleteUser {}

impl Name for DeleteUser {
    fn name(&self) -> &'static str {
        module_path!()
            .split("::")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
    }
}

impl Presenter<DeleteUser, DatabaseConnection, InternalMessage, MvpError, MvpResult> for DeleteUser {}

impl Handler<DeleteUser, DatabaseConnection, InternalMessage, MvpError, MvpResult> for DeleteUser {}

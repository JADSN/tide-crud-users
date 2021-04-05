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
pub struct AddUser;

impl Endpoint for AddUser {}

impl Name for AddUser {
    fn name(&self) -> &'static str {
        module_path!()
            .split("::")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
    }
}

impl Presenter<AddUser, DatabaseConnection, InternalMessage, MvpError, MvpResult> for AddUser {}

impl Handler<AddUser, DatabaseConnection, InternalMessage, MvpError, MvpResult> for AddUser {}

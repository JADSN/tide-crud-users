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
pub struct ExportUsers;

impl Endpoint for ExportUsers {}

impl Name for ExportUsers {
    fn name(&self) -> &'static str {
        module_path!()
            .split("::")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
    }
}

impl Presenter<ExportUsers, DatabaseConnection, InternalMessage, MvpError, MvpResult> for ExportUsers {}

impl Handler<ExportUsers, DatabaseConnection, InternalMessage, MvpError, MvpResult> for ExportUsers {}

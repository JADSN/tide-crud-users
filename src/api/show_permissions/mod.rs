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
pub struct ShowPermissions;

impl Endpoint for ShowPermissions {}

impl Name for ShowPermissions {
    fn name(&self) -> &'static str {
        module_path!()
            .split("::")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
    }
}

impl Presenter<ShowPermissions, DatabaseConnection, InternalMessage, MvpError, MvpResult> for ShowPermissions {}

impl Handler<ShowPermissions, DatabaseConnection, InternalMessage, MvpError, MvpResult> for ShowPermissions {}

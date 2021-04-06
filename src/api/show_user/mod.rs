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
pub struct ShowUser;

impl Endpoint for ShowUser {}

impl Name for ShowUser {
    fn name(&self) -> &'static str {
        module_path!()
            .split("::")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
    }
}

impl Presenter<ShowUser, DatabaseConnection, InternalMessage, MvpError, MvpResult> for ShowUser {}

impl Handler<ShowUser, DatabaseConnection, InternalMessage, MvpError, MvpResult> for ShowUser {}

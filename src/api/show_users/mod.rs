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
pub struct ShowUsers;

impl Endpoint for ShowUsers {}

impl Name for ShowUsers {
    fn name(&self) -> &'static str {
        module_path!()
            .split("::")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
    }
}

impl Presenter<ShowUsers, DatabaseConnection, InternalMessage, MvpError, MvpResult> for ShowUsers {}

impl Handler<ShowUsers, DatabaseConnection, InternalMessage, MvpError, MvpResult> for ShowUsers {}

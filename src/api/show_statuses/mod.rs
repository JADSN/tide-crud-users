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
pub struct ShowStatuses;

impl Endpoint for ShowStatuses {}

impl Name for ShowStatuses {
    fn name(&self) -> &'static str {
        module_path!()
            .split("::")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
    }
}

impl Presenter<ShowStatuses, DatabaseConnection, InternalMessage, MvpError, MvpResult> for ShowStatuses {}

impl Handler<ShowStatuses, DatabaseConnection, InternalMessage, MvpError, MvpResult> for ShowStatuses {}

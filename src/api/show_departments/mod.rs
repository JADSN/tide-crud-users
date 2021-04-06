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
pub struct ShowDepartments;

impl Endpoint for ShowDepartments {}

impl Name for ShowDepartments {
    fn name(&self) -> &'static str {
        module_path!()
            .split("::")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
    }
}

impl Presenter<ShowDepartments, DatabaseConnection, InternalMessage, MvpError, MvpResult> for ShowDepartments {}

impl Handler<ShowDepartments, DatabaseConnection, InternalMessage, MvpError, MvpResult> for ShowDepartments {}

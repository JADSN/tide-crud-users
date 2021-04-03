mod model;
mod outcome;
mod view;

use crate::database::DatabaseConnection;
use crate::endpoint::{Endpoint, Name, Presenter};

use outcome::{InternalMessage, MyError, MyResult};

// Endpoint definition
#[derive(Debug)]
struct ShowUsers;
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

impl Presenter<ShowUsers, DatabaseConnection, InternalMessage, MyError, MyResult> for ShowUsers {}

pub fn handler(db_connection: &DatabaseConnection, request_body: &String) -> tide::Result {
    ShowUsers::presenter(ShowUsers, db_connection, request_body).get()
}

pub mod dispatcher;

// * Endpoint modules - BEGIN

mod add_user;
mod show_users;
mod export_users;
mod show_user;
mod update_user;
mod delete_user;
mod show_departments;
mod show_permissions;
mod show_statuses;

// * Endpoint modules - END

use std::num::TryFromIntError;

use tide::log;

use crate::endpoint::{EndpointError, EndpointResult};

#[derive(Debug)]
pub struct MvpResult(tide::Result);

impl Into<tide::Result> for MvpResult {
    fn into(self) -> tide::Result {
        self.0
    }
}

impl EndpointResult for MvpResult {}

#[derive(Debug)]
pub struct MvpError(tide::Error);

impl MvpError {
    pub fn take_error(self) -> tide::Error {
        self.0
    }
}

impl EndpointError for MvpError {}

impl From<rusqlite::Error> for MvpError {
    fn from(err: rusqlite::Error) -> Self {
        log::info!("Error Triggered from [rusqlite::Error]");
        MvpError(tide::Error::from_str(tide::StatusCode::Conflict, err.to_string()))
    }
}

impl From<r2d2::Error> for MvpError {
    fn from(err: r2d2::Error) -> Self {
        log::info!("Error Triggered from [r2d2::Error]");
        MvpError(tide::Error::from_str(tide::StatusCode::Conflict, err.to_string()))
    }
}

impl Into<tide::Error> for MvpError {
    fn into(self) -> tide::Error {
        tide::Error::from_str(tide::StatusCode::Conflict, self.take_error())
    }
}

impl From<TryFromIntError> for MvpError {
    fn from(err: TryFromIntError) -> Self {
        log::info!("Error Triggered from [r2d2::Error]");
        MvpError(tide::Error::from_str(tide::StatusCode::Conflict, err.to_string()))
    }
}



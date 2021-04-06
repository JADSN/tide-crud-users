pub mod dispatcher;

// * Endpoint modules - BEGIN

mod add_user;
mod show_users;
mod update_user;

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
        MvpError(tide::Error::from(err))
    }
}

impl From<r2d2::Error> for MvpError {
    fn from(err: r2d2::Error) -> Self {
        log::info!("Error Triggered from [r2d2::Error]");
        MvpError(tide::Error::from(err))
    }
}

impl Into<tide::Error> for MvpError {
    fn into(self) -> tide::Error {
        tide::Error::from_str(tide::StatusCode::InternalServerError, self.take_error())
    }
}

impl From<TryFromIntError> for MvpError {
    fn from(err: TryFromIntError) -> Self {
        log::info!("Error Triggered from [r2d2::Error]");
        MvpError(tide::Error::from(err))
    }
}



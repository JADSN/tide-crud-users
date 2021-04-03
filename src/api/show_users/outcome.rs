use std::convert::From;
use std::convert::TryFrom;

use crate::endpoint::{EndpointDbConnection, EndpointError, EndpointResult, Outcome};
// use brickpack_derive::Outcome;

use rusqlite::NO_PARAMS;
use serde::{Deserialize, Serialize};

use crate::database::DatabaseConnection;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u16,
    pub name: String,
    pub email: String,
    pub department: u16,
    pub permission: u16,
}

// Outcome definition
#[derive(Debug, Serialize)]
pub struct InternalMessage(Vec<User>);
impl Outcome for InternalMessage {}

impl TryFrom<Vec<User>> for InternalMessage {
    type Error = MyError;

    fn try_from(data: Vec<User>) -> Result<Self, Self::Error> {
        Ok(InternalMessage(data))
    }
}

impl InternalMessage {
    pub fn retrieve_users(db_connection: &DatabaseConnection) -> Result<Vec<User>, MyError> {
        let conn = db_connection.get()?;
        let mut stmt = conn.prepare("SELECT * FROM users")?;
        let retrieved_users = stmt.query_map(NO_PARAMS, |row| {
            Ok(User {
                id: row.get(0)?,
                email: row.get(1)?,
                name: row.get(2)?,
                department: row.get(3)?,
                permission: row.get(4)?,
            })
        })?;

        let mut users: Vec<User> = Vec::new();

        for row in retrieved_users {
            if let Ok(user) = row {
                users.push(user);
            }
        }

        Ok(users)
    }
}

#[derive(Debug)]
pub struct MyError(pub tide::Error);
impl EndpointError for MyError {}
impl From<rusqlite::Error> for MyError {
    fn from(err: rusqlite::Error) -> Self {
        MyError(tide::Error::from(err))
    }
}

impl From<r2d2::Error> for MyError {
    fn from(err: r2d2::Error) -> Self {
        MyError(tide::Error::from(err))
    }
}

impl EndpointDbConnection for DatabaseConnection {}
// impl From<serde::error::Error> for MyError {
//     fn from(err: rusqlite::Error) -> Self {
//         MyError(tide::Error::from(err))
//     }
// }

// #[derive(Debug)]
// pub struct MyDbConnection(pub rusqlite::Connection);
// impl EndpointDbConnection for MyDbConnection {}

// impl From<rusqlite::Connection> for MyDbConnection {
//     fn from(db_connection: rusqlite::Connection) -> Self {
//         MyDbConnection(db_connection)
//     }
// }

// impl MyDbConnection {
//     pub fn get(self) -> rusqlite::Connection {
//         self.0
//     }
// }

#[derive(Debug)]
pub struct MyResult(pub tide::Result);
impl EndpointResult for MyResult {}

impl MyResult {
    pub fn get(self) -> tide::Result {
        self.0
    }
}

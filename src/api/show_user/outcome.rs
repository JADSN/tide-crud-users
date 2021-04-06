use std::convert::TryFrom;

use rusqlite::params;
use serde::{Deserialize, Serialize};
use tide::{Error as TideError, StatusCode};

use crate::{api::MvpError, database::DatabaseConnection, endpoint::Outcome};

#[derive(Debug, Deserialize)]
pub struct UserId {
    id: u16,
}

impl UserId {
    pub fn id(&self) -> u16 {
        self.id
    }
}

#[derive(Debug, Serialize)]
pub struct User {
    pub id: u16,
    pub name: String,
    pub email: String,
    pub department: u16,
    pub permission: u16,
}

// Outcome definition
#[derive(Debug, Serialize)]
pub struct InternalMessage(User);
impl Outcome for InternalMessage {}

impl TryFrom<User> for InternalMessage {
    type Error = MvpError;

    fn try_from(data: User) -> Result<Self, Self::Error> {
        Ok(InternalMessage(data))
    }
}

impl InternalMessage {
    pub fn retrieve_user(
        db_connection: &DatabaseConnection,
        user_id: u16,
    ) -> Result<User, MvpError> {
        let conn = db_connection.get()?;
        let mut stmt = conn.prepare("SELECT * FROM users WHERE id = ?1")?;
        let mut retrieved_user = stmt.query_map(params![user_id], |row| {
            Ok(User {
                id: row.get(0)?,
                email: row.get(1)?,
                name: row.get(2)?,
                department: row.get(3)?,
                permission: row.get(4)?,
            })
        })?;
        match retrieved_user.next() {
            Some(user_result) => {
                if let Ok(user) = user_result {
                    return Ok(user);
                } else {
                    let status_code = StatusCode::BadRequest;
                    return Err(MvpError(TideError::from_str(
                        status_code,
                        status_code.to_string(),
                    )));
                }
            }
            None => {
                let status_code = StatusCode::BadRequest;
                return Err(MvpError(TideError::from_str(
                    status_code,
                    status_code.to_string(),
                )));
            }
        }
    }
}

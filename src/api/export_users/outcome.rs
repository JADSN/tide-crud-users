use std::convert::TryFrom;

use rusqlite::NO_PARAMS;
use serde::{Deserialize, Serialize};

use crate::{api::MvpError, database::DatabaseConnection, endpoint::Outcome};

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
    type Error = MvpError;

    fn try_from(data: Vec<User>) -> Result<Self, Self::Error> {
        Ok(InternalMessage(data))
    }
}

impl InternalMessage {
    pub fn retrieve_users(db_connection: &DatabaseConnection) -> Result<Vec<User>, MvpError> {
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

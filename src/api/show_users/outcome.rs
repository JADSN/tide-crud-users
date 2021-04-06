use std::convert::TryFrom;

use rusqlite::params;
use serde::{Deserialize, Serialize};

use crate::{api::MvpError, database::DatabaseConnection, endpoint::Outcome};

// Serde struct - BEGIN
#[derive(Debug, Deserialize)]
pub struct Paging {
    #[serde(default)]
    limit: PagingLimit,
    #[serde(default)]
    offset: PagingOffset,
}

#[derive(Debug, Deserialize)]
pub struct PagingLimit(u16);
impl Default for PagingLimit {
    fn default() -> Self {
        PagingLimit(20)
    }
}
impl PagingLimit {
    fn get(&self) -> u16 {
        self.0
    }
}

#[derive(Debug, Deserialize)]
pub struct PagingOffset(u16);
impl Default for PagingOffset {
    fn default() -> Self {
        PagingOffset(1)
    }
}
impl PagingOffset {
    fn get(&self) -> u16 {
        self.0
    }
}
// Serde struct - END

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
    pub fn retrieve_users(
        db_connection: &DatabaseConnection,
        paging: Paging,
    ) -> Result<Vec<User>, MvpError> {
        let conn = db_connection.get()?;
        let mut stmt = conn.prepare("SELECT * FROM users LIMIT ?1 OFFSET ?2")?;
        let retrieved_users =
            stmt.query_map(params![paging.limit.get(), paging.offset.get()], |row| {
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

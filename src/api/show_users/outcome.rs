use std::convert::TryFrom;

use rusqlite::params;
use serde::{Deserialize, Serialize};

use crate::{
    api::MvpError,
    database::DatabaseConnection,
    endpoint::Outcome,
    models::users::{UserDepartment, UserEmail, UserId, UserName, UserPermission, UserStatus},
};
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
        PagingOffset(0)
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
    id: UserId,
    name: UserName,
    email: UserEmail,
    department: UserDepartment,
    permission: UserPermission,
    status: UserStatus,
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
        let mut stmt = conn.prepare("SELECT id, email, name, department, permission, status, deleted FROM `users` WHERE deleted = 0 LIMIT ?1 OFFSET ?2;")?;
        let retrieved_users =
            stmt.query_map(params![paging.limit.get(), paging.offset.get()], |row| {
                Ok(User {
                    id: UserId::new(row.get(0)?),
                    email: UserEmail::new(row.get(1)?),
                    name: UserName::new(row.get(2)?),
                    department: UserDepartment::new(row.get(3)?),
                    permission: UserPermission::new(row.get(4)?),
                    status: UserStatus::new(row.get(5)?),
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

use std::convert::TryFrom;

use rusqlite::params;
use serde::{Deserialize, Serialize};
use tide::{Error as TideError, StatusCode};

use crate::{
    api::MvpError,
    database::DatabaseConnection,
    endpoint::Outcome,
    models::users::{UserDepartment, UserEmail, UserId, UserName, UserPermission, UserStatus},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryUserById {
    pub id: UserId,
}

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
        let mut stmt =
            conn.prepare("SELECT id, email, name, department, permission, status, deleted FROM `users` WHERE id = ?1 AND deleted = 0;")?;
        let mut retrieved_user = stmt.query_map(params![user_id], |row| {
            Ok(User {
                id: UserId::new(row.get(0)?),
                email: UserEmail::new(row.get(1)?),
                name: UserName::new(row.get(2)?),
                department: UserDepartment::new(row.get(3)?),
                permission: UserPermission::new(row.get(4)?),
                status: UserStatus::new(row.get(5)?),
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

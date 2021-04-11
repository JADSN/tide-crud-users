use std::convert::TryFrom;

use serde::{Deserialize, Serialize};

use crate::{api::MvpError, database::DatabaseConnection, endpoint::Outcome};

use crate::models::users::{
    UserDepartment, UserEmail, UserId, UserName, UserPermission, UserStatus,
};
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
    pub fn retrieve_users(db_connection: &DatabaseConnection) -> Result<Vec<User>, MvpError> {
        let conn = db_connection.get()?;
        let mut stmt = conn.prepare("SELECT id, email, name, department, permission, status, deleted FROM `users` WHERE deleted = 0;")?;
        let retrieved_users = stmt.query_map([], |row| {
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

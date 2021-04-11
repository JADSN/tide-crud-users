use std::convert::TryFrom;

use crate::{api::MvpError, database::DatabaseConnection, endpoint::Outcome};

use rusqlite::params;
use serde::{Deserialize, Serialize};

use crate::models::users::{
    UserDepartment, UserEmail, UserName, UserPermission, UserStatus,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser {
    name: UserName,
    email: UserEmail,
    department: UserDepartment,
    permission: UserPermission,
    #[serde(skip_deserializing, default)]
    status: UserStatus,
}

// Outcome definition
#[derive(Debug, Serialize)]
pub struct InternalMessage(u16);

impl InternalMessage {
    pub fn get(self) -> u16 {
        self.0
    }
}

impl Outcome for InternalMessage {}

impl TryFrom<i64> for InternalMessage {
    type Error = MvpError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        use std::convert::TryInto;
        let new_id = value.try_into()?;
        Ok(InternalMessage(new_id))
    }
}

impl InternalMessage {
    pub fn db_adduser(
        db_connection: &DatabaseConnection,
        new_user: &NewUser,
    ) -> Result<i64, MvpError> {
        let mut conn = db_connection.get()?;
        let tx = conn.transaction()?;

        tx.execute(
            "INSERT INTO users (email, name, department, permission, status, deleted ) VALUES (?1, ?2, ?3, ?4, ?5, 0)",
            params![
                new_user.email.get(),
                new_user.name.get(),
                new_user.department.get(),
                new_user.permission.get(),
                new_user.status.get()
            ],
        )?;

        let last_row_id = tx.last_insert_rowid();

        tx.commit()?;

        Ok(last_row_id)
    }
}

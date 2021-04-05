use std::convert::TryFrom;

use crate::{api::MvpError, database::DatabaseConnection, endpoint::Outcome};

use rusqlite::params;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct NewUser {
    name: String,
    email: String,
    department: u16,
    permission: u16,
}

// Outcome definition
#[derive(Debug, Serialize)]
pub struct InternalMessage(u16);

impl Outcome for InternalMessage {}

impl TryFrom<i64> for InternalMessage {
    type Error = MvpError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        use std::convert::TryInto;
        let id = value.try_into()?;
        Ok(InternalMessage(id))
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
            "INSERT INTO users (email, name, department, permission) VALUES (?1, ?2, ?3, ?4)",
            params![
                new_user.email,
                new_user.name,
                new_user.department,
                new_user.permission
            ],
        )?;

        let last_row_id = tx.last_insert_rowid();

        tx.commit()?;

        Ok(last_row_id)
    }
}

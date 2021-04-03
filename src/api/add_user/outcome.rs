use std::convert::TryFrom;

use brickpack::endpoint::Outcome;
use brickpack_derive::Outcome;

use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use tide::Error as TideError;

#[derive(Debug, Deserialize)]
pub struct NewUser {
    name: String,
    email: String,
    department: u16,
    permission: u16,
}

// Outcome definition
#[derive(Debug, Outcome, Serialize)]
pub struct InternalMessage(u16);

// impl InternalMessage {
//     pub fn get(&self) -> u16 {
//         self.0
//     }
// }

impl TryFrom<i64> for InternalMessage {
    // type Error = &'static str;
    type Error = TideError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        use std::convert::TryInto;
        let id = value.try_into()?;
        Ok(InternalMessage(id))
    }
}

impl InternalMessage {
    pub fn db_adduser(mut conn: Connection, new_user: &NewUser) -> rusqlite::Result<i64> {
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

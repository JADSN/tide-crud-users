use crate::{api::MvpError, database::DatabaseConnection, endpoint::Outcome};

use rusqlite::{params, Transaction};
use serde::{Deserialize, Serialize};
use tide::log;

#[derive(Debug, Deserialize)]
pub struct UserId {
    id: u16,
}

impl UserId {
    pub fn get(&self) -> u16 {
        self.id
    }
}

// Outcome definition
#[derive(Debug, Serialize, Default)]
pub struct InternalMessage {
    deleted_rows: u16,
}

impl From<u16> for InternalMessage {
    fn from(data: u16) -> Self {
        InternalMessage {
            deleted_rows: data,
        }
    }
}
impl Outcome for InternalMessage {}

impl InternalMessage {
    pub fn db_delete_user(
        db_connection: &DatabaseConnection,
        user_id: &UserId,
    ) -> Result<u16, MvpError> {
        use std::convert::TryInto;
        let mut conn = db_connection.get()?;
        let tx = conn.transaction()?;
        let id = user_id.get();

        if db_check_user(&tx, id)? > 0 {
            log::debug!("Deleting row with id = {}", id);
            let affected_rows =
                tx.execute("UPDATE `users` SET deleted = 1 WHERE id = ?1;", params![id])?;
            let affected_rows: u16 = affected_rows.try_into()?;
            tx.commit()?;
            Ok(affected_rows)
        } else {
            tx.commit()?;
            Err(MvpError(tide::Error::from_str(
                tide::StatusCode::BadRequest,
                "User id not found!",
            )))
        }
    }
}

fn db_check_user(tx: &Transaction, id: u16) -> Result<u16, MvpError> {
    let mut stmt = tx.prepare("SELECT COUNT(*) FROM `users` WHERE id = ?1 AND deleted = 0;")?;

    let mut found = stmt.query_map(params![id], |row| {
        let rows_found: u16 = row.get(0)?;
        Ok(rows_found)
    })?;

    if let Some(found_result) = found.next() {
        let rows = found_result?;
        Ok(rows)
    } else {
        Ok(0)
    }
}

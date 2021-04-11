use crate::{api::MvpError, database::DatabaseConnection, endpoint::Outcome};

use rusqlite::{params, Transaction};
use serde::{Deserialize, Serialize};
use tide::log;

use crate::models::users::UserId;

#[derive(Debug, Deserialize)]
pub struct User {
    id: UserId,
}

// Outcome definition
#[derive(Debug, Serialize, Default)]
pub struct InternalMessage {
    deleted_rows: u16,
}

impl From<u16> for InternalMessage {
    fn from(data: u16) -> Self {
        InternalMessage { deleted_rows: data }
    }
}
impl Outcome for InternalMessage {}

impl InternalMessage {
    pub fn db_delete_user(
        db_connection: &DatabaseConnection,
        user: &User,
    ) -> Result<u16, MvpError> {
        use std::convert::TryInto;
        let mut conn = db_connection.get()?;
        let tx = conn.transaction()?;
        let id = user.id.get();

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

    let found = stmt.query_row(params![id], |r| r.get::<_, u16>(0));

    match found {
        Ok(found) => Ok(found),
        Err(error) => {
            tide::log::warn!("{}", error);
            Ok(0)
        }
    }
}

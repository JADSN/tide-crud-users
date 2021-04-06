use crate::{api::MvpError, database::DatabaseConnection, endpoint::Outcome};

use rusqlite::{params, Transaction};
use serde::{Deserialize, Serialize};
use tide::log;

#[derive(Debug, Deserialize)]
pub struct ParsedUser {
    id: u16,
    name: Option<String>,
    email: Option<String>,
    department: Option<u16>,
    permission: Option<u16>,
    status: Option<u16>,
}

impl ParsedUser {
    pub fn id(&self) -> u16 {
        self.id
    }
    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }
    pub fn email(&self) -> Option<String> {
        self.email.clone()
    }
    pub fn department(&self) -> Option<u16> {
        self.department
    }
    pub fn permission(&self) -> Option<u16> {
        self.permission
    }
    pub fn status(&self) -> Option<u16> {
        self.status
    }
}

// Outcome definition
#[derive(Debug, Serialize, Default)]
pub struct InternalMessage {
    updated_fields: u16,
}

impl From<u16> for InternalMessage {
    fn from(data: u16) -> Self {
        InternalMessage {
            updated_fields: data,
        }
    }
}
impl Outcome for InternalMessage {}

impl InternalMessage {
    pub fn db_adduser(
        db_connection: &DatabaseConnection,
        parsed_user: &ParsedUser,
    ) -> Result<u16, MvpError> {
        let mut conn = db_connection.get()?;
        let id = parsed_user.id();
        let mut affected_fields: u16 = 0;

        let tx = conn.transaction()?;
        if db_check_user(&tx, id)? > 0 {
            if let Some(email) = parsed_user.email() {
                let id = parsed_user.id();
                log::debug!("Updating field: id = {} email = {}", id, &email);
                tx.execute(
                    "UPDATE `users` SET email = ?2 WHERE id = ?1;",
                    params![id, email],
                )?;
                affected_fields += 1;
            }

            if let Some(name) = parsed_user.name() {
                let id = parsed_user.id();
                log::debug!("Updating field: id = {} name = {}", id, &name);
                tx.execute(
                    "UPDATE `users` SET name = ?2 WHERE id = ?1;",
                    params![id, name],
                )?;
                affected_fields += 1;
            }

            if let Some(department) = parsed_user.department() {
                let id = parsed_user.id();
                log::debug!("Updating field: id = {} department = {}", id, &department);
                tx.execute(
                    "UPDATE `users` SET department = ?2 WHERE id = ?1;",
                    params![parsed_user.id(), department],
                )?;
                affected_fields += 1;
            }

            if let Some(permission) = parsed_user.permission() {
                let id = parsed_user.id();
                log::debug!("Updating field: id = {} permission = {}", id, &permission);
                tx.execute(
                    "UPDATE `users` SET permission = ?2 WHERE id = ?1;",
                    params![parsed_user.id(), permission],
                )?;
                affected_fields += 1;
            }

            if let Some(status) = parsed_user.status() {
                let id = parsed_user.id();
                log::debug!("Updating field: id = {} status = {}", id, &status);
                tx.execute(
                    "UPDATE `users` SET status = ?2 WHERE id = ?1;",
                    params![parsed_user.id(), status],
                )?;
                affected_fields += 1;
            }

            tx.commit()?;
            Ok(affected_fields)
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

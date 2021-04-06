use std::convert::TryFrom;

use crate::{api::MvpError, database::DatabaseConnection, endpoint::Outcome};

use rusqlite::params;
use serde::{Deserialize, Serialize};
use tide::log;

#[derive(Debug, Deserialize)]
pub struct ParsedUser {
    id: u16,
    name: Option<String>,
    email: Option<String>,
    department: Option<u16>,
    permission: Option<u16>,
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
        parsed_user: &ParsedUser,
    ) -> Result<i64, MvpError> {
        let mut conn = db_connection.get()?;

        let mut affected_fields = 0;

        let tx = conn.transaction()?;

        // tx.execute(
        //     "INSERT INTO users (email, name, department, permission) VALUES (?1, ?2, ?3, ?4)",
        //     params![
        //         new_user.email,
        //         new_user.name,
        //         new_user.department,
        //         new_user.permission
        //     ],
        // )?;

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

        tx.commit()?;

        Ok(affected_fields)
    }
}

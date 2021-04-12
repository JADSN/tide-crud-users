use crate::{api::MvpError, database::DatabaseConnection, endpoint::Outcome};

use crate::models::users::{
    UserDepartment, UserEmail, UserId, UserName, UserPermission, UserStatus,
};
use rusqlite::{params, Transaction};
use serde::{Deserialize, Serialize};
use tide::log;

#[derive(Debug, Deserialize)]
pub struct ParsedUser {
    id: UserId,
    name: Option<UserName>,
    email: Option<UserEmail>,
    department: Option<UserDepartment>,
    permission: Option<UserPermission>,
    status: Option<UserStatus>,
}

impl ParsedUser {
    pub fn id(&self) -> UserId {
        self.id.clone()
    }
    pub fn name(&self) -> Option<UserName> {
        self.name.clone()
    }
    pub fn email(&self) -> Option<UserEmail> {
        self.email.clone()
    }
    pub fn department(&self) -> Option<UserDepartment> {
        self.department.clone()
    }
    pub fn permission(&self) -> Option<UserPermission> {
        self.permission.clone()
    }
    pub fn status(&self) -> Option<UserStatus> {
        self.status.clone()
    }
}

// Outcome definition
#[derive(Debug, Serialize, Default)]
pub struct InternalMessage {
    updated_fields: u8,
}

impl From<u8> for InternalMessage {
    fn from(data: u8) -> Self {
        InternalMessage {
            updated_fields: data,
        }
    }
}
impl Outcome for InternalMessage {}

impl InternalMessage {
    pub fn db_updateuser(
        db_connection: &DatabaseConnection,
        parsed_user: &ParsedUser,
    ) -> Result<u8, MvpError> {
        let mut conn = db_connection.get()?;
        let id = parsed_user.id().get();
        let mut affected_fields: u8 = 0;

        let tx = conn.transaction()?;
        // * Check if user exists
        if db_check_user(&tx, id)? > 0 {
            if let Some(email) = parsed_user.email() {
                let id = parsed_user.id().get();
                let email = email.get();
                log::debug!("Updating field: id = {} email = {}", id, &email);
                tx.execute(
                    "UPDATE `users` SET email = ?2 WHERE id = ?1;",
                    params![id, email],
                )?;
                affected_fields += 1;
            }

            if let Some(name) = parsed_user.name() {
                let id = parsed_user.id().get();
                let name = name.get();
                log::debug!("Updating field: id = {} name = {}", id, &name);
                tx.execute(
                    "UPDATE `users` SET name = ?2 WHERE id = ?1;",
                    params![id, name],
                )?;
                affected_fields += 1;
            }

            if let Some(department) = parsed_user.department() {
                let id = parsed_user.id().get();
                let department = department.get();
                log::debug!("Updating field: id = {} department = {}", id, &department);
                tx.execute(
                    "UPDATE `users` SET department = ?2 WHERE id = ?1;",
                    params![id, department],
                )?;
                affected_fields += 1;
            }

            if let Some(permission) = parsed_user.permission() {
                let id = parsed_user.id().get();
                let permission = permission.get();
                log::debug!("Updating field: id = {} permission = {}", id, &permission);
                tx.execute(
                    "UPDATE `users` SET permission = ?2 WHERE id = ?1;",
                    params![id, permission],
                )?;
                affected_fields += 1;
            }

            if let Some(status) = parsed_user.status() {
                let id = parsed_user.id().get();
                let status = status.get();
                log::debug!("Updating field: id = {} status = {}", id, &status);
                tx.execute(
                    "UPDATE `users` SET status = ?2 WHERE id = ?1;",
                    params![id, status],
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
    Ok(stmt.query_row(params![id], |r| r.get::<_, u16>(0))?)
}

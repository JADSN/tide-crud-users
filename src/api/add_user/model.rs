use tide::Error as TideError;

use crate::api::MvpError;
use crate::database::DatabaseConnection;
use crate::endpoint::Model;

use super::{outcome::InternalMessage, AddUser};

impl Model<DatabaseConnection, InternalMessage, MvpError> for AddUser {
    fn model(
        &self,
        db_connection: &DatabaseConnection,
        request_body: &String,
    ) -> Result<InternalMessage, MvpError> {
        use super::outcome::NewUser;
        use crate::endpoint::Name;
        use std::convert::TryFrom;
        use tide::{log, StatusCode};

        let new_user: NewUser = match serde_json::from_str(&request_body) {
            Ok(new_user) => new_user,
            Err(_) => {
                // * Security: Custom error to verify if system is under attack.
                let status_code = StatusCode::BadRequest;
                log::warn!("VIOLATION: Endpoint /{} is under attack!", self.name());
                return Err(MvpError(TideError::from_str(status_code, status_code.to_string())))
            }
        };

        let last_rowid = InternalMessage::db_adduser(db_connection, &new_user)?;
        log::info!("User created with id: {:?}", &last_rowid);
        InternalMessage::try_from(last_rowid)
    }
}

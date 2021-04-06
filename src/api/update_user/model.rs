use tide::Error as TideError;

use crate::api::MvpError;
use crate::database::DatabaseConnection;
use crate::endpoint::Model;

use super::{outcome::InternalMessage, UpdateUser};

impl Model<DatabaseConnection, InternalMessage, MvpError> for UpdateUser {
    fn model(
        &self,
        db_connection: &DatabaseConnection,
        request_body: &String,
    ) -> Result<InternalMessage, MvpError> {
        use super::outcome::ParsedUser;
        use crate::endpoint::Name;
        use std::convert::TryFrom;
        use tide::{log, StatusCode};

        let parsed_user: ParsedUser = match serde_json::from_str(&request_body) {
            Ok(parsed_user) => parsed_user,
            Err(_) => {
                // * Security: Custom error to verify if system is under attack.
                let status_code = StatusCode::BadRequest;
                log::warn!("VIOLATION: Endpoint /{} is under attack!", self.name());
                return Err(MvpError(TideError::from_str(
                    status_code,
                    status_code.to_string(),
                )));
            }
        };

        dbg!(&parsed_user);

        let last_rowid = InternalMessage::db_adduser(db_connection, &parsed_user)?;
        log::info!("User created with id: {:?}", &last_rowid);
        InternalMessage::try_from(last_rowid)
    }
}

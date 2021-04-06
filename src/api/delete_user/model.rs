use tide::Error as TideError;

use crate::api::MvpError;
use crate::database::DatabaseConnection;
use crate::endpoint::Model;

use super::{outcome::InternalMessage, DeleteUser};

impl Model<DatabaseConnection, InternalMessage, MvpError> for DeleteUser {
    fn model(
        &self,
        db_connection: &DatabaseConnection,
        request_body: &String,
    ) -> Result<InternalMessage, MvpError> {
        use super::outcome::UserId;
        use crate::endpoint::Name;
        use tide::{log, StatusCode};

        let parsed_userid: UserId = match serde_json::from_str(&request_body) {
            Ok(parsed_userid) => parsed_userid,
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

        let last_rowid = InternalMessage::db_delete_user(db_connection, &parsed_userid)?;
        log::info!("User deleted with id: {:?}", &last_rowid);
        Ok(InternalMessage::from(last_rowid))
    }
}

use tide::Error as TideError;

use brickpack::endpoint::Model;

use super::{outcome::InternalMessage, AddUser};

impl Model<InternalMessage, TideError> for AddUser {
    fn model(&self, request_body: String) -> Result<InternalMessage, TideError> {
        use super::outcome::NewUser;
        use brickpack::endpoint::Name;
        use rusqlite::Connection;
        use std::convert::TryFrom;
        use tide::{log, StatusCode};

        let new_user: NewUser = match serde_json::from_str(&request_body) {
            Ok(new_user) => new_user,
            Err(_) => {
                // * Security: Custom error to verify if system is under attack.
                let status_code = StatusCode::BadRequest;
                log::warn!("VIOLATION: Endpoint /{} is under attack!", self.name());
                return Err(TideError::from_str(status_code, status_code.to_string()));
            }
        };

        let conn = Connection::open("database.sqlite3")?;
        let last_rowid = InternalMessage::db_adduser(conn, &new_user)?;
        log::info!("User created with id: {:?}", &last_rowid);
        InternalMessage::try_from(last_rowid)
    }
}

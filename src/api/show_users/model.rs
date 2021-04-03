use tide::Error as TideError;

use crate::endpoint::Model;

use crate::database::DatabaseConnection;

use super::{
    outcome::{InternalMessage, MyError},
    ShowUsers,
};


impl Model<DatabaseConnection, InternalMessage, MyError> for ShowUsers {
    fn model(
        &self,
        db_connection: &DatabaseConnection,
        request_body: &String,
    ) -> Result<InternalMessage, MyError> {
        use crate::endpoint::Name;
        use std::convert::TryFrom;
        use tide::{log, StatusCode};
        match request_body.as_str() {
            "{}" => {
                let retrieved_users = InternalMessage::retrieve_users(db_connection)?;
                log::info!("Found [{}] users!", retrieved_users.len());
                InternalMessage::try_from(retrieved_users)
            }
            _ => {
                // * Security: Custom error to verify if system is under attack.
                let status_code = StatusCode::BadRequest;
                log::warn!("VIOLATION: Endpoint /{} is under attack!", self.name());
                Err(MyError(TideError::from_str(
                    status_code,
                    status_code.to_string(),
                )))
            }
        }
    }
}

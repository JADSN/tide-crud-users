use tide::Error as TideError;

use crate::{api::MvpError, database::DatabaseConnection, endpoint::Model};

use super::{
    outcome::{InternalMessage, QueryUserById},
    ShowUser,
};

impl Model<DatabaseConnection, InternalMessage, MvpError> for ShowUser {
    fn model(
        &self,
        db_connection: &DatabaseConnection,
        request_body: &String,
    ) -> Result<InternalMessage, MvpError> {
        use crate::endpoint::Name;
        use std::convert::TryFrom;
        use tide::{log, StatusCode};
        let user: QueryUserById = match serde_json::from_str(&request_body) {
            Ok(user) => user,
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
        let retrieved_user = InternalMessage::retrieve_user(db_connection, user.id.get())?;
        log::info!("Found 1 user!");
        InternalMessage::try_from(retrieved_user)
    }
}

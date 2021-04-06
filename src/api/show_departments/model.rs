use tide::Error as TideError;

use crate::api::MvpError;
use crate::database::DatabaseConnection;
use crate::endpoint::Model;

use super::{
    outcome::{InternalMessage, Paging},
    ShowDepartments,
};

impl Model<DatabaseConnection, InternalMessage, MvpError> for ShowDepartments {
    fn model(
        &self,
        db_connection: &DatabaseConnection,
        request_body: &String,
    ) -> Result<InternalMessage, MvpError> {
        use crate::endpoint::Name;
        use std::convert::TryFrom;
        use tide::{log, StatusCode};
        let paging: Paging = match serde_json::from_str(&request_body) {
            Ok(paging_settings) => paging_settings,
            Err(error) => {
                // * Security: Custom error to verify if system is under attack.
                let status_code = StatusCode::BadRequest;
                log::warn!("VIOLATION: Endpoint /{} is under attack!", self.name());
                log::error!("DEBUG_SERDE: {} ", error);
                return Err(MvpError(TideError::from_str(
                    status_code,
                    status_code.to_string(),
                )));
            }
        };
        let retrieved_departments = InternalMessage::retrieve_departments(db_connection,paging)?;
        log::info!("Found [{}] departments!", retrieved_departments.len());
        InternalMessage::try_from(retrieved_departments)
    }
}

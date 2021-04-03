use super::{outcome::InternalMessage, AddUser};

use brickpack::endpoint::{Name, View};

use serde_json::to_string as serde_json_to_string;
use tide::{
    http::mime, log, prelude::Serialize, Error as TideError, Response, Result as TideResult,
    StatusCode,
};

#[derive(Serialize)]
struct ResponseBodyOk {
    id: InternalMessage,
}

impl View<InternalMessage, TideError, TideResult> for AddUser {
    fn view(&self, result: Result<InternalMessage, TideError>) -> TideResult {
        match result {
            Ok(outcome) => {
                let response_body = ResponseBodyOk { id: outcome };
                let json_body = serde_json_to_string(&response_body).unwrap_or("".to_owned());
                Ok(Response::builder(StatusCode::Ok)
                    .content_type(mime::JSON)
                    .body(json_body)
                    .build())
            }
            Err(error) => {
                log::error!(r#"Endpoint [{}]: {}"#, self.name(), error);
                Err(TideError::from(error))
            }
        }
    }
}

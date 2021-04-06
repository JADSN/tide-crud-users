use serde_json::to_string as serde_json_to_string;
use tide::{http::mime, log, prelude::Serialize, Error as TideError, Response, StatusCode};

use crate::{
    api::{MvpError, MvpResult},
    endpoint::{Name, View},
};

use super::{outcome::InternalMessage, AddUser};
#[derive(Serialize)]
struct ResponseBodyOk {
    id: u16,
}
impl From<InternalMessage> for ResponseBodyOk {
    fn from(data: InternalMessage) -> Self {
        ResponseBodyOk {
            id: data.get(),
        }
    }
}
#[derive(Serialize)]
struct ResponseBodyError {
    status: String,
    description: String,
}

impl View<InternalMessage, MvpError, MvpResult> for AddUser {
    fn view(&self, result: Result<InternalMessage, MvpError>) -> MvpResult {
        match result {
            Ok(outcome) => {
                let json_body = serde_json_to_string(&ResponseBodyOk::from(outcome)).unwrap_or("".to_owned());
                MvpResult(Ok(Response::builder(StatusCode::Ok)
                    .content_type(mime::JSON)
                    .body(json_body)
                    .build()))
            }
            Err(error) => {
                log::error!(r#"Endpoint [{}]: {:?}"#, self.name(), error);
                MvpResult(Err(TideError::from(error.take_error())))
            }
        }
    }
}

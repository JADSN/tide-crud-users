use super::{
    outcome::{InternalMessage, MyError, MyResult},
    ShowUsers,
};

use crate::endpoint::{Name, View};

use serde_json::to_string as serde_json_to_string;
use tide::{http::mime, log, prelude::Serialize, Error as TideError, Response, StatusCode};

#[derive(Serialize)]
struct ResponseBodyError {
    status: String,
    description: String,
}

impl View<InternalMessage, MyError, MyResult> for ShowUsers {
    fn view(&self, result: Result<InternalMessage, MyError>) -> MyResult {
        match result {
            Ok(outcome) => {
                let json_body = serde_json_to_string(&outcome).unwrap_or("".to_owned());
                MyResult(Ok(Response::builder(StatusCode::Ok)
                    .content_type(mime::JSON)
                    .body(json_body)
                    .build()))
            }
            Err(error) => {
                log::error!(r#"Endpoint [{}]: {:?}"#, self.name(), error);
                MyResult(Err(TideError::from(error.0)))
            }
        }
    }
}

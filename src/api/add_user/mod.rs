mod model;
mod outcome;
mod view;

use tide::{Error as TideError, Result as TideResult};

use brickpack::{
    build_presenter,
    endpoint::{Endpoint, Name, Presenter},
};

use brickpack_derive::Endpoint;

use outcome::InternalMessage;

// TODO: Verify all pathway `src/api/MODULE_NAME/` using module
// let path = module_path!();
// println!("{}", path);

// Endpoint definition
#[derive(Debug, Endpoint)]
#[endpoint_name = "add_user"]
struct AddUser;

build_presenter!(AddUser, InternalMessage, TideError, TideResult);

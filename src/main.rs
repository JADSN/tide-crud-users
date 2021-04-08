#![warn(clippy::all)]

mod api;
mod app;
mod database;
mod endpoint;
mod frontend_endpoints;
mod internal_endpoints;
mod web_server;

use app::App;
use clap::{crate_authors, crate_description, App as ClapApp, Arg as ClapArg};

use tide::log;

struct MyApp;
impl App for MyApp {
    fn name(&self) -> &'static str {
        env!("CARGO_PKG_NAME")
    }
    fn version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let clap_matches = ClapApp::new(MyApp.name())
        .version(MyApp.version())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            ClapArg::with_name("ENDPOINTS")
                .short("e")
                .long("endpoints")
                .value_name("FILE")
                .help("Show endpoint names")
                .takes_value(false),
        )
        .get_matches();

    if clap_matches.args.contains_key("ENDPOINTS") {
        show_endpoints();
        Ok(())
    } else {
        log::info!("Starting App [{} v{}]:", MyApp.name(), MyApp.version());
        web_server::start().await
    }
}

// TODO: User Journey Map (https://uxplanet.org/a-beginners-guide-to-user-journey-mapping-bd914f4c517c)

fn show_endpoints() {
    let endpoints_header = r#"
  Internal Endpoints:
    /                - index_page
    /maintenance     - maintenance
    /auth            - check_auth
  
  Endpoints:"#;
    println!("{}", endpoints_header);
    println!(include_str!("../endpoints_found.txt"));
}



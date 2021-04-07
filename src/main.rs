#![warn(clippy::all)]

mod api;
mod app;
mod database;
mod endpoint;
mod frontend_endpoints;
mod internal_endpoints;

use app::App;
use clap::{crate_authors, crate_description, App as ClapApp, Arg as ClapArg};
use log::LevelFilter;
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
        start_tide_server().await
    }
}

// TODO: User Journey Map (https://uxplanet.org/a-beginners-guide-to-user-journey-mapping-bd914f4c517c)

fn show_endpoints() {
    let banner_listen = r#"
  Internal Endpoints:
    /                - index_page
    /maintenance     - maintenance
    /auth            - check_auth
  
  Endpoints:"#;
    //   banner_listen::banner_listen()
    println!("{}", banner_listen);
    // TODO: Implement using build.rs
    println!(include_str!("../banner_listen.txt"));
}

async fn start_tide_server() -> tide::Result<()> {
    #[cfg(debug_assertions)]
    let addr = "127.0.0.1";
    #[cfg(not(debug_assertions))]
    let addr = "0.0.0.0";
    let mut port: String = "8080".into();

    if let Ok(value) = std::env::var("PORT") {
        let port_number = value.parse::<u16>()?;
        port = port_number.to_string();
    }

    let listen = format!("{}:{}", addr, port);

    start_tide_log();

    log::info!("Starting App [{} v{}]:", MyApp.name(), MyApp.version());

    let database_connection = match database::DatabaseConnection::new() {
        Ok(db_conn) => db_conn,
        Err(error) => return Err(error.into()),
    };

    let mut app = tide::with_state(database_connection);

    // * Frontend - BEGIN
    app.at("/favicon.ico")
        .get(crate::frontend_endpoints::favicon::handler);

    app.at("/")
        .get(crate::frontend_endpoints::index_page::handler);

    app.at("/js/main.js")
        .get(crate::frontend_endpoints::main_js::handler);

    app.at("/css/style.css")
        .get(crate::frontend_endpoints::style_css::handler);

    app.at("/css/uikit.min.css")
        .get(crate::frontend_endpoints::uikit_css::handler);

    app.at("/js/uikit-icons.min.js")
        .get(crate::frontend_endpoints::uikit_icons_min_js::handler);

    app.at("/js/uikit.min.js")
        .get(crate::frontend_endpoints::uikit_js::handler);

    // * Frontend - END

    app.at("/maintenance")
        .patch(crate::internal_endpoints::maintenance_mode::handler);
    app.at("/auth")
        .get(crate::internal_endpoints::check_auth::handler); // TODO: JWT Logon
    app.at("/api/:endpoint")
        .post(crate::api::dispatcher::handler);

    app.listen(listen).await?;
    Ok(())
}

fn start_tide_log() {
    if let Ok(value) = std::env::var("LOG_LEVEL") {
        let loglevel = match value.as_str() {
            "DEBUG" => LevelFilter::Debug,
            "ERROR" => LevelFilter::Error,
            "INFO" => LevelFilter::Info,
            "TRACE" => LevelFilter::Trace,
            "WARN" => LevelFilter::Warn,
            _ => LevelFilter::Off,
        };
        log::with_level(loglevel);
    } else {
        log::start();
    }
}

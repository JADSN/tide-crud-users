#![warn(clippy::all)]

mod api;
mod app;
mod database;
mod endpoint;
mod internal_endpoints;

use app::App;
use clap::{crate_authors, crate_description, App as ClapApp};
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
    ClapApp::new(MyApp.name())
        .version(MyApp.version())
        .author(crate_authors!())
        .about(crate_description!())
        .get_matches();

    let addr = "127.0.0.1";
    let port = "8080";
    let listen = format!("{}:{}", addr, port);

    log::start();

    log::info!("Starting App [{} v{}]:", MyApp.name(), MyApp.version());
    log::info!(
        "Powered by {} v{}",
        MyApp.powered_desc(),
        MyApp.powered_ver()
    );

    let database_connection = match database::DatabaseConnection::new() {
        Ok(db_conn) => db_conn,
        Err(error) => return Err(error.into()),
    };

    let mut app = tide::with_state(database_connection);

    app.at("/")
        .get(crate::internal_endpoints::index_page::handler);
    app.at("/maintenance")
        .patch(crate::internal_endpoints::maintenance_mode::handler);
    app.at("/auth")
        .get(crate::internal_endpoints::check_auth::handler); // TODO: JWT Logon
    app.at("/api/:endpoint")
        .post(crate::api::dispatcher::handler);

    #[cfg(debug_assertions)]
    println!(
        r#"
  Internal Endpoints:
    /                - index_page
    /maintenance     - maintenance
    /auth            - check_auth
  
  Endpoints:
    /api/add_user    - Users: Create
    /api/show_users  - Users: Read all
    /api/del_user    - Users: Delete
"#
    );

    app.listen(listen).await?;
    Ok(())
}

// TODO: User Journey Map (https://uxplanet.org/a-beginners-guide-to-user-journey-mapping-bd914f4c517c)

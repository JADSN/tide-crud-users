pub type SqlitePool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub type SqlitePooledConnection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;
#[derive(Debug, Clone)]
pub struct DatabaseConnection(SqlitePool);

impl DatabaseConnection {
    fn bootstrap(&self) -> Result<(), DbError> {
        use std::time::Duration;
        let conn = self.get()?;
        conn.busy_timeout(Duration::from_secs(5))?;

        conn.execute_batch("PRAGMA journal_mode=WAL")?;
        conn.execute_batch("PRAGMA foreign_keys=ON")?;
        if let Err(error) = conn.execute_batch("SELECT * from `users`") {
            tide::log::warn!("{}", error);
            
            tide::log::warn!("Creating table `users`");
            conn.execute_batch(
        r#"
        CREATE TABLE `users` (
            "id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
            "email"	TEXT NOT NULL,
            "name"	TEXT NOT NULL,
            "department"	INTEGER NOT NULL,
            "permission"	INTEGER NOT NULL
        );
        INSERT INTO `users` ("email","name","department","permission") VALUES ('root@example.net','Charlie Root',1,1);
        INSERT INTO `users` ("email","name","department","permission") VALUES ('admin@example.net','Administrator',1,2);
        INSERT INTO `users` ("email","name","department","permission") VALUES ('staff@example.net','Staff',1,3);
    "#,
    )?;
        }

        // TODO: Check if PRAGMAs was applied
        assert!(conn.is_autocommit());
        Ok(())
    }

    pub fn new() -> Result<DatabaseConnection, DbError> {
        const DB_FILE: &str = "database.sqlite3";
        const MAX_THREADS: u32 = 1;

        let manager = r2d2_sqlite::SqliteConnectionManager::file(DB_FILE);
        // let manager = r2d2_sqlite::SqliteConnectionManager::memory();
        let db_conn =
            DatabaseConnection(r2d2::Pool::builder().max_size(MAX_THREADS).build(manager)?);
        db_conn.bootstrap()?;
        Ok(db_conn)
    }

    pub fn get(&self) -> Result<SqlitePooledConnection, r2d2::Error> {
        self.0.get()
    }
}

pub struct DbError(String);

impl From<r2d2::Error> for DbError {
    fn from(error: r2d2::Error) -> Self {
        DbError(error.to_string())
    }
}

impl From<rusqlite::Error> for DbError {
    fn from(error: rusqlite::Error) -> Self {
        DbError(error.to_string())
    }
}

impl Into<tide::Error> for DbError {
    fn into(self) -> tide::Error {
        tide::Error::from_str(tide::StatusCode::InternalServerError, self.get())
    }
}

impl DbError {
    pub fn get(self) -> String {
        self.0
    }
}

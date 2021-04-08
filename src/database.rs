use crate::endpoint::EndpointDbConnection;

pub type SqlitePool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub type SqlitePooledConnection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;
#[derive(Debug, Clone)]
pub struct DatabaseConnection(SqlitePool);

impl EndpointDbConnection for DatabaseConnection {}

impl DatabaseConnection {
    fn bootstrap(&self) -> Result<(), DbError> {
        use std::time::Duration;
        let conn = self.get()?;
        conn.busy_timeout(Duration::from_secs(5))?;

        // conn.execute_batch("PRAGMA journal_mode=WAL")?;
        // conn.execute_batch("PRAGMA foreign_keys=ON")?;

        if let Err(error) = conn.execute_batch("SELECT COUNT(*) from `departments`") {
            tide::log::warn!("{}", error);
            tide::log::warn!("Creating tables `departments`");
            conn.execute_batch(
                r#"
        CREATE TABLE `departments` (
            "id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
            "name"	TEXT NOT NULL,
            "deleted" BOOLEAN NOT NULL
        );
        INSERT INTO `departments` ("id","name","deleted") VALUES (1,'IT',0);
        INSERT INTO `departments` ("id","name","deleted") VALUES (2,'Accounting',0);
        INSERT INTO `departments` ("id","name","deleted") VALUES (3,'Marketing',0);
        "#,
            )?;
        }
        if let Err(error) = conn.execute_batch("SELECT COUNT(*) from `permissions`") {
            tide::log::warn!("{}", error);
            tide::log::warn!("Creating tables `permissions`");
            conn.execute_batch(
                r#"
        CREATE TABLE `permissions` (
            "id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
            "name"	TEXT NOT NULL,
            "deleted" BOOLEAN NOT NULL
        );
        INSERT INTO `permissions` ("id","name","deleted") VALUES (1,'Administrator',0);
        INSERT INTO `permissions` ("id","name","deleted") VALUES (2,'Technical',0);
        INSERT INTO `permissions` ("id","name","deleted") VALUES (3,'User',0);  
        "#,
            )?;
        }
        if let Err(error) = conn.execute_batch("SELECT COUNT(*) from `statuses`") {
            tide::log::warn!("{}", error);
            tide::log::warn!("Creating tables `statuses`");
            conn.execute_batch(
                r#"
        CREATE TABLE `statuses` (
            "id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
            "name"	TEXT NOT NULL,
            "deleted" BOOLEAN NOT NULL
        );
        INSERT INTO `statuses` ("id","name","deleted") VALUES (1,'Enabled',0);
        INSERT INTO `statuses` ("id","name","deleted") VALUES (2,'Disabled',0);
        INSERT INTO `statuses` ("id","name","deleted") VALUES (3,'Blocked',0);  
    "#,
            )?;
        }

        if let Err(error) = conn.execute_batch("SELECT COUNT(*) from `users`") {
            tide::log::warn!("{}", error);
            tide::log::warn!("Creating tables `users`");
            conn.execute_batch(
        r#"
        CREATE TABLE `users` (
            "id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
            "email"	TEXT NOT NULL,
            "name"	TEXT NOT NULL,
            "department"	INTEGER NOT NULL,
            "permission"	INTEGER NOT NULL,
            "status"	INTEGER NOT NULL,
            "deleted" BOOLEAN NOT NULL,
			FOREIGN KEY(department) REFERENCES departments(id)
			FOREIGN KEY(permission) REFERENCES permissions(id)
			FOREIGN KEY(status) REFERENCES statuses(id)
        );
        INSERT INTO `users` ("email","name","department","permission","status","deleted") VALUES ('root@example.net','Charlie Root',1,1,1,0);
        INSERT INTO `users` ("email","name","department","permission","status","deleted") VALUES ('admin@example.net','Administrator',1,2,1,0);
        INSERT INTO `users` ("email","name","department","permission","status","deleted") VALUES ('staff@example.net','Staff',1,3,1,0);
        "#)?;
        }

        // TODO: Check if PRAGMAs was applied
        assert!(conn.is_autocommit());
        Ok(())
    }

    pub fn new() -> Result<DatabaseConnection, DbError> {
        const DB_FILE: &str = "database.sqlite3";
        const MAX_THREADS: u32 = 1;

        let manager = r2d2_sqlite::SqliteConnectionManager::file(DB_FILE).with_init(|c| {
            c.execute_batch("PRAGMA journal_mode=WAL")?;
            c.execute_batch("PRAGMA foreign_keys=ON;")
        });
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

use std::convert::TryFrom;

use rusqlite::params;
use serde::{Deserialize, Serialize};

use crate::{api::MvpError, database::DatabaseConnection, endpoint::Outcome};

// Serde struct - BEGIN
#[derive(Debug, Deserialize)]
pub struct Paging {
    #[serde(default)]
    limit: PagingLimit,
    #[serde(default)]
    offset: PagingOffset,
}

#[derive(Debug, Deserialize)]
pub struct PagingLimit(u16);
impl Default for PagingLimit {
    fn default() -> Self {
        PagingLimit(20)
    }
}
impl PagingLimit {
    fn get(&self) -> u16 {
        self.0
    }
}

#[derive(Debug, Deserialize)]
pub struct PagingOffset(u16);
impl Default for PagingOffset {
    fn default() -> Self {
        PagingOffset(0)
    }
}
impl PagingOffset {
    fn get(&self) -> u16 {
        self.0
    }
}
// Serde struct - END

#[derive(Debug, Serialize, Deserialize)]
pub struct Permission {
    pub id: u16,
    pub name: String,
    pub deleted: bool,
}

// Outcome definition
#[derive(Debug, Serialize)]
pub struct InternalMessage(Vec<Permission>);
impl Outcome for InternalMessage {}

impl TryFrom<Vec<Permission>> for InternalMessage {
    type Error = MvpError;

    fn try_from(data: Vec<Permission>) -> Result<Self, Self::Error> {
        Ok(InternalMessage(data))
    }
}

impl InternalMessage {
    pub fn retrieve_permissions(
        db_connection: &DatabaseConnection,
        paging: Paging,
    ) -> Result<Vec<Permission>, MvpError> {
        let conn = db_connection.get()?;
        let mut stmt =
            conn.prepare("SELECT id, name, deleted FROM `permissions` WHERE deleted = 0 LIMIT ?1 OFFSET ?2;")?;
        let retrieved_permissions =
            stmt.query_map(params![paging.limit.get(), paging.offset.get()], |row| {
                let deleted_column: u8 = row.get(2)?;
                Ok(Permission {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    deleted: deleted_column > 0,
                })
            })?;

        let mut permissions: Vec<Permission> = Vec::new();

        for row in retrieved_permissions {
            if let Ok(user) = row {
                permissions.push(user);
            }
        }

        Ok(permissions)
    }
}

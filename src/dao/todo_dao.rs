use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

#[serde_with::serde_as]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Todo {
    pub id: Option<i32>,
    pub owner: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    #[serde_as(as = "Option<serde_with::TimestampSecondsWithFrac<String>>")]
    pub created: Option<DateTime<Utc>>,
    #[serde_as(as = "Option<serde_with::TimestampSecondsWithFrac<String>>")]
    pub modified: Option<DateTime<Utc>>,
}

impl Todo {
    pub fn from_record(row: &Row) -> Todo {
        Todo {
            id: row.try_get(0).map_or(None, |x| Some(x)),
            owner: row.try_get(1).map_or(None, |x| Some(x)),
            name: row.try_get(2).map_or(None, |x| Some(x)),
            description: row.try_get(3).map_or(None, |x| Some(x)),
            status: row.try_get(4).map_or(None, |x| Some(x)),
            created: row.try_get(5).map_or(None, |x| Some(x)),
            modified: row.try_get(6).map_or(None, |x| Some(x)),
        }
    }
}
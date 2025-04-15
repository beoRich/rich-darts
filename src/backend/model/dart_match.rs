#[cfg(feature = "server")]
use diesel::prelude::*;
use crate::domain::{Match};

#[cfg_attr(feature = "server", derive(Queryable, Selectable))]
#[cfg_attr(feature = "server", diesel(table_name = crate::schema_manual::guard::dartmatch))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
pub struct DartMatch {
    pub id: i32,
    pub status: String,
    pub title: Option<String>
}

#[cfg_attr(feature = "server", derive(Insertable))]
#[cfg_attr(feature = "server", diesel(table_name = crate::schema_manual::guard::dartmatch))]
pub struct NewDartMatch {
    pub status: String
}

impl NewDartMatch {
    pub(crate) fn new () -> NewDartMatch {
        NewDartMatch {status: "ONGOING".to_string()}
    }
}

pub fn map_db_to_domain(db: DartMatch) -> Match {
    Match {
        id: db.id as u16,
        status: db.status,
        title: db.title.unwrap_or(format!("Match {}", db.id))
    }
}

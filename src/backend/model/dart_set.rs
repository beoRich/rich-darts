#[cfg(feature = "server")]
use diesel::prelude::*;

use crate::domain::Set;

#[cfg_attr(feature = "server", derive(Queryable, Selectable))]
#[cfg_attr(feature = "server", diesel(table_name = crate::schema_manual::guard::dartset))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
pub struct DartSet {
    pub id: i32,
    pub match_id: i32,
    pub set_order: i32,
    pub status: String,
}

#[cfg_attr(feature = "server", derive(Insertable))]
#[cfg_attr(feature = "server", diesel(table_name = crate::schema_manual::guard::dartset))]
pub struct NewDartSet {
    pub match_id: i32,
    pub set_order: i32,
    pub status: String,
}

impl NewDartSet {
    pub(crate) fn new(match_id: i32, set_order: i32) -> NewDartSet {
        NewDartSet {
            status: "ONGOING".to_string(),
            match_id,
            set_order,
        }
    }
}

pub fn map_db_to_domain(db: DartSet) -> Set {
    Set {
        id: db.id as u16,
        status: db.status,
        set_order: db.set_order as u16,
    }
}

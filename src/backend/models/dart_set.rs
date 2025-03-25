#[cfg(feature = "server")]
use diesel::prelude::*;

#[cfg_attr(feature = "server", derive(Queryable, Selectable))]
#[cfg_attr(feature = "server", diesel(table_name = crate::schema_manual::guard::dartset))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
pub struct DartSet {
    pub id: i32,
    pub match_id: i32,
    pub status: String
}
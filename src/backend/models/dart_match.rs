#[cfg(feature = "server")]
use diesel::prelude::*;

#[cfg_attr(feature = "server", derive(Queryable, Selectable))]
#[cfg_attr(feature = "server", diesel(table_name = crate::schema_manual::guard::dartmatch))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
pub struct DartMatch {
    pub id: i32,
    pub status: String
}

#[cfg_attr(feature = "server", derive(Insertable))]
#[cfg_attr(feature = "server", diesel(table_name = crate::schema_manual::guard::dartmatch))]
pub struct NewMatch {
    pub status: String
}

impl NewMatch {
    pub(crate) fn new () -> NewMatch {
        NewMatch {status: "ONGOING".to_string()}
    }
}

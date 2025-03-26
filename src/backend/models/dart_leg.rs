#[cfg(feature = "server")]
use diesel::prelude::*;

#[cfg_attr(feature = "server", derive(Queryable, Selectable))]
#[cfg_attr(feature = "server", diesel(table_name = crate::schema_manual::guard::dartleg))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
pub struct DartLeg {
    pub id: i32,
    pub set_id: i32,
    pub leg_order: i32,
    pub status: String
}

#[cfg_attr(feature = "server", derive(Insertable))]
#[cfg_attr(feature = "server", diesel(table_name = crate::schema_manual::guard::dartleg))]
pub struct NewDartLeg {
    pub set_id: i32,
    pub status: String,
    pub leg_order: i32,
}

impl NewDartLeg {
    pub(crate) fn new (set_id: i32, leg_order: i32) -> NewDartLeg {
        NewDartLeg {status: "ONGOING".to_string(), set_id, leg_order}
    }
}

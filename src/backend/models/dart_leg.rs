#[cfg(feature = "server")]
use diesel::prelude::*;

#[cfg_attr(feature = "server", derive(Queryable, Selectable))]
#[cfg_attr(feature = "server", diesel(table_name = crate::schema_manual::guard::leg))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
pub struct DartLeg {
    pub id: i32,
    pub set_id: i32,
    pub status: String
}

#[cfg_attr(feature = "server", derive(Insertable))]
#[cfg_attr(feature = "server", diesel(table_name = crate::schema_manual::guard::leg))]
pub struct NewLeg {
    pub id: i32,
    pub set_id: i32,
    pub status: String
}

//todo restructure the programm such that leg_id is not needed before creating NewLeg
impl NewLeg {
    pub(crate) fn new (set_id: i32, leg_id: i32) -> NewLeg {
        NewLeg {status: "ONGOING".to_string(), set_id, id: leg_id}
    }
}

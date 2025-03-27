#[cfg(feature = "server")]
use diesel::prelude::*;

#[cfg_attr(feature = "server", derive(Queryable, Selectable))]
#[cfg_attr(feature = "server", diesel(table_name = crate::schema_manual::guard::score))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
pub struct DartScore {
    pub id: i32,
    pub leg_id: i32,
    pub throw_order: i32,
    pub thrown: i32,
    pub remaining: i32,
    pub deleted: bool
}

#[cfg_attr(feature = "server", derive(Insertable))]
#[cfg_attr(feature = "server", diesel(table_name = crate::schema_manual::guard::score))]
pub struct NewDartScore {
    pub leg_id: i32,
    pub throw_order: i32,
    pub thrown: i32,
    pub remaining: i32,
    pub deleted: bool
}

impl NewDartScore {
    pub(crate) fn new (leg_id: i32, throw_order:i32, thrown: i32, remaining: i32) -> NewDartScore {
        NewDartScore {leg_id, throw_order, thrown, remaining, deleted: false}
    }
}

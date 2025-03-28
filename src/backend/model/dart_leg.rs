#[cfg(feature = "server")]
use diesel::prelude::*;
use crate::domain::{Leg, LegStatus};

#[cfg_attr(feature = "server", derive(Queryable, Selectable))]
#[cfg_attr(feature = "server", diesel(table_name = crate::schema_manual::guard::dartleg))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
#[derive(Debug)]
pub struct DartLeg {
    pub id: i32,
    pub set_id: i32,
    pub leg_order: i32,
    pub start_score: i32,
    pub status: String
}

#[cfg_attr(feature = "server", derive(Insertable))]
#[cfg_attr(feature = "server", diesel(table_name = crate::schema_manual::guard::dartleg))]
#[derive(Debug)]
pub struct NewDartLeg {
    pub set_id: i32,
    pub status: String,
    pub leg_order: i32,
    pub start_score: i32,
}

impl NewDartLeg {
    pub(crate) fn new (set_id: i32, leg_order: i32, start_score: i32) -> NewDartLeg {
        NewDartLeg {status: LegStatus::Ongoing.value(), set_id, leg_order, start_score}
    }
}

pub fn map_db_to_domain(db: DartLeg) -> Leg {
    Leg {
        id: db.id as u16,
        status: db.status,
        leg_order: db.leg_order as u16,
        start_score: db.start_score as u16
    }
}

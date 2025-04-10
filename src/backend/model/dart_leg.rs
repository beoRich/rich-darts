use crate::backend::model::DartScore;
use crate::domain::{Leg, LegStatus};
#[cfg(feature = "server")]
use diesel::prelude::*;
use serde::Serialize;

#[cfg_attr(feature = "server", derive(Serialize))]
struct LegWithScores {
    #[cfg_attr(feature = "server", serde(flatten))]
    leg: DartLeg,
    scores: Vec<DartScore>,
}

#[cfg_attr(feature = "server", derive(Queryable, Selectable, Serialize))]
#[cfg_attr(feature = "server", diesel(table_name = crate::schema_manual::guard::dartleg))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
#[derive(Debug)]
pub struct DartLeg {
    pub id: i32,
    pub set_id: i32,
    pub leg_order: i32,
    pub start_score: i32,
    pub status: String,
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
    pub(crate) fn new(set_id: u16, leg_order: u16, start_score: u16) -> NewDartLeg {
        NewDartLeg {
            status: LegStatus::Future.display(),
            set_id: set_id as i32,
            leg_order: leg_order as i32,
            start_score: start_score as i32,
        }
    }
}

pub fn map_db_to_domain(db: DartLeg) -> Leg {
    Leg {
        id: db.id as u16,
        status: db.status,
        leg_order: db.leg_order as u16,
        start_score: db.start_score as u16,
        last_score: None,
    }
}
pub fn map_db_to_domain_with_last_score(db: DartLeg) -> Leg {
    Leg {
        id: db.id as u16,
        status: db.status,
        leg_order: db.leg_order as u16,
        start_score: db.start_score as u16,
        last_score: None,
    }
}

#[cfg(feature = "server")]
use diesel::prelude::*;
use serde::Serialize;
use crate::backend::model::DartLeg;
use crate::domain::Score;



#[cfg_attr(feature = "server", derive(Identifiable,Queryable, Selectable, Serialize, Associations))]
#[cfg_attr(feature = "server", diesel(table_name = crate::schema_manual::guard::score))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
#[cfg_attr(feature = "server", diesel(belongs_to(DartLeg)))]
pub struct DartScore {
    pub id: i32,
    pub dart_leg_id: i32,
    pub throw_order: i32,
    pub thrown: i32,
    pub remaining: i32,
    pub double_attempt: Option<i32>,
    pub deleted: bool
}

#[cfg_attr(feature = "server", derive(Insertable))]
#[cfg_attr(feature = "server", diesel(table_name = crate::schema_manual::guard::score))]
pub struct NewDartScore {
    pub dart_leg_id: i32,
    pub throw_order: i32,
    pub thrown: i32,
    pub remaining: i32,
    pub double_attempt: Option<i32>,
    pub deleted: bool,
}

pub fn map_db_to_domain(db: DartScore) -> Score {
    Score {
        leg_id: db.dart_leg_id as u16,
        remaining: db.remaining as u16,
        thrown: db.thrown as u16,
        throw_order: db.throw_order as u16,
        double_attempt: db.double_attempt.map(|val| val as u16)
    }
}

pub fn map_domain_to_undeleted_db(domain: Score) -> NewDartScore {
    NewDartScore {
        dart_leg_id: domain.leg_id as i32,
        throw_order: domain.throw_order as i32,
        thrown: domain.thrown as i32,
        remaining: domain.remaining as i32,
        double_attempt: domain.double_attempt.map(|val| val as i32),
        deleted: false
    }
}

use dioxus::prelude::*;
use dioxus::prelude::{server, ServerFnError};
use tracing::debug;
use crate::domain::{IdOrder, IdOrderParent, Leg, Score, INIT_SCORE};

#[cfg(feature = "server")]
mod server_deps {
    pub use crate::backend::backend::DB2;
    pub use crate::backend::model::*;
    pub use crate::schema_manual::guard::dartleg::set_id;
    pub use crate::schema_manual::guard::dartleg::dsl::dartleg;
    pub use crate::schema_manual::guard::dartleg::start_score;
    pub use diesel::prelude::*;
    pub use diesel::result;
}

#[cfg(feature = "server")]
use server_deps::*;

#[server]
pub async fn list_leg(set_id_input: i32) -> Result<Vec<Leg>, ServerFnError> {
    use crate::schema_manual::guard::dartleg::dsl::*;

    let mut conn = DB2.lock()?; // Lock to get mutable access
    let conn_ref = &mut *conn;

    let legs_db = dartleg
        .filter(set_id.eq(set_id_input))
        .select(DartLeg::as_select())
        .load(conn_ref)?;

    let legs = legs_db
        .into_iter()
        .map(|db| Leg {
            id: db.id as u16,
            status: db.status,
            leg_order: db.leg_order as u16,
            start_score: db.start_score as u16
        })
        .collect();
    Ok(legs)
}

#[server]
pub async fn get_latest_leg() -> Result<(IdOrderParent, Leg), ServerFnError> {
    use crate::schema_manual::guard::dartleg::dsl::*;
    use crate::schema_manual::guard::dartleg::dsl::id;
    use crate::schema_manual::guard::dartset::dsl::*;
    let mut conn = DB2.lock()?;
    let conn_ref = &mut *conn;

    let db = QueryDsl::order(dartleg, id.desc()).first::<DartLeg>(conn_ref)?;
    let leg = Leg  {
        id: db.id as u16,
        leg_order: db.leg_order as u16,
        status: db.status,
        start_score: db.start_score as u16
    };
    let set_result = dartset.find(db.set_id).first::<DartSet>(conn_ref)?;
    let set_id_order = IdOrderParent {
        id: set_result.id as u16,
        order: set_result.set_order as u16,
        parent_id: set_result.match_id as u16
    };
    Ok((set_id_order, leg))
}

#[server]
pub async fn get_leg_by_id(id_input: i32) -> Result<Leg, ServerFnError> {
    use crate::schema_manual::guard::dartleg::dsl::*;
    let mut conn = DB2.lock()?; // Lock to get mutable access
    let conn_ref = &mut *conn;

    let db = dartleg.find(id_input).first::<DartLeg>(conn_ref)?;
    let leg = Leg {
        id: db.id as u16,
        status: db.status,
        leg_order: db.leg_order as u16,
        start_score: db.start_score as u16
    };

    Ok(leg)
}

#[server]
pub async fn new_leg_init_score(set_id_input: i32, start_score_input: u16) -> Result<Leg, ServerFnError> {
    use crate::schema_manual::guard::dartleg;

    let mut conn = DB2.lock()?; // Lock to get mutable access
    let conn_ref = &mut *conn;

    let latest_leg_of_set_test =
        QueryDsl::order(dartleg.filter(set_id.eq(set_id_input)), dartleg::id.desc())
            .first::<DartLeg>(conn_ref)
            .optional();

    let latest_leg_of_set = latest_leg_of_set_test.expect("Failed to get latest leg of set");

    let leg_order_val: u16;
    match latest_leg_of_set {
        Some(val) => leg_order_val = (val.leg_order + 1) as u16,
        None => leg_order_val = 1,
    }

    let insert_leg = NewDartLeg::new(set_id_input, leg_order_val as i32, start_score_input as i32);

    debug!("{:?}", insert_leg);
    let leg_result = diesel::insert_into(dartleg::table)
        .values(insert_leg)
        .returning(DartLeg::as_returning())
        .get_result(conn_ref)?;

    let init_score_struct: Score = Score {
        remaining: start_score_input,
        thrown: 0,
        throw_order: 0,
    };
    crate::backend::api::dart_score::new_score_with_connection(conn_ref, leg_result.id, init_score_struct)?;
    Ok((Leg {
        id: leg_result.id as u16,
        status: leg_result.status,
        leg_order: leg_order_val,
        start_score: start_score_input
    }))
}

#[server]
pub async fn update_leg_status(leg_id_input: i32, new_status: String) -> Result<Leg, ServerFnError> {
    let mut conn = DB2.lock()?; // Lock to get mutable access
    let conn_ref = &mut *conn;
    use crate::schema_manual::guard::dartleg::dsl::*;
    let result = diesel::update(dartleg).filter(id.eq(leg_id_input))
        .set(status.eq(new_status))
        .returning(DartLeg::as_returning())
        .get_result(conn_ref)?;
    Ok(Leg{id: result.id as u16, leg_order: result.leg_order as u16, status: result.status, start_score: result.start_score as u16 })
}

#[server]
pub async fn create_leg_chain() -> Result<(), ServerFnError> {
    // use later as quickstart from main panel
    use crate::schema_manual::guard::dartleg;
    use crate::schema_manual::guard::dartmatch;
    use crate::schema_manual::guard::dartset;

    let mut conn = DB2.lock()?; // Lock to get mutable access
    let conn_ref = &mut *conn;

    let insert_match = NewDartMatch::new();
    let match_result = diesel::insert_into(dartmatch::table)
        .values(insert_match)
        .returning(DartMatch::as_returning())
        .get_result(conn_ref)?;

    let insert_set = NewDartSet::new(match_result.id, 1);
    let set_result = diesel::insert_into(dartset::table)
        .values(insert_set)
        .returning(DartSet::as_returning())
        .get_result(conn_ref)?;

    let insert_leg = NewDartLeg::new(set_result.id, 1, 501);
    let leg_result = diesel::insert_into(dartleg::table)
        .values(insert_leg)
        .returning(DartLeg::as_returning())
        .get_result(conn_ref)?;
    Ok(())
}

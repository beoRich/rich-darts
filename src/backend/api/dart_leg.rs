use crate::domain::{IdOrder, IdOrderParent, Leg, LegStatus, Score, Set, INIT_SCORE};
use dioxus::prelude::*;
use dioxus::prelude::{server, ServerFnError};
use tracing::debug;

#[cfg(feature = "server")]
mod server_deps {
    pub use crate::backend::backend::DB2;
    pub use crate::backend::model::*;
    pub use crate::schema_manual::guard::dartleg::dsl::dartleg;
    pub use crate::schema_manual::guard::dartleg::set_id;
    pub use diesel::prelude::*;
}

#[cfg(feature = "server")]
use server_deps::*;

#[server]
pub async fn list_leg(set_id_input: u16) -> Result<Vec<Leg>, ServerFnError> {
    use crate::schema_manual::guard::dartleg::dsl::*;

    let mut conn = DB2.lock()?; // Lock to get mutable access
    let conn_ref = &mut *conn;

    let db_leg_results = dartleg
        .filter(set_id.eq(set_id_input as i32))
        .select(DartLeg::as_select())
        .load(conn_ref)?;

    let legs = db_leg_results
        .into_iter()
        .map(|db| dart_leg::map_db_to_domain(db))
        .collect();
    Ok(legs)
}

#[server]
pub async fn get_latest_leg() -> Result<(u16, Set, Leg), ServerFnError> {
    use crate::schema_manual::guard::dartleg::dsl::id;
    use crate::schema_manual::guard::dartleg::dsl::*;
    use crate::schema_manual::guard::dartset::dsl::*;
    let mut conn = DB2.lock()?;
    let conn_ref = &mut *conn;

    let db_leg_result = QueryDsl::order(dartleg, id.desc()).first::<DartLeg>(conn_ref)?;
    let set_result = dartset
        .find(db_leg_result.set_id)
        .first::<DartSet>(conn_ref)?;
    let leg = dart_leg::map_db_to_domain(db_leg_result);
    let match_id_val = set_result.match_id as u16;
    let set = dart_set::map_db_to_domain(set_result);
    Ok((match_id_val, set, leg))
}

#[server]
pub async fn get_leg_by_id(id_input: i32) -> Result<Leg, ServerFnError> {
    use crate::schema_manual::guard::dartleg::dsl::*;
    let mut conn = DB2.lock()?; // Lock to get mutable access
    let conn_ref = &mut *conn;

    let db_leg_result = dartleg.find(id_input).first::<DartLeg>(conn_ref)?;
    Ok(dart_leg::map_db_to_domain(db_leg_result))
}

#[server]
pub async fn new_leg_init_score(
    set_id_input: u16,
    start_score_input: u16,
) -> Result<Leg, ServerFnError> {
    use crate::schema_manual::guard::dartleg;

    let mut conn = DB2.lock()?; // Lock to get mutable access
    let conn_ref = &mut *conn;

    let latest_leg_of_set_test =
        QueryDsl::order(dartleg.filter(set_id.eq(set_id_input as i32)), dartleg::id.desc())
            .first::<DartLeg>(conn_ref)
            .optional();

    let latest_leg_of_set = latest_leg_of_set_test.expect("Failed to get latest leg of set");

    let leg_order_val: u16;
    match latest_leg_of_set {
        Some(val) => leg_order_val = (val.leg_order + 1) as u16,
        None => leg_order_val = 1,
    }

    let insert_leg = NewDartLeg::new(set_id_input, leg_order_val , start_score_input);

    debug!("{:?}", insert_leg);
    let db_leg_result = diesel::insert_into(dartleg::table)
        .values(insert_leg)
        .returning(DartLeg::as_returning())
        .get_result(conn_ref)?;

    let init_score_struct: Score = Score {
        remaining: start_score_input,
        thrown: 0,
        throw_order: 0,
    };
    crate::backend::api::dart_score::new_score_with_connection(
        conn_ref,
        db_leg_result.id,
        init_score_struct,
    )?;
    Ok(dart_leg::map_db_to_domain(db_leg_result))
}

#[server]
pub async fn update_leg_status(
    leg_id_input: u16,
    new_status: LegStatus,
) -> Result<Leg, ServerFnError> {
    let mut conn = DB2.lock()?; // Lock to get mutable access
    let conn_ref = &mut *conn;
    use crate::schema_manual::guard::dartleg::dsl::*;
    let db_leg_result = diesel::update(dartleg)
        .filter(id.eq(leg_id_input as i32))
        .set(status.eq(new_status.value()))
        .returning(DartLeg::as_returning())
        .get_result(conn_ref)?;
    Ok(dart_leg::map_db_to_domain(db_leg_result))
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

    let insert_set = NewDartSet::new(match_result.id, 1, 3);
    let set_result = diesel::insert_into(dartset::table)
        .values(insert_set)
        .returning(DartSet::as_returning())
        .get_result(conn_ref)?;

    let insert_leg = NewDartLeg::new(set_result.id as u16, 1, 501);
    let leg_result = diesel::insert_into(dartleg::table)
        .values(insert_leg)
        .returning(DartLeg::as_returning())
        .get_result(conn_ref)?;
    Ok(())
}

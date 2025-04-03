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
pub async fn list_leg_with_last_score(set_id_input: u16) -> Result<Vec<Leg>, ServerFnError> {
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

#[cfg(feature = "server")]
fn leg_by_order_if_exists(
    conn_ref: &mut SqliteConnection,
    set_id_input: u16,
    leg_order_input: u16,
) -> Result<Option<DartLeg>, ServerFnError> {
    use crate::schema_manual::guard::dartleg::dsl::*;

    let test = vec![1];
    !test.is_empty();

    let db_leg_result = dartleg
        .filter(
            set_id
                .eq(set_id_input as i32)
                .and(leg_order.eq(leg_order_input as i32))
                .and(status.ne(LegStatus::Cancelled.display())),
        )
        .first::<DartLeg>(conn_ref)
        .optional()?;
    Ok(db_leg_result)
}

#[server]
pub async fn new_leg_with_init_score_if_not_exists(
    set_id_input: u16,
    start_score_input: u16,
    leg_order_input: u16,
) -> Result<Leg, ServerFnError> {
    let mut conn = DB2.lock()?; // Lock to get mutable access
    let conn_ref = &mut *conn;
    let exists_maybe = leg_by_order_if_exists(conn_ref, set_id_input, leg_order_input)?;
    if exists_maybe.is_none() {
        let test = new_legs_with_init_score_func(conn_ref, set_id_input, start_score_input, 1)?;
        Ok(test.get(0).unwrap().clone())
    } else {
        Ok(dart_leg::map_db_to_domain(exists_maybe.unwrap()))
    }
}

#[server]
pub async fn new_legs_with_init_score(
    set_id_input: u16,
    start_score_input: u16,
    leg_amount_input: u16,
) -> Result<Vec<Leg>, ServerFnError> {
    let mut conn = DB2.lock()?; // Lock to get mutable access
    let conn_ref = &mut *conn;
    new_legs_with_init_score_func(conn_ref, set_id_input, start_score_input, leg_amount_input)
}

#[cfg(feature = "server")]
fn new_legs_with_init_score_func(
    conn_ref: &mut SqliteConnection,
    set_id_input: u16,
    start_score_input: u16,
    leg_amount_input: u16,
) -> Result<Vec<Leg>, ServerFnError> {
    use crate::schema_manual::guard::dartleg;
    debug!("leg_amount_input {:?}", leg_amount_input);

    let leg_order_start: u16 = get_next_ongoing_leg_order_of_set(conn_ref, set_id_input)?;
    let insert_legs: Vec<NewDartLeg> = (leg_order_start..leg_order_start + leg_amount_input)
        .map(|leg_order_input| {
            NewDartLeg::new_cond(
                set_id_input,
                leg_order_input,
                start_score_input,
                leg_order_input == leg_order_start,
            )
        })
        .collect();

    debug!("insert_leg {:?}", insert_legs);

    //batch insert did not work with sqlite :(
    let db_leg_results_maybe: Result<Vec<DartLeg>, _> = insert_legs
        .into_iter()
        .map(|insert_leg| {
            diesel::insert_into(dartleg::table)
                .values(insert_leg)
                .returning(DartLeg::as_returning())
                .get_result(conn_ref)
        })
        .collect();

    let db_leg_results = db_leg_results_maybe?;

    let init_score_struct: Score = Score {
        remaining: start_score_input,
        thrown: 0,
        throw_order: 0,
    };
    let res_score: Result<Vec<_>, _> = db_leg_results
        .iter()
        .map(|db_leg_result| {
            crate::backend::api::dart_score::new_score_with_connection(
                conn_ref,
                db_leg_result.id,
                init_score_struct.clone(),
            )
        })
        .collect();
    let res: Vec<Leg> = db_leg_results
        .into_iter()
        .map(|db_leg_result| dart_leg::map_db_to_domain(db_leg_result))
        .collect();
    debug!("end of {:?}", res);
    Ok(res)
}

#[cfg(feature = "server")]
fn get_next_ongoing_leg_order_of_set(
    conn_ref: &mut SqliteConnection,
    set_id_input: u16,
) -> Result<u16, ServerFnError> {
    use crate::schema_manual::guard::dartleg;
    let latest_leg_of_set_test = QueryDsl::order(
        dartleg
            .filter(set_id.eq(set_id_input as i32))
            .filter(dartleg::status.ne(LegStatus::Cancelled.display())),
        dartleg::id.desc(),
    )
    .first::<DartLeg>(conn_ref)
    .optional();

    let latest_leg_of_set = latest_leg_of_set_test.expect("Failed to get latest leg of set");
    let leg_order_val: u16 = latest_leg_of_set
        .map(|val| (val.leg_order + 1) as u16)
        .unwrap_or(1);
    debug!("leg_order_val {:?}", leg_order_val);
    Ok(leg_order_val)
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
        .set(status.eq(new_status.display()))
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

    let insert_leg = NewDartLeg::new(set_result.id as u16, 1, 501, LegStatus::Ongoing);
    let leg_result = diesel::insert_into(dartleg::table)
        .values(insert_leg)
        .returning(DartLeg::as_returning())
        .get_result(conn_ref)?;
    Ok(())
}

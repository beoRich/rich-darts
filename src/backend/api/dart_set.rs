use crate::domain::{LegStatus, Metric, Set, SetStatus};
use dioxus::prelude::*;
use dioxus::prelude::{server, ServerFnError};
use tracing::debug;

#[cfg(feature = "server")]
mod server_deps {
    pub use crate::backend::backend::DB2;
    pub use crate::backend::model::*;
    pub use crate::schema_manual::guard::dartleg::dsl::dartleg;
    pub use crate::schema_manual::guard::dartleg::id;
    pub use crate::schema_manual::guard::dartleg::set_id;
    pub use crate::schema_manual::guard::dartleg::status;
    pub use crate::schema_manual::guard::dartset::dsl::dartset;
    pub use crate::schema_manual::guard::dartset::match_id;
    pub use crate::schema_manual::guard::score::deleted;
    pub use diesel::prelude::*;
}

#[cfg(feature = "server")]
use server_deps::*;

#[server]
pub async fn list_set(match_id_input: i32) -> Result<Vec<Set>, ServerFnError> {
    use crate::schema_manual::guard::dartset::dsl::*;

    let mut conn = DB2.lock()?; // Lock to get mutable access
    let conn_ref = &mut *conn;

    let sets_db = dartset
        .filter(match_id.eq(match_id_input))
        .select(DartSet::as_select())
        .load(conn_ref)?;

    let sets = sets_db
        .into_iter()
        .map(|set_db_result| dart_set::map_db_to_domain(set_db_result))
        .collect();
    Ok(sets)
}

#[server]
pub async fn get_set_by_id(id_input: u16) -> Result<Set, ServerFnError> {
    use crate::schema_manual::guard::dartset::dsl::*;
    let mut conn = DB2.lock()?; // Lock to get mutable access
    let conn_ref = &mut *conn;

    let set_db_result = dartset.find(id_input as i32).first::<DartSet>(conn_ref)?;
    let set = dart_set::map_db_to_domain(set_db_result);
    Ok(set)
}


#[server]
pub async fn new_set(match_id_input: u16, leg_amount_input: u16) -> Result<Set, ServerFnError> {
    use crate::schema_manual::guard::dartset;

    let mut conn = DB2.lock()?; // Lock to get mutable access
    let conn_ref = &mut *conn;

    let latest_set_of_match: Option<DartSet> = QueryDsl::order(
        dartset.filter(match_id.eq(match_id_input as i32)),
        dartset::id.desc(),
    )
    .first::<DartSet>(conn_ref)
    .optional()?;

    let set_order_val: u16;
    match latest_set_of_match {
        Some(val) => set_order_val = (val.set_order + 1) as u16,
        None => set_order_val = 1,
    }

    let insert_set = NewDartSet::new(match_id_input as i32, set_order_val as i32, leg_amount_input as i32);
    let set_result = diesel::insert_into(dartset::table)
        .values(insert_set)
        .returning(DartSet::as_returning())
        .get_result(conn_ref)?;
    Ok(dart_set::map_db_to_domain(set_result))
}

#[server]
pub async fn get_latest_set() -> Result<(u16, Set), ServerFnError> {
    use crate::schema_manual::guard::dartset::dsl::*;
    let mut conn = DB2.lock()?; // Lock to get mutable access
    let conn_ref = &mut *conn;

    let set_db_result = QueryDsl::order(dartset, id.desc()).first::<DartSet>(conn_ref)?;
    let parent_id = set_db_result.match_id as u16;
    let set = dart_set::map_db_to_domain(set_db_result);
    Ok((parent_id, set))
}

#[server]
pub async fn update_set_status(
    set_id_input: u16,
    new_status: SetStatus,
) -> Result<Set, ServerFnError> {
    let mut conn = DB2.lock()?; // Lock to get mutable access
    let conn_ref = &mut *conn;
    use crate::schema_manual::guard::dartset::dsl::*;
    let db_set_result = diesel::update(dartset)
        .filter(id.eq(set_id_input as i32))
        .set(status.eq(new_status.value()))
        .returning(DartSet::as_returning())
        .get_result(conn_ref)?;
    Ok(dart_set::map_db_to_domain(db_set_result))
}

#[server]
pub async fn get_metrics_of_other_legs_by_set_id(set_id_input: u16, exclude_leg_id: u16) -> Result<Metric, ServerFnError> {
    debug!("get_metrics_of_other_legs_by_set_id");
    let mut conn = DB2.lock()?; // Lock to get mutable access
    let conn_ref = &mut *conn;
    // see https://diesel.rs/guides/relations.html for the following approach (avoiding n+1 problem)

    let db_legs_for_set = dartleg
        .filter(set_id.eq(set_id_input as i32).and(
            status.eq(LegStatus::Ongoing.display())
            .or(status.eq(LegStatus::Finished.display())
            ))
            .and(id.ne(exclude_leg_id as i32)))
        .select(DartLeg::as_select())
        .load(conn_ref)?;

    let all_scores = DartScore::belonging_to(&db_legs_for_set).filter(deleted.eq(false))
        .select(DartScore::as_select()).load(conn_ref)?;

    //group scores per leg
    let scores_per_leg = all_scores.grouped_by(&db_legs_for_set).into_iter().zip(&db_legs_for_set)
        .map(|(scores, leg)| (leg, scores))
        .map(|(leg, mut scores) | {
            let new_scores = scores.split_off(1);
            (leg, new_scores)
        })
        .collect::<Vec<(&DartLeg, Vec<DartScore>)>>();

    let only_scores: Vec<&DartScore> = scores_per_leg.iter().flat_map(|(_,scores)|  scores).collect();
    if only_scores.is_empty() {
        Ok(Metric{sum: 0, score_amount: 0, throws: 0, amount_of_legs: 0,
            first_nine_per_leg_sum: 0, hundred_plus_amount: 0, double_attempts: 0})
    } else {
        let metric = Metric { sum: only_scores.iter().map(|dart_score| dart_score.thrown as u16).sum(),
            score_amount: only_scores.iter().count() as u16,
            throws: (only_scores.iter().count() * 3) as u16,
            amount_of_legs: db_legs_for_set.iter().count() as u16,
            hundred_plus_amount: only_scores.iter().filter(| score | score.thrown >= 100).count() as u16,
            first_nine_per_leg_sum: 100,//todo
            double_attempts:  only_scores.iter().map(|score| score.double_attempt.unwrap_or(0) as u16).sum()
        };
        Ok(metric)
    }



}

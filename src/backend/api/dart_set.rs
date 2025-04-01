use crate::domain::{IdOrder, Leg, LegStatus, Set, SetStatus};
use dioxus::prelude::*;
use dioxus::prelude::{server, ServerFnError};

#[cfg(feature = "server")]
mod server_deps {
    pub use crate::backend::backend::DB2;
    pub use crate::backend::model::*;
    pub use crate::schema_manual::guard::dartset::dsl::dartset;
    pub use crate::schema_manual::guard::dartset::match_id;
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
pub async fn get_set_by_id(id_input: i32) -> Result<Set, ServerFnError> {
    use crate::schema_manual::guard::dartset::dsl::*;
    let mut conn = DB2.lock()?; // Lock to get mutable access
    let conn_ref = &mut *conn;

    let set_db_result = dartset.find(id_input).first::<DartSet>(conn_ref)?;
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

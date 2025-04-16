use crate::domain::{Score};
use dioxus::prelude::*;
use dioxus::prelude::{server, ServerFnError};

#[cfg(feature = "server")]
mod server_deps {
    pub use crate::backend::backend::DB2;
    pub use crate::backend::model::*;
    pub use diesel::prelude::*;
    pub use crate::backend::model::dart_score::map_domain_to_undeleted_db;
}

#[cfg(feature = "server")]
use server_deps::*;

#[server]
pub async fn list_score(leg_id_input: u16) -> Result<Vec<Score>, ServerFnError> {
    use crate::schema_manual::guard::score::dsl::*;
    let mut conn = DB2.lock()?; // Lock to get mutable access
    let conn_ref = &mut *conn;
    let db_score_results = score.filter(dart_leg_id.eq(leg_id_input as i32).and(deleted.eq(false))).select(DartScore::as_select()).load(conn_ref)?;

    let scores = db_score_results.into_iter().map(dart_score::map_db_to_domain).collect();
    Ok(scores)
}

#[server]
pub async fn new_score(score_input: Score) -> Result<(), ServerFnError> {
    let mut conn = DB2.lock()?; // Lock to get mutable access
    let conn_ref = &mut *conn;

    new_score_with_connection(conn_ref, score_input)?;
    Ok(())
}

#[cfg(feature = "server")]
pub fn new_score_with_connection(
    conn_ref: &mut SqliteConnection,
    score_input: Score,
) -> Result<(), ServerFnError> {
    use crate::schema_manual::guard::score;
    let _ = diesel::insert_into(score::table)
        .values(map_domain_to_undeleted_db(score_input))
        .returning(DartScore::as_returning())
        .get_result(conn_ref)?;
    Ok(())
}


#[server]
pub async fn delete_score_by_order(leg_id_input: u16, throw_order_input: u16) -> Result<(), ServerFnError> {
    let mut conn = DB2.lock()?; // Lock to get mutable access
    let conn_ref = &mut *conn;
    use crate::schema_manual::guard::score::dsl::*;

    let _ = diesel::update(score).filter(throw_order.eq(throw_order_input as i32).and(dart_leg_id.eq(leg_id_input as i32)))
        .set(deleted.eq(true))
        .returning(DartScore::as_returning())
        .get_result(conn_ref)?;
    Ok(())
}

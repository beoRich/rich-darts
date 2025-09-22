use dioxus::html::completions::CompleteWithBraces::map;
use crate::domain::{Match, Metric, Set};
use dioxus::prelude::*;
use dioxus::prelude::{server, ServerFnError};
use tracing::debug;

#[cfg(feature = "server")]
mod server_deps {
    pub use crate::backend::backend::DB2;
    pub use crate::backend::model::DartMatch;
    pub use crate::backend::model::*;
    pub use crate::schema_manual::guard::dartmatch::dsl::dartmatch;
    pub use diesel::prelude::*;
    pub use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
}

#[cfg(feature = "server")]
use server_deps::*;
use crate::backend::model::dart_match::map_db_to_domain;

#[server]
pub async fn list_matches() -> Result<Vec<Match>, ServerFnError> {
    use crate::schema_manual::guard::dartmatch::dsl::*;

    let mut conn = DB2.lock()?; // Lock to get mutable access
    let conn_ref = &mut *conn;

    let match_db = dartmatch.select(DartMatch::as_select()).load(conn_ref)?;

    let matches = match_db
        .into_iter()
        .map(map_db_to_domain)
        .collect();
    Ok(matches)
}

#[server]
pub async fn new_match(title_maybe: Option<String>) -> Result<Match, ServerFnError> {
    use crate::schema_manual::guard::dartmatch;
    use crate::schema_manual::guard::dartmatch::dsl::*;

    let mut conn = DB2.lock()?; // Lock to get mutable access
    let conn_ref = &mut *conn;

    let insert_match = NewDartMatch::new();
    let match_result = diesel::insert_into(dartmatch::table)
        .values(insert_match)
        .returning(DartMatch::as_returning())
        .get_result(conn_ref)?;
    debug!("NewMatchTest");

    let title_input = title_maybe.unwrap_or(format!("Match {}", match_result.id));
    let match_result_with_title = diesel::update(dartmatch).filter(id.eq(match_result.id)).set(title.eq(title_input))
        .returning(DartMatch::as_returning()).get_result(conn_ref)?;
    Ok(map_db_to_domain(match_result_with_title))
}

#[server]
pub async fn get_latest_match() -> Result<Match, ServerFnError> {
    use crate::schema_manual::guard::dartmatch::dsl::*;
    let mut conn = DB2.lock()?; // Lock to get mutable access
    let conn_ref = &mut *conn;

    let match_db_result = QueryDsl::order(dartmatch, id.desc()).first::<DartMatch>(conn_ref)?;
    Ok(map_db_to_domain(match_db_result))
}

#[server]
pub async fn get_match_by_id(id_input: u16) -> Result<Match, ServerFnError> {
    use crate::schema_manual::guard::dartmatch::dsl::*;
    let mut conn = DB2.lock()?; // Lock to get mutable access
    let conn_ref = &mut *conn;

    let db_result = dartmatch.find(id_input as i32).first::<DartMatch>(conn_ref)?;
    let set = dart_match::map_db_to_domain(db_result);
    Ok(set)
}

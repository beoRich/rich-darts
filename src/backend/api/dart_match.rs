use crate::domain::{Match, Set};
use dioxus::prelude::*;
use dioxus::prelude::{server, ServerFnError};

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

#[server]
pub async fn list_matches() -> Result<Vec<Match>, ServerFnError> {
    use crate::schema_manual::guard::dartmatch::dsl::*;

    let mut conn = DB2.lock()?; // Lock to get mutable access
    let conn_ref = &mut *conn;

    let match_db = dartmatch.select(DartMatch::as_select()).load(conn_ref)?;

    let matches = match_db
        .into_iter()
        .map(|db| Match {
            id: db.id as u16,
            status: db.status,
        })
        .collect();
    Ok(matches)
}

#[server]
pub async fn new_match() -> Result<Match, ServerFnError> {
    use crate::schema_manual::guard::dartmatch;

    let mut conn = DB2.lock()?; // Lock to get mutable access
    let conn_ref = &mut *conn;

    let insert_match = NewDartMatch::new();
    let match_result = diesel::insert_into(dartmatch::table)
        .values(insert_match)
        .returning(DartMatch::as_returning())
        .get_result(conn_ref)?;
    Ok((Match {
        id: match_result.id as u16,
        status: match_result.status,
    }))
}

#[server]
pub async fn get_latest_match() -> Result<(u16), ServerFnError> {
    use crate::schema_manual::guard::dartmatch::dsl::*;
    let mut conn = DB2.lock()?; // Lock to get mutable access
    let conn_ref = &mut *conn;

    let set_db_result = QueryDsl::order(dartmatch, id.desc()).first::<DartMatch>(conn_ref)?;
    Ok(set_db_result.id as u16)
}

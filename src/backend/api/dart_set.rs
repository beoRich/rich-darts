use dioxus::prelude::*;
use dioxus::prelude::{server, ServerFnError};
use crate::domain::{IdOrder, Leg, Set};

#[cfg(feature = "server")]
mod server_deps {
    pub use crate::backend::backend::DB2;
    pub use crate::backend::model::*;
    pub use diesel::prelude::*;
    pub use crate::schema_manual::guard::dartset::dsl::dartset;
    pub use crate::schema_manual::guard::dartset::match_id;
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
        .map(|db| Set {
            id: db.id as u16,
            status: db.status,
            set_order: db.set_order as u16,
        })
        .collect();
    Ok(sets)
}

#[server]
pub async fn get_set_by_id(id_input: i32) -> Result<Set, ServerFnError> {
    use crate::schema_manual::guard::dartset::dsl::*;
    let mut conn = DB2.lock()?; // Lock to get mutable access
    let conn_ref = &mut *conn;

    let set_result = dartset.find(id_input).first::<DartSet>(conn_ref)?;
    let set = Set {
        id: set_result.id as u16,
        status: set_result.status,
        set_order: set_result.set_order as u16,
    };
    Ok(set)
}

#[server]
pub async fn new_set(match_id_input: i32) -> Result<Set, ServerFnError> {
    use crate::schema_manual::guard::dartset;

    let mut conn = DB2.lock()?; // Lock to get mutable access
    let conn_ref = &mut *conn;

    let latest_set_of_match: Option<DartSet> = QueryDsl::order(
        dartset.filter(match_id.eq(match_id_input)),
        dartset::id.desc(),
    )
        .first::<DartSet>(conn_ref)
        .optional()?;

    let set_order_val: u16;
    match latest_set_of_match {
        Some(val) => set_order_val = (val.set_order + 1) as u16,
        None => set_order_val = 1,
    }

    let insert_set = NewDartSet::new(match_id_input, set_order_val as i32);
    let set_result = diesel::insert_into(dartset::table)
        .values(insert_set)
        .returning(DartSet::as_returning())
        .get_result(conn_ref)?;
    Ok((Set {
        id: set_result.id as u16,
        status: set_result.status,
        set_order: set_order_val,
    }))
}

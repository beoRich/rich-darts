use dioxus::prelude::*;
use dioxus::prelude::{server, ServerFnError};
use crate::domain::{Leg};

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
        })
        .collect();
    Ok(legs)
}

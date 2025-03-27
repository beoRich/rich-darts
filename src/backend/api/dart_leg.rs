use dioxus::prelude::*;
use dioxus::prelude::{server, ServerFnError};
use tracing::debug;
use crate::domain::{IdOrder, Leg, INIT_SCORE};

#[cfg(feature = "server")]
mod server_deps {
    pub use crate::backend::backend::DB2;
    pub use crate::backend::model::*;
    pub use crate::schema_manual::guard::dartleg::set_id;
    pub use crate::schema_manual::guard::dartleg::dsl::dartleg;
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

#[server]
pub async fn get_latest_leg() -> Result<Option<(IdOrder, Leg)>, ServerFnError> {
    use crate::schema_manual::guard::dartleg::dsl::*;
    let mut conn = DB2.lock()?; // Lock to get mutable access
    let conn_ref = &mut *conn;

    let leg_result = diesel::QueryDsl::order(dartleg, id.desc()).first::<DartLeg>(conn_ref)?;
    let leg = Leg {
        id: leg_result.id as u16,
        status: leg_result.status,
        leg_order: leg_result.leg_order as u16,
    };

    //todo fix
    Ok(Some((
        IdOrder {
            id: leg_result.set_id as u16,
            order: leg_result.set_id as u16,
        },
        leg,
    )))
}

#[server]
pub async fn get_leg_by_id(id_input: i32) -> Result<Leg, ServerFnError> {
    use crate::schema_manual::guard::dartleg::dsl::*;
    let mut conn = DB2.lock()?; // Lock to get mutable access
    let conn_ref = &mut *conn;

    let leg_result = dartleg.find(id_input).first::<DartLeg>(conn_ref)?;
    let leg = Leg {
        id: leg_result.id as u16,
        status: leg_result.status,
        leg_order: leg_result.leg_order as u16,
    };

    Ok(leg)
}

#[server]
pub async fn new_leg_init_score(set_id_input: i32) -> Result<Leg, ServerFnError> {
    debug!("{:?}", set_id_input);
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

    let insert_leg = NewDartLeg::new(set_id_input, leg_order_val as i32);

    debug!("{:?}", insert_leg);
    let leg_result = diesel::insert_into(dartleg::table)
        .values(insert_leg)
        .returning(DartLeg::as_returning())
        .get_result(conn_ref)?;
    crate::backend::api::dart_score::new_score_with_connection(conn_ref, leg_result.id, INIT_SCORE)?;
    Ok((Leg {
        id: leg_result.id as u16,
        status: leg_result.status,
        leg_order: leg_order_val,
    }))
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

    let insert_leg = NewDartLeg::new(set_result.id, 1);
    let leg_result = diesel::insert_into(dartleg::table)
        .values(insert_leg)
        .returning(DartLeg::as_returning())
        .get_result(conn_ref)?;
    Ok(())
}

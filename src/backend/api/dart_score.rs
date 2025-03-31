use crate::domain::{IdOrder, Leg, Score, INIT_SCORE};
use dioxus::prelude::*;
use dioxus::prelude::{server, ServerFnError};
use tracing::debug;

#[cfg(feature = "server")]
mod server_deps {
    pub use crate::backend::backend::DB;
    pub use crate::backend::backend::DB2;
    pub use crate::backend::model::*;
    pub use diesel::prelude::*;
}

#[cfg(feature = "server")]
use server_deps::*;

#[server]
pub async fn list_score(leg_id: u16) -> Result<Vec<Score>, ServerFnError> {
    let scores = DB.with(|f| {
        f.prepare(
            "SELECT remaining, thrown, throw_order from score where deleted = 0 and leg_id =?1",
        )
        .unwrap()
        .query_map([leg_id], move |row| {
            Ok(Score {
                remaining: row.get(0)?,
                thrown: row.get(1)?,
                throw_order: row.get(2)?,
            })
        })
        .unwrap()
        .map(|r| r.unwrap())
        .collect()
    });
    Ok(scores)
}

#[server]
pub async fn new_score(leg_id_input: i32, score_input: Score) -> Result<(), ServerFnError> {
    let mut conn = DB2.lock()?; // Lock to get mutable access
    let conn_ref = &mut *conn;

    new_score_with_connection(conn_ref, leg_id_input, score_input)?;
    Ok(())
}

#[cfg(feature = "server")]
pub fn new_score_with_connection(
    conn_ref: &mut SqliteConnection,
    leg_id_input: i32,
    score_input: Score,
) -> Result<(), ServerFnError> {
    let insert_score = NewDartScore::new(
        leg_id_input,
        score_input.throw_order as i32,
        score_input.thrown as i32,
        score_input.remaining as i32,
    );
    use crate::schema_manual::guard::score;
    let _ = diesel::insert_into(score::table)
        .values(insert_score)
        .returning(DartScore::as_returning())
        .get_result(conn_ref)?;
    Ok(())
}

#[server]
pub async fn save_score(leg_id: u16, score: Score) -> Result<(), ServerFnError> {
    log::info!("Save score leg_id:{:?}, score:{:?}", leg_id, score);
    DB.with(|f| {
        f.execute(
            "INSERT INTO score (leg_id, throw_order, thrown, remaining) VALUES (?1,?2, ?3, ?4)",
            (&leg_id, &score.throw_order, &score.thrown, &score.remaining),
        )
    })?;
    Ok(())
}

#[server]
pub async fn delete_score_by_order(leg_id: u16, throw_order: u16) -> Result<(), ServerFnError> {
    DB.with(|f| {
        f.execute(
            "UPDATE score SET deleted = 1 where throw_order = ?1 and leg_id = ?2",
            &[&throw_order, &leg_id],
        )
    })?;
    Ok(())
}

use crate::domain::{Leg, Match, Score, Set, INIT_SCORE};
use dioxus::prelude::*;
use dioxus::prelude::{server, ServerFnError};
use dotenv::dotenv;
use std::env;
use tracing::debug;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use std::sync::Arc;

#[cfg(feature = "server")]
mod server_deps {
    pub use diesel::prelude::*;
    pub use crate::backend::models::*;
    pub use crate::schema_manual::guard::dartset::match_id;
    pub use crate::schema_manual::guard::dartleg::dsl::dartleg;
    pub use diesel::sqlite::SqliteConnection;
    pub use crate::backend::models::DartLeg;
    pub use diesel::query_dsl::methods::OrderDsl;
    pub use crate::schema_manual::guard::dartset::dsl::dartset;
}

#[cfg(feature = "server")]
use server_deps::*;

#[cfg(feature = "server")]
pub static DB2: Lazy<Mutex<SqliteConnection>> =
    Lazy::new(|| {
    dotenv().ok();
    let url_maybe = env::var("DATABASE_URL");
    let database_url: String;
    match url_maybe {
        Ok(conn_val) => {
            database_url = conn_val;
            log::debug!("Connecting via env to Rusqlite  at {}", database_url);
        }
        _ => {
            panic!("Could not read DB connection")
        }
    }

    let conn = SqliteConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
    Mutex::new(conn)
    });



#[cfg(feature = "server")]
thread_local! {
    pub static DB: rusqlite::Connection = {
        dotenv().ok();
        let url_maybe = env::var("DATABASE_URL");
        let conn: String;
        match url_maybe {
            Ok(conn_val) => {
                conn = conn_val;
                log::debug!("Connecting via env to Rusqlite  at {}", conn);
            }
            _ => {
                panic!("Could not read DB connection")
            }
        }

        let conn = rusqlite:: Connection::open(conn).expect("Failed to open Database");
        conn
    };
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
pub async fn list_leg(set_id_input: i32) -> Result<Vec<Leg>, ServerFnError> {
    use crate::schema_manual::guard::dartleg::dsl::*;

    let mut conn = DB2.lock()?; // Lock to get mutable access
    let conn_ref = &mut *conn;

    let legs_db = dartleg.filter(set_id.eq(set_id_input))
        .select(DartLeg::as_select())
        .load(conn_ref)?;

    let legs = legs_db.into_iter().map(|db| Leg{id: db.id as u16, status: db.status}).collect();
    Ok(legs)
}


#[server]
pub async fn list_set(match_id_input: i32) -> Result<Vec<Set>, ServerFnError> {
    use crate::schema_manual::guard::dartset::dsl::*;

    let mut conn = DB2.lock()?; // Lock to get mutable access
    let conn_ref = &mut *conn;

    let sets_db = dartset.filter(match_id.eq(match_id_input))
        .select(DartSet::as_select())
        .load(conn_ref)?;

    let sets = sets_db.into_iter().map(|db| Set{id: db.id as u16, status: db.status}).collect();
    Ok(sets)
}
#[server]
pub async fn list_matches() -> Result<Vec<Match>, ServerFnError> {
    use crate::schema_manual::guard::dartmatch::dsl::*;

    let mut conn = DB2.lock()?; // Lock to get mutable access
    let conn_ref = &mut *conn;

    let match_db = dartmatch.select(DartMatch::as_select())
        .load(conn_ref)?;

    let matches = match_db.into_iter().map(|db| Match{id: db.id as u16, status: db.status}).collect();
    Ok(matches)
}


#[server]
pub async fn get_latest_leg() -> Result<Option<(u16, Leg)>, ServerFnError> {
    use crate::schema_manual::guard::dartleg::dsl::*;
    let mut conn = DB2.lock()?; // Lock to get mutable access
    let conn_ref = &mut *conn;

    let leg_result = diesel::QueryDsl::order(dartleg, id.desc()).first::<DartLeg>(conn_ref)?;
    let leg = Leg{id: leg_result.id as u16, status: leg_result.status};

    Ok(Some((leg_result.set_id as u16, leg)))
}

#[server]
pub async fn leg_exists(leg_id: u16) -> Result<bool, ServerFnError> {
    let res: Option<u16> = DB.with(|f| {
        let mut stmt = f.prepare("SELECT count(id) from leg where id = ?1")?;
        stmt.query_row([leg_id], |row| row.get(0))
    })?;
    res.map(|e| e > 0).ok_or(ServerFnError::MissingArg(
        "DB Error for leg_exists".to_string(),
    ))
}

#[server]
pub async fn create_leg_chain() -> Result<(), ServerFnError> {
    // use later as quickstart from main panel
    use crate::schema_manual::guard::dartmatch;
    use crate::schema_manual::guard::dartset;
    use crate::schema_manual::guard::dartleg;

    let mut conn = DB2.lock()?; // Lock to get mutable access
    let conn_ref = &mut *conn;

    let insert_match = NewDartMatch::new();
    let match_result = diesel::insert_into(dartmatch::table).values(insert_match)
        .returning(DartMatch::as_returning())
        .get_result(conn_ref)?;

    let insert_set = NewDartSet::new(match_result.id);
    let set_result = diesel::insert_into(dartset::table).values(insert_set)
        .returning(DartSet::as_returning())
        .get_result(conn_ref)?;

    let insert_leg = NewDartLeg::new(set_result.id);
    let leg_result = diesel::insert_into(dartleg::table).values(insert_leg)
        .returning(DartLeg::as_returning())
        .get_result(conn_ref)?;
    Ok(())
}

#[server]
pub async fn new_match() -> Result<Match, ServerFnError> {
    use crate::schema_manual::guard::dartmatch;

    let mut conn = DB2.lock()?; // Lock to get mutable access
    let conn_ref = &mut *conn;

    let insert_match = NewDartMatch::new();
    let match_result = diesel::insert_into(dartmatch::table).values(insert_match)
        .returning(DartMatch::as_returning())
        .get_result(conn_ref)?;
    Ok((Match{id:match_result.id as u16, status:match_result.status}))
}

#[server]
pub async fn new_set(match_id_input: i32) -> Result<Set, ServerFnError> {
    use crate::schema_manual::guard::dartset;

    let mut conn = DB2.lock()?; // Lock to get mutable access
    let conn_ref = &mut *conn;

    let insert_set = NewDartSet::new(match_id_input);
    let set_result = diesel::insert_into(dartset::table).values(insert_set)
        .returning(DartSet::as_returning())
        .get_result(conn_ref)?;
    Ok((Set{id:set_result.id as u16, status:set_result.status}))
}

#[server]
pub async fn new_score(leg_id_input: i32, score_input: Score) -> Result<(), ServerFnError> {

    let mut conn = DB2.lock()?; // Lock to get mutable access
    let conn_ref = &mut *conn;

    new_score_with_connection(conn_ref, leg_id_input, score_input)?;
    Ok(())
}

#[cfg(feature = "server")]
fn new_score_with_connection(conn_ref: &mut SqliteConnection, leg_id_input: i32, score_input: Score) -> Result<(), ServerFnError> {
    let insert_score = NewDartScore::new(leg_id_input, score_input.throw_order as i32,
                                         score_input.thrown as i32, score_input.remaining as i32);
    use crate::schema_manual::guard::score;
    let _ = diesel::insert_into(score::table).values(insert_score)
        .returning(DartScore::as_returning())
        .get_result(conn_ref)?;
    Ok(())
}



#[server]
pub async fn new_leg_init_score(set_id_input: i32) -> Result<Leg, ServerFnError> {
    use crate::schema_manual::guard::dartleg;

    let mut conn = DB2.lock()?; // Lock to get mutable access
    let conn_ref = &mut *conn;

    let insert_leg = NewDartLeg::new(set_id_input);
    let leg_result = diesel::insert_into(dartleg::table).values(insert_leg)
        .returning(DartLeg::as_returning())
        .get_result(conn_ref)?;
    new_score_with_connection(conn_ref, leg_result.id, INIT_SCORE)?;
    Ok((Leg{id: leg_result.id as u16, status: leg_result.status}))
}

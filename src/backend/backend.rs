use crate::domain::{Leg, Score};
use dioxus::prelude::*;
use dioxus::prelude::{server, ServerFnError};
use dotenv::dotenv;
use std::env;
use tracing::debug;
use std::sync::Mutex;
use once_cell::sync::Lazy;

#[cfg(feature = "server")]
use diesel::prelude::*;
#[cfg(feature = "server")]
use crate::backend::models::*;
#[cfg(feature = "server")]
use crate::schema_manual::*;

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
        let url_maybe = env::var("SQLITE_URL");
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
pub async fn list_leg() -> Result<Vec<Leg>, ServerFnError> {
    let legs = DB.with(|f| {
        f.prepare("SELECT id, status from leg")
            .unwrap()
            .query_map([], move |row| {
                Ok(Leg {
                    id: row.get(0)?,
                    status: row.get(1).unwrap_or("Unknown status".to_string()),
                })
            })
            .unwrap()
            .map(|r| r.unwrap())
            .collect()
    });
    Ok(legs)
}

#[server]
pub async fn get_latest_leg() -> Result<u16, ServerFnError> {
    let res: Option<u16> = DB.with(|f| {
        let mut stmt = f.prepare("SELECT max(id) from leg")?;
        stmt.query_row([], |row| row.get(0))
    })?;
    res.ok_or(ServerFnError::MissingArg(
        "Could not find max id".to_string(),
    ))
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
pub async fn save_leg(leg: Leg) -> Result<(), ServerFnError> {
    use crate::schema_manual::guard::dartmatch::dsl::*;
    use crate::schema_manual::guard::dartmatch;

    let new_match = NewMatch::new();
    let mut conn = DB2.lock()?; // Lock to get mutable access
    let conn_ref = &mut *conn;
    diesel::insert_into(dartmatch::table).values(new_match).returning(DartMatch::as_returning())
        .get_result(conn_ref).expect("Error saving new Match");

    Ok(())
}

use crate::domain::{Leg, Score};
use dioxus::prelude::*;
use dioxus::prelude::{server, ServerFnError};
use std::env;
use tracing::debug;

#[cfg(feature = "server")]
#[rustfmt::skip] 
#[allow(clippy::unused)]
thread_local! {
    pub static DB: rusqlite::Connection = {

        let url_maybe = env::var("SQLITE_URL");
        let conn: String;
        if url_maybe.is_ok() {
            conn = url_maybe.unwrap();
            debug!("Connecting via env to Rusqlite  at {:?}", conn);

        } else {
            conn = "richDarts.db".to_string();
            debug!("Connecting directly to Rusqlite  at {:?}", conn);
            }
        let conn = rusqlite:: Connection::open(conn).expect("Failed to open Database");
        conn.execute_batch (
            "

                            CREATE TABLE IF NOT EXISTS leg
                            (
                                id     INTEGER PRIMARY KEY,
                                status TEXT
                            );


                            CREATE TABLE IF NOT EXISTS score
                            (
                                id          INTEGER PRIMARY KEY,
                                leg_id      INTEGER,
                                throw_order INTEGER,
                                thrown      INTEGER,
                                remaining   INTEGER,
                                deleted     BOOLEAN NOT NULL CHECK (deleted in (0, 1)) DEFAULT 0,
                                FOREIGN KEY (leg_id) REFERENCES leg (id)
                            );


            ",
        ).expect("Table creation failed");

        conn
    };
}

#[server]
pub async fn save_score(leg_id: u16, score: Score) -> Result<(), ServerFnError> {
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
    DB.with(|f| {
        f.execute(
            "INSERT INTO leg (id, status) VALUES (?1,?2)",
            (&leg.id, &leg.status),
        )
    })?;
    Ok(())
}

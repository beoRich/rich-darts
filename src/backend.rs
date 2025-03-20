use dioxus::prelude::*;
use dioxus::prelude::{server, ServerFnError};
use dioxus::prelude::server_fn::error::ServerFnErrorErr;
use itertools::Itertools;
use crate::domain::CurrentScore;

#[cfg(feature = "server")]
thread_local! {
    pub static DB: rusqlite::Connection = {
        let conn = rusqlite:: Connection::open("richDarts.db").expect("Failed to open Database");
        conn.execute_batch (
            "

                            CREATE TABLE IF NOT EXISTS leg
                            (
                                id     INTEGER PRIMARY KEY,
                                status TEXT
                            );


                            CREATE TABLE IF NOT EXISTS throw
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
pub async fn save_throw(leg_id: u16, current_score: CurrentScore) -> Result<(), ServerFnError> {
    DB.with(|f| f.execute("INSERT INTO throw (leg_id, throw_order, thrown, remaining) VALUES (?1,?2, ?3, ?4)", (&leg_id, &current_score.throw_order, &current_score.thrown, &current_score.remaining)))?;
    Ok(())
}

#[server]
pub async fn delete_throw_by_order(leg_id: u16, throw_order: u16) -> Result<(), ServerFnError> {
    DB.with(|f| f.execute("UPDATE throw SET deleted = 1 where throw_order = ?1 and leg_id = ?2", &[&throw_order, &leg_id]))?;
    Ok(())
}

#[server]
pub async fn list_throws(leg_id: u16) -> Result<Vec<CurrentScore>, ServerFnError> {
    let throws = DB.with(|f| {
        f.prepare("SELECT remaining, thrown, throw_order from throw where deleted = 0 and leg_id =?1")
            .unwrap()
            .query_map( [leg_id], move |row| {
                Ok(CurrentScore {
                    remaining: row.get(0)?,
                    thrown: row.get(1)?,
                    throw_order: row.get(2) ?,
                })
            })
            .unwrap()
            .map(|r| r.unwrap())
            .collect()
    });
    //let res: Vec<CurrentScore> = throws.filter_map(|e| e.ok()).collect();
    println!("{:?}", throws);
    Ok(throws)
}

#[server]
pub async fn get_latest_leg() -> Result<u16, ServerFnError> {
    let res: Option<u16> = DB.with(|f| {
        let mut stmt = f.prepare("SELECT max(id) from leg")?;
        stmt.query_row([], |row| row.get(0))
    })?;
    res.ok_or(ServerFnError::MissingArg("Could not find max id".to_string()))
}

#[server]
pub async fn save_leg(leg_id: u16) -> Result<(), ServerFnError> {
    DB.with(|f| f.execute("INSERT INTO leg (id) VALUES (?1)", &[&leg_id]))?;
    Ok(())
}

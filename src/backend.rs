use dioxus::prelude::*;
use dioxus::prelude::{server, ServerFnError};
use itertools::Itertools;
use crate::domain::CurrentScore;

#[cfg(feature = "server")]
thread_local! {
    pub static DB: rusqlite::Connection = {
        let conn = rusqlite:: Connection::open("richDarts.db").expect("Failed to open Database");
        conn.execute_batch (
            "
            CREATE TABLE IF NOT EXISTS leg (
                id INTEGER PRIMARY KEY,
                leg_id INTEGER,
                throw_order INTEGER,
                thrown INTEGER,
                remaining INTEGER,
                deleted BOOLEAN NOT NULL CHECK (deleted in (0,1)) DEFAULT 0);
            ",
        ).expect("Table creation failed");

        conn
    };
}

#[server]
pub async fn save_throw(leg_id: u16, current_score: CurrentScore) -> Result<(), ServerFnError> {
    DB.with(|f| f.execute("INSERT INTO leg (leg_id, throw_order, thrown, remaining) VALUES (?1,?2, ?3, ?4)", (&leg_id, &current_score.throw_order, &current_score.thrown, &current_score.remaining)))?;
    Ok(())
}

#[server]
pub async fn delete_throw_by_order(throw_order: u16) -> Result<(), ServerFnError> {
    DB.with(|f| f.execute("UPDATE leg SET deleted = 1 where throw_order = ?1", &[&throw_order]))?;
    Ok(())
}

#[server]
pub async fn list_throws() -> Result<Vec<CurrentScore>, ServerFnError> {
    let throws = DB.with(|f| {
        f.prepare("SELECT remaining, thrown, throw_order from leg where deleted = 0")
            .unwrap()
            .query_map( [], move |row| {
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
pub async fn list_throws_test() -> Result<Vec<(usize, u16)>, ServerFnError> {
    let throws = DB.with(|f| {
        f.prepare("SELECT id, thrown from leg where deleted = 0")
            .unwrap()
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
            .unwrap()
            .map(|r| r.unwrap())
            .collect()
    });
    //let res: Vec<CurrentScore> = throws.filter_map(|e| e.ok()).collect();
    //println!("{:?}", throws);
    Ok(throws)
}

#[server]
pub async fn list_throws_test2() -> Result<(usize, u16), ServerFnError> {
    //let res: Vec<CurrentScore> = throws.filter_map(|e| e.ok()).collect();
    //println!("{:?}", throws);
    Ok((1,1))
}

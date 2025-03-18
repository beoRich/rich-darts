use dioxus::prelude::*;
use dioxus::prelude::{server, ServerFnError};
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

use crate::domain::{IdOrder, Leg, Match, Score, Set, INIT_SCORE};
use dioxus::prelude::*;
use dioxus::prelude::{server, ServerFnError};
use dotenv::dotenv;
use once_cell::sync::Lazy;
use std::env;
use std::error::Error;
use std::sync::Arc;
use std::sync::Mutex;
use tracing::debug;

#[cfg(feature = "server")]
mod server_deps {
    pub use crate::backend::models::DartLeg;
    pub use crate::backend::models::*;
    pub use crate::schema_manual::guard::dartleg::dsl::dartleg;
    pub use crate::schema_manual::guard::dartleg::set_id;
    pub use crate::schema_manual::guard::dartmatch::dsl::dartmatch;
    pub use crate::schema_manual::guard::dartset::dsl::dartset;
    pub use crate::schema_manual::guard::dartset::match_id;
    pub use diesel::prelude::*;
    pub use diesel::query_dsl::methods::OrderDsl;
    pub use diesel::sqlite::SqliteConnection;
    pub use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
}

#[cfg(feature = "server")]
use server_deps::*;

#[cfg(feature = "server")]
pub static DB2: Lazy<Mutex<SqliteConnection>> = Lazy::new(|| {
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

    let conn = SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
    Mutex::new(conn)
});

#[server]
pub async fn run_migrations() -> Result<(), ServerFnError> {
    let mut conn = DB2.lock()?; // Lock to get mutable access
    let conn_ref = &mut *conn;
    // This will run the necessary migrations.
    //
    // See the documentation for `MigrationHarness` for
    // all available methods.
    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
    conn_ref.run_pending_migrations(MIGRATIONS).unwrap();

    Ok(())
}

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
pub async fn list_matches() -> Result<Vec<Match>, ServerFnError> {
    use crate::schema_manual::guard::dartmatch::dsl::*;

    let mut conn = DB2.lock()?; // Lock to get mutable access
    let conn_ref = &mut *conn;

    let match_db = dartmatch.select(DartMatch::as_select()).load(conn_ref)?;

    let matches = match_db
        .into_iter()
        .map(|db| Match {
            id: db.id as u16,
            status: db.status,
        })
        .collect();
    Ok(matches)
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

#[server]
pub async fn new_match() -> Result<Match, ServerFnError> {
    use crate::schema_manual::guard::dartmatch;

    let mut conn = DB2.lock()?; // Lock to get mutable access
    let conn_ref = &mut *conn;

    let insert_match = NewDartMatch::new();
    let match_result = diesel::insert_into(dartmatch::table)
        .values(insert_match)
        .returning(DartMatch::as_returning())
        .get_result(conn_ref)?;
    Ok((Match {
        id: match_result.id as u16,
        status: match_result.status,
    }))
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

#[server]
pub async fn new_score(leg_id_input: i32, score_input: Score) -> Result<(), ServerFnError> {
    let mut conn = DB2.lock()?; // Lock to get mutable access
    let conn_ref = &mut *conn;

    new_score_with_connection(conn_ref, leg_id_input, score_input)?;
    Ok(())
}

#[cfg(feature = "server")]
fn new_score_with_connection(
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
    new_score_with_connection(conn_ref, leg_result.id, INIT_SCORE)?;
    Ok((Leg {
        id: leg_result.id as u16,
        status: leg_result.status,
        leg_order: leg_order_val,
    }))
}

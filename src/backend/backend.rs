use dioxus::prelude::*;
use dioxus::prelude::{server, ServerFnError};
use std::env;
use tracing::debug;

#[cfg(feature = "server")]
mod server_deps {
    pub use diesel::prelude::*;
    pub use diesel::query_dsl::methods::OrderDsl;
    pub use diesel::sqlite::SqliteConnection;
    pub use diesel_migrations::FileBasedMigrations;
    pub use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
    pub use dotenv::dotenv;
    pub use once_cell::sync::Lazy;
    pub use std::sync::Mutex;
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
    dotenv::dotenv().ok();
    let url_maybe = env::var("MIGRATION_URI");
    let migration_uri: String;
    match url_maybe {
        Ok(val) => {
            migration_uri = val;
            log::debug!("Found migration dir  at {}", migration_uri);
        }
        _ => {
            panic!("Could not find diesel migration files")
        }
    }

    let mut conn = DB2.lock()?; // Lock to get mutable access
    let conn_ref = &mut *conn;
    // This will run the necessary migrations.
    //
    // See the documentation for `MigrationHarness` for
    // all available methods.
    let file_base_migration = FileBasedMigrations::from_path(migration_uri)?;
    conn_ref
        .run_pending_migrations(file_base_migration)
        .unwrap();

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

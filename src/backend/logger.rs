use dioxus::prelude::*;
use dioxus::prelude::{server, ServerFnError};

#[server]
pub async fn log_init() -> Result<(), ServerFnError> {
    dotenv::dotenv().ok();
    let url_maybe = env::var("LOG_URL");
    let log_file: String;
    match url_maybe {
        Ok(val) => {
            log_file = val;
        }
        _ => {
            panic!("Could not read log file env path")
        }
    }

    use log::LevelFilter;
    use log4rs::append::console::ConsoleAppender;
    use log4rs::append::file::FileAppender;
    use log4rs::config::{Appender, Config, Logger, Root};
    use log4rs::encode::pattern::PatternEncoder;
    use std::env;

    let stdout = ConsoleAppender::builder().build();
    let server = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{h({d(%Y-%m-%d %H:%M:%S)(utc)} - {l}: {m}{n})}",
        )))
        .build(log_file)?;

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("server", Box::new(server)))
        .logger(
            Logger::builder()
                .appender("server")
                .build("app::server", LevelFilter::Debug),
        )
        .build(
            Root::builder()
                .appender("stdout")
                .appender("server")
                .build(LevelFilter::Debug),
        )?;

    // Log the initialization
    let handle = log4rs::init_config(config)?;
    log::info!("Logger initialized successfully in the backend.");
    Ok(())
}
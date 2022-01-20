mod config;
mod tasks;

use std::io::Error;

use env_logger;
use log;

use crate::config::Config;
use crate::tasks::manager::{Manager, refresh_definitions};

fn main() -> Result<(), Error> {
    env_logger::builder().parse_env("LOG_LEVEL").init();
    log::info!("Initialization...");
    let config = Config::new()?;
    let mut task_manager = Manager::new();
    log::info!("Starting...");

    // Definitions might be refreshed with some interval in thread
    refresh_definitions(&config, &mut task_manager)?;

    log::info!("Application terminated.");
    Ok(())
}

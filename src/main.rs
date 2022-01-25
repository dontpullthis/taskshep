mod config;
mod tasks;

use std::collections::HashMap;
use std::io::Error;

use crate::tasks::task_definition::TaskDefinition;

use env_logger;
use log;

use crate::config::Config;
use crate::tasks::manager::refresh_definitions;

fn main() -> Result<(), Error> {
    env_logger::builder().parse_env("LOG_LEVEL").init();
    log::info!("Initialization...");
    let config = Config::new()?;
    let mut definitions: HashMap<String, TaskDefinition> = HashMap::new();
    log::info!("Starting...");

    // Definitions might be refreshed with some interval in thread
    refresh_definitions(&config, &mut definitions)?;

    log::info!("Application terminated.");
    Ok(())
}

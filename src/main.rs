mod config;
mod tasks;

use std::fs;
use std::io::Error;

use env_logger;
use log;
use serde_yaml;

use crate::config::Config;
use crate::tasks::manager::Manager;

fn main() -> Result<(), Error> {
    env_logger::builder().parse_env("LOG_LEVEL").init();
    log::info!("Starting...");
    let config = Config::new()?;
    let mut task_manager = Manager::new();
    for entry in fs::read_dir(config.dir_tasks)? {
        let entry = entry?;
        let path = entry.path().display().to_string();
        if entry.path().is_dir() || !path.ends_with(".yaml") {
            continue;
        }

        log::debug!("Scanning file {}...", &path);
        let contents = fs::read_to_string(&path)?;
        let tasks = match serde_yaml::from_str(&contents) {
            Ok(t) => t,
            Err(e) => {
                log::error!("Failed to parse the YAML file \"{}\": {}", &path, e);
                continue
            }  
        };

        task_manager.merge_definitions(tasks);
    }
    log::info!("Application terminated.");
    Ok(())
}

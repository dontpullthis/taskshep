mod config;
mod tasks;

use std::fs;
use std::io::{Error, ErrorKind};

use env_logger;
use log;
use serde_yaml;

use crate::config::Config;
use crate::tasks::task_definition::TaskDefinition;


fn main() -> Result<(), Error> {
    env_logger::builder().parse_env("LOG_LEVEL").init();
    let config = Config::new()?;
    for entry in fs::read_dir(config.dir_tasks)? {
        let entry = entry?;
        let path = entry.path().display().to_string();
        if entry.path().is_dir() || !path.ends_with(".yaml") {
            continue;
        }

        log::debug!("Scanning file {}...", &path);
        let contents = fs::read_to_string(&path)?;
        let _tasks: Vec<TaskDefinition> = match serde_yaml::from_str(&contents) {
            Ok(t) => Ok(t),
            Err(_) => Err(Error::new(ErrorKind::Other, format!("Failed to parse the YAML file \"{}\"", &path))), // TODO: format a proper YAML error message
        }?;
    }
    Ok(())
}

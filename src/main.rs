use std::env;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;

use env_logger;
use log;
use serde::{Serialize, Deserialize};
use serde_yaml;

use dirs;

fn get_default_home_dir() -> Result<String, Error> {
    let home_dir = match dirs::home_dir() {
        Some(d) => Ok(d),
        None => Err(Error::new(ErrorKind::Other, "Cannot locate a default home directory.")),
    }?;

    let home_dir = match Path::new(&home_dir).join(".taskshep").into_os_string().into_string() {
        Ok(h) => Ok(h),
        Err(_) => Err(Error::new(ErrorKind::Other, "Cannot format the path of home directory.")),
    }?;

    Ok(home_dir)
}

fn get_home_dir() -> Result<String, Error> {
    match env::var("TASKSHEP_HOME") {
        Ok(h) => Ok(h),
        Err(_) => get_default_home_dir(),
    }
}

struct Config {
    dir_tasks: String,
}

impl Config {
    fn new() -> Result<Config, Error> {
        let dir_home = get_home_dir()?;
        Ok(Config {
            dir_tasks: Path::new(dir_home.clone().as_str()).join("tasks").display().to_string(),
        })
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ScheduleRuleInterval {
    min_value: Option<u16>,
    max_value: Option<u16>,
    value: Option<u16>,
    step: Option<u16>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ScheduleRule {
    days_of_month: Option<Vec<ScheduleRuleInterval>>,
    days_of_week: Option<Vec<ScheduleRuleInterval>>,
    hours: Option<Vec<ScheduleRuleInterval>>,
    months: Option<Vec<ScheduleRuleInterval>>,
    minutes: Option<Vec<ScheduleRuleInterval>>,
    seconds: Option<Vec<ScheduleRuleInterval>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct RunAfter {
    task: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Task {
    id: String,
    name: Option<String>,
    test: Option<String>,
    command: String,
    run_after: Option<RunAfter>,
    schedule: Option<Vec<ScheduleRule>>,
}

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
        let _tasks: Vec<Task> = match serde_yaml::from_str(&contents) {
            Ok(t) => Ok(t),
            Err(_) => Err(Error::new(ErrorKind::Other, format!("Failed to parse the YAML file \"{}\"", &path))), // TODO: format a proper YAML error message
        }?;
    }
    Ok(())
}

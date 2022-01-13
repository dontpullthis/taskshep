use std::env;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;

use env_logger;
use log;

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

fn main() -> Result<(), Error> {
    env_logger::builder().parse_env("LOG_LEVEL").init();
    let config = Config::new()?;
    for entry in fs::read_dir(config.dir_tasks)? {
        let entry = entry?;
        let path = entry.path().display().to_string();
        if entry.path().is_dir() || !path.ends_with(".yaml") {
            continue;
        }
        log::debug!("Scanning file {}", &path);
    }
    Ok(())
}

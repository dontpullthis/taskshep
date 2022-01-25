mod config;
mod tasks;

use std::collections::HashMap;
use std::collections::LinkedList;
use std::io::Error;


use crate::tasks::task_definition::TaskDefinition;

use chrono::{Duration, Utc};
use env_logger;
use log;

use crate::config::Config;
use crate::tasks::manager::refresh_definitions;

struct ScheduleItem {
    task_id: String,
    time: i64,
}

impl ScheduleItem {
    fn new(task_id: String,  time: i64) -> ScheduleItem {
        ScheduleItem {task_id, time}
    }
}

fn refresh_schedule(definitions: &HashMap<String, TaskDefinition>, schedule: &mut HashMap<String, LinkedList<ScheduleItem>>) -> Result<(), Error> {
    let time_start = chrono::Local::now();
    let time_end = time_start + Duration::hours(1);
    for (task_id, task_def) in definitions {
        let task_def_schedule = match &task_def.schedule {
            Some(ts) => ts,
            None => continue,
        };

        if !schedule.contains_key(&task_id.clone()) {
            schedule.insert(task_id.clone(), LinkedList::new());
        }
        let task_schedule = schedule.get_mut(task_id).unwrap();

        let mut time = time_start;
        while time < time_end {
            time = time + Duration::minutes(15); // TODO: implement calculation of time
            task_schedule.push_back(ScheduleItem::new(task_id.clone(), time.timestamp()));
        }
    }
    Ok(())
}

fn main() -> Result<(), Error> {
    env_logger::builder().parse_env("LOG_LEVEL").init();
    log::info!("Initialization...");
    let config = Config::new()?;
    let mut definitions: HashMap<String, TaskDefinition> = HashMap::new();
    log::info!("Starting...");

    // Definitions might be refreshed with some interval in thread
    refresh_definitions(&config, &mut definitions)?;

    let mut schedule: HashMap<String, LinkedList<ScheduleItem>> = HashMap::new();
    refresh_schedule(&definitions, &mut schedule)?;

    log::info!("Application terminated.");
    Ok(())
}

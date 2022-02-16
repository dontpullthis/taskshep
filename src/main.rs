mod config;
mod scheduler;
mod tasks;

use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::Error;
use std::process::Command;
use std::thread;

use crate::scheduler::types::{TaskSchedule, ScheduleItem};
use crate::tasks::task_definition::TaskDefinition;

use chrono::Duration;
use env_logger;
use log;
use shlex;

use crate::config::Config;
use crate::tasks::manager::refresh_definitions;

fn flatten_schedule(schedule: HashMap<String, TaskSchedule>) -> TaskSchedule {
    let mut result: VecDeque<ScheduleItem> = VecDeque::new();
    for (_, mut schedule_item) in schedule {
        result.append(&mut schedule_item);
    }

    let mut result = result.into_iter().collect::<Vec<ScheduleItem>>();
    result.sort_by(|a, b| b.time.partial_cmp(&a.time).unwrap());

    VecDeque::from(result)
}

fn main() -> Result<(), Error> {
    env_logger::builder().parse_env("LOG_LEVEL").init();
    log::info!("Initialization...");
    let config = Config::new()?;
    let mut definitions: HashMap<String, TaskDefinition> = HashMap::new();
    log::info!("Starting...");

    // Definitions might be refreshed with some interval in thread
    refresh_definitions(&config, &mut definitions)?;

    let schedule = scheduler::generate_schedule(&definitions)?;

    let mut schedule = flatten_schedule(schedule);

    loop {
        let scheduled_task = match schedule.pop_back() {
            Some(t) => t,
            None => break,
        };
        let task_def = match definitions.get(&scheduled_task.task_id) {
            Some(d) => d,
            None => {
                log::warn!("Cannot find a task definition: {}", &scheduled_task.task_id);
                continue;
            }
        };

        let now = chrono::Local::now().timestamp();
        let sleep_duration = match Duration::seconds(scheduled_task.time - now).to_std() {
            Ok(d) => d,
            Err(_) => std::time::Duration::from_secs(0),
        };
        thread::sleep(sleep_duration);

        log::debug!("Running a task: {}", &task_def.command);
        let (cmd, args) = match shlex::split(&task_def.command) {
            Some(a) => {
                let cmd = a[0].clone();
                let args = Vec::from(&a[1..]);
                (cmd, args)
            },
            None => {
                log::warn!("Failed to parse arguments for command: {}", &task_def.command);
                continue;
            }
        };
        match Command::new(cmd).args(args).spawn() {
            Ok(_c) => {},
            Err(e) => {
                log::warn!("Process failed: {}\nOutput:\n{}", &task_def.command, e);
            }
        }
    }

    log::info!("Application terminated.");
    Ok(())
}

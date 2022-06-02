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



fn main() -> Result<(), Error> {
    env_logger::builder().parse_env("LOG_LEVEL").init();
    log::info!("Initialization...");
    let config = Config::new()?;
    let mut definitions: HashMap<String, TaskDefinition> = HashMap::new();
    log::info!("Starting...");

    // Definitions might be refreshed with some interval in thread
    refresh_definitions(&config, &mut definitions)?;

    let schedule = scheduler::generate_schedule(&definitions)?;
    let mut schedule_items = schedule.flatten_items();

    loop {
        let scheduled_task = match schedule_items.pop_front() {
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

        let mut process = Command::new(cmd)
            .args(args)
            .spawn()
            .expect(format!("Failed to start a process: {}", &task_def.command).as_str()); // TODO: more detailed error message
        process.wait().expect(format!("Failed to wait for process: {}", &task_def.command).as_str()); // TODO: more detailed error message
        log::debug!("Task \"{}\" is finished.", &task_def.id);

        match schedule.dependencies.get(&task_def.id) {
            Some(deps) => {
                for dep in deps {
                    schedule_items.push_front(ScheduleItem::new(dep.clone(), chrono::Local::now().timestamp()));
                }
            },
            None => {},
        }
    }

    log::info!("Application terminated.");
    Ok(())
}

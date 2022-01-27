mod config;
mod tasks;

use std::collections::HashMap;
use std::collections::VecDeque;
use std::ffi::OsStr;
use std::io::Error;
use std::process::Command;
use std::thread;

use crate::tasks::task_definition::TaskDefinition;

use chrono::{Duration, Utc};
use env_logger;
use log;
use shlex;

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

enum ScheduleInterval {
    Seconds,
    Minutes,
    Hours,
    DaysOfWeek,
    DaysOfMonth,
    Months,
}

fn refresh_schedule(definitions: &HashMap<String, TaskDefinition>, schedule: &mut HashMap<String, VecDeque<ScheduleItem>>) -> Result<(), Error> {
    let time_start = chrono::Local::now();
    let time_end = time_start + Duration::minutes(10);
    for (task_id, task_def) in definitions {
        let task_rules = match &task_def.schedule {
            Some(sr) => sr,
            None => continue,
        };

        if !schedule.contains_key(&task_id.clone()) {
            schedule.insert(task_id.clone(), VecDeque::new());
        }
        let task_schedule = schedule.get_mut(task_id).unwrap();

        for (i, task_rule) in task_rules.iter().enumerate() {
            let mut least_interval: Option<ScheduleInterval> = None;
            if task_rule.seconds.is_some() {
                least_interval = Some(ScheduleInterval::Seconds);
            }
            if task_rule.minutes.is_some() {
                least_interval = Some(ScheduleInterval::Minutes);
            }
            if task_rule.hours.is_some() {
                least_interval = Some(ScheduleInterval::Hours);
            }
            if task_rule.days_of_week.is_some() {
                least_interval = Some(ScheduleInterval::DaysOfWeek);
            }
            if task_rule.days_of_month.is_some() {
                least_interval = Some(ScheduleInterval::DaysOfMonth);
            }
            if task_rule.months.is_some() {
                least_interval = Some(ScheduleInterval::Months);
            }
            
            if least_interval.is_none() {
                log::warn!("Interval is not defined in rule #{}, task \"{}\"", i, task_id);
                continue;
            }

            // TODO: implement scheduling

            let mut time = time_start;
            while time < time_end {
                time = time + Duration::seconds(3); // TODO: implement calculation of time
                task_schedule.push_back(ScheduleItem::new(task_id.clone(), time.timestamp()));
            }
        }
    }
    Ok(())
}

fn flatten_schedule(schedule: HashMap<String, VecDeque<ScheduleItem>>) -> VecDeque<ScheduleItem> {
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

    let mut schedule: HashMap<String, VecDeque<ScheduleItem>> = HashMap::new();
    refresh_schedule(&definitions, &mut schedule)?;

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
            Ok(c) => {},
            Err(e) => {
                log::warn!("Process failed: {}\nOutput:\n{}", &task_def.command, e);
            }
        }
    }

    log::info!("Application terminated.");
    Ok(())
}

pub mod types;

use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::Error;

use crate::tasks::task_definition::{ScheduleRule, TaskDefinition};
use crate::scheduler::types::{ScheduleItem, ScheduleInterval, TaskSchedule};

use chrono::Duration;
use log;

struct TaskContext<'a> {
    task_def: &'a TaskDefinition,
    schedule: &'a mut TaskSchedule,
    time_end: chrono::DateTime<chrono::Local>,
    time_start: chrono::DateTime<chrono::Local>,
}

fn process_task_rule(rule_index: usize, rule: &ScheduleRule, context: TaskContext) {
    let mut least_interval: Option<ScheduleInterval> = None;
    if rule.seconds.is_some() {
        least_interval = Some(ScheduleInterval::Seconds);
    }
    if rule.minutes.is_some() {
        least_interval = Some(ScheduleInterval::Minutes);
    }
    if rule.hours.is_some() {
        least_interval = Some(ScheduleInterval::Hours);
    }
    if rule.days_of_week.is_some() {
        least_interval = Some(ScheduleInterval::DaysOfWeek);
    }
    if rule.days_of_month.is_some() {
        least_interval = Some(ScheduleInterval::DaysOfMonth);
    }
    if rule.months.is_some() {
        least_interval = Some(ScheduleInterval::Months);
    }
    
    if least_interval.is_none() {
        log::warn!("Interval is not defined in rule #{}, task \"{}\"", rule_index, context.task_def.id);
        return;
    }

    // TODO: implement scheduling

    let mut time = context.time_start;
    while time < context.time_end {
        time = time + Duration::seconds(3); // TODO: implement calculation of time
        context.schedule.push_back(ScheduleItem::new(context.task_def.id.clone(), time.timestamp()));
    }
}

pub fn generate_schedule(definitions: &HashMap<String, TaskDefinition>) -> Result<HashMap<String, VecDeque<ScheduleItem>>, Error> {
    let mut schedule: HashMap<String, TaskSchedule> = HashMap::new();
    let time_start = chrono::Local::now();
    let time_end = time_start + Duration::minutes(10);
    for (task_id, task_def) in definitions {
        let task_rules = match &task_def.schedule {
            Some(sr) => sr,
            None => continue,
        };

        let mut task_schedule = VecDeque::new();
        for (i, task_rule) in task_rules.iter().enumerate() {
            process_task_rule(i, task_rule, TaskContext{
                schedule: &mut task_schedule,
                task_def: task_def,
                time_end: time_end,
                time_start: time_start,
            });
        }
        schedule.insert(task_id.clone(), task_schedule);
    }
    Ok(schedule)
}
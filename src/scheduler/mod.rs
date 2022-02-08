pub mod types;

use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::Error;

use crate::tasks::task_definition::TaskDefinition;
use crate::scheduler::types::{ScheduleItem, ScheduleInterval};

use chrono::Duration;
use log;


pub fn refresh_schedule(schedule: &mut HashMap<String, VecDeque<ScheduleItem>>, definitions: &HashMap<String, TaskDefinition>) -> Result<(), Error> {
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
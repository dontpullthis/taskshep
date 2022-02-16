use std::collections::VecDeque;

pub struct ScheduleItem {
    pub task_id: String,
    pub time: i64,
}

impl ScheduleItem {
    pub fn new(task_id: String,  time: i64) -> ScheduleItem {
        ScheduleItem {task_id, time}
    }
}

pub enum ScheduleInterval {
    Seconds,
    Minutes,
    Hours,
    DaysOfWeek,
    DaysOfMonth,
    Months,
}

/// Ordered sequence of schedule items for one or more tasks
pub type TaskSchedule = VecDeque<ScheduleItem>;
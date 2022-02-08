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
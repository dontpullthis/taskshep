use std::collections::{HashMap, VecDeque};

/// Contains scheduled tasks + helper data
pub struct Schedule {
    pub items: HashMap<String, VecDeque<ScheduleItem>>,
    pub dependencies: HashMap<String, Vec<String>>, // key: task, value: dependent task
}

/// Specifies a task which is scheduled for particular time
#[derive(Clone)]
pub struct ScheduleItem {
    pub task_id: String,
    pub time: i64,
}

impl Schedule {
    pub fn new() -> Schedule {
        Schedule {
            items: HashMap::new(),
            dependencies: HashMap::new(),
        }
    }

    /// Merges a schedule items map (task_id -> time) into single set
    pub fn flatten_items(&self) -> TaskSchedule {
        let mut result: VecDeque<ScheduleItem> = VecDeque::new();
        for (_, schedule_item) in &self.items {
            let mut i = schedule_item.clone();
            result.append(&mut i);
        }

        let mut result = result.into_iter().collect::<Vec<ScheduleItem>>();
        result.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());

        VecDeque::from(result)
    }

    pub fn add_dependency(&mut self, main_task_id: &String, dependent_task_id: &String) {
        if !self.dependencies.contains_key(main_task_id) {
            self.dependencies.insert(main_task_id.clone(), Vec::new());
        }
        let items = self.dependencies.get_mut(main_task_id).unwrap();
        items.push(dependent_task_id.clone());
    }
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
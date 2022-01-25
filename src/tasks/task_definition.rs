use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
struct ScheduleRuleInterval {
    min_value: Option<u16>,
    max_value: Option<u16>,
    value: Option<u16>,
    step: Option<u16>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ScheduleRule {
    days_of_month: Option<Vec<ScheduleRuleInterval>>,
    days_of_week: Option<Vec<ScheduleRuleInterval>>,
    hours: Option<Vec<ScheduleRuleInterval>>,
    months: Option<Vec<ScheduleRuleInterval>>,
    minutes: Option<Vec<ScheduleRuleInterval>>,
    seconds: Option<Vec<ScheduleRuleInterval>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
struct RunAfter {
    task: String,
}

/// Task definition: command to run, schedule or reference to upstream task
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TaskDefinition {
    pub id: String,
    name: Option<String>,
    test: Option<String>,
    command: String,
    run_after: Option<RunAfter>,
    pub schedule: Option<Vec<ScheduleRule>>,
}
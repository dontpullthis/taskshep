use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ScheduleRuleInterval {
    pub min_value: Option<u16>,
    pub max_value: Option<u16>,
    pub value: Option<u16>,
    pub step: Option<u16>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ScheduleRule {
    pub days_of_month: Option<Vec<ScheduleRuleInterval>>,
    pub days_of_week: Option<Vec<ScheduleRuleInterval>>,
    pub hours: Option<Vec<ScheduleRuleInterval>>,
    pub months: Option<Vec<ScheduleRuleInterval>>,
    pub minutes: Option<Vec<ScheduleRuleInterval>>,
    pub seconds: Option<Vec<ScheduleRuleInterval>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RunAfter {
    pub task: String,
}

/// Task definition: command to run, schedule or reference to upstream task
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TaskDefinition {
    pub id: String,
    pub name: Option<String>,
    pub command: String,
    pub run_after: Option<RunAfter>,
    pub schedule: Option<Vec<ScheduleRule>>,
}
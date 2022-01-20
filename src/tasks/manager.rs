use std::collections::HashMap;

use crate::tasks::task_definition::TaskDefinition;

pub struct Manager {
    pub definitions: HashMap<String, TaskDefinition>
}

impl Manager {
    pub fn new() -> Manager {
        Manager {
            definitions: HashMap::new(),
        }
    }

    pub fn merge_definitions(&mut self, new_defs: &Vec<TaskDefinition>) {
        for def in new_defs {
            self.definitions.insert(def.id.clone(), def.clone());
        }
        // self.definitions.append(&mut new_defs); // TODO: lookup tasks by ID, merge exisiting tasks
    }
}
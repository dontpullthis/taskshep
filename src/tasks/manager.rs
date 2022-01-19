use crate::tasks::task_definition::TaskDefinition;

pub struct Manager {
    pub definitions: Vec<TaskDefinition>
}

impl Manager {
    pub fn new() -> Manager {
        Manager {
            definitions: Vec::new(),
        }
    }

    pub fn merge_definitions(&mut self, mut new_defs: Vec<TaskDefinition>) {
        self.definitions.append(&mut new_defs); // TODO: lookup tasks by ID, merge exisiting tasks
    }
}
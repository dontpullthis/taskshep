
use std::collections::HashMap;
use std::fs;

use std::io::Error;

use serde_yaml;

use crate::config::Config;
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
    }
}

pub fn refresh_definitions(config: &Config, manager: &mut Manager) -> Result<(), Error> {
    for entry in fs::read_dir(&config.dir_tasks)? {
        let entry = entry?;
        let path = entry.path().display().to_string();
        if entry.path().is_dir() || !path.ends_with(".yaml") {
            continue;
        }

        log::debug!("Scanning file {}...", &path);
        let contents = fs::read_to_string(&path)?;
        let tasks = match serde_yaml::from_str(&contents) {
            Ok(t) => t,
            Err(e) => {
                log::error!("Failed to parse the YAML file \"{}\": {}", &path, e);
                continue
            }
        };

        manager.merge_definitions(&tasks);
    }

    Ok(())
}

use std::collections::HashMap;
use std::fs;

use std::io::Error;

use serde_yaml;

use crate::config::Config;
use crate::tasks::task_definition::TaskDefinition;

pub fn refresh_definitions(config: &Config, definitions: &mut HashMap<String, TaskDefinition>) -> Result<(), Error> {
    for entry in fs::read_dir(&config.dir_tasks)? {
        let entry = entry?;
        let path = entry.path().display().to_string();
        if entry.path().is_dir() || !path.ends_with(".yaml") {
            continue;
        }

        log::debug!("Scanning file {}...", &path);
        let contents = fs::read_to_string(&path)?;
        let tasks: Vec<TaskDefinition> = match serde_yaml::from_str(&contents) {
            Ok(t) => t,
            Err(e) => {
                log::error!("Failed to parse the YAML file \"{}\": {}", &path, e);
                continue
            }
        };
        
        for task in &tasks {
            definitions.insert(task.id.clone(), task.clone());
        }
    }

    Ok(())
}
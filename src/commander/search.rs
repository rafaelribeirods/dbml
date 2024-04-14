use std::collections::HashMap;

use anyhow::{anyhow, Result};
use regex::Regex;
use crate::config::{self, Config};

use super::Command;

pub struct SearchCommand {
    pub project: String,
    pub regex: String,
    pub referenced_key: Option<String>,
}

impl Command for SearchCommand {

    async fn execute(&self) -> Result<()> {
        println!("Searching for columns that match '{}' in the '{}' project", self.regex, self.project);

        let mut config: Config = config::load(&self.project)?;
        let column_name_regex = Regex::new(&self.regex).unwrap();
        let referenced_key_regex = Regex::new(r"^(.+)___(.+)\.(.+)$").unwrap();
        let mut referenced_database_name = "";
        let mut referenced_table_name = "";
        let mut referenced_column_name = "";
        let mut should_add_references = false;

        let table_name_joined;
        if let Some(referenced_key) = &self.referenced_key {
            if !referenced_key_regex.is_match(self.referenced_key.as_ref().unwrap()) {
                return Err(anyhow!("The referenced key must follow this pattern: DATABASE_NAME___TABLE_NAME.COLUMN_NAME"));
            }

            let parts_by_period: Vec<&str> = referenced_key.split(".").collect();
            referenced_column_name = parts_by_period[1];
            let parts_by_underscore: Vec<&str> = parts_by_period[0].split("___").collect();
            referenced_database_name = parts_by_underscore[0];
            table_name_joined = parts_by_underscore[1..].join("___");
            referenced_table_name = &table_name_joined;

            if !config.databases.contains_key(referenced_database_name)
            || !config.databases.get(referenced_database_name).unwrap().tables.clone().unwrap().contains_key(referenced_table_name)
            || !config.databases.get(referenced_database_name).unwrap().tables.clone().unwrap().get(referenced_table_name).unwrap().columns.contains_key(referenced_column_name) {
                return Err(anyhow!("The given refereced key '{}' does not match a mapped column in the config file for project {}", self.referenced_key.clone().unwrap(), self.project));
            }

            should_add_references = true;
        }

        for (database_name, database) in &mut config.databases {
            if let Some(tables) = &mut database.tables {
                println!("Searching on {} tables...", database_name);
                for (table_name, table) in tables {    
                    for (column_name, _) in &table.columns {
                        if column_name_regex.is_match(column_name) {
                            let key = format!("{}___{}.{}", database_name, table_name, column_name);
                            if (config.references.is_none() 
                            || !config.references.as_ref().unwrap().contains_key(&key))
                            && (config.custom_references.is_none()
                            || !config.custom_references.clone().unwrap().contains_key(&key)) {
                                println!("{}", format!("Found an unmapped column matching '{}': {} ({}___{})", self.regex, column_name, database_name, table_name));
                                if should_add_references {
                                    
                                    let referenced_key = format!("{}___{}.{}", referenced_database_name, referenced_table_name, referenced_column_name);
                                    match config.custom_references {
                                        Some(ref mut map) => {
                                            map.entry(key.clone())
                                                .or_insert_with(Vec::new)
                                                .push(referenced_key);
                                        }
                                        None => {
                                            let mut new_map = HashMap::new();
                                            let mut new_vec = Vec::new();
                                            new_vec.push(referenced_key);
                                            new_map.insert(key.clone(), new_vec);
                                            config.custom_references = Some(new_map);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        config.save()?;
        
        Ok(())
    }
    
    fn get_starting_message(&self) -> String {
        format!("Looking for unmapped columns in the {} project that match {}", self.project, self.regex)
    }

}
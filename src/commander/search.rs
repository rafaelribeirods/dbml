use anyhow::Result;
use regex::Regex;
use crate::config::{self, Config};

use super::Command;

pub struct SearchCommand {
    pub project: String,
    pub regex: String,
}

impl Command for SearchCommand {

    async fn execute(&self) -> Result<()> {
        println!("Searching for columns that match '{}' in the '{}' project", self.regex, self.project);

        let mut config: Config = config::load(&self.project)?;
        let re = Regex::new(&self.regex).unwrap();

        for (database_name, database) in &mut config.databases {
            if let Some(tables) = &mut database.tables {
                println!("Searching on {} tables...", database_name);
                for (table_name, table) in tables {    
                    for (column_name, _) in &table.columns {
                        if re.is_match(column_name) {
                            if database.references.is_none() || !database.references.as_ref().unwrap().contains_key(&format!("{}___{}.{}", database_name, table_name, column_name)) {
                                println!("{}", format!("Found an unmapped column matching '{}': {} ({}___{})", self.regex, column_name, database_name, table_name));
                            }
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    fn get_starting_message(&self) -> String {
        format!("Looking for unmapped columns in the {} project that match {}", self.project, self.regex)
    }

}
use anyhow::Result;
use crate::config::{self, Config};

use super::Command;

pub struct CleanCommand {
    pub project: String,
}

impl Command for CleanCommand {

    fn get_starting_message(&self) -> String {
        format!("Validating the config file of the '{}' project", self.project)
    }

    async fn execute(&self) -> Result<()> {
        let mut config: Config = config::load(&self.project)?;
        
        if config.references.is_some() {
            config.references = None;
        }

        for (database_name, database) in &mut config.databases {
            println!("Cleaning database {}", database_name);
            
            if database.tables.is_some() {
                database.tables = None;
            }
        }

        config.save()?;
        Ok(())
    }

}
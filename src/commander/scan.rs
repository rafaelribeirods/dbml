use anyhow::Result;
use crate::{commander::Command, config::{self, Config}};

pub struct ScanCommand {
    pub project: String
}

impl Command for ScanCommand {

    fn get_starting_message(&self) -> String {
        format!("Scanning project '{}'", self.project)
    }

    fn execute(&self) -> Result<()> {
        let config: Config = config::load(&self.project)?;

        for (database_name, database) in config.databases {
            println!(
                "Scanning database {} ({}:{}@{}:{}/{})", 
                database_name, 
                database.connection.username,
                database.connection.password,
                database.connection.host,
                database.connection.port,
                database.connection.database
            )
        }
        
        Ok(())
    }

}
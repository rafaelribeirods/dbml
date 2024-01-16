use anyhow::Result;
use crate::{commander::Command, config::{self, Config}, db};

pub struct ScanCommand {
    pub project: String
}

impl Command for ScanCommand {

    fn get_starting_message(&self) -> String {
        format!("Scanning project '{}'", self.project)
    }

    async fn execute(&self) -> Result<()> {
        let config: Config = config::load(&self.project)?;

        for database in config.databases {
            println!("Scanning {}", database.connection.get_connection_string());

            let result = db::scan(database.connection).await?;
            println!("length: {}", result.len());

            for column_info in result {
                println!("{:?}", column_info)
            }
        }
        
        Ok(())
    }

}
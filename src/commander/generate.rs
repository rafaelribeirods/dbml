use anyhow::Result;
use crate::{config::{Config, self}, dbml::{DBML, self}};

use super::Command;

pub struct GenerateCommand {
    pub project: String
}

impl Command for GenerateCommand {

    fn get_starting_message(&self) -> String {
        format!("Generating the DBML file for project '{}'", self.project)
    }

    async fn execute(&self) -> Result<()> {
        let config: Config = config::load(&self.project)?;
        let mut dbml: DBML = dbml::init(&self.project)?;

        for (database_name, database) in &config.databases {
            if let Some(tables) = &database.tables {
                for (table_name, table) in tables {
                    dbml.write(table.to_dbml(database_name, table_name))?;
                }
            }
        }

        if let Some(references) = &config.references {
            for (key, referenced_keys) in references {
                for referenced_key in referenced_keys {
                    dbml.write(format!(
                        "Ref: {} - {}\n",
                        key,
                        referenced_key
                    ))?;
                }
            }
        }

        if let Some(references) = &config.custom_references {
            for (key, referenced_keys) in references {
                for referenced_key in referenced_keys {
                    dbml.write(format!(
                        "Ref: {} - {}\n",
                        key,
                        referenced_key
                    ))?;
                }
            }
        }

        dbml.save()?;
        Ok(())
    }

}
use std::collections::HashMap;

use anyhow::{anyhow, Result};
use regex::Regex;
use crate::{config::{self, Config, ProjectDatabaseTable}, dbml::{self, DBML}};

use super::Command;

pub struct GenerateCommand {
    pub project: String,
    pub starting_table: Option<String>,
}

struct TableInfo {
    pub database_name: String,
    pub table_name: String,
    pub table: ProjectDatabaseTable,
}

impl Command for GenerateCommand {

    fn get_starting_message(&self) -> String {
        format!("Generating the DBML file for project '{}'", self.project)
    }

    async fn execute(&self) -> Result<()> {
        let config: Config = config::load(&self.project)?;
        let mut dbml: DBML = dbml::init(&self.project)?;

        if self.starting_table.is_some() {
            return generate_from_starting_table(&config, &mut dbml, self.starting_table.as_ref().unwrap());
        }

        generate_all(&config, &mut dbml)
    }

}

fn generate_all(config: &Config, dbml: &mut DBML) -> Result<()> {

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

fn generate_from_starting_table(config: &Config, dbml: &mut DBML, starting_table: &String) -> Result<()> {
    let table_info = get_table(config, starting_table)?;
    let dependencies = get_dependencies(config, starting_table);
    let mut printed_tables: HashMap<String, ()> = HashMap::new();

    dbml.write(table_info.table.to_dbml(&table_info.database_name, &table_info.table_name))?;
    printed_tables.insert(starting_table.to_string(), ());
    for (dependency_table_key, (dependent_table_referencing_column, dependency_table_referenced_column)) in dependencies {
        if !printed_tables.contains_key(&dependency_table_key) {
            let dependency_table = get_table(config, &dependency_table_key)?;
            dbml.write(dependency_table.table.to_dbml(&dependency_table.database_name, &dependency_table.table_name))?;
        }
        dbml.write(format!(
            "Ref: {} - {}\n\n",
            dependent_table_referencing_column,
            dependency_table_referenced_column
        ))?;
    }

    Ok(())
}

fn get_table(config: &Config, table_key: &String) -> Result<TableInfo> {
    let table_key_regex = Regex::new(r"^(.+)___(.+)$").unwrap();
    if !table_key_regex.is_match(&table_key) {
        return Err(anyhow!("The starting table must follow this pattern: DATABASE_NAME___TABLE_NAME"));
    }

    let parts: Vec<&str> = table_key.split("___").collect();
    let database_name = parts[0];
    let table_name = parts[1..].join("___");

    if !config.databases.contains_key(database_name)
    || !config.databases.get(database_name).unwrap().tables.clone().unwrap().contains_key(&table_name) {
        return Err(anyhow!("The given starting table '{}' does not match an existing table from the config file for project {}", table_key, config.project));
    }

    Ok(TableInfo {
        database_name: database_name.to_string(),
        table_name: table_name.to_string(),
        table: config.databases.get(database_name).unwrap().tables.clone().unwrap().get(&table_name).unwrap().clone()
    })
}

fn get_dependencies(config: &Config, table_key: &String) -> HashMap<String, (String, String)> {
    let mut dependencies: HashMap<String, (String, String)> = HashMap::new();
    if let Some(config_references) = &config.references {
        for (reference_key, references) in config_references {
            if reference_key.starts_with(format!("{}.", table_key).as_str()) {
                for reference in references {
                    if let Some((table_key, _)) = reference.split_once('.') {
                        dependencies.insert(table_key.to_string(), (reference_key.to_string(), reference.to_string()));
                    }
                }
            }
        }
    }

    dependencies
}
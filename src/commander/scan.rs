use std::collections::HashMap;

use anyhow::Result;
use crate::{commander::Command, config::{self, Config, ProjectDatabaseColumn, ProjectDatabaseTable, ProjectDatabaseIndex}, db::{self}};

pub struct ScanCommand {
    pub project: String
}

impl Command for ScanCommand {

    fn get_starting_message(&self) -> String {
        format!("Scanning project '{}'", self.project)
    }

    async fn execute(&self) -> Result<()> {
        let mut config: Config = config::load(&self.project)?;
        scan_tables_and_columns(&mut config).await?;
        search_for_composite_primary_keys(&mut config);
        scan_references(&mut config).await?;
        config.save()
    }

}

async fn scan_tables_and_columns(config: &mut Config) -> Result<()> {
    for (database_name, database) in &mut config.databases {
        println!("Scanning database {} at {}", database_name, database.connection.get_connection_string());

        let mut tables: HashMap<String, ProjectDatabaseTable> = HashMap::new();
        let mut table =  ProjectDatabaseTable {
            columns: HashMap::new(),
            indexes: None
        };
        let mut table_name = String::from("");

        let result = db::scan_tables_and_columns(database.connection.clone()).await?;
        for column_info in result {
            let current_table_name = format!("{}___{}", column_info.schema_name, column_info.table_name);
            if current_table_name != table_name {
                if !table_name.is_empty() {
                    tables.insert(table_name, ProjectDatabaseTable {
                        columns: table.columns,
                        indexes: None
                    });
                }

                table_name = current_table_name;
                table.columns = HashMap::new();
            }

            let project_database_column = ProjectDatabaseColumn { 
                data_type: column_info.data_type, 
                data_precision: column_info.data_precision.map(|x| x.to_string()), 
                is_primary_key: column_info.is_primary_key,
                is_nullable: column_info.is_nullable, 
                is_unique: column_info.is_unique, 
                is_auto_increment: column_info.is_auto_increment,
                default_value: column_info.default_value,
                ordinal_position: column_info.ordinal_position
            };

            table.columns.insert(column_info.column_name, project_database_column);
        }

        tables.insert(table_name, ProjectDatabaseTable {
            columns: table.columns,
            indexes: None
        });

        database.tables = Some(tables);
        println!("Finished scanning!")
    }

    Ok(())
}

fn search_for_composite_primary_keys(config: &mut Config) {
    println!("Searching for composite primary keys...");
    for (_, database) in &mut config.databases {
        if let Some(tables) = &mut database.tables {
            for (table_name, table) in tables {

                let mut primary_keys: Vec<String> = Vec::new();

                for (column_name, column) in &table.columns {
                    if column.is_primary_key {
                        primary_keys.push(column_name.to_string());
                    }
                }

                if primary_keys.len() > 1 {
                    println!("Table {} has a composite primary key: {}", table_name, primary_keys.join(", "));

                    let index = ProjectDatabaseIndex {
                        columns: primary_keys,
                        is_primary_key: true,
                    };
                    let mut indexes: Vec<ProjectDatabaseIndex> = Vec::new();
                    indexes.push(index);
                    table.indexes = Some(indexes);

                    for (_, column) in &mut table.columns {
                        if column.is_primary_key {
                            column.is_primary_key = false;
                        }
                    }
                }
            }
        }
    }
    println!("Finished searching for composite primary keys!");
}

async fn scan_references(config: &mut Config) -> Result<()> {
    for (database_name, database) in &mut config.databases {
        println!("Scanning references from {} at {}", database_name, database.connection.get_connection_string());

        let result = db::scan_references(database.connection.clone()).await?;
        let mut references: HashMap<String, Vec<String>> = HashMap::new();
        for reference_info in result {
            let key = format!(
                "{}___{}___{}.{}",
                database_name,
                reference_info.schema_name,
                reference_info.table_name,
                reference_info.column_name
            );
            let referenced_key = format!(
                "{}___{}___{}.{}", 
                database_name,
                reference_info.referenced_schema_name,
                reference_info.referenced_table_name,
                reference_info.referenced_column_name
            );

            let mut referenced_keys: Vec<String> = Vec::new();
            referenced_keys.push(referenced_key);
            references.insert(key, referenced_keys);
        }

        database.references = Some(references);

        println!("Finished scanning references!")
    }

    Ok(())
}
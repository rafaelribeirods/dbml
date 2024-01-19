use anyhow::Result;
use crate::{commander::Command, config::{self, Config, ProjectDatabaseColumn, ProjectDatabaseTable, ProjectDatabaseIndex, ProjectDatabaseReference}, db::{self}};

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
    for database in &mut config.databases {
        println!("Scanning {}", database.connection.get_connection_string());

        let mut tables: Vec<ProjectDatabaseTable> = Vec::new();
        let mut table =  ProjectDatabaseTable {
            name: String::from(""),
            columns: Vec::new(),
            indexes: None
        };

        let result = db::scan_tables_and_columns(database.connection.clone()).await?;
        for column_info in result {
            let current_table_name = format!("\"{}\".\"{}\"", column_info.schema_name, column_info.table_name);
            if current_table_name != table.name {
                if !table.name.is_empty() {
                    tables.push(ProjectDatabaseTable {
                        name: table.name,
                        columns: table.columns,
                        indexes: None
                    });
                }

                table.name = current_table_name;
                table.columns = Vec::new();
            }

            let project_database_column = ProjectDatabaseColumn { 
                column_name: column_info.column_name, 
                data_type: column_info.data_type, 
                data_precision: column_info.data_precision.map(|x| x.to_string()), 
                is_primary_key: column_info.is_primary_key,
                is_nullable: column_info.is_nullable, 
                is_unique: column_info.is_unique, 
                is_auto_increment: column_info.is_auto_increment
            };

            table.columns.push(project_database_column);
        }

        tables.push(ProjectDatabaseTable {
            name: table.name,
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
    for database in &mut config.databases {
        if let Some(tables) = &mut database.tables {
            for table in tables {

                let mut primary_keys: Vec<String> = Vec::new();

                for column in &table.columns {
                    if column.is_primary_key {
                        primary_keys.push(column.column_name.to_string());
                    }
                }

                if primary_keys.len() > 1 {
                    println!("Table {} has a composite primary key: {}", table.name, primary_keys.join(", "));

                    let index = ProjectDatabaseIndex {
                        columns: primary_keys,
                        is_primary_key: true,
                    };
                    let mut indexes: Vec<ProjectDatabaseIndex> = Vec::new();
                    indexes.push(index);
                    table.indexes = Some(indexes);

                    for column in &mut table.columns {
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
    for database in &mut config.databases {
        println!("Scanning references from {}", database.connection.get_connection_string());

        let result = db::scan_references(database.connection.clone()).await?;
        let mut references: Vec<ProjectDatabaseReference> = Vec::new();
        for reference_info in result {
            let key = format!(
                "{}.{}.{}",
                reference_info.schema_name,
                reference_info.table_name,
                reference_info.column_name
            );
            let referenced_key = format!(
                "{}.{}.{}", 
                reference_info.referenced_schema_name,
                reference_info.referenced_table_name,
                reference_info.referenced_column_name
            );

            let reference = ProjectDatabaseReference {
                key,
                referenced_key,
                operator: String::from("-"),
            };

            references.push(reference);
        }

        database.references = Some(references);

        println!("Finished scanning references!")
    }

    Ok(())
}
use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};
use std::{collections::HashMap, fs, io::Write, path::PathBuf};

use crate::db::DatabaseType;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileConfig {
    pub databases: HashMap<String, ProjectDatabase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_references: Option<HashMap<String, Vec<String>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub project: String,
    pub databases: HashMap<String, ProjectDatabase>,
    pub custom_references: Option<HashMap<String, Vec<String>>>,
}

impl Config {

    pub fn save(&self) -> Result<()> {
        let config = FileConfig {
            databases: self.databases.clone(),
            custom_references: self.custom_references.clone(),
        };

        let contents = serde_yaml::to_string(&config)
            .map_err(|err| anyhow!(format!("Could not generate the updated configuration file for project '{}': {}", self.project, err)))?;

        let path = get_path(&self.project)?;
        let mut file = fs::OpenOptions::new().write(true).truncate(true).open(&path)
            .map_err(|err| anyhow!(format!("Could not open configuration file for project '{}': {}", self.project, err)))?;
        
        file.write_all(contents.as_bytes())
            .map_err(|err| anyhow!(format!("Could save updated configuration for project '{}': {}", self.project, err)))
    }

}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectDatabase {
    pub connection: ProjectDatabaseConnection,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables: Option<HashMap<String, ProjectDatabaseTable>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub references: Option<HashMap<String, Vec<String>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDatabaseConnection {
    pub r#type: DatabaseType,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

impl ProjectDatabaseConnection {

    pub fn get_connection_string(&self) -> String {
        match self.r#type {
            DatabaseType::MySql => format!("mysql://{}:{}@{}:{}",
                self.username,
                self.password,
                self.host,
                self.port
            )
        }
    }

}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectDatabaseTable {
    pub columns: HashMap<String, ProjectDatabaseColumn>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexes: Option<Vec<ProjectDatabaseIndex>>
}

impl ProjectDatabaseTable {

    pub fn to_dbml(&self, database_name: &String, name: &String) -> String {
        let mut dbml = format!("Table {}___{} {{\n", database_name, name);

        let mut ordered_columns: Vec<Option<(&String, &ProjectDatabaseColumn)>> = Vec::new();
        for (column_name, column) in &self.columns {
            let index: usize = column.ordinal_position.wrapping_sub(1) as usize;

            if index >= ordered_columns.len() {
                ordered_columns.resize(index + 1, None);
            }

            ordered_columns[index] = Some((column_name, column));
        }

        for item in ordered_columns.iter() {

            if let None = item {
                continue;
            }

            let (column_name, column) = item.unwrap();
            let mut column_options: Vec<String> = Vec::new();

            if column.is_primary_key {
                column_options.push("pk".to_string());
            }

            if column.is_nullable {
                column_options.push("null".to_string());
            }
            else {
                column_options.push("not null".to_string());
            }

            if column.is_unique {
                column_options.push("unique".to_string());
            }

            if column.is_auto_increment {
                column_options.push("increment".to_string());
            }

            if column.default_value.is_some() {
                column_options.push(format!("default: \"{}\"", column.default_value.clone().unwrap()));
            }

            let options = format!("[ {} ]", column_options.join(", "));

            let precision = match &column.data_precision {
                Some(x) if x != "0" => format!("({})", x),
                _ => String::new(),
            };

            dbml = dbml 
                + &'\t'.to_string() 
                + &column_name + " "
                + &column.data_type
                + &precision + " "
                + &options + &'\n'.to_string()

        }

        let mut idxs:Vec<String> = Vec::new();
        if let Some(indexes) = &self.indexes {
            for index in indexes {
                let mut idx = index.columns.join(", ");
                if index.columns.len() > 1 {
                    idx = format!("({})", idx);
                }

                let mut index_options: Vec<&str> = Vec::new();
                if index.is_primary_key {
                    index_options.push("pk");
                }

                let mut formatted_index_options = String::from("");
                if index_options.len() > 0 {
                    formatted_index_options = format!(" [ {} ]", index_options.join(", "));
                }

                idx = idx + &formatted_index_options;
                idxs.push(idx);
             }
        }

        let mut indexes_output = String::from("");
        if idxs.len() > 0 {
            indexes_output = indexes_output + &"\n\tindexes {\n".to_string();
            for idx in idxs {
                indexes_output = indexes_output
                    + &'\t'.to_string() + &'\t'.to_string()
                    + &idx + &'\n'.to_string()
            } 
                indexes_output = indexes_output + &'\t'.to_string() + "}" + &'\n'.to_string();
        }
        dbml = dbml + &indexes_output + "}" + &'\n'.to_string() + &'\n'.to_string();
        dbml
    }

}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectDatabaseColumn {
    pub data_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_precision: Option<String>,
    pub is_primary_key: bool,
    pub is_nullable: bool,
    pub is_unique: bool,
    pub is_auto_increment: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    pub ordinal_position: u8
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectDatabaseIndex {
    pub columns: Vec<String>,
    pub is_primary_key: bool
}

pub fn load(project: &String) -> Result<Config> {
    let contents = get_file_contents(project)?;
    let config = serde_yaml::from_str::<FileConfig>(&contents)
        .map_err(|err| anyhow!(format!("Could not parse the config file for the '{}' project: {}", project, err)))?;

    Ok(Config { project: project.to_string(), databases: config.databases, custom_references: config.custom_references })
}

#[cfg(not(test))]
fn get_home() -> Result<PathBuf> {
    return match home::home_dir() {
        Some(path) => Ok(path),
        None => Err(anyhow!("Could not detect the current user's home directory.")),
    };
}

#[cfg(not(test))]
fn get_path(project: &String) -> Result<String> {
    let home = get_home()?;
    Ok(format!("{}/.dbml/{}.yaml", home.to_str().unwrap(), project))
}

#[cfg(test)]
fn get_path(project: &String) -> Result<String> {
    let project_root = env!("CARGO_MANIFEST_DIR");
    let path = PathBuf::from(project_root).join(format!("{}.yaml", project));

    match path.to_str() {
        Some(path) => Ok(String::from(path)),
        None => Err(anyhow!("Could not get the path for the test file")),
    }
}

fn get_file_contents(project: &String) -> Result<String> {
    let path = get_path(project)?;
    fs::read_to_string(path.to_string())
        .map_err(|err| anyhow!(format!("Error reading file at '{}': {}", path, err)))
}

#[cfg(test)]
mod tests {

    use crate::db::DatabaseType;

    use super::load;

    #[test]
    fn test_load() {
        let result = load(&String::from("example_project"));
        assert!(result.is_ok());

        let config = result.unwrap();
        let database = config.databases.get("test_db").unwrap();

        assert_eq!(database.connection.r#type, DatabaseType::MySql);
        assert_eq!(database.connection.host, "localhost");
        assert_eq!(database.connection.port, 3306);
        assert_eq!(database.connection.username, "test_user");
        assert_eq!(database.connection.password, "test_password");
    }

    #[test]
    fn test_load_with_invalid_project() {
        let result = load(&String::from("invalid_project"));
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert!(error.to_string().starts_with("Error reading file at "));
    }

}
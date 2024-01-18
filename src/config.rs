use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};
use std::{path::PathBuf, fs, io::Write};

use crate::db::DatabaseType;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub project: String,
    pub databases: Vec<ProjectDatabase>,
}

impl Config {

    pub fn save(&self) -> Result<()> {
        let contents = serde_yaml::to_string(&self.databases)
            .map_err(|err| anyhow!(format!("Could not generate the updated configuration file for project '{}': {}", self.project, err)))?;

        let path = get_path(&self.project)?;
        let mut file = fs::OpenOptions::new().write(true).truncate(true).open(&path)
            .map_err(|err| anyhow!(format!("Could not open configuration file for project '{}': {}", self.project, err)))?;
        
        file.write_all(contents.as_bytes())
            .map_err(|err| anyhow!(format!("Could save updated configuration for project '{}': {}", self.project, err)))
    }

}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectDatabase {
    pub connection: ProjectDatabaseConnection,
    pub tables: Option<Vec<ProjectDatabaseTable>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDatabaseConnection {
    pub r#type: DatabaseType,
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
}

impl ProjectDatabaseConnection {

    pub fn get_connection_string(&self) -> String {
        match self.r#type {
            DatabaseType::MySql => format!("mysql://{}:{}@{}:{}/{}",
                self.username,
                self.password,
                self.host,
                self.port,
                self.database
            )
        }
    }

}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectDatabaseTable {
    pub name: String,
    pub columns: Vec<ProjectDatabaseColumn>
}

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct ProjectDatabaseColumn {
    pub column_name: String,
    pub data_type: String,
    pub data_precision: Option<u32>,
    pub is_primary_key: bool,
    pub is_nullable: bool,
    pub is_unique: bool,
    pub is_auto_increment: bool
}

pub fn load(project: &String) -> Result<Config> {
    let contents = get_file_contents(project)?;
    let databases = serde_yaml::from_str::<Vec<ProjectDatabase>>(&contents)
        .map_err(|err| anyhow!(format!("Could not parse the config file for the '{}' project: {}", project, err)))?;

    Ok(Config { project: project.to_string(), databases })
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
        assert_eq!(config.databases.len(), 1);
        assert_eq!(config.databases[0].connection.r#type, DatabaseType::MySql);
        assert_eq!(config.databases[0].connection.host, "localhost");
        assert_eq!(config.databases[0].connection.port, 3306);
        assert_eq!(config.databases[0].connection.database, "test_db");
        assert_eq!(config.databases[0].connection.username, "test_user");
        assert_eq!(config.databases[0].connection.password, "test_password");
    }

    #[test]
    fn test_load_with_invalid_project() {
        let result = load(&String::from("invalid_project"));
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert!(error.to_string().starts_with("Error reading file at "));
    }

}
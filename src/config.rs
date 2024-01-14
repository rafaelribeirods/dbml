use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};
use std::{path::PathBuf, fs};

use crate::db::DatabaseType;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub project: String,
    pub databases: Vec<ProjectDatabase>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectDatabase {
    pub connection: ProjectDatabaseConnection,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectDatabaseConnection {
    pub r#type: DatabaseType,
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
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
use anyhow::Result;
use serde::{Serialize, Deserialize};
use crate::config::{ProjectConfiguration, ProjectDatabaseConnection};
use mysql::MysqlDatabase;

mod mysql;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DatabaseType {
    MySql,
}

impl DatabaseType {
    fn as_string(&self) -> &'static str {
        match self {
            DatabaseType::MySql => "mysql",
        }
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct ColumnInfo {
    pub schema_name: String,
    pub table_name: String,
    pub column_name: String,
    pub data_type: String,
    pub data_precision: Option<u32>,
    pub is_primary_key: bool,
    pub is_nullable: bool,
    pub is_unique: bool,
    pub is_auto_increment: bool,
    pub default_value: Option<String>,
    pub ordinal_position: u8
}

#[derive(Debug, sqlx::FromRow)]
pub struct ReferenceInfo {
    pub schema_name: String,
    pub table_name: String,
    pub column_name: String,
    pub referenced_schema_name: String,
    pub referenced_table_name: String,
    pub referenced_column_name: String
}

pub trait DatabaseEngine {
    async fn scan_tables_and_columns(&self) -> Result<Vec<ColumnInfo>>;
    async fn scan_references(&self) -> Result<Vec<ReferenceInfo>>;
}

pub async fn scan_tables_and_columns(
    connection_info: ProjectDatabaseConnection,
    configurations: Option<ProjectConfiguration>
) -> Result<Vec<ColumnInfo>> {
    match connection_info.r#type {
        DatabaseType::MySql => MysqlDatabase{ connection_info, configurations }.scan_tables_and_columns().await
    }
}

pub async fn scan_references(
    connection_info: ProjectDatabaseConnection,
    configurations: Option<ProjectConfiguration>
) -> Result<Vec<ReferenceInfo>> {
    match connection_info.r#type {
        DatabaseType::MySql => MysqlDatabase{ connection_info, configurations }.scan_references().await
    }
}
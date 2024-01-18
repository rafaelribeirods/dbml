use anyhow::Result;
use serde::{Serialize, Deserialize};
use crate::config::ProjectDatabaseConnection;
use mysql::MysqlDatabase;

mod mysql;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DatabaseType {
    MySql,
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
    pub is_auto_increment: bool
}

pub trait DatabaseEngine {
    async fn scan_tables_and_columns(&self) -> Result<Vec<ColumnInfo>>;
}

pub async fn scan_tables_and_columns(connection_info: ProjectDatabaseConnection) -> Result<Vec<ColumnInfo>> {
    match connection_info.r#type {
        DatabaseType::MySql => MysqlDatabase{ connection_info }.scan_tables_and_columns().await
    }
}
use anyhow::Result;
use serde::{Serialize, Deserialize};
use crate::config::ProjectDatabaseConnection;
use mysql::MysqlDatabase;

mod mysql;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DatabaseType {
    MySql,
}

#[derive(Debug, sqlx::FromRow)]
pub struct ColumnInfo {
    schema_name: String,
    table_name: String,
    column_name: String,
    data_type: String,
    data_precision: u16,
    is_primary_key: bool,
    is_nullable: bool,
    is_unique: bool,
    is_auto_increment: bool
}

pub trait DatabaseEngine {
    async fn scan(&self) -> Result<Vec<ColumnInfo>>;
}

pub async fn scan(connection_info: ProjectDatabaseConnection) -> Result<Vec<ColumnInfo>> {
    match connection_info.r#type {
        DatabaseType::MySql => MysqlDatabase{ connection_info }.scan().await
    }
}
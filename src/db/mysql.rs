use anyhow::{anyhow, Result};
use sqlx::{MySqlConnection, Connection};
use crate::config::ProjectDatabaseConnection;

use super::{DatabaseEngine, ColumnInfo};

pub struct MysqlDatabase {
    pub connection_info: ProjectDatabaseConnection
}

impl DatabaseEngine for MysqlDatabase {

    async fn scan(&self) -> Result<Vec<ColumnInfo>> {
        let query: String = format!("
        SELECT 
            table_schema schema_name,
            table_name,
            column_name,
            data_type,
            COALESCE(character_maximum_length, numeric_precision, datetime_precision) AS data_precision,
            CASE WHEN column_key = 'PRI' THEN 1 ELSE 0 END AS is_primary_key,
            CASE WHEN is_nullable = 'YES' THEN 1 ELSE 0 END AS is_nullable,
            CASE WHEN column_key = 'UNI' THEN 1 ELSE 0 END AS is_unique,
            CASE WHEN extra = 'auto_increment' THEN 1 ELSE 0 END AS is_auto_increment,
            column_default AS default_value
        FROM information_schema.columns
        WHERE table_schema = '{}'
        ORDER BY table_name, ordinal_position;
        ", self.connection_info.database);

        let mut conn = MySqlConnection::connect(&self.connection_info.get_connection_string()).await
            .map_err(|err| anyhow!(format!("Could not connect to '{}': {}", &self.connection_info.get_connection_string(), err)))?;
        
        sqlx::query_as::<_, ColumnInfo>(query.as_str()).fetch_all(&mut conn).await
            .map_err(|err| anyhow!(format!("Could not run the scan query on '{}': {}", &self.connection_info.get_connection_string(), err)))
    }

}
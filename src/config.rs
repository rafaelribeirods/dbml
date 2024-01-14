pub struct Config {
    pub project: String,
    pub databases: HashMap<String, ProjectDatabase>,
}

pub struct ProjectDatabase {
    pub connection: ConfigDatabaseConnection,
}
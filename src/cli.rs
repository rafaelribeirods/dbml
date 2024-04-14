use clap::{command, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: SubCommands,
}

#[derive(Subcommand, Debug)]
pub enum SubCommands {

    /// Scans the databases of a project to discover its schemas, tables, columns and relationships
    Scan {
        /// The project to be scanned (there must be a file named {project}.yaml at $HOME/.dbml/)
        project: String,
    },

    /// Generates the .dbml file for a project
    Generate {
        /// The project to be have it's DBML file generated
        project: String,
    },

    /// Searches for columns that match a given regex and offers an option to add them to the custom references if they are not referenced anywhere
    Search {
        /// The project that has the databases in which the search will be performed
        project: String,
        /// The regex that will be used to search for the columns
        regex: String,
        /// The key referenced by the columns that match regex (format: {database_name}___{table_name}.{column_name})
        referenced_key: Option<String>,
    },

    /// Performs a few validations in a project's config file (does not modify it)
    Validate {
        /// The project to be validated
        project: String,
    },

    /// Removes the scanned tables and references of every database in the project (does not change the custom references)
    Clean {
        /// The project to be cleaned
        project: String,
    }

}

pub fn parse() -> Cli {
    return Cli::parse();
}
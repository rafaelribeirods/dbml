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

}

pub fn parse() -> Cli {
    return Cli::parse();
}
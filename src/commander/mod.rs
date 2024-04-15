use anyhow::Result;
use crate::cli::{Cli, SubCommands};
use self::{clean::CleanCommand, generate::GenerateCommand, scan::ScanCommand, search::SearchCommand, validate::ValidateCommand};

mod clean;
mod scan;
mod search;
mod generate;
mod validate;

pub trait Command {
    fn get_starting_message(&self) -> String;
    async fn execute(&self) -> Result<()>;
}

pub async fn execute(cli: Cli) -> Result<()> {
    match cli.command {
        SubCommands::Scan { project } => ScanCommand { project }.execute().await,
        SubCommands::Generate { project, starting_table } => GenerateCommand { project, starting_table }.execute().await,
        SubCommands::Search { project, regex, referenced_key } => SearchCommand { project, regex, referenced_key }.execute().await,
        SubCommands::Validate { project } => ValidateCommand { project }.execute().await,
        SubCommands::Clean { project } => CleanCommand { project }.execute().await,
    }
}
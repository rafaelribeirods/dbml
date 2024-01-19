use anyhow::Result;
use crate::cli::{Cli, SubCommands};
use self::{scan::ScanCommand, generate::GenerateCommand};

mod scan;
mod generate;
pub trait Command {
    fn get_starting_message(&self) -> String;
    async fn execute(&self) -> Result<()>;
}

pub async fn execute(cli: Cli) -> Result<()> {
    match cli.command {
        SubCommands::Scan { project } => ScanCommand { project }.execute().await,
        SubCommands::Generate { project } => GenerateCommand { project }.execute().await,
    }
}
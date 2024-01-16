use anyhow::Result;
use crate::cli::{Cli, SubCommands};
use self::scan::ScanCommand;

mod scan;
pub trait Command {
    fn get_starting_message(&self) -> String;
    async fn execute(&self) -> Result<()>;
}

pub async fn execute(cli: Cli) -> Result<()> {
   let command = resolve_command(cli.command);
   println!("{}", command.get_starting_message());
   command.execute().await
}

fn resolve_command(command: SubCommands) -> impl Command {
    match command {
        SubCommands::Scan { project } => ScanCommand { project },
    }
}

#[cfg(test)]
mod tests {
    use crate::{cli::{Cli, SubCommands}, commander::{scan::ScanCommand, Command}};

    use super::resolve_command;

    #[test]
    fn test_scan_should_return_correct_command_and_params() {
        let project_name: &str = "test_project";
        let cli = Cli { command: SubCommands::Scan { project: String::from(project_name) }};

        let result = resolve_command(cli.command);
        let expected_command = ScanCommand { project: String::from(project_name) };
        assert_eq!(expected_command.get_starting_message(), result.get_starting_message());
    }

}
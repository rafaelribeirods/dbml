use anyhow::Result;
use crate::commander::Command;

pub struct ScanCommand {
    pub project: String
}

impl Command for ScanCommand {

    fn get_starting_message(&self) -> String {
        format!("Scanning project {}", self.project)
    }

    fn execute(&self) -> Result<()> {
        println!("Scanning project {}", self.project);
        Ok(())
    }

}
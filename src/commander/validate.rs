use anyhow::Result;
use crate::config::{self, Config};

use super::Command;

pub struct ValidateCommand {
    pub project: String,
}

impl Command for ValidateCommand {

    fn get_starting_message(&self) -> String {
        format!("Validating the config file of the '{}' project", self.project)
    }

    async fn execute(&self) -> Result<()> {
        let config: Config = config::load(&self.project)?;
        validate_keys_in_references_and_custom_references(&config);
        validate_keys_with_multiple_referenced_keys(&config);
        Ok(())
    }

}

fn validate_keys_in_references_and_custom_references(config: &Config) {    
    if let (Some(references), Some(custom_references)) = (&config.references, &config.custom_references) {
        for key in references.keys() {
            if custom_references.contains_key(key) {
                println!("Key '{}' exists in both 'references' and 'custom_references'", key);
            }
        }
    }
}

fn validate_keys_with_multiple_referenced_keys(config: &Config) {
    if let Some(references) = &config.references {
        for (key, referenced_keys) in references {
            if referenced_keys.len() > 1 {
                println!("Key '{}' in 'references' has more than one referenced key", key);
            }
        }
    }

    if let Some(custom_references) = &config.custom_references {
        for (key, referenced_keys) in custom_references {
            if referenced_keys.len() > 1 {
                println!("Key '{}' in 'custom_references' has more than one referenced key", key);
            }
        }
    }
}
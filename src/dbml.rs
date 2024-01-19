use std::{fs::{OpenOptions, File}, io::Write};

use anyhow::{anyhow, Result};

pub struct DBML {
    project: String,
    file: File
}

impl DBML {

    pub fn write(&mut self, content: String) -> Result<()> {
        self.file.write_all(content.as_bytes())
            .map_err(|err| anyhow!(format!("Could not write content to the DBML file for project {}: {}", self.project, err)))?;
        Ok(())
    }

    pub fn save(&mut self) -> Result<()> {
        self.file.flush()
            .map_err(|err| anyhow!(format!("Could not save the DBML file for project {}: {}", self.project, err)))?;
        Ok(())
    }

}

pub fn init(project: &String) -> Result<DBML> {
    let file_path = get_file_path(project)?;

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)
        .map_err(|err| anyhow!(format!("Could not open/create the DBML file for project {}: {}", project, err)))?;

    Ok(DBML { 
        project: project.to_string(),
        file
    })
}

fn get_file_path(project: &String) -> Result<String> {
    let home = home::home_dir().ok_or(anyhow!("Could not detect the current user's home directory."))?;
    Ok(format!("{}/.dbml/{}.dbml", home.to_str().unwrap(), project))
}
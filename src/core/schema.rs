use std::{collections::HashMap, fs, io};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ServiceDefinition {
    pub services: HashMap<String, Vec<String>>,
}

fn load_services_definition(filepath: &str) -> Result<ServiceDefinition, io::Error> {
    let contents = fs::read_to_string(filepath)?;
    let root: ServiceDefinition = serde_yaml::from_str(&contents)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    Ok(root)
}

pub fn init() -> Result<ServiceDefinition, io::Error> {
    let filepath = "horbo.yml";
    match load_services_definition(filepath) {
        Ok(definition) => {
            Ok(definition)
        }
        Err(e) => {
            Err(e)
        }
    }
}
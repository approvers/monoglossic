use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub db_address: String,
}

pub fn read_json_config(path: impl AsRef<Path>) -> Result<Config, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // read json from file
    let json_data = serde_json::from_reader(reader)?;

    Ok(json_data)
}

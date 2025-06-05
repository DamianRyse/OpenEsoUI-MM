use std::error::Error;
use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub target_directory: String,
    pub addon_ids: Vec<u16>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            target_directory: get_config_path().unwrap().parent().unwrap().to_string_lossy().to_string(),
            addon_ids: vec![4063, 3501], // "Simple Skyshards" and "BindAll" addon as default example ids.
        }
    }
}

impl Config {
    pub fn load_from_file() -> Result<Config,Box<dyn Error>> {
        let filename = get_config_path()?;

        // Create config directory (openesoui-mm) if it doesn't exist
        if let Some(parent) = filename.parent() {
            fs::create_dir_all(parent)?;
        }

        if filename.exists() {
            // Load existing configuration
            let config_str = fs::read_to_string(&filename)?;
            let config: Config = serde_json::from_str(&config_str)?;
            Ok(config)
        } else {
            // Create default configuration
            let config = Config::default();
            let config_str = serde_json::to_string_pretty(&config)?;
            fs::write(&filename, config_str)?;

            println!(
                "Created default configuration at: {}",
                filename.display()
            );
            println!(
                "Please edit the configuration file to set your desired file IDs and target directory."
            );

            Ok(config)
        }
    }
}

fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let config_dir = if cfg!(target_os = "windows") {
        // Windows: Documents/openesoui-mm/
        dirs::document_dir()
            .ok_or("Could not find Documents directory")?
            .join("openesoui-mm")
    } else {
        // Linux: ~/.config/openesoui-mm/
        dirs::config_dir()
            .ok_or("Could not find config directory")?
            .join("openesoui-mm")
    };

    Ok(config_dir.join("config.json"))
}
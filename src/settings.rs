use std::error::Error;
use std::fmt::Display;
use std::path::Path;

// Config is the center of config crate
use config::Config;
use serde_derive::Deserialize;

const SETTINGS_PATH: &str = "/home/mukund/.config/noteapp/Settings.toml";

#[derive(Debug)]
pub struct SettingsValidationError {
    message: String,
}

impl Display for SettingsValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "SettingsValidationError: {}", self.message)
    }
}

impl Error for SettingsValidationError {}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Paths {
    pub notes_path: String,
    pub scans_path: String,
}
impl Paths {
    pub fn validate(&self) -> Result<(), SettingsValidationError> {
        if !Path::new(&self.notes_path).exists() {
            return Err(SettingsValidationError {
                message: format!("Notes path does not exist: {}", &self.notes_path),
            });
        }
        if !Path::new(&self.scans_path).exists() {
            return Err(SettingsValidationError {
                message: format!("Scans path does not exist: {}", &self.scans_path),
            });
        };
        Ok(())
    }
}

pub fn load_settings() -> Result<Paths, Box<dyn Error>> {
    let settings = Config::builder()
        .add_source(config::File::with_name(SETTINGS_PATH))
        .add_source(config::File::with_name("Settings.Dev.toml").required(false))
        .build()?;

    match settings.try_deserialize::<Paths>() {
        Ok(paths) => Ok(paths),
        Err(e) => Err(Box::new(e)),
    }
}

mod actions;
mod application;
mod left_column;
mod messages;
mod scans;
mod settings;

use log::info;

use application::NoteApp;
use iced::{Application, Result as ICEDResult, Settings as IcedSettings};

fn main() -> ICEDResult {
    // initialize logging
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    info!("Loading configuration and sanity checks...");

    // custom settings
    let paths = settings::load_settings().expect("Error loading settings");

    // Check if the paths are valid
    paths.validate().expect("Invalid paths exiting");

    let noteapp_settings = IcedSettings {
        flags: paths,
        ..IcedSettings::default()
    };
    NoteApp::run(noteapp_settings)
}

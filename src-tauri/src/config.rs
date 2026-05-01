use std::{
    fs::{DirBuilder, File, remove_file},
    io,
    path::PathBuf,
};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub acknowledged_dev_warning: bool,
}
impl Default for AppConfig {
    fn default() -> Self {
        Self {
            acknowledged_dev_warning: false,
        }
    }
}

fn project_dirs() -> ProjectDirs {
    ProjectDirs::from("audio", "xpans", "Frontier")
        .expect("Could not generate application directories.")
}

fn config_dir() -> PathBuf {
    project_dirs().config_dir().to_path_buf()
}

fn config_file_path() -> PathBuf {
    config_dir().join("settings.json")
}
fn ensure_config_directory() -> io::Result<()> {
    if !config_dir().exists() {
        return DirBuilder::new().recursive(true).create(config_dir());
    }
    Ok(())
}

fn create_config_file() -> io::Result<File> {
    ensure_config_directory()?;
    let path = config_file_path();
    File::create(path)
}

fn open_config_file() -> io::Result<File> {
    let path = config_file_path();
    println!("trying to open {path:?}");
    File::open(path)
    // let file = File::open(path).expect("Could not open application configuration file.");
    // file
}

fn create_config() -> io::Result<AppConfig> {
    let file = create_config_file()?;
    let config = AppConfig::default();
    let _ = serde_json::to_writer_pretty(file, &config);
    Ok(config)
}

fn open_config() -> io::Result<AppConfig> {
    let file = open_config_file()?;
    let config = serde_json::from_reader(file)
        .expect("Could not deserialize application configuration file.");
    Ok(config)
}

pub fn update_config(config: &AppConfig) -> io::Result<()> {
    remove_file(config_file_path())?;
    let file = create_config_file()?;
    let _ = serde_json::to_writer_pretty(file, config);
    Ok(())
}

pub fn create_or_open_config() -> AppConfig {
    if let Ok(config) = open_config() {
        return config;
    }
    create_config().expect("Could not create application configuration file.")
}

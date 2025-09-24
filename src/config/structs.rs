use serde::{Serialize, Deserialize};
use std::{collections::BTreeMap, path::PathBuf};
use crate::config;

#[derive(Serialize, Deserialize, Debug)]
pub struct BackuperConfig {
    // Where to backup to
    pub destination: PathBuf,

    // How often to backup
    pub interval_minutes: u32,

    // How many days to keep backups
    pub keep_days: u64,

    // backup_sources: Vec<BackupSource>

    pub backup_sources: BTreeMap<String, Vec<PathBuf>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BackupSource {
    pub name: String,
    pub paths: Vec<String>
}

impl BackuperConfig {
    pub fn new() -> Self {
        let mut default_backup_sources = BTreeMap::new();

        default_backup_sources.insert(
            "sublime".to_string(),
            vec![PathBuf::from("C:/Users/Username/AppData/Roaming/Sublime Text/Packages/User")],
        );

        default_backup_sources.insert(
            "photoshop".to_string(),
            vec![
                PathBuf::from("C:/Users/Username/AppData/Roaming/Adobe/Adobe Photoshop 2024/Presets"),
                PathBuf::from("C:/Users/Username/Documents/Photoshop"),
            ],
        );

        default_backup_sources.insert(
            "flashpaste".to_string(),
            vec![
                PathBuf::from("C:/Program Files/FlashPaste/config.ini"),
                PathBuf::from("C:/Users/Username/AppData/Local/FlashPaste"),
            ],
        );

        Self {
            // default config
            destination: config::get_config_path().to_path_buf().parent().unwrap().join("backups"),
            interval_minutes: 60,
            keep_days: 7,
            backup_sources: default_backup_sources,
        }
    }

}
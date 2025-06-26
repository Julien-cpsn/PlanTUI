use crate::app::APP_NAME;
use directories::BaseDirs;
use std::fs;
use std::path::PathBuf;

pub fn get_data_dir() -> PathBuf {
    let base_dir = BaseDirs::new().unwrap();

    let data_dir = base_dir.data_local_dir().join(APP_NAME.to_lowercase());

    if !data_dir.exists() {
        fs::create_dir_all(&data_dir).expect(&format!("Could not create data directory \"{}\"", data_dir.display()));
    }

    data_dir.to_path_buf()
}
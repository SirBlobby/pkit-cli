use std::fs;
use std::io;
use std::path::PathBuf;
use super::common::{get_home_dir};

pub fn get_pkit_dir() -> io::Result<PathBuf> {
    let pkit_dir = get_home_dir()?.join(".pkit");
    fs::create_dir_all(&pkit_dir)?;
    Ok(pkit_dir)
}

pub fn get_pkit_data_dir() -> io::Result<PathBuf> {
    let data_dir = get_pkit_dir()?.join("data");
    fs::create_dir_all(&data_dir)?;
    Ok(data_dir)
}

pub fn get_pkit_cache_dir() -> io::Result<PathBuf> {
    let cache_dir = get_pkit_dir()?.join("cache");
    fs::create_dir_all(&cache_dir)?;
    Ok(cache_dir)
}

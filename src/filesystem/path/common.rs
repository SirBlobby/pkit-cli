use std::fs;
use std::io;
use std::path::PathBuf;
use home;

#[derive(Debug, Clone, PartialEq)]
pub enum OperatingSystem {
    Windows,
    MacOS,
    Linux,
    Other,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ShellConfig {
    Bash,
    Zsh,
    Fish,
    PowerShell,
    Cmd,
}

pub fn detect_os() -> OperatingSystem {
    if cfg!(target_os = "windows") {
        OperatingSystem::Windows
    } else if cfg!(target_os = "macos") {
        OperatingSystem::MacOS
    } else if cfg!(target_os = "linux") {
        OperatingSystem::Linux
    } else {
        OperatingSystem::Other
    }
}

pub fn get_home_dir() -> io::Result<PathBuf> {
    home::home_dir().ok_or_else(|| {
        io::Error::new(io::ErrorKind::NotFound, "Could not find home directory")
    })
}

pub fn copy_dir_all(src: &PathBuf, dst: &PathBuf) -> io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = dst.join(entry.file_name());
        if path.is_dir() {
            copy_dir_all(&path, &dest_path)?;
        } else {
            fs::copy(&path, &dest_path)?;
        }
    }
    Ok(())
}

pub fn migrate_old_pkit_dir() -> io::Result<()> {
    let old_pkit_dir = get_home_dir()?.join(".pkit");
    if !old_pkit_dir.exists() {
        return Ok(());
    }
    
    let new_pkit_dir = super::get_pkit_dir()?;
    if new_pkit_dir.exists() && fs::read_dir(&new_pkit_dir)?.next().is_some() {
        return Ok(());
    }
    
    println!("Migrating pkit configuration from {:?} to {:?}", old_pkit_dir, new_pkit_dir);
    copy_dir_all(&old_pkit_dir, &new_pkit_dir)?;
    println!("Migration completed successfully!");
    Ok(())
}

pub fn get_pkit_dir_with_migration() -> io::Result<PathBuf> {
    if let Err(e) = migrate_old_pkit_dir() {
        eprintln!("Warning: Failed to migrate old pkit directory: {}", e);
    }
    super::get_pkit_dir()
}

pub fn get_pkit_directories_info() -> io::Result<String> {
    let old_pkit = get_home_dir()?.join(".pkit");
    let mut info = format!(
        "Operating System: {:?}\nConfig Directory: {:?}\nData Directory: {:?}\nCache Directory: {:?}\nPrimary Shell Config: {:?}\n",
        detect_os(),
        super::get_pkit_dir()?,
        super::get_pkit_data_dir()?,
        super::get_pkit_cache_dir()?,
        super::get_primary_shell_config_path()?
    );
    
    if old_pkit.exists() {
        info.push_str(&format!("Legacy Directory (exists): {:?}\n", old_pkit));
    }
    
    Ok(info)
}

pub fn print_pkit_directories() {
    match get_pkit_directories_info() {
        Ok(info) => println!("{}", info),
        Err(e) => eprintln!("Error getting pkit directories info: {}", e),
    }
}

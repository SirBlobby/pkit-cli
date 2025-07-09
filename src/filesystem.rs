use std::fs::{self, File};
use std::io::{self};
use std::path::{Path, PathBuf};
use zip::ZipArchive;
use flate2::read::GzDecoder;
use tar::Archive;
use crate::formatter::{print_message, MessageType};

pub mod config;
pub mod path;

pub use path::{
    get_pkit_dir, get_home_dir, get_bashrc_path,
    setup_shell_environment, detect_os, get_primary_shell_config_path, 
    generate_env_setup_lines, generate_path_export, get_shell_config_files, 
    get_pkit_data_dir, get_pkit_cache_dir,
    migrate_old_pkit_dir, get_pkit_dir_with_migration, get_pkit_directories_info,
    print_pkit_directories, reload_environment
};

pub fn read(path: &Path) -> io::Result<String> {
    fs::read_to_string(path)
}

pub fn write(path: &Path, contents: &str) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, contents)
}

pub fn delete(path: &Path) -> io::Result<()> {
    fs::remove_file(path)
}

pub fn extract(archive_path: &Path) -> io::Result<()> {
    let destination = archive_path.parent().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "Invalid archive path")
    })?;

    let extension = archive_path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("");

    match extension {
        "zip" => unzip_file(archive_path, destination),
        "gz" if archive_path.to_str().unwrap_or("").ends_with(".tar.gz") => {
            extract_tar_gz(archive_path, destination)
        }
        "tgz" => extract_tar_gz(archive_path, destination),
        _ => {
            let err_msg = format!("Unsupported archive format: {:?}", archive_path);
            print_message(MessageType::Error(&err_msg));
            Err(io::Error::new(io::ErrorKind::InvalidData, err_msg))
        }
    }
}

fn unzip_file(zip_path: &Path, destination: &Path) -> io::Result<()> {
    let file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;

    let archive_name = zip_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("extracted");
    let extract_to = destination.join(archive_name);

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => extract_to.join(path),
            None => continue,
        };

        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p)?;
                }
            }
            let mut outfile = File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode))?;
            }
        }
    }
    
    find_folder_with_bin_and_copy(&extract_to, destination)?;
    fs::remove_dir_all(&extract_to)?;
    Ok(())
}

fn extract_tar_gz(tar_gz_path: &Path, destination: &Path) -> io::Result<()> {
    let file = File::open(tar_gz_path)?;
    let gz = GzDecoder::new(file);
    let mut archive = Archive::new(gz);

    let archive_name = tar_gz_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("extracted");
    let extract_to = destination.join(archive_name);
    
    archive.unpack(&extract_to)?;
    
    find_folder_with_bin_and_copy(&extract_to, destination)?;
    fs::remove_dir_all(&extract_to)?;
    Ok(())
}

fn find_folder_with_bin_and_copy(search_path: &Path, target_path: &Path) -> io::Result<()> {
    if !target_path.exists() {
        fs::create_dir_all(target_path)?;
    }

    if let Some(dir) = find_dir_with_bin(search_path)? {
        copy_dir_contents(&dir, target_path)?;
    } else {
        let err_msg = format!("No directory with 'bin' subdirectory found in {:?}", search_path);
        print_message(MessageType::Error(&err_msg));
        return Err(io::Error::new(io::ErrorKind::NotFound, err_msg));
    }

    Ok(())
}

fn find_dir_with_bin(dir: &Path) -> io::Result<Option<PathBuf>> {
    if !dir.is_dir() {
        return Ok(None);
    }

    if dir.join("bin").is_dir() {
        return Ok(Some(dir.to_path_buf()));
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            if let Some(found_dir) = find_dir_with_bin(&path)? {
                return Ok(Some(found_dir));
            }
        }
    }

    Ok(None)
}

fn copy_dir_contents(src: &Path, dst: &Path) -> io::Result<()> {
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = path.file_name().unwrap();
        let dst_path = dst.join(file_name);

        if path.is_dir() {
            fs::create_dir_all(&dst_path)?;
            copy_dir_contents(&path, &dst_path)?;
        } else {
            fs::copy(&path, &dst_path)?;
        }
    }
    Ok(())
}

use std::fs::{self, File};
use std::io::{self, prelude::*, Read};
use std::path::{Path, PathBuf};
use zip::ZipArchive;
use flate2::read::GzDecoder;
use tar::Archive;

pub mod config;

pub fn open(path: &str) -> std::fs::File {
    let file = std::fs::File::open(path).unwrap();
    file
}

pub fn read(file: &std::fs::File) -> String {
    let mut contents = String::new();
    let _ = file
        .take(100)
        .read_to_string(&mut contents);
    contents
}

pub fn write(path: &str, contents: &str) -> bool {
    let mut file = File::create(path).unwrap();
    let _ = file.write_all(contents.as_bytes());
    true
}

pub fn append(path: &str, contents: &str) -> bool {
    let mut file = File
        ::open(path)
        .expect("Unable to open file");
    let _ = file.write_all(contents.as_bytes());
    true
}

pub fn delete(path: &str) -> bool {
    let _ = fs::remove_file(path);
    true
}

pub fn extract(archive_path: &str) -> bool {

    let path = Path::new(archive_path);
    if !path.exists() {
        eprintln!("Archive not found: {}", archive_path);
        return false;
    }

    let destination = match path.parent() {
        Some(parent) => parent.to_str().unwrap_or("."),
        None => ".",
    };
    
    if archive_path.ends_with(".zip") {
        unzip_file(archive_path, destination)
    } else if archive_path.ends_with(".tar.gz") || archive_path.ends_with(".tgz") {
        extract_tar_gz(archive_path, destination)
    } else {
        eprintln!("Unsupported archive format: {}", archive_path);
        false
    }
}

fn unzip_file(zip_path: &str, destination: &str) -> bool {

    let file = match File::open(zip_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open zip file: {}", e);
            return false;
        }
    };

    let mut archive = match ZipArchive::new(file) {
        Ok(archive) => archive,
        Err(e) => {
            eprintln!("Failed to read zip archive: {}", e);
            return false;
        }
    };

    let archive_name = Path::new(zip_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("extracted");
        
    for i in 0..archive.len() {
        let mut file = match archive.by_index(i) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Failed to read file in archive: {}", e);
                continue;
            }
        };

        let outpath = match file.enclosed_name() {
            Some(path) => PathBuf::from(destination).join(archive_name).join(path),
            None => {
                eprintln!("Invalid file path in zip: {}", file.name());
                continue;
            }
        };

        if file.name().ends_with('/') {
            if let Err(e) = fs::create_dir_all(&outpath) {
                eprintln!("Failed to create directory {}: {}", outpath.display(), e);
                continue;
            }
            continue;
        }

        if let Some(parent) = outpath.parent() {
            if !parent.exists() {
                if let Err(e) = fs::create_dir_all(parent) {
                    eprintln!("Failed to create directory {}: {}", parent.display(), e);
                    continue;
                }
            }
        }

        let mut outfile = match File::create(&outpath) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Failed to create file {}: {}", outpath.display(), e);
                continue;
            }
        };

        if let Err(e) = io::copy(&mut file, &mut outfile) {
            eprintln!("Failed to write file {}: {}", outpath.display(), e);
            continue;
        }

        #[cfg(unix)] {
            use std::os::unix::fs::PermissionsExt;
            
            if let Some(mode) = file.unix_mode() {
                if let Err(e) = fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)) {
                    eprintln!("Failed to set permissions for {}: {}", outpath.display(), e);
                }
            }
        }
    }

    find_folder_with_bin_and_copy(&Path::new(destination).join(archive_name), &Path::new(destination)).unwrap();

    println!("Extracted zip to {}/{}", destination, archive_name);
    true
}

fn extract_tar_gz(tar_gz_path: &str, destination: &str) -> bool {

    let file = match File::open(tar_gz_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open tar.gz file: {}", e);
            return false;
        }
    };

    let archive_name = Path::new(tar_gz_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("extracted");
    

    let extract_dir = PathBuf::from(destination).join(archive_name);
    if let Err(e) = fs::create_dir_all(&extract_dir) {
        eprintln!("Failed to create extraction directory: {}", e);
        return false;
    }

    let gz: GzDecoder<File> = GzDecoder::new(file);
    
    let mut archive: Archive<GzDecoder<File>> = Archive::new(gz);
    
    if let Err(e) = archive.unpack(&extract_dir) {
        eprintln!("Failed to extract tar.gz archive: {}", e);
        return false;
    }

    find_folder_with_bin_and_copy(&extract_dir, &Path::new(destination)).unwrap();

    fs::remove_dir_all(&extract_dir).unwrap();
    
    println!("Extracted tar.gz to {}/{}", destination, archive_name);
    true
}


pub fn find_folder_with_bin_and_copy(search_path: &Path, target_path: &Path) -> Result<(), io::Error> {

    if !target_path.exists() {
        fs::create_dir_all(target_path)?;
    }
    
    let source_dir = find_dir_with_bin(search_path)?;

    if let Some(dir) = source_dir {
        copy_dir_contents(&dir, target_path)?;
    } else {
        eprintln!("No directory with 'bin' subdirectory found in {:?}", search_path);
    }

    Ok(())
}

fn find_dir_with_bin(dir: &Path) -> Result<Option<PathBuf>, io::Error> {
    if !dir.is_dir() {
        return Ok(None);
    }
    
    let bin_path = dir.join("bin");
    if bin_path.is_dir() {
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

fn copy_dir_contents(src: &Path, dst: &Path) -> Result<(), io::Error> {

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = path.file_name().unwrap();
        let dst_path = dst.join(file_name);
        
        if path.is_dir() {
            if !dst_path.exists() {
                fs::create_dir_all(&dst_path)?;
            }
            
            copy_dir_contents(&path, &dst_path)?;
        } else {
            fs::copy(&path, &dst_path)?;
        }
    }
    
    Ok(())
}
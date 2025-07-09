use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use super::common::ShellConfig;

pub fn get_pkit_dir() -> io::Result<PathBuf> {
    let pkit_dir = std::env::var("APPDATA")
        .map(PathBuf::from)
        .map_err(|_| io::Error::new(io::ErrorKind::NotFound, "Could not find APPDATA directory"))?
        .join("pkit");
    fs::create_dir_all(&pkit_dir)?;
    Ok(pkit_dir)
}

pub fn get_pkit_config_dir() -> io::Result<PathBuf> {
    get_pkit_dir()
}

pub fn get_pkit_data_dir() -> io::Result<PathBuf> {
    let data_dir = std::env::var("LOCALAPPDATA")
        .map(PathBuf::from)
        .map_err(|_| io::Error::new(io::ErrorKind::NotFound, "Could not find LOCALAPPDATA directory"))?
        .join("pkit");
    fs::create_dir_all(&data_dir)?;
    Ok(data_dir)
}

pub fn get_pkit_cache_dir() -> io::Result<PathBuf> {
    let cache_dir = std::env::var("LOCALAPPDATA")
        .map(|p| PathBuf::from(p).join("cache"))
        .map_err(|_| io::Error::new(io::ErrorKind::NotFound, "Could not find LOCALAPPDATA directory"))?
        .join("pkit");
    fs::create_dir_all(&cache_dir)?;
    Ok(cache_dir)
}

pub fn get_bashrc_path() -> io::Result<PathBuf> {
    get_powershell_profile_path()
}

pub fn get_shell_config_files() -> io::Result<Vec<(ShellConfig, PathBuf)>> {
    Ok(vec![(ShellConfig::PowerShell, get_powershell_profile_path()?)])
}

pub fn get_powershell_profile_path() -> io::Result<PathBuf> {
    let documents = std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .map_err(|_| io::Error::new(io::ErrorKind::NotFound, "Could not find user profile directory"))?;
    
    Ok(PathBuf::from(documents)
        .join("Documents")
        .join("PowerShell")
        .join("Microsoft.PowerShell_profile.ps1"))
}

pub fn get_primary_shell_config_path() -> io::Result<PathBuf> {
    get_powershell_profile_path()
}

pub fn generate_env_setup_lines(pkit_home_str: &str) -> String {
    let current_exe_path = std::env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("C:\\Program Files\\pkit"));
    
    let exe_path_str = current_exe_path.to_str().unwrap_or("C:\\Program Files\\pkit");
    
    format!(
        "\n# pkit-cli-env-start\n$env:PKIT_HOME = \"{}\"\n$env:PATH = \"$env:PATH;{}\"\nif (Test-Path \"$env:PKIT_HOME\\pkit_env.ps1\") {{ . \"$env:PKIT_HOME\\pkit_env.ps1\" }}\n# pkit-cli-env-end\n",
        pkit_home_str, exe_path_str
    )
}

pub fn generate_path_export(bin_path: &str) -> String {
    format!("$env:PATH = \"{};{}\"\n", bin_path, "$env:PATH")
}

fn clean_pkit_entries_from_file(config_path: &PathBuf) -> io::Result<()> {
    if !config_path.exists() {
        return Ok(());
    }

    let content = fs::read_to_string(config_path)?;
    let cleaned_lines: Vec<String> = content
        .lines()
        .scan(false, |in_pkit_block, line| {
            if line.contains("# pkit-cli-env-start") {
                *in_pkit_block = true;
                return Some(None);
            }
            if line.contains("# pkit-cli-env-end") {
                *in_pkit_block = false;
                return Some(None);
            }
            if line.contains("$env:PKIT_HOME =") ||
               line.contains("if (Test-Path \"$env:PKIT_HOME\\pkit_env.ps1\")") {
                return Some(None);
            }
            
            if !*in_pkit_block {
                Some(Some(line.to_string()))
            } else {
                Some(None)
            }
        })
        .filter_map(|x| x)
        .collect();

    fs::write(config_path, cleaned_lines.join("\n"))?;
    Ok(())
}

pub fn clean_bashrc_pkit_entries() -> io::Result<()> {
    clean_pkit_entries_from_file(&get_powershell_profile_path()?)
}

pub fn setup_shell_environment() -> io::Result<()> {
    let shell_config_path = get_primary_shell_config_path()?;
    let pkit_home_path = get_pkit_dir()?;
    let pkit_home_str = pkit_home_path.to_str().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidData, "Invalid pkit home path")
    })?;

    let pkit_env_setup_lines = generate_env_setup_lines(pkit_home_str);

    if shell_config_path.exists() {
        let shell_content = fs::read_to_string(&shell_config_path)?;
        if !shell_content.contains("# pkit-cli-env-start") {
            clean_shell_pkit_entries()?;
            let mut file = fs::OpenOptions::new()
                .append(true)
                .open(&shell_config_path)?;
            file.write_all(pkit_env_setup_lines.as_bytes())?;
            file.flush()?;
            println!("Added pkit environment setup to {:?}. Please restart your terminal or reload your shell configuration.", shell_config_path);
        }
    } else {
        if let Some(parent) = shell_config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = fs::File::create(&shell_config_path)?;
        file.write_all(pkit_env_setup_lines.as_bytes())?;
        file.flush()?;
        println!("Created {:?} and added pkit environment setup. Please restart your terminal or reload your shell configuration.", shell_config_path);
    }

    Ok(())
}

pub fn clean_shell_pkit_entries() -> io::Result<()> {
    for (_, config_path) in get_shell_config_files()? {
        if config_path.exists() {
            clean_pkit_entries_from_file(&config_path)?;
        }
    }
    Ok(())
}

pub fn reload_environment() {
    if let Ok(pkit_dir) = get_pkit_dir() {
        let env_script_path = pkit_dir.join("pkit_env.ps1");
        
        if env_script_path.exists() {
            let _ = setup_shell_environment();
        }
    }
}

pub fn reload_environment_with_new_shell() -> io::Result<()> {
    if let Ok(pkit_dir) = get_pkit_dir() {
        let env_script_path = pkit_dir.join("pkit_env.ps1");
        
        if env_script_path.exists() {
            let _ = setup_shell_environment();
            
            println!("Starting new PowerShell session with updated environment...");
            println!("Type 'exit' to return to the previous session.");
            println!();
            
            let mut cmd = std::process::Command::new("powershell");
            cmd.arg("-Command").arg(format!(". '{}'; powershell", env_script_path.display()));
            
            let status = cmd.status()?;
            
            println!("{}", if status.success() {
                "PowerShell session ended. Environment changes are now active."
            } else {
                "PowerShell session ended with error."
            });
        }
    }
    
    Ok(())
}

pub fn replace_current_shell() -> io::Result<()> {
    if let Ok(pkit_dir) = get_pkit_dir() {
        let env_script_path = pkit_dir.join("pkit_env.ps1");
        
        if env_script_path.exists() {
            let _ = setup_shell_environment();
            
            println!("Starting new PowerShell session with updated environment...");
            println!();
            
            let mut cmd = std::process::Command::new("powershell");
            cmd.arg("-Command").arg(format!(". '{}'; powershell", env_script_path.display()));
            
            let status = cmd.status()?;
            
            println!("{}", if status.success() {
                "PowerShell session ended. Environment changes are now active."
            } else {
                "PowerShell session ended with error."
            });
        }
    }
    
    Ok(())
}

pub fn setup_shell_function() -> io::Result<()> {
    let shell_config_path = get_primary_shell_config_path()?;
    let pkit_dir = get_pkit_dir()?;
    let env_script_path = pkit_dir.join("pkit_env.ps1");
    
    let shell_function = format!(
        "\nfunction pkit {{\n    & pkit @args\n    if ($args[0] -eq \"default\" -or $args[0] -eq \"install\") {{\n        if (Test-Path \"{}\") {{\n            . \"{}\"\n        }}\n    }}\n}}\n",
        env_script_path.display(),
        env_script_path.display()
    );
    
    if shell_config_path.exists() {
        let shell_content = fs::read_to_string(&shell_config_path)?;
        if !shell_content.contains("function pkit {") {
            let mut file = fs::OpenOptions::new()
                .append(true)
                .open(&shell_config_path)?;
            file.write_all(shell_function.as_bytes())?;
            file.flush()?;
            println!("Added pkit shell function to {:?}", shell_config_path);
            println!("Please restart your terminal or run: . {}", shell_config_path.display());
        } else {
            println!("pkit shell function already exists in {:?}", shell_config_path);
        }
    } else {
        if let Some(parent) = shell_config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = fs::File::create(&shell_config_path)?;
        file.write_all(shell_function.as_bytes())?;
        file.flush()?;
        println!("Created {:?} with pkit shell function", shell_config_path);
        println!("Please restart your terminal or run: . {}", shell_config_path.display());
    }
    
    Ok(())
}

pub fn update_shell_function() -> io::Result<()> {
    let shell_config_path = get_primary_shell_config_path()?;
    
    if !shell_config_path.exists() {
        return setup_shell_function();
    }
    
    let shell_content = fs::read_to_string(&shell_config_path)?;
    
    let lines: Vec<&str> = shell_content.lines().collect();
    let mut cleaned_lines = Vec::new();
    let mut skip_function = false;
    
    for line in lines {
        if line.trim().starts_with("function pkit {") {
            skip_function = true;
            continue;
        }
        
        if skip_function {
            if line.trim() == "}" {
                skip_function = false;
                continue;
            }
            continue;
        }
        
        cleaned_lines.push(line);
    }
    
    fs::write(&shell_config_path, cleaned_lines.join("\n"))?;
    
    setup_shell_function()
}

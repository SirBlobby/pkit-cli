use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use super::common::{OperatingSystem, ShellConfig, detect_os, get_home_dir};

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

pub fn get_bashrc_path() -> io::Result<PathBuf> {
    Ok(get_home_dir()?.join(".bashrc"))
}

pub fn get_shell_config_files() -> io::Result<Vec<(ShellConfig, PathBuf)>> {
    let home_dir = get_home_dir()?;
    let configs = match detect_os() {
        OperatingSystem::MacOS => vec![
            (ShellConfig::Zsh, home_dir.join(".zshrc")),
            (ShellConfig::Bash, home_dir.join(".bash_profile")),
            (ShellConfig::Bash, home_dir.join(".bashrc")),
        ],
        OperatingSystem::Linux => vec![
            (ShellConfig::Bash, home_dir.join(".bashrc")),
            (ShellConfig::Zsh, home_dir.join(".zshrc")),
            (ShellConfig::Fish, home_dir.join(".config/fish/config.fish")),
        ],
        OperatingSystem::Other => vec![
            (ShellConfig::Bash, home_dir.join(".bashrc")),
            (ShellConfig::Bash, home_dir.join(".bash_profile")),
        ],
        _ => unreachable!("This should only be called on Unix systems"),
    };
    Ok(configs)
}

pub fn get_primary_shell_config_path() -> io::Result<PathBuf> {
    match detect_os() {
        OperatingSystem::MacOS => {
            let home_dir = get_home_dir()?;
            let zshrc = home_dir.join(".zshrc");
            if zshrc.exists() {
                Ok(zshrc)
            } else {
                Ok(home_dir.join(".bash_profile"))
            }
        }
        OperatingSystem::Linux | OperatingSystem::Other => get_bashrc_path(),
        _ => unreachable!("This should only be called on Unix systems"),
    }
}

pub fn generate_env_setup_lines(pkit_home_str: &str) -> String {
    format!(
        "\n\n# pkit-cli-env-start\nexport PKIT_HOME=\"{}\"\n[[ -s \"$PKIT_HOME/pkit_env.sh\" ]] && source \"$PKIT_HOME/pkit_env.sh\"\n# pkit-cli-env-end\n",
        pkit_home_str
    )
}

pub fn generate_path_export(bin_path: &str) -> String {
    format!("export PATH=\"{}:$PATH\"\n", bin_path)
}

fn clean_pkit_entries_from_file(config_path: &PathBuf) -> io::Result<()> {
    if !config_path.exists() {
        return Ok(());
    }

    let content = fs::read_to_string(config_path)?;
    let lines: Vec<&str> = content.lines().collect();
    let mut cleaned_lines = Vec::new();
    let mut in_pkit_env_block = false;
    let mut in_pkit_function = false;
    
    for line in lines {
        if line.contains("# pkit-cli-env-start") {
            in_pkit_env_block = true;
            continue;
        }
        if line.contains("# pkit-cli-env-end") {
            in_pkit_env_block = false;
            continue;
        }
        
        if line.trim().starts_with("pkit() {") {
            in_pkit_function = true;
            continue;
        }
        if in_pkit_function && line.trim() == "}" {
            in_pkit_function = false;
            continue;
        }
        
        if line.contains("export PKIT_HOME=") ||
           line.contains("[[ -s \"$PKIT_HOME/pkit_env.sh\" ]] && source \"$PKIT_HOME/pkit_env.sh\"") {
            continue;
        }
        
        if !in_pkit_env_block && !in_pkit_function {
            cleaned_lines.push(line);
        }
    }

    fs::write(config_path, cleaned_lines.join("\n"))?;
    Ok(())
}

pub fn setup_shell_environment() -> io::Result<()> {
    let shell_config_path = get_primary_shell_config_path()?;
    let pkit_home_path = get_pkit_dir()?;
    let pkit_home_str = pkit_home_path.to_str().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidData, "Invalid pkit home path")
    })?;

    let pkit_env_setup_lines = generate_env_setup_lines(pkit_home_str);
    let shell_function = generate_shell_function();
    let combined_setup = format!("{}\n{}", pkit_env_setup_lines, shell_function);

    if let Some(parent) = shell_config_path.parent() {
        fs::create_dir_all(parent)?;
    }

    if shell_config_path.exists() {
        let shell_content = fs::read_to_string(&shell_config_path)?;
        let has_env_setup = shell_content.contains("# pkit-cli-env-start");
        let has_shell_function = shell_content.contains("pkit() {");
        
        if !has_env_setup || !has_shell_function {
            clean_shell_pkit_entries()?;
            let mut file = fs::OpenOptions::new()
                .append(true)
                .open(&shell_config_path)?;
            file.write_all(combined_setup.as_bytes())?;
            file.flush()?;
            println!("Added pkit environment setup and shell function to {:?}. Please restart your terminal or reload your shell configuration.", shell_config_path);
        }
    } else {
        let mut file = fs::File::create(&shell_config_path)?;
        file.write_all(combined_setup.as_bytes())?;
        file.flush()?;
        println!("Created {:?} and added pkit environment setup and shell function. Please restart your terminal or reload your shell configuration.", shell_config_path);
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
        let env_script_path = pkit_dir.join("pkit_env.sh");
        
        if env_script_path.exists() {
            let _ = setup_shell_environment();
        }
    }
}

pub fn generate_shell_function() -> String {
    format!(
        "pkit() {{\n  command pkit \"$@\"\n\n  local env_file=\"${{PKIT_HOME:-$HOME/.pkit}}/pkit_env.sh\"\n\n  if [[ -f \"$env_file\" && -r \"$env_file\" ]]; then\n    case \"$1\" in\n      default|install|uninstall)\n        source \"$env_file\" && echo \"pkit environment reloaded.\"\n        ;;\n    esac\n  elif [[ \"$1\" == \"default\" || \"$1\" == \"install\" || \"$1\" == \"uninstall\" ]]; then\n    echo \"Warning: Environment file not found at $env_file\" >&2\n  fi\n}}\n"
    )
}

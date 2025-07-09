use std::fs;
use std::path::{Path, PathBuf};
use std::io::{self, Write};
use crate::filesystem::{self, get_pkit_dir};
use json;

#[derive(Clone)]
pub struct Installed {
    pub language: String,
    pub version: String,
    pub path: String,
    pub default: bool,
}

pub struct Config {
    pub path: PathBuf,
    pub installed: Vec<Installed>,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Config {
        let pkit_dir = get_pkit_dir().expect("Failed to get .pkit directory");
        Self::ensure_required_dirs_exist(&pkit_dir).expect("Failed to create required directories");

        let config_path = pkit_dir.join("pkit.json");

        if !config_path.exists() {
            let config = Config {
                path: pkit_dir,
                installed: Vec::new(),
            };
            config.write().expect("Failed to write initial config");
            config
        } else {
            Self::read().expect("Failed to read config file")
        }
    }

    fn ensure_required_dirs_exist(pkit_dir: &Path) -> std::io::Result<()> {
        fs::create_dir_all(pkit_dir.join("bin"))?;
        Ok(())
    }

    pub fn read() -> std::io::Result<Config> {
        let config_path = get_pkit_dir()?.join("pkit.json");
        let json_str = filesystem::read(&config_path)?;
        let json_data = json::parse(&json_str).map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, e)
        })?;

        let mut installed = Vec::new();
        for install in json_data["installed"].members() {
            installed.push(Installed {
                language: install["language"].to_string(),
                version: install["version"].to_string(),
                path: install["path"].to_string(),
                default: install["default"].as_bool().unwrap_or(false),
            });
        }

        let pkit_dir = get_pkit_dir()?;
        Ok(Config {
            path: pkit_dir,
            installed,
        })
    }

    pub fn write(&self) -> std::io::Result<()> {
        let mut json_data = json::JsonValue::new_object();
        let mut installed = json::JsonValue::new_array();

        for install in &self.installed {
            let mut install_json = json::JsonValue::new_object();
            install_json["language"] = install.language.clone().into();
            install_json["version"] = install.version.clone().into();
            install_json["path"] = install.path.clone().into();
            install_json["default"] = install.default.into();
            let _ = installed.push(install_json);
        }

        json_data["installed"] = installed;
        json_data["path"] = self.path.to_str().unwrap_or("").into();

        let config_path = self.path.join("pkit.json");
        filesystem::write(&config_path, &json_data.pretty(2))
    }

    pub fn add(&mut self, language: &str, version: &str, path: &str, default: bool) {
        if self.get(language, version).is_some() {
            self.update(language, version, path);
            if default {
                self.set_default(language, version);
                return; // set_default already calls write()
            }
        } else {
            // If setting as default, ensure no other version of this language is default
            if default {
                for install in &mut self.installed {
                    if install.language == language {
                        install.default = false;
                    }
                }
            }
            
            self.installed.push(Installed {
                language: language.to_string(),
                version: version.to_string(),
                path: path.to_string(),
                default,
            });
        }
        self.write().expect("Failed to save config after add");
    }

    pub fn remove(&mut self, language: &str, version: &str) {
        self.installed.retain(|pkg| !(pkg.language == language && pkg.version == version));
        self.write().expect("Failed to save config after remove");
    }

    pub fn set_default(&mut self, language: &str, version: &str) {
        for install in &mut self.installed {
            if install.language == language {
                install.default = install.version == version;
            }
        }
        self.write().expect("Failed to save config after set_default");
    }

    pub fn get_default(&self, language: &str) -> Option<&Installed> {
        self.installed.iter().find(|&install| install.language == language && install.default)
    }

    pub fn get(&self, language: &str, version: &str) -> Option<&Installed> {
        self.installed.iter().find(|&install| install.language == language && install.version == version)
    }

    pub fn update(&mut self, language: &str, version: &str, path: &str) {
        for install in &mut self.installed {
            if install.language == language && install.version == version {
                install.path = path.to_string();
            }
        }
        self.write().expect("Failed to save config after update");
    }

    pub fn write_env_script(&self) -> io::Result<()> {
        let pkit_dir = get_pkit_dir()?;
        
        let env_script_filename = if cfg!(windows) {
            "pkit_env.ps1"
        } else {
            "pkit_env.sh"
        };
        
        let env_script_path = pkit_dir.join(env_script_filename);

        let mut file = fs::File::create(&env_script_path)?;
        
        if env_script_filename.ends_with(".ps1") {
            writeln!(file, "# pkit environment script - automatically generated")?;
        } else {
            writeln!(file, "#!/bin/sh")?;
            writeln!(file, "# pkit environment script - automatically generated")?;
        }
        writeln!(file)?;

        for install in &self.installed {
            if install.default {
                let bin_path = PathBuf::from(&install.path).join("bin");
                if cfg!(windows) {
                    writeln!(file, "$env:PATH = \"{};$env:PATH\"", bin_path.display())?;
                } else {
                    writeln!(file, "export PATH=\"{}:$PATH\"", bin_path.display())?;
                }
            }
        }
        Ok(())
    }
}
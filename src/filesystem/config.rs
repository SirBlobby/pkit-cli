use std::fs;
use std::path::{Path, PathBuf};
use std::io::{self, Write};
use crate::filesystem::{self, get_pkit_dir};
use json;
use crate::formatter::{capitalize_first, print_box, BoxAlignment, BoxOptions};


#[derive(Clone)]
pub struct Installed {
    pub language: String,
    pub version: String,
    pub path: String,
    pub default: bool,
}

pub struct Source {
    pub name: String,
    pub path: String,
}

pub struct Config {
    pub path: PathBuf,
    pub installed: Vec<Installed>,
    pub sources: Vec<Source>
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
                sources: Vec::new(),
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

        let mut sources = Vec::new();
        for source in json_data["sources"].members() {
            sources.push(Source {
                name: source["name"].to_string(),
                path: source["path"].to_string(),
            });
        }

        let pkit_dir = get_pkit_dir()?;
        Ok(Config {
            path: pkit_dir,
            installed,
            sources
        })
    }

    pub fn write(&self) -> std::io::Result<()> {
        let mut json_data = json::JsonValue::new_object();
        let mut installed = json::JsonValue::new_array();
        let mut sources_array = json::JsonValue::new_array();

        for install in &self.installed {
            let mut install_json = json::JsonValue::new_object();
            install_json["language"] = install.language.clone().into();
            install_json["version"] = install.version.clone().into();
            install_json["path"] = install.path.clone().into();
            install_json["default"] = install.default.into();
            let _ = installed.push(install_json);
        }

        for source in &self.sources {
            let mut source_json = json::JsonValue::new_object();
            source_json["name"] = source.name.clone().into();
            source_json["path"] = source.path.clone().into();
            let _ = sources_array.push(source_json);
        }
        
        json_data["sources"] = sources_array;
        json_data["installed"] = installed;
        json_data["path"] = self.path.to_str().unwrap_or("").into();

        let config_path = self.path.join("pkit.json");
        filesystem::write(&config_path, &json_data.pretty(2))
    }

    pub fn add_install(&mut self, language: &str, version: &str, path: &str, default: bool) {
        if self.get(language, version).is_some() {
            self.update_install(language, version, path);
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

    pub fn remove_install(&mut self, language: &str, version: &str) {
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

    pub fn update_install(&mut self, language: &str, version: &str, path: &str) {
        for install in &mut self.installed {
            if install.language == language && install.version == version {
                install.path = path.to_string();
            }
        }
        self.write().expect("Failed to save config after update");
    }

    pub fn add_path_source(&mut self, name: &str, path: &str) {
        if self.sources.iter().any(|s| s.name == name) {
            eprintln!("Source with name '{}' already exists.", name);
            return;
        }
        self.sources.push(Source {
            name: name.to_string(),
            path: path.to_string(),
        });

        self.write().expect("Failed to save config after add_path_source");
    }

    pub fn remove_path_source(&mut self, name: &str) {
        self.sources.retain(|source| source.name != name);
        self.write().expect("Failed to save config after remove_path_source");
    }

    pub fn get_path_source(&self, name: &str) -> Option<&Source> {
        self.sources.iter().find(|source| source.name == name)
    }

    pub fn get_path_source_mut(&mut self, name: &str) -> Option<&mut Source> {
        self.sources.iter_mut().find(|source| source.name == name)
    }

    pub fn set_path_source(&mut self, name: &str, path: &str) {
        if let Some(source) = self.get_path_source_mut(name) {
            if !Path::new(path).exists() {
                eprintln!("Path '{}' does not exist.", path);
                return;
            }
            source.path = path.to_string();
            self.write().expect("Failed to save config after set_path_source");
        } else {
            eprintln!("Source with name '{}' not found.", name);
        }
    }

    pub fn print_installed(&self) {
        if self.installed.is_empty() {
            println!("No packages installed.");
            return;
        }

        let mut lines: Vec<(&str, BoxAlignment)> = Vec::new();
        for install in &self.installed {
            let default_marker = if install.default { "*" } else { " " };
            let line = format!(
                "{} {} {} ({})",
                default_marker,
                capitalize_first(&install.language),
                install.version,
                install.path
            );
            lines.push((Box::leak(line.into_boxed_str()), BoxAlignment::Left));
        }

        print_box(
            &lines,
            &BoxOptions {
                ..Default::default()
            },
        );
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

        for source in &self.sources {
            if cfg!(windows) {
                writeln!(file, "$env:PATH = \"{};$env:PATH\"", source.path)?;
            } else {
                writeln!(file, "export PATH=\"{}:$PATH\"", source.path)?;
            }
        }

        Ok(())
    }
}
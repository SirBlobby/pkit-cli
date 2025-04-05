
use std::{io::Read, path};

use crate::filesystem;

use json;

pub struct Installed {
    pub language: String,
    pub version: String,
    pub path: String,
    pub default: bool
}

pub struct Config {
    pub path: String,
    pub installed: Vec<Installed>
}
 
impl Config {

    pub fn new() -> Config {
        let root_path = std::env::current_exe().unwrap();
        let parent_path = root_path.parent().unwrap();

        println!("Parent path: {}", parent_path.display());

        Self::files_check(&format!("{}", parent_path.display()));

        let config_path = format!("{}/bin/pkit.json", parent_path.display());

        let config: Config;

        if !path::Path::new(&config_path).exists() {
            config = Config { path: format!("{}", parent_path.display()), installed: Vec::new() };
            config.write();
        } else {
            config = Config::read();
        }
        config
    }

    pub fn files_check(path: &str) {
        let home_path = std::fs::create_dir_all(path);

        if home_path.is_err() {
            println!("Error creating directory: {}", home_path.err().unwrap());
            std::process::exit(1);
        }

        let _ = std::fs::create_dir_all(&format!("{}/bin", path));
    }

    pub fn read() -> Config {
        let file: std::fs::File = filesystem::open("./bin/pkit.json");
        let reader = std::io::BufReader::new(file);
        let json_str: String = reader.bytes().map(|x| x.unwrap() as char).collect();
        let json_data: json::JsonValue = json::parse(&json_str).unwrap();

        let mut installed: Vec<Installed> = Vec::new();

        for install in json_data["installed"].members() {
            installed.push(Installed { 
                language: install["language"].to_string(), 
                version: install["version"].to_string(), 
                path: install["path"].to_string(), 
                default: install["default"].as_bool().unwrap() 
            });
        }

        Config { path: json_data["path"].to_string(), installed }
    }

    pub fn write(&self) {
        let mut json_data: json::JsonValue = json::JsonValue::new_object();
        let mut installed: json::JsonValue = json::JsonValue::new_array();

        for install in &self.installed {
            let mut install_json: json::JsonValue = json::JsonValue::new_object();
            install_json["language"] = install.language.clone().into();
            install_json["version"] = install.version.clone().into();
            install_json["path"] = install.path.clone().into();
            install_json["default"] = install.default.into();
            let _ = installed.push(install_json);
        }

        json_data["installed"] = installed;
        json_data["path"] = self.path.clone().into();

        filesystem::write("./bin/pkit.json", &json_data.dump());
    }


    pub fn add(&mut self, language: &str, version: &str, path: &str, default: bool) {
        let install: Option<&Installed> = self.get(language, version);
        if install.is_some() {
            self.update(language, version, path);
        } else {
            self.installed.push(Installed { language: language.to_string(), version: version.to_string(), path: path.to_string(), default });
            self.write();
        }
    }

    pub fn remove(&mut self, language: &str) {
        let mut index = 0;
        for (i, install) in self.installed.iter().enumerate() {
            if install.language == language {
                index = i;
                break;
            }
        }
        self.installed.remove(index);
        self.write();
    }

    pub fn set_default(&mut self, language: &str, version: &str) {
        for install in &mut self.installed {
            if install.language == language {
                install.default = install.version == version;
            }
        }
        self.write();
    }

    pub fn get_default(&self, language: &str) -> Option<&Installed> {
        for install in &self.installed {
            if install.language == language && install.default {
                return Some(install);
            }
        }
        None
    }

    pub fn get(&self, language: &str, version: &str) -> Option<&Installed> {
        for install in &self.installed {
            if install.language == language && install.version == version {
                return Some(install);
            }
        }
        None
    }

    pub fn update(&mut self, language: &str, version: &str, path: &str) {
        for install in &mut self.installed {
            if install.language == language && install.version == version {
                install.path = path.to_string();
            }
        }
        self.write();
    }

}
use crate::filesystem::config::Config;
use crate::formatter::{
    capitalize_first, print_box, BoxAlignment, BoxOptions,
};
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use crate::filesystem::get_pkit_dir;

pub fn handle_switch_command(language: &str, version: &str) {
    let config = Config::new();

    // Check if the specified language and version is installed
    if let Some(installed) = config.get(language, version) {
        // Create session-specific environment script
        if let Err(e) = write_session_env_script(&config, language, &installed.path) {
            print_error_message(&format!("Failed to create session environment: {}", e));
            std::process::exit(1);
        }
        
        print_success_message(language, version);

    } else {
        print_not_installed_message(language, version);
        std::process::exit(1);
    }
}

fn write_session_env_script(config: &Config, session_language: &str, session_path: &str) -> std::io::Result<()> {
    let pkit_dir = get_pkit_dir()?;
    
    let env_script_filename = if cfg!(windows) {
        "pkit_session_env.ps1"
    } else {
        "pkit_session_env.sh"
    };
    
    let env_script_path = pkit_dir.join(env_script_filename);
    let mut file = fs::File::create(&env_script_path)?;
    
    if env_script_filename.ends_with(".ps1") {
        writeln!(file, "# pkit session environment script - automatically generated")?;
        writeln!(file, "# This temporarily overrides the default language for this session")?;
    } else {
        writeln!(file, "#!/bin/sh")?;
        writeln!(file, "# pkit session environment script - automatically generated")?;
        writeln!(file, "# This temporarily overrides the default language for this session")?;
    }
    writeln!(file)?;

    // Add the session-specific language first (highest priority)
    let session_bin_path = PathBuf::from(session_path).join("bin");
    if cfg!(windows) {
        writeln!(file, "$env:PATH = \"{};$env:PATH\"", session_bin_path.display())?;
    } else {
        writeln!(file, "export PATH=\"{}:$PATH\"", session_bin_path.display())?;
    }
    
    // Add other default languages (excluding the session language to avoid duplication)
    for install in &config.installed {
        if install.default && install.language != session_language {
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

fn print_success_message(language: &str, version: &str) {
    let box_options = BoxOptions {
        title: Some("Session Switch Successful"),
        title_color: 'a',
        border_color: 'a',
    };

    let success_msg = format!("Temporarily switched to &e{} {}&r", capitalize_first(language), version);
    let lines = vec![
        (success_msg.as_str(), BoxAlignment::Center),
        ("", BoxAlignment::Center),
        ("This change only affects the current session.", BoxAlignment::Center),
        ("To make this permanent, use 'pkit default' instead.", BoxAlignment::Center),
    ];

    print_box(&lines, &box_options);
}

fn print_not_installed_message(language: &str, version: &str) {
    let box_options = BoxOptions {
        title: Some("Package Not Found"),
        title_color: 'c',
        border_color: 'c',
    };

    let not_found_msg = format!("{} {} is not installed", capitalize_first(language), version);
    let install_cmd = format!("pkit install {} {}", language, version);
    let lines = vec![
        (not_found_msg.as_str(), BoxAlignment::Center),
        ("", BoxAlignment::Center),
        ("Try installing it first with:", BoxAlignment::Center),
        (install_cmd.as_str(), BoxAlignment::Center),
    ];

    print_box(&lines, &box_options);
}

fn print_error_message(message: &str) {
    let box_options = BoxOptions {
        title: Some("Error"),
        title_color: 'c',
        border_color: 'c',
    };

    let lines = vec![
        (message, BoxAlignment::Center),
    ];

    print_box(&lines, &box_options);
}

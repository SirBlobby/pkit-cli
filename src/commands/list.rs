use crate::{
    formatter::{
        capitalize_first, print_colored, print_box_top, print_box_bottom, 
        print_box_line_centered, print_usage_box_top, print_usage_box_bottom, 
        print_usage_box_line, print_table_header, print_table_row, print_table_footer
    },
    api::{self},
    filesystem::config::{Config, Installed}
};

pub async fn print_available_languages() {
    let mut languages: Vec<String> = api::get_languages().await;

    for lang in languages.iter_mut() {
        *lang = capitalize_first(lang);
    }

    println!();
    print_box_top();
    print_box_line_centered("&aAvailable Languages&r");
    print_box_bottom();
    println!();
    
    // Display languages in a nice grid format
    let mut output = String::new();
    for (i, lang) in languages.iter().enumerate() {
        if i > 0 {
            output.push_str("&8  •  &r");
        }
        output.push_str(&format!("&e{}&r", lang));
    }
    
    print_colored(&format!("  {}", output));
    println!();
    println!();
    print_usage_box_top("Usage");
    print_usage_box_line("  &3pkit list <language>&r  &8-&r  List available versions for a specific language     ");
    print_usage_box_bottom();
    println!();
}

pub async fn print_available_language_versions(language: &str) {
    let language: api::Language = api::get_language(language.to_lowercase().as_str()).await;

    println!();
    print_box_top();
    print_box_line_centered(&format!("&aAvailable Versions for &e{}&r", capitalize_first(&language.name)));
    print_box_bottom();
    println!();
    
    // Define table columns: (title, width)
    let columns = [
        ("Version", 20),
        ("Platform", 16), 
        ("Architecture", 14)
    ];
    
    print_table_header(&columns);
    
    for version in language.versions.iter() {
        let version_str = format!("&e{}&r", version.version);
        let platform_str = format!("&3{}&r", version.platform);
        let arch_str = format!("&5{}&r", version.arch);
        
        let values = [
            version_str.as_str(),
            platform_str.as_str(),
            arch_str.as_str()
        ];
        print_table_row(&columns, &values);
    }
    
    print_table_footer(&columns);
    println!();
    print_usage_box_top("Installation");
    print_usage_box_line(&format!("  &3pkit install {} <version>&r  &8-&r  Install a specific version                   ", language.name));
    print_usage_box_bottom();
    println!();
}

pub fn print_installed_languages() {
    let config = Config::new();
    let installed: Vec<Installed> = config.installed;

    println!();
    print_box_top();
    print_box_line_centered("&aInstalled Languages&r");
    print_box_bottom();
    println!();
    
    if installed.is_empty() {
        print_colored("&8  No languages are currently installed.&r");
        println!();
        print_usage_box_top("Getting Started");
        print_usage_box_line("  &3pkit list&r              &8-&r  View available languages                           ");
        print_usage_box_line("  &3pkit install <lang>&r    &8-&r  Install a language                               ");
        print_usage_box_bottom();
    } else {
        // Define table columns: (title, width)
        let columns = [
            ("Language", 20),
            ("Version", 17),
            ("Status", 12)
        ];
        
        print_table_header(&columns);
        
        for lang in installed.iter() {
            let language_str = format!("&e{}&r", lang.language);
            let version_str = format!("&3{}&r", lang.version);
            let status_str = if lang.default { "&aDefault&r".to_string() } else { "&8Available&r".to_string() };
            
            let values = [
                language_str.as_str(),
                version_str.as_str(),
                status_str.as_str()
            ];
            print_table_row(&columns, &values);
        }
        
        print_table_footer(&columns);
        println!();
        print_usage_box_top("Management");
        print_usage_box_line("  &3pkit default <language>&r  &8-&r  Set a language as default                      ");
        print_usage_box_line("  &3pkit list <language>&r    &8-&r  View available versions for a language         ");
        print_usage_box_bottom();
    }
    println!();
}

pub async fn handle_list_command(language: Option<&String>, installed: bool) {
    if installed {
        print_installed_languages();
    } else if let Some(lang) = language {
        print_available_language_versions(lang).await;
    } else {
        print_available_languages().await;
    }
}

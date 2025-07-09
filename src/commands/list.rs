use crate::{
    api::{self},
    filesystem::config::{Config, Installed},
    formatter::{
        capitalize_first, colorize, print_box, print_table_footer, print_table_header,
        print_table_row, BoxAlignment, BoxOptions,
    },
};

pub async fn print_available_languages() {
    let mut languages: Vec<String> = api::get_languages().await;

    for lang in languages.iter_mut() {
        *lang = capitalize_first(lang);
    }

    println!();
    print_box(&[("&aAvailable Languages&r", BoxAlignment::Center)], &BoxOptions::default());
    println!();

    let mut output = String::new();
    for (i, lang) in languages.iter().enumerate() {
        if i > 0 {
            output.push_str("&8  •  &r");
        }
        output.push_str(&format!("&e{}&r", lang));
    }

    println!("{}", colorize(&format!("  {}", output)));
    println!();
    println!();
    print_box(
        &[(" &3pkit list <language>&r  &8-&r  List available versions for a specific language     ", BoxAlignment::Left)],
        &BoxOptions {
            title: Some("Usage"),
            ..Default::default()
        },
    );
    println!();
}

pub async fn print_available_language_versions(language: &str) {
    let language_data: api::Language = api::get_language(language.to_lowercase().as_str()).await;

    println!();
    let title = format!("&aAvailable Versions for &e{}&r", capitalize_first(&language_data.name));
    print_box(&[(title.as_str(), BoxAlignment::Center)], &BoxOptions::default());
    println!();

    let columns = [("Version", 20), ("Platform", 16), ("Architecture", 14)];

    print_table_header(&columns);

    for version in language_data.versions.iter() {
        let version_str = format!("&e{}&r", version.version);
        let platform_str = format!("&3{}&r", version.platform);
        let arch_str = format!("&5{}&r", version.arch);

        let values = [version_str.as_str(), platform_str.as_str(), arch_str.as_str()];
        print_table_row(&columns, &values);
    }

    print_table_footer(&columns);
    println!();
    let usage_line = format!(" &3pkit install {} <version>&r  &8-&r  Install a specific version                   ", language_data.name);
    print_box(
        &[(usage_line.as_str(), BoxAlignment::Left)],
        &BoxOptions {
            title: Some("Installation"),
            ..Default::default()
        },
    );
    println!();
}

pub fn print_installed_languages() {
    let config = Config::new();
    let installed: Vec<Installed> = config.installed;

    println!();
    print_box(&[("&aInstalled Languages&r", BoxAlignment::Center)], &BoxOptions::default());
    println!();

    if installed.is_empty() {
        println!("{}", colorize("&8  No languages are currently installed.&r"));
        println!();
        let lines = [
            (" &3pkit list&r              &8-&r  View available languages                           ", BoxAlignment::Left),
            (" &3pkit install <lang>&r    &8-&r  Install a language                               ", BoxAlignment::Left),
        ];
        print_box(
            &lines,
            &BoxOptions {
                title: Some("Getting Started"),
                ..Default::default()
            },
        );
    } else {
        let columns = [("Language", 20), ("Version", 17), ("Status", 12)];

        print_table_header(&columns);

        for lang in installed.iter() {
            let language_str = format!("&e{}&r", lang.language);
            let version_str = format!("&3{}&r", lang.version);
            let status_str = if lang.default { "&aDefault&r".to_string() } else { "&8Available&r".to_string() };

            let values = [language_str.as_str(), version_str.as_str(), status_str.as_str()];
            print_table_row(&columns, &values);
        }

        print_table_footer(&columns);
        println!();
        let lines = [
            (" &3pkit default <language>&r  &8-&r  Set a language as default                      ", BoxAlignment::Left),
            (" &3pkit list <language>&r    &8-&r  View available versions for a language         ", BoxAlignment::Left),
        ];
        print_box(
            &lines,
            &BoxOptions {
                title: Some("Management"),
                ..Default::default()
            },
        );
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
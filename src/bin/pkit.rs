use pkit::filesystem::config::Config;
use pkit::cli::{Cli, Commands};
use pkit::commands::{list, install, default, uninstall, switch, path};

// PATH="$(pwd):$PATH"

#[tokio::main]
async fn main() {
    let cli = Cli::parse_args();

    let _ = Config::new();

    match &cli.command {
        Commands::List { language, installed } => {
            list::handle_list_command(language.as_ref(), *installed).await;
        }
        Commands::Install { language, version } => {
            install::handle_install_command(language, version).await;
        }
        Commands::Default { language, version, show } => {
            default::handle_default_command(language, version.as_ref(), *show);
        }
        Commands::Uninstall { language, version, all } => {
            uninstall::handle_uninstall_command(language, version.as_ref(), *all);
        }
        Commands::Switch { language, version } => {
            switch::handle_switch_command(language, version);
        }
        Commands::Path { action, name, path } => {
            path::handle_path_command(action, name.as_deref(), path.as_deref());
        }
    }
}
use pkit::filesystem::{config::Config, setup_shell_environment};
use pkit::cli::{Cli, Commands};
use pkit::commands::{list, install, default};

// PATH="$(pwd):$PATH"

#[tokio::main]
async fn main() {
    let cli = Cli::parse_args();

    let _ = Config::new();

    setup_shell_environment().expect("Failed to setup shell environment");

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
    }
}
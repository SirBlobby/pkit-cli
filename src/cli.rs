use clap::{Parser, Subcommand, builder::styling};
use crate::formatter::colorize;

// Custom styling for clap using your color formatter
fn get_custom_styles() -> styling::Styles {
    styling::Styles::styled()
        .header(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
        .usage(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
        .literal(styling::AnsiColor::Cyan.on_default() | styling::Effects::BOLD)
        .placeholder(styling::AnsiColor::Yellow.on_default())
        .error(styling::AnsiColor::Red.on_default() | styling::Effects::BOLD)
        .valid(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
        .invalid(styling::AnsiColor::Red.on_default() | styling::Effects::BOLD)
}

#[derive(Parser)]
#[command(name = "pkit")]
#[command(about = colorize("&aA package manager for programming languages&r"))]
#[command(version = "0.0.2")]
#[command(styles = get_custom_styles())]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List available languages and versions
    #[command(about = colorize("&aList available languages and versions&r"))]
    List {
        /// Language to list versions for
        #[arg(help = colorize("&eLanguage to list versions for&r"))]
        language: Option<String>,
        /// List only installed packages
        #[arg(long, help = colorize("&bList only installed packages&r"))]
        installed: bool,
    },
    /// Install a language and version
    #[command(about = colorize("&aInstall a language and version&r"))]
    Install {
        /// Language to install
        #[arg(help = colorize("&eLanguage to install&r"))]
        language: String,
        /// Version to install
        #[arg(help = colorize("&eVersion to install&r"))]
        version: String,
    },
    /// Set default language
    #[command(about = colorize("&aSet default language&r"))]
    Default {
        /// Language to set as default
        #[arg(help = colorize("&eLanguage to set as default&r"))]
        language: String,
        /// Version to set as default (optional)
        #[arg(help = colorize("&eVersion to set as default (optional)&r"))]
        version: Option<String>,
        /// Show the default version for the specified language
        #[arg(long, help = colorize("&bShow the default version for the specified language&r"))]
        show: bool,
    },
    /// Uninstall a language and version
    #[command(about = colorize("&aUninstall a language and version&r"))]
    Uninstall {
        /// Language to uninstall
        #[arg(help = colorize("&eLanguage to uninstall&r"))]
        language: String,
        /// Version to uninstall (optional)
        #[arg(help = colorize("&eVersion to uninstall (optional)&r"))]
        version: Option<String>,
        /// Uninstall all versions of the specified language
        #[arg(long, help = colorize("&bUninstall all versions of the specified language&r"))]
        all: bool,
    },
    /// Switch to a different language version for the current session
    #[command(about = colorize("&aSwitch to a different language version for the current session&r"))]
    Switch {
        /// Language to switch to
        #[arg(help = colorize("&eLanguage to switch to&r"))]
        language: String,
        /// Version to switch to
        #[arg(help = colorize("&eVersion to switch to&r"))]
        version: String,
    },
    /// Manage path sources for custom installations
    #[command(about = colorize("&aManage path sources for custom installations&r"))]
    Path {
        /// Action to perform (add, remove, list)
        #[arg(help = colorize("&eAction to perform (add, remove, list)&r"))]
        action: String,
        /// Name of the path source
        #[arg(help = colorize("&eName of the path source&r"))]
        name: Option<String>,
        /// Path to the source
        #[arg(help = colorize("&ePath to the source&r"))]
        path: Option<String>,
    },
}

impl Cli {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}

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
#[command(version = "0.0.1")]
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
    },
}

impl Cli {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}

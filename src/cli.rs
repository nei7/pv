use crate::commands;
use clap::{AppSettings, Parser, Subcommand};
use dialoguer::{theme::ColorfulTheme, Password};
use std::fs::File;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(name = "pv", author, version, about, long_about = None)]
#[clap(setting = AppSettings::SubcommandRequired)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]

struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(name = "init", about = "Create a vault", long_about = None)]
    Init,

    #[clap(name = "add", about = "Adding a password")]
    Add,

    #[clap(name = "list", about = "Listing stored passwords")]
    List,

    #[clap(name = "get", about = "Get specific password")]
    Get,

    #[clap(name = "delete", about = "Delete specific password")]
    Delete,
}

pub fn get_password_file_path() -> PathBuf {
    let mut file_default = PathBuf::from(dirs::home_dir().ok_or(1).unwrap());
    file_default.push(".pv");

    file_default
}

pub fn cli_match() -> i32 {
    let cli: Cli = Cli::parse();

    if matches!(cli.command, Commands::Init) {
        return match commands::init() {
            Err(i) => i,
            _ => 0,
        };
    }

    let master_password = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter master password: ")
        .interact()
        .map_err(|_| 1);
    
    if let Err(i) = master_password {
        return i;
    }

    match cli.command {
        _ => 127,
    }
}

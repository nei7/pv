use crate::{commands, pass};
use clap::{AppSettings, Args, Parser, Subcommand};
use dialoguer::{theme::ColorfulTheme, Password};
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
    Add(AddCommand),

    #[clap(name = "list", about = "Listing stored passwords")]
    List,

    #[clap(name = "get", about = "Get specific password")]
    Get(GetCommand),

    #[clap(name = "delete", about = "Delete specific password")]
    Delete,
}

#[derive(Debug, Args)]
pub struct AddCommand {
    pub name: String,
}

#[derive(Debug, Args)]
pub struct GetCommand {
    pub name: String,
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

    let password_file_path = get_password_file_path();

    let mut options = std::fs::OpenOptions::new();
    options.read(true);
    options.write(true);
    options.create(false);

    let mut file = match options.open(password_file_path) {
        Ok(f) => f,
        Err(_) => return 1,
    };

    let mut store = match pass::PasswordStore::load_store(master_password.unwrap(), &file) {
        Ok(store) => store,
        Err(err) => {
            println!("Error: {}", err);
            return 1;
        }
    };

    let res = match cli.command {
        Commands::Add(args) => commands::add_password(args, &mut store),
        Commands::Get(args) => commands::get_password(args, &mut store),
        _ => Err(127),
    };

    match res {
        Ok(_) => {
            if let Err(e) = store.save_store(&mut file) {
                println!("Failed to save data to store: {}", e)
            }

            0
        }
        Err(code) => code,
    }
}

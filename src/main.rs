mod lang_reader;
mod global;
mod utils;
mod cli {
    pub mod denis;
}
mod repository;

use std::io;
use std::io::{Read, Write};
use clap::{arg, CommandFactory, Error, Parser, Subcommand};
use crate::cli::denis::{create_denis_properties, watch_denis_service};
use crate::global::initialize_cli_language;
use crate::lang_reader::read_json_file;
use crate::repository::get_latest_version;
use crate::utils::print_element;

#[derive(Parser)]
#[command(name = "Denis CLI")]
#[command(version = "1.0.0")]
#[command(about = "DenisCLI, Manage your denisdb with cli.", long_about = None)]
#[command(disable_help_subcommand = true, disable_help_subcommand = true)]
struct Cli {
    /// Alt komutlar
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Manage or create denis.properties file.")]
    Properties {
        #[arg(short, long)]
        create: bool,
        #[arg(short, long)]
        delete: bool,
    },
    #[command(about = "Display help information for the CLI")]
    Help,
    #[command(about = "Watch ddb memory usage.")]
    Watch,
    #[command(about = "Get current denis-cli version")]
    Version,
    #[command(about = "Exit the application")]
    Exit,
}

#[tokio::main]
async fn main() {
    initialize_cli_language();
    match read_json_file() {
        Ok(language_modal) => {
            print_element(language_modal.startup_information);
            loop {
                print!("denis: ");
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();

                let args = std::iter::once("denis")
                    .chain(input.trim().split_whitespace())
                    .collect::<Vec<_>>();

                if args.iter().any(|&arg| arg == "exit" || arg == "--exit") {
                    println!("{}", language_modal.bye.to_string());
                    break;
                }

                match Cli::try_parse_from(args) {
                    Ok(cli) => {
                        if let Some(command) = cli.command {
                            match command {
                                Commands::Properties {create, delete} => {
                                    if create {
                                        if let Err(e) = create_denis_properties() {
                                            eprintln!("Error: {}", e);
                                        }
                                    } else {
                                        println!("Missing required options for denis CLI");
                                        continue;
                                    }
                                }
                                Commands::Help => {
                                    print_element(language_modal.help.clone());
                                    continue;
                                }
                                Commands::Exit => {
                                    println!("{:?}", language_modal.bye);
                                    break;
                                }
                                Commands::Version => {
                                    get_latest_version(language_modal.version.to_string()).await;
                                }
                                Commands::Watch => {
                                    watch_denis_service();
                                }
                            }
                        }
                    }
                    Err(e) => {
                        println!("Err: {}", e);
                    }
                }
            }

        }
        Err(e) => {
            print!("Err: {}", e);
        }
    }
}

mod lang_reader;
mod global;
mod utils;

use std::io;
use std::io::{Read, Write};
use clap::{arg, CommandFactory, Error, Parser, Subcommand};
use crate::global::initialize_cli_language;
use crate::lang_reader::read_json_file;
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
    #[command(about = "Display help information for the CLI")]
    Help,
    #[command(about = "Exit the application")]
    Exit,
}

fn main() {
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
                                Commands::Help => {
                                    print_element(language_modal.help.clone());
                                    continue;
                                }
                                Commands::Exit => {
                                    println!("{:?}", language_modal.bye);
                                    break;
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

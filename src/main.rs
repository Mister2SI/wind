use std::process::ExitCode;

use clap::{Parser, Subcommand};

mod client;
mod serializable_key;
mod server;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    mode: Modes,
}

#[derive(Subcommand)]
enum Modes {
    /// Connect to a remote listening socket at addr
    Client { addr: String },

    /// Open a listening socket on addr
    Server { addr: String },
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match &cli.mode {
        Modes::Client { addr } => {
            println!("Attempting to connect to remote socket at \"{addr}\"...");
            if let Err(_) = crate::client::client(&addr) {
                eprintln!("Failed to execute the client program.");
                return ExitCode::FAILURE;
            }
        }
        Modes::Server { addr } => {
            println!("Opening listening socket at \"{addr}\"...");
            if let Err(_) = crate::server::server(&addr) {
                eprintln!("Failed to execute the server program.");
                return ExitCode::FAILURE;
            }
        }
    }

    ExitCode::SUCCESS
}

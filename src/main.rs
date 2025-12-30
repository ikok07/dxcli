use std::process::exit;
use clap::Parser;
use crate::cli::Cli;
use crate::handlers::CommandHandler;

mod cli;
mod utils;
mod handlers;
mod output;

fn main() {
    let cli = Cli::parse();
    let handler = CommandHandler::new(cli.command);
    match handler.handle() {
        Ok(result) => output::print_success(&result),
        Err(err) => output::print_error(&err.to_string())
    }
}

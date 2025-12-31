use std::process::exit;
use clap::Parser;
use colored::Colorize;
use crate::cli::Cli;
use crate::handlers::CommandHandler;

mod cli;
mod utils;
mod handlers;
mod output;

fn main() {
    let cli = Cli::parse();
    let handler = CommandHandler::new(cli.command);
    let (results, is_error) = match handler.handle() {
        Ok(result) => (result, false),
        Err(err) => (err.to_string(), true)
    };

    if is_error {
        output::print_error(&results);
    } else if cli.output.is_some() {
        output::save_to_file(&results, cli.output.as_ref().unwrap());
        println!("{}", format!("Results saved successfully to {}", cli.output.unwrap()).green().bold());
    } else {
        output::print_success(&results);
    }
}

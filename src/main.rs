use clap::Parser;
use colored::Colorize;
use crate::cli::Cli;
use crate::handlers::CommandHandler;

mod cli;
mod handlers;
mod output;

fn main() {
    let cli = Cli::parse();
    let handler = CommandHandler::new(cli.command);
    match handler.handle() {
        Ok(result) => {
            if cli.output.is_some() {
                output::save_to_file(&result, cli.output.as_ref().unwrap());
                println!("{}", format!("Results saved successfully to {}", cli.output.unwrap()).green().bold());
            } else {
                output::print_success(&result);
            }
        },
        Err(err) => output::print_error(&err.to_string())
    };
}

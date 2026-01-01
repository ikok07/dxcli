use std::fs;
use std::process::exit;
use colored::Colorize;

pub fn save_to_file(content: &str, path: &str) {
    if let Err(err) = fs::write(path, content) {
        eprintln!("{}", format!("Failed to save results to file! {err}").red().bold());
        exit(1);
    }
}

pub fn print_success(content: &str) {
    println!("{}", format!("\nProgram finished successfully!\n").green());
    println!("{}", format!("------ RESULT START ------\n").green().bold());
    println!("{content}");
    println!("{}", format!("\n------ RESULT END ------").green().bold());
}

pub fn print_error(error: &str) {
    println!("{}", format!("\nProgram didn't finish successfully!\n").red());
    println!("{}", format!("------ ERROR START ------\n").red().bold());
    println!("{error}");
    println!("{}", format!("\n------ ERROR END ------").red().bold());
}
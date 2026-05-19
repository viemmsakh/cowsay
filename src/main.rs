use std::io::{self, IsTerminal, Read, Result};
use clap::{CommandFactory, Parser};

// Custom Modules
mod commands;
mod helper;
mod structs;

use structs::Args;

fn main() -> Result<()> {
    let args = Args::parse();
    let text = match args.text {
        Some(t) => t,
        None => {
            if io::stdin().is_terminal() {
                let mut cmd = Args::command();
                cmd.print_help()?;
                println!();
                std::process::exit(1);
            }
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            buffer
        }
    };
    let text_check = text.trim();
    if text_check.is_empty() {
        eprintln!("ERROR: Cannot have empty string for cow to say!");
        let mut cmd = Args::command();
        cmd.print_help()?;
        println!();
        std::process::exit(1);
    }
    commands::cowsay(text.to_string());
    Ok(())
}

use clap::{CommandFactory, Parser, Subcommand};

use crate::commands::{check, init, skill};

#[derive(Debug, Parser)]
#[command(name = "growl", about = "Grey Owl wiki validator")]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    #[command(about = "Validate a wiki")]
    Check(check::Args),
    #[command(about = "Create a starter configuration")]
    Init(init::Args),
    #[command(about = "Write the Grey Owl Agent Skill")]
    Skill(skill::Args),
}

pub fn run(args: Vec<String>) -> Result<u8, String> {
    let cli = match Cli::try_parse_from(std::iter::once("growl".to_string()).chain(args)) {
        Ok(cli) => cli,
        Err(error) => {
            let exit_code = error.exit_code();
            error.print().map_err(|error| format!("cannot print CLI error: {error}"))?;
            return Ok(exit_code as u8);
        }
    };

    match cli.command {
        Some(Command::Check(args)) => check::run(&args),
        Some(Command::Init(args)) => init::run(&args),
        Some(Command::Skill(args)) => skill::run(&args),
        None => {
            print_help();
            Ok(0)
        }
    }
}

fn print_help() {
    println!("{}", Cli::command().render_help());
}

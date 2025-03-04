use std::fs;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use miette::{IntoDiagnostic, WrapErr};

// use vibec_lexer::Lexer;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Tokenize { filepath: PathBuf },
}

fn main() -> miette::Result<()> {
    let args = Args::parse();
    match args.command {
        Command::Tokenize { filepath } => {
            let source_code = fs::read_to_string(&filepath)
                .into_diagnostic()
                .wrap_err_with(|| format!("Failed to read file {}", filepath.display()))?;

            // let lexer = Lexer::from(&source_code);
            // for token in lexer {
            // let token = token?;
            // println!("{}", token);
            // }
        }
    }
    Ok(())
}

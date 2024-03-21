use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::Context;
use clap::{Parser, Subcommand};
use semver::Version;
use slang_solidity::kinds::RuleKind;
use slang_solidity::language::Language;

#[derive(Parser)]
#[clap(name = "tasks", version, author)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Run linter
    Lint {
        /// The path to the solidity file to lint
        file_path: PathBuf,
    },
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let solidity_version = Version::new(0, 8, 0);

    match args.command {
        Command::Lint { file_path } => {
            println!("Linting file: {:?}", file_path);
            let path = file_path.to_str().context("path is utf-8")?;
            dbg!("pre execute_parse_command");
            execute_parse_command(path, solidity_version)?;
        }
    }
    Ok(())
}

fn execute_parse_command(file_path_string: &str, version: Version) -> anyhow::Result<ExitCode> {
    let file_path = PathBuf::from(&file_path_string)
        .canonicalize()
        .with_context(|| format!("Failed to find file path: {file_path_string:?}"))?;

    let input = fs::read_to_string(file_path)?;
    let language = Language::new(version)?;
    let output = language.parse(RuleKind::SourceUnit, &input);
    let cursor = output.cursor();

    let errors = output.errors();
    for error in errors {
        let report = error.to_error_report(file_path_string, &input, /* with_color */ true);
        eprintln!("{report}");
    }

    if errors.is_empty() {
        Ok(ExitCode::SUCCESS)
    } else {
        eprintln!("Couldn't parse the Solidity source file.");
        Ok(ExitCode::FAILURE)
    }
}

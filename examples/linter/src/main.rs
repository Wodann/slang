use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::{anyhow, Context};
use clap::{Parser, Subcommand};
use semver::Version;
use slang_solidity::kinds::RuleKind;
use slang_solidity::language::Language;
use slang_solidity::query::Query;

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

    let cursor = output.create_tree_cursor();

    let query = Query::parse(
        r#"@foo [MemberAccessExpression
        ...
        [Expression
            ...
            ["tx"]
            ...
        ]
        ...
        [MemberAccess
            ...
            ["origin"]
            ...
        ]
        ...
    ]"#,
    )
    .map_err(|error| anyhow!(error))?;

    let mut result = cursor.query(vec![query]);

    let mut failed = false;
    for node in result {
        failed = true;

        let cursors = node.bindings.get("foo").expect("foo must exist");
        let cursor = cursors.first().expect("At least one must exist");

        let start_position = cursor.text_offset();
        let chars = &input[0..start_position.utf8];
        let line_number = chars.lines().count();

        let char_offset = chars
            .chars()
            .rev()
            .enumerate()
            .find_map(|c| if c.1 == '\n' { Some(c.0 + 1) } else { None })
            .unwrap_or(start_position.utf8);

        let node = cursor.node();
        let rule = node.as_rule().expect("Must be a rule").clone();
        let text = rule.unparse();

        println!(r#"Linter error at {line_number}:{char_offset} "{text}""#);
    }

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

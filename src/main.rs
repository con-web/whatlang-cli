use std::io::{stdout, Write};
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Args, Parser};
use log::{debug, error};

use whatlang_cli::{process_files, process_stdin, process_string};

/// CLI application for detecting the language of a text wrapping the amazing whatlang-rs crate.
///
/// Results are printed to stdout as JSON objects as follows:
/// {
///     "Ok": {
///         "confidence": float [0,1],
///         "is_reliable": bool,
///         "language": string,
///         "script": string
///     }
/// }
///
/// Erroneous results are printed to stdout as JSON objects as follows:
/// {
///     "Error": "failed to detect language"
/// }
///
/// If --json flag is used to process input as an array of JSON strings, results are also printed as an Array:
/// [
///     {
///         "Ok": {
///            "confidence": float [0,1],
///             "is_reliable": bool,
///             "language": string,
///             "script": string
///         }
///     },
///     { ... }
/// ]
///
/// If one or multiple files are chosen as input, results are printed as JSON objects as follows:
/// [
///     {
///         "file": "path/to/file.txt,
///         "results": [
///             { "Ok": "{ ... }" }
///         ]
///     },
///     { ... }
///]
///
/// Set the desired log level with the environment variable RUST_LOG, e.g. 'export RUST_LOG=debug'.
/// All logging goes to stderr.
///
/// For further documentation see:
/// https://github.com/con-web/whatlang-cli
///
/// See also:
/// https://github.com/greyblake/whatlang-rs
///
#[derive(Parser)]
#[command(version, about, verbatim_doc_comment)]
struct Cli {
    #[command(flatten)]
    input: Input,

    /// Process input as a JSON array of strings.
    #[arg(long, short)]
    json: bool,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
struct Input {
    /// The text that you want to detect the language of
    #[arg()]
    text: Option<String>,

    /// Get input from stdin
    #[arg(long, short)]
    stdin: bool,

    /// Get input from one or multiple files
    #[arg(long, short, action)]
    file: Vec<PathBuf>,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    env_logger::init();

    let format = if cli.json { "JSON" } else { "plain text" };

    let result = if cli.input.stdin {
        debug!("Processing stdin as {}", format);
        process_stdin(cli.json)
    } else if !cli.input.file.is_empty() {
        debug!("Processing files {:?} as {}", cli.input.file, format);
        process_files(cli.input.file, cli.json)
    } else {
        // safe unwrap because if the program hits this branch, text arg must be there
        let text = cli.input.text.unwrap();
        debug!("Processing argument '{}' as {}", text, format);
        process_string(text, cli.json)
    };

    if let Err(e) = result {
        error!("{}", e);
        return ExitCode::FAILURE;
    }
    debug!("Finished processing, printing results");
    if let Err(e) = serde_json::to_writer_pretty(stdout(), &result.unwrap()) {
        error!("{}", e);
        return ExitCode::FAILURE;
    }
    stdout().write_all("\n".as_bytes()).unwrap();

    ExitCode::SUCCESS
}

#[cfg(test)]
mod test {
    use crate::Cli;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;

        Cli::command().debug_assert();
    }
}

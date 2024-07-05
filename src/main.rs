use std::io::{stdout, Write};
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Args, Parser};
use log::{debug, error};

use whatlang_cli::{process_files, process_stdin, process_string};

/// CLI application for detecting the language of a text wrapping the amazing whatlang-rs crate.
///
///#### Input Modes
///
///You can use any of the input modes `TEXT`, `--stdin` and `--file` exclusively. If you try to use multiple of them,
///the application will return an error.
///
///Any input mode can be combined with the `--json` flag to tell the application, that the input is a JSON array of strings.
///So for the combination of `--file` and `--json`, the application expects a file which contains a JSON array of strings, e.g.
///`file.json`:
///```json
///[
///  "Text 1",
///  "Text 2",
///  "Text 3"
///]
///```
///For the combination of `--stdin` and `--json`, as well as for the combination of `TEXT` and `--json`, this holds true, as
///the application expects a JSON array of strings in stdin or as argument.
///
///#### Output
///
///If the application returns with exit code 0 which means it did process the input data successfully, it will print
///the results as JSON objects into stdout. Depending on the input mode, the output will be a different JSON object.
///
///For the `TEXT` and the `--stdin` input mode, the output will be a single `Result` JSON object:
///```json
///{
///  "Ok": {
///    "confidence": "float [0,1]",
///    "is_reliable": "bool",
///    "language": "string",
///    "script": "string"
///  }
///}
///```
///
///If language detection fails, the single `Result` JSON object will contain an error message instead of the language detection result:
///```json
///{
///    "Error": "Failed to detect language"
///}
///```
///
///
///If the `--json` flag is set in addition to the `TEXT` or `--stdin` input mode, the output will be an array of `Result` JSON objects:
///```json
///[
///  {
///    "Ok": {
///      "confidence": "float [0,1]",
///      "is_reliable": "bool",
///      "language": "string",
///      "script": "string"
///    }
///  },
///  {
///    "Ok": {
///      "confidence": "float [0,1]",
///      "is_reliable": "bool",
///      "language": "string",
///      "script": "string"
///    }
///  }
///]
///```
///
///If the input mode is `--file`, the output will be an array of composed JSON objects containing the file path of the processed file
///and an array of `Result` JSON objects:
///
///```json
///[
///  {
///    "file": "string",
///    "results": [
///      {
///        "Ok": {
///          "confidence": "float [0,1]",
///          "is_reliable": "bool",
///          "language": "string",
///          "script": "string"
///        }
///      },
///      {
///        "Ok": {
///          "confidence": "float [0,1]",
///          "is_reliable": "bool",
///          "language": "string",
///          "script": "string"
///        }
///      }
///    ]
///  }
///]
///```
///
///#### Logging
///The application uses the [`env_logger`](https://github.com/rust-cli/env_logger) crate for logging. You can set the log
///level by setting the `RUST_LOG` environment variable, e.g. `export RUST_LOG=debug`. The application will allways log to stderr.
///
/// For examples see:
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
    // safe unwrap because we checked for error above
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

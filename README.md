# whatlang-cli

![build](https://img.shields.io/github/actions/workflow/status/con-web/whatlang-cli/general.yml)
![release](https://img.shields.io/github/v/release/con-web/whatlang-cli)
![license](https://img.shields.io/github/license/con-web/whatlang-cli)

This is a cli application wrapping the amazing [`whatlang-rs`](https://github.com/greyblake/whatlang-rs) crate.
It detects the language of a given text.

### Table of Contents

* [Getting Started](#getting-started)
    * [Binaries](#binaries)
    * [Build From Source](#build-from-source)
* [Usage](#usage)
    * [Input Modes](#input-modes)
    * [Output](#output)
    * [Logging](#logging)
* [Examples](#examples)
    * [Success Examples](#success-examples)
    * [Error Examples](#error-examples)

### Getting Started

#### Binaries

The latest binaries are available for download on
the [release page of this repository](https://github.com/con-web/whatlang-cli/releases).

#### Build From Source

This expects you to have a recent cargo toolchain installed on your machine.

```shell
git clone https://github.com/con-web/whatlang-cli.git
cd whatlang-cli
cargo test
cargo build --release
```

### Usage

Print the usage information:

```shell
./whatlang-cli -h
```

Result:

```
CLI application for detecting the language of a text wrapping the amazing whatlang-rs crate.

Usage: whatlang-cli [OPTIONS] <TEXT|--stdin|--file <FILE>>

Arguments:
  [TEXT]  The text that you want to detect the language of

Options:
  -s, --stdin        Get input from stdin
  -f, --file <FILE>  Get input from one or multiple files
  -j, --json         Process input as a JSON array of strings
  -h, --help         Print help (see more with '--help')
  -V, --version      Print version
```

#### Input Modes

You can use any of the input modes `TEXT`, `--stdin` and `--file` exclusively. If you try to use multiple of them,
the application will return an error.

Any input mode can be combined with the `--json` flag to tell the application, that the input is a JSON array of
strings.
So for the combination of `--file` and `--json`, the application expects a file which contains a JSON array of strings,
e.g.
`file.json`:

```json
[
  "Text 1",
  "Text 2",
  "Text 3"
]
```

For the combination of `--stdin` and `--json`, as well as for the combination of `TEXT` and `--json`, this holds true,
as
the application expects a JSON array of strings in stdin or as argument.

#### Output

If the application returns with exit code 0 which means it did process the input data successfully, it will print
the results as JSON objects into stdout. Depending on the input mode, the output will be a different JSON object.

For the `TEXT` and the `--stdin` input mode, the output will be a single `Result` JSON object:

```json
{
  "Ok": {
    "confidence": "float [0,1]",
    "is_reliable": "bool",
    "language": "string",
    "script": "string"
  }
}
```

If language detection fails, the single `Result` JSON object will contain an error message instead of the language
detection result:

```json
{
  "Error": "Failed to detect language"
}
```

If the `--json` flag is set in addition to the `TEXT` or `--stdin` input mode, the output will be an array of `Result`
JSON objects:

```json
[
  {
    "Ok": {
      "confidence": "float [0,1]",
      "is_reliable": "bool",
      "language": "string",
      "script": "string"
    }
  },
  {
    "Ok": {
      "confidence": "float [0,1]",
      "is_reliable": "bool",
      "language": "string",
      "script": "string"
    }
  }
]
```

If the input mode is `--file`, the output will be an array of composed JSON objects containing the file path of the
processed file
and an array of `Result` JSON objects:

```json
[
  {
    "file": "string",
    "results": [
      {
        "Ok": {
          "confidence": "float [0,1]",
          "is_reliable": "bool",
          "language": "string",
          "script": "string"
        }
      },
      {
        "Ok": {
          "confidence": "float [0,1]",
          "is_reliable": "bool",
          "language": "string",
          "script": "string"
        }
      }
    ]
  }
]
```

#### Logging

The application uses the [`env_logger`](https://github.com/rust-cli/env_logger) crate for logging. You can set the log
level by setting the `RUST_LOG` environment variable, e.g. `export RUST_LOG=debug`. The application will allways log to
stderr.

### Examples

#### Success Examples

```shell
./whatlang-cli "Trigrams are a special case of the n-gram, where n equals 3."
```

or

```shell
echo "Trigrams are a special case of the n-gram, where n equals 3." | ./whatlang-cli --stdin
```

Result:

```json
{
  "Ok": {
    "confidence": 1.0,
    "is_reliable": true,
    "language": "English",
    "script": "Latin"
  }
}
```

___

```shell
./whatlang-cli -f /path/to/file1 -f /path/to/file2
```

Result:

```json
[
  {
    "file": "/path/to/file1",
    "results": [
      {
        "Ok": {
          "confidence": 1.0,
          "is_reliable": true,
          "language": "German",
          "script": "Latin"
        }
      }
    ]
  },
  {
    "file": "/path/to/file2",
    "results": [
      {
        "Ok": {
          "confidence": 1.0,
          "is_reliable": true,
          "language": "English",
          "script": "Latin"
        }
      }
    ]
  }
]
```

___

```shell
./whatlang-cli --json -f /path/to/file1 -f /path/to/file2
```

Result:

```json
[
  {
    "file": "/path/to/file1",
    "results": [
      {
        "Ok": {
          "confidence": 1.0,
          "is_reliable": true,
          "language": "Turkish",
          "script": "Latin"
        }
      },
      {
        "Ok": {
          "confidence": 1.0,
          "is_reliable": true,
          "language": "Bulgarian",
          "script": "Cyrillic"
        }
      }
    ]
  },
  {
    "file": "/path/to/file2",
    "results": [
      {
        "Ok": {
          "confidence": 1.0,
          "is_reliable": true,
          "language": "Arabic",
          "script": "Arabic"
        }
      },
      {
        "Ok": {
          "confidence": 1.0,
          "is_reliable": true,
          "language": "Russian",
          "script": "Cyrillic"
        }
      }
    ]
  }
]
```

---

```shell
export RUST_LOG=debug
./whatlang-cli "Trigrams are a special case of the n-gram, where n equals 3."
```

Result stderr:

```
[[TIMESTAMP] DEBUG whatlang_cli] Processing argument 'Trigrams are a special case of the n-gram, where n equals 3.' as plain text
[[TIMESTAMP] DEBUG whatlang_cli] Finished processing, printing results
```

Result stdout:

```json

{
  "Ok": {
    "confidence": 1.0,
    "is_reliable": true,
    "language": "English",
    "script": "Latin"
  }
}
```

#### Error Examples

```shell
./whatlang-cli "123456789"
```

Result:

```json
{
  "Error": "Failed to detect language"
}
```

---
Feeding the application with invalid utf-8 data will cause the application to return with exit code 1 and log an error:

```shell
cat whatlang-cli | ./whatlang-cli --stdin
```

Result:

```
[[TIMESTAMP] ERROR whatlang_cli] invalid utf-8 sequence of 1 bytes from index
```

---
When batch processing multiple files, the application will continue processing the remaining files even if one file
causes
an error. The error will be logged and the application will return with exit code 0:

```shell
./whatlang-cli -f /i/do/not/exist -f /path/to/file
```

Result stderr:

```
[[TIMESTAMP] ERROR whatlang_cli] Invalid file "/i/do/not/exist": No such file or directory (os error 2). Skipping file
```

Result stdout:

```json

[
  {
    "file": "/path/to/file",
    "results": [
      {
        "Ok": {
          "confidence": 1.0,
          "is_reliable": true,
          "language": "English",
          "script": "Latin"
        }
      }
    ]
  }
]
```

---
If no file is processed successfully, the application will log an error and return with exit code 1:

```shell
./whatlang-cli -f /i/do/not/exist -f /i/dont/contain/valid/utf8
```

Result stderr:

```
[[TIMESTAMP] ERROR whatlang_cli] Invalid file "/i/do/not/exist": No such file or directory (os error 2). Skipping file
[[TIMESTAMP] ERROR whatlang_cli] Invalid file "/i/dont/contain/valid/utf8": invalid utf-8 sequence of 1 bytes from index 0. Skipping file
[[TIMESTAMP] ERROR whatlang_cli] Didn't process any file due to errors
```

# whatlang-cli

![build](https://img.shields.io/github/actions/workflow/status/con-web/whatlang-cli/general.yml)
![release](https://img.shields.io/github/v/release/con-web/whatlang-cli)
![license](https://img.shields.io/github/license/con-web/whatlang-cli)


This is a cli application wrapping the amazing [`whatlang-rs`](https://github.com/greyblake/whatlang-rs) crate. 
It detects the language of a given text.

### Getting Started

#### Binaries

The latest binaries are available for download on the [release page of this repository](https://github.com/con-web/whatlang-cli/releases).

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
./whatlang-cli --help
```
Result:
```
CLI application for detecting the language of a text wrapping the amazing whatlang-rs crate.

Results are printed to stdout as JSON objects as follows:
{
    "Ok": {
        "confidence": float [0,1],
        "is_reliable": bool,
        "language": string,
        "script": string
    }
}

Erroneous results are printed to stdout as JSON objects as follows:
{
    "Error": "failed to detect language"
}

If --json flag is used to process input as an array of JSON strings, results are also printed as an Array:
[
    {
        "Ok": {
           "confidence": float [0,1],
            "is_reliable": bool,
            "language": string,
            "script": string
        }
    },
    { ... }
]

If one or multiple files are chosen as input, results are printed as JSON objects as follows:
[
    {
        "file": "path/to/file.txt,
        "results": [
            { "Ok": "{ ... }" }
        ]
    },
    { ... }
]

Set the desired log level with the environment variable RUST_LOG, e.g. 'export RUST_LOG=debug'.
All logging goes to stderr.

For examples see:
https://github.com/con-web/whatlang-cli

See also:
https://github.com/greyblake/whatlang-rs

Usage: whatlang-cli [OPTIONS] <TEXT|--stdin|--file <FILE>>

Arguments:
  [TEXT]
          The text that you want to detect the language of

Options:
  -s, --stdin
          Get input from stdin

  -f, --file <FILE>
          Get input from one or multiple files

  -j, --json
          Process input as a JSON array of strings

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
You can use any of the input modes `TEXT`, `--stdin` and `--file` exclusively. If you try to use multiple of them,
the application will return an error. Any input mode can be combined with the `--json` flag to process input as a JSON
array of strings.


### Examples

#### Success Examples 

The application prints results as JSON objects into stdout:

```shell
./whatlang-cli "Trigrams are a special case of the n-gram, where n equals 3. They are often used in nlp"
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


Read input from stdin:


```shell
echo "Trigrams are a special case of the n-gram, where n equals 3." | ./whatlang-cli --stdin
```


Read input from one or multiple files:

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

This output is a little bit more verbose. It contains the file path and the results for each file. If combined with the
`--json` flag to process input as a JSON array of strings, this makes more sense since any file can contain multiple texts:

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

#### Error Examples


If language detection fails, the application prints an error as JSON object into stdout:

```shell
./whatlang-cli "123456789"
```
Result:
```json
{
    "Error": "Failed to detect language"
}
```

Any error occurring during processing the input (e.g. invalid utf-8 strings) will cause the application to return with
exit code 1 and logging an error (this example also shows getting input from stdin):
```shell
cat whatlang-cli | ./whatlang-cli --stdin
```

Result:
```
[[TIMESTAMP] ERROR whatlang_cli] invalid utf-8 sequence of 1 bytes from index
```

When batch processing multiple files, the application will continue processing the remaining files even if one file causes
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

### Logging

The application uses the [`env_logger`](https://github.com/rust-cli/env_logger) crate for logging. You can set the desired log level with the environment variable RUST_LOG, e.g. 'export RUST_LOG=debug'. 
All logging goes to stderr.

```shell
export RUST_LOG=debug
./whatlang-cli "Trigrams are a special case of the n-gram, where n equals 3."
```

Result stderr: 
```
[[TIMESTAMP] DEBUG whatlang_cli] Processing argument 'Trigrams are a special case of the n-gram, where n equals 3.' as plain text
[[TIMESTAMP] DEBUG whatlang_cli] Finished processing, printing results
```


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
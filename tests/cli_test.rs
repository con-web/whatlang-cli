use rand::random;
use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn cli_with_stdin_works() {
    let mut cmd = Command::new("target/debug/whatlang-cli")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .arg("--stdin")
        .spawn()
        .unwrap();
    {
        let stdin = cmd.stdin.as_mut().expect("failed to open stdin");
        stdin
            .write_all(SENTENCE.as_bytes())
            .expect("failed to write to stdin");
    }

    let output = cmd.wait_with_output().unwrap();
    assert!(output.status.success());
    let output_str = String::from_utf8(output.stdout).expect("Output is not valid UTF-8");
    assert_eq!(output_str.trim(), SENTENCE_EXPECTED.trim());
}

#[test]
fn cli_with_stdin_returns_err_for_invalid_utf8_input() {
    let mut cmd = Command::new("target/debug/whatlang-cli")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .arg("--stdin")
        .spawn()
        .unwrap();
    {
        let stdin = cmd.stdin.as_mut().expect("failed to open stdin");
        let random_bytes: [u8; 32] = random();
        stdin
            .write_all(&random_bytes)
            .expect("failed to write to stdin");
    }

    let output = cmd.wait_with_output().unwrap();
    assert!(!output.status.success());
}

#[test]
fn cli_with_stdin_json_works() {
    let mut cmd = Command::new("target/debug/whatlang-cli")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .arg("--stdin")
        .arg("--json")
        .spawn()
        .unwrap();
    {
        let stdin = cmd.stdin.as_mut().expect("failed to open stdin");
        stdin
            .write_all(JSON.as_bytes())
            .expect("failed to write to stdin");
    }

    let output = cmd.wait_with_output().unwrap();
    assert!(output.status.success());
    let output_str = String::from_utf8(output.stdout).expect("Output is not valid UTF-8");
    assert_eq!(output_str.trim(), JSON_EXPECTED.trim());
}

#[test]
fn cli_with_stdin_returns_err_for_invalid_json_works() {
    let mut cmd = Command::new("target/debug/whatlang-cli")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .arg("--stdin")
        .arg("--json")
        .spawn()
        .unwrap();
    {
        let stdin = cmd.stdin.as_mut().expect("failed to open stdin");
        stdin
            .write_all(SENTENCE.as_bytes())
            .expect("failed to write to stdin");
    }

    let output = cmd.wait_with_output().unwrap();
    assert!(!output.status.success());
}

#[test]
fn cli_with_arg_works() {
    let cmd = Command::new("target/debug/whatlang-cli")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .arg(SENTENCE)
        .spawn()
        .unwrap();

    let output = cmd.wait_with_output().unwrap();
    assert!(output.status.success());
    let output_str = String::from_utf8(output.stdout).expect("Output is not valid UTF-8");
    assert_eq!(output_str.trim(), SENTENCE_EXPECTED.trim());
}

#[test]
fn cli_with_arg_json_works() {
    let cmd = Command::new("target/debug/whatlang-cli")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .arg("--json")
        .arg(SENTENCE_JSON.trim())
        .spawn()
        .unwrap();

    let output = cmd.wait_with_output().unwrap();
    assert!(output.status.success());
    let output_str = String::from_utf8(output.stdout).expect("Output is not valid UTF-8");
    assert_eq!(output_str.trim(), SENTENCE_JSON_EXPECTED);
}

#[test]
fn cli_with_files_works() {
    let cmd = Command::new("target/debug/whatlang-cli")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .arg("-f")
        .arg("tests/text.txt")
        .spawn()
        .unwrap();

    let output = cmd.wait_with_output().unwrap();
    assert!(output.status.success());
    let output_str = String::from_utf8(output.stdout).expect("Output is not valid UTF-8");
    assert_eq!(output_str.trim(), SENTENCE_FILE_EXPECTED.trim());
}

#[test]
fn cli_with_files_returns_err_if_the_every_given_file_throws_an_error() {
    let cmd = Command::new("target/debug/whatlang-cli")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .arg("-f")
        .arg("i/do/not/exist.txt")
        .arg("-f")
        .arg("i/do/not/exist/either.txt")
        .spawn()
        .unwrap();

    let output = cmd.wait_with_output().unwrap();
    assert!(!output.status.success());
}

#[test]
fn cli_with_files_skips_file_if_file_throws_an_error_but_successfully_processes_other_files() {
    let cmd = Command::new("target/debug/whatlang-cli")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .arg("-f")
        .arg("i/donot/exist.txt")
        .arg("-f")
        .arg("tests/text.txt")
        .spawn()
        .unwrap();

    let output = cmd.wait_with_output().unwrap();
    assert!(output.status.success());
    let output_str = String::from_utf8(output.stdout).expect("Output is not valid UTF-8");
    assert_eq!(output_str.trim(), SENTENCE_FILE_EXPECTED.trim());
}

#[test]
fn cli_with_json_files_works() {
    let cmd = Command::new("target/debug/whatlang-cli")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .arg("--json")
        .arg("-f")
        .arg("tests/texts.json")
        .spawn()
        .unwrap();

    let output = cmd.wait_with_output().unwrap();
    assert!(output.status.success());
    let output_str = String::from_utf8(output.stdout).expect("Output is not valid UTF-8");
    assert_eq!(output_str.trim(), JSON_FILE_EXPECTED.trim());
}

const SENTENCE: &str = "Trigramme sind ein Spezialfall des n-Gramms, wobei n gleich 3 ist. Sie werden häufig in der Verarbeitung natürlicher Sprache zur statistischen Analyse von Texten und in der Kryptographie zur Kontrolle und Verwendung von Chiffren und Codes verwendet.";

const SENTENCE_EXPECTED: &str = r#"{
  "Ok": {
    "confidence": 1.0,
    "is_reliable": true,
    "language": "German",
    "script": "Latin"
  }
}"#;

const SENTENCE_JSON: &str = "[\"Trigramme sind ein Spezialfall des n-Gramms, wobei n gleich 3 ist. Sie werden häufig in der Verarbeitung natürlicher Sprache zur statistischen Analyse von Texten und in der Kryptographie zur Kontrolle und Verwendung von Chiffren und Codes verwendet.\"]";

const SENTENCE_JSON_EXPECTED: &str = r#"[
  {
    "Ok": {
      "confidence": 1.0,
      "is_reliable": true,
      "language": "German",
      "script": "Latin"
    }
  }
]"#;

const SENTENCE_FILE_EXPECTED: &str = r#"[
  {
    "file": "tests/text.txt",
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
  }
]"#;

const JSON: &str = r#"[
  "Trigramme sind ein Spezialfall des n-Gramms, wobei n gleich 3 ist. Sie werden häufig in der Verarbeitung natürlicher Sprache zur statistischen Analyse von Texten und in der Kryptographie zur Kontrolle und Verwendung von Chiffren und Codes verwendet.",
  "التريجرامات هي حالة خاصة من النغرام، حيث يساوي n 3. وغالبًا ما تُستخدم في معالجة اللغة الطبيعية لتحليل النصوص إحصائيًا وفي التشفير للتحكم في الشفرات والرموز واستخدامها.",
  "Триграммы - это частный случай n-грамм, где n равно 3. Они часто используются в обработке естественного языка для статистического анализа текстов и в криптографии для управления и использования шифров и кодов.",
  "Trigramlar, n'nin 3'e eşit olduğu n-gram'ın özel bir durumudur. Genellikle metinleri istatistiksel olarak analiz etmek için doğal dil işlemede ve şifreleri ve kodları kontrol etmek ve kullanmak için kriptografide kullanılırlar.",
  "Триграмите са специален случай на n-грамата, където n е равно на 3. Те често се използват в обработката на естествен език за статистически анализ на текстове и в криптографията за контрол и използване на шифри и кодове.",
  "Trigrams are a special case of the n-gram, where n equals 3. They are often used in natural language processing for the statistical analysis of texts and in cryptography for the control and use of ciphers and codes.",
  "三段论是 n-gram 的一种特例，其中 n 等于 3。在自然语言处理中，它们常用于对文本进行统计分析；在密码学中，它们常用于控制和使用密码和编码",
  "Trigrammi sono un caso speciale di n-gramma, dove n è uguale a 3. Sono spesso utilizzati nell'elaborazione del linguaggio naturale per l'analisi statistica dei testi e in crittografia per il controllo e l'uso di cifrari e codici.",
  "Trigramy są specjalnym przypadkiem n-gramu, gdzie n wynosi 3. Są one często używane w przetwarzaniu języka naturalnego do statystycznej analizy tekstów i w kryptografii do kontroli i użycia szyfrów i kodów.",
  "Trigramy jsou speciálním případem n-gramu, kde n je rovno 3. Jsou často používány v zpracování přirozeného jazyka pro statistickou analýzu textů a v kryptografii pro kontrolu a použití šifer a kódů.",
  "Trigramy jsou speciálnym prípadom n-gramu, kde n je rovné 3. Sú často používané v spracovaní prirodzeného jazyka na štatistickú analýzu textov a v kryptografii na kontrolu a použitie šifier a kódov."
]"#;

const JSON_EXPECTED: &str = r#"[
  {
    "Ok": {
      "confidence": 1.0,
      "is_reliable": true,
      "language": "German",
      "script": "Latin"
    }
  },
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
  },
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
  },
  {
    "Ok": {
      "confidence": 1.0,
      "is_reliable": true,
      "language": "English",
      "script": "Latin"
    }
  },
  {
    "Ok": {
      "confidence": 1.0,
      "is_reliable": true,
      "language": "Mandarin",
      "script": "Mandarin"
    }
  },
  {
    "Ok": {
      "confidence": 1.0,
      "is_reliable": true,
      "language": "Italian",
      "script": "Latin"
    }
  },
  {
    "Ok": {
      "confidence": 1.0,
      "is_reliable": true,
      "language": "Polish",
      "script": "Latin"
    }
  },
  {
    "Ok": {
      "confidence": 1.0,
      "is_reliable": true,
      "language": "Czech",
      "script": "Latin"
    }
  },
  {
    "Ok": {
      "confidence": 1.0,
      "is_reliable": true,
      "language": "Slovak",
      "script": "Latin"
    }
  }
]"#;

const JSON_FILE_EXPECTED: &str = r#"[
  {
    "file": "tests/texts.json",
    "results": [
      {
        "Ok": {
          "confidence": 1.0,
          "is_reliable": true,
          "language": "German",
          "script": "Latin"
        }
      },
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
      },
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
      },
      {
        "Ok": {
          "confidence": 1.0,
          "is_reliable": true,
          "language": "English",
          "script": "Latin"
        }
      },
      {
        "Ok": {
          "confidence": 1.0,
          "is_reliable": true,
          "language": "Mandarin",
          "script": "Mandarin"
        }
      },
      {
        "Ok": {
          "confidence": 1.0,
          "is_reliable": true,
          "language": "Italian",
          "script": "Latin"
        }
      },
      {
        "Ok": {
          "confidence": 1.0,
          "is_reliable": true,
          "language": "Polish",
          "script": "Latin"
        }
      },
      {
        "Ok": {
          "confidence": 1.0,
          "is_reliable": true,
          "language": "Czech",
          "script": "Latin"
        }
      },
      {
        "Ok": {
          "confidence": 1.0,
          "is_reliable": true,
          "language": "Slovak",
          "script": "Latin"
        }
      }
    ]
  }
]"#;

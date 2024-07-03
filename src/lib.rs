use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use log::error;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct LangInfo {
    language: String,
    script: String,
    confidence: f64,
    is_reliable: bool,
}

impl LangInfo {
    pub fn from_info(info: whatlang::Info) -> LangInfo {
        LangInfo {
            language: info.lang().eng_name().to_string(),
            script: info.script().to_string(),
            confidence: info.confidence(),
            is_reliable: info.is_reliable(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WhatLangResult {
    Ok(LangInfo),
    Error(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WhatLangFromFileResult {
    file: PathBuf,
    results: Vec<WhatLangResult>,
}

pub fn process_string(arg: String, json: bool) -> Result<Value, Box<dyn Error>> {
    match json {
        true => {
            let texts = validate_json(&arg)?;
            let result = detect_many(texts);
            Ok(json!(result))
        }
        false => {
            let result = detect(&arg);
            Ok(json!(result))
        }
    }
}

pub fn process_stdin(json: bool) -> Result<Value, Box<dyn Error>> {
    let text = validate_stdin_input()?;
    process_string(text, json)
}

pub fn process_files(files: Vec<PathBuf>, json: bool) -> Result<Value, Box<dyn Error>> {
    let mut result: Vec<WhatLangFromFileResult> = vec![];
    for file in files {
        let text: String = match validate_file_input(&file) {
            Ok(t) => t,
            Err(e) => {
                error!("Invalid file {:?}: {}. Skipping file", file, e);
                continue;
            }
        };

        match json {
            true => {
                let texts = match validate_json(&text) {
                    Ok(t) => t,
                    Err(e) => {
                        error!("Invalid json in file {:?}: {}. Skipping file", file, e);
                        continue;
                    }
                };
                let results = detect_many(texts);
                result.push(WhatLangFromFileResult { file, results })
            }
            false => {
                let results = detect(&text);
                result.push(WhatLangFromFileResult {
                    file,
                    results: vec![results],
                })
            }
        }
    }
    if result.is_empty() {
        return Err("Didn't process any file due to errors".into());
    }
    Ok(json!(result))
}

fn validate_stdin_input() -> Result<String, Box<dyn Error>> {
    let mut buffer = Vec::new();
    std::io::stdin().read_to_end(&mut buffer)?;
    let string = String::from_utf8(buffer)?;
    Ok(string)
}

fn validate_file_input(path: &PathBuf) -> Result<String, Box<dyn Error>> {
    let mut buffer = Vec::new();
    File::open(path)?.read_to_end(&mut buffer)?;
    let string = String::from_utf8(buffer)?;
    Ok(string)
}

fn validate_json(string: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let vec: Vec<String> = serde_json::from_str(string)?;
    Ok(vec)
}

fn detect(text: &str) -> WhatLangResult {
    if let Some(info) = whatlang::detect(text) {
        WhatLangResult::Ok(LangInfo::from_info(info))
    } else {
        WhatLangResult::Error("Failed to detect language".to_string())
    }
}

fn detect_many(texts: Vec<String>) -> Vec<WhatLangResult> {
    texts.iter().map(|text| detect(text)).collect()
}

use std::collections::HashMap;
use std::error::Error;
use crate::hosts::errors::{FileSizeLimitError, SectionKeyMissingError, SectionValueEmptyError};
use crate::hosts::sizes::Size;

pub fn check_file_size_limit(size: usize, limit: Size) -> Result<(), FileSizeLimitError> {
    if limit.is_exceeded_by(size) {
        Err(FileSizeLimitError)
    } else {
        Ok(())
    }
}

pub fn get_section_or_empty(cfg: &HashMap<String, HashMap<String, String>>, key: &str) -> HashMap<String, String> {
    let empty: HashMap<String, String> = HashMap::new();
    let section = cfg.get(key).unwrap_or(&empty);
    section.to_owned()
}

pub fn get_session_val(s: &HashMap<String, HashMap<String, String>>, host: &str, key: &str) -> String {
    let empty = "".to_string();
    let value = s.get(host)
        .and_then(|inner_map| inner_map.get(key))
        .unwrap_or(&empty);
    value.to_string()
}

pub fn set_session_val(s: &mut HashMap<String, HashMap<String, String>>, host: &str, key: &str, value: &str) {
    let inner_map = s.entry(host.to_string()).or_insert_with(HashMap::new);
    inner_map.insert(key.to_string(), value.to_string());
}

pub fn get_config_val(map: &HashMap<String, String>, key: &str, required: bool) -> Result<String, Box<dyn Error>> {
    if required {
        let value = map.get(key).ok_or_else(|| SectionKeyMissingError::new(key))?;

        if value.is_empty() {
            Err(Box::new(SectionValueEmptyError::new(key)))
        } else {
            Ok(value.to_string())
        }
    } else {
        match map.get(key) {
            Some(value) if !value.is_empty() => Ok(value.to_string()),
            Some(_) => Ok(String::new()),
            None => Ok(String::new()),
        }
    }
}
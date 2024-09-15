use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;
use regex::Regex;
use reqwest::header::{HeaderMap, HeaderValue, REFERER};
use crate::client::UploaderClient;
use crate::hosts::pixeldrain::structs::UploadResp;
use crate::hosts::sizes::Size;
use crate::hosts::utils;

const BASE_URL: &str = "https://pixeldrain.com/";
const HOST: &str = "pixeldrain";

// User info endpoint please :p.
fn get_size_limit(c: &mut UploaderClient, cookies: &HashMap<String, String>) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("{}user", BASE_URL);

    let resp = c.do_get(&url, None, Some(cookies))?;
    let html = resp.text()?;

    let re = Regex::new(r#""file_size_limit":(\d+)"#)?;

    if let Some(capture) = re.captures(&html) {
        if let Some(m) = capture.get(1) {
            let size_limit = m.as_str().to_string();
            return Ok(size_limit)
        }
    }

    Err("failed to extract file size limit from html".into())
}

fn upload(c: &mut UploaderClient, file_path: &PathBuf, cookies: &HashMap<String, String>) -> Result<String, Box<dyn Error>> {
    let url = format!("{}api/file", BASE_URL);

    let mut headers = HeaderMap::new();
    headers.insert(REFERER, HeaderValue::from_static(BASE_URL));

    let resp = c.multipart_upload(&url, "file".to_string(), file_path, Some(headers), None, Some(cookies))?;
    let json_obj: UploadResp = resp.json()?;

    let url = format!("{}u/{}", BASE_URL, json_obj.id);
    Ok(url)
}

pub fn run(c: &mut UploaderClient, file_path: &PathBuf, file_size: usize, cfg: &HashMap<String, HashMap<String, String>>, s: &mut HashMap<String, HashMap<String, String>>) -> Result<String, Box<dyn Error>> {
    let mut api_key = utils::get_session_val(s, HOST, "api_key");
    let mut size_limit = utils::get_session_val(s, HOST, "size_limit");

    let mut cookies: HashMap<String, String> = HashMap::new();

    if api_key.is_empty() {
        let pd_config = utils::get_section_or_empty(cfg, HOST);
        api_key = utils::get_config_val(&pd_config, "api_key", true)?;

        cookies.insert("pd_auth_key".to_string(), api_key);
        size_limit = get_size_limit(c, &cookies)?;
        utils::set_session_val(s, HOST, "size_limit", &size_limit);
    }

    let size_limit_obj = Size::custom_from_str(&size_limit)?;
    utils::check_file_size_limit(file_size, size_limit_obj)?;
    let file_url = upload(c, file_path, &cookies)?;
    Ok(file_url)
}
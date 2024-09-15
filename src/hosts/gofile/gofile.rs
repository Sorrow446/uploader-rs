use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue, REFERER};
use crate::client::UploaderClient;
use crate::hosts::errors::{BadAPIResponseError};
use crate::hosts::gofile::structs::*;
use crate::hosts::utils;

const API_BASE: &str = "https://api.gofile.io/";
const BASE_URL: &str = "https://gofile.io/";
const HOST: &str = "gofile";

fn get_account_meta(c: &mut UploaderClient) -> Result<AccountResp, Box<dyn Error>> {
    let mut headers = HeaderMap::new();
    headers.insert(REFERER, HeaderValue::from_static(BASE_URL));

    let url = format!("{}accounts", API_BASE);
    let resp = c.do_post::<()>(&url, Some(headers), None)?;
    let json_obj: AccountResp = resp.json()?;
    if json_obj.status != "ok" {
        return Err(BadAPIResponseError.into())
    }
    Ok(json_obj)
}
fn get_server(c: &mut UploaderClient) -> Result<String, Box<dyn Error>> {
    let mut headers = HeaderMap::new();
    headers.insert(REFERER, HeaderValue::from_static(BASE_URL));

    let resp = c.do_get(&format!("{}servers", API_BASE), Some(headers), None)?;

    let json_obj: GetServersResp = resp.json()?;
    if json_obj.status != "ok" {
        return Err(BadAPIResponseError.into())
    }

    Ok(json_obj.data.servers[0].name.clone())
}

fn create_folder(c: &mut UploaderClient, token: &str, root_folder_id: String) -> Result<String, Box<dyn Error>> {
    let create_folder_data = CreateFolderPost{
        parent_folder_id: root_folder_id,
    };

    let mut headers = HeaderMap::new();
    headers.insert(REFERER, HeaderValue::from_static(BASE_URL));
    headers.insert(AUTHORIZATION, HeaderValue::from_str(token)?);

    let url = format!("{}contents/createfolder", API_BASE);
    let resp = c.do_post(&url, Some(headers), Some(&create_folder_data))?;
    let json_obj: CreateFolderResp = resp.json()?;
    if json_obj.status != "ok" {
        return Err(BadAPIResponseError.into())
    }

    let folder_id = json_obj.data.id;
    set_folder_attributes(c, &folder_id, &token)?;

    Ok(folder_id)
}

fn set_folder_attributes(c: &mut UploaderClient, folder_id: &str, token: &str) -> Result<(), Box<dyn Error>> {
    let folder_attrib_data = FolderAttributesPut{
        attribute: "public".to_string(),
        attribute_value: "true".to_string(),
    };

    let mut headers = HeaderMap::new();
    headers.insert(REFERER, HeaderValue::from_static(BASE_URL));
    headers.insert(AUTHORIZATION, HeaderValue::from_str(token)?);

    let url = format!("{}contents/{}/update", API_BASE, folder_id);
    let resp = c.do_json_put(&url, Some(headers), &folder_attrib_data)?;
    let json_obj: CreateFolderResp = resp.json()?;
    if json_obj.status != "ok" {
        return Err(BadAPIResponseError.into())
    }

    Ok(())
}

fn upload(c: &mut UploaderClient, server: &str, token: &str, folder_id: String, file_path: &PathBuf) -> Result<String, Box<dyn Error>> {
    let url = format!("https://{}.gofile.io/contents/uploadFile", server);

    let mut headers = HeaderMap::new();
    headers.insert(REFERER, HeaderValue::from_static(BASE_URL));
    headers.insert(AUTHORIZATION, HeaderValue::from_str(token)?);

    let fields: Vec<(String, String)>= vec![
        ("folderId".to_string(), folder_id),
    ];

    let resp = c.multipart_upload(&url, "file".to_string(), file_path, Some(headers), Some(fields), None)?;
    let json_obj: UploadResp = resp.json()?;
    if json_obj.status != "ok" {
        return Err(BadAPIResponseError.into())
    }

    Ok(json_obj.data.download_page)
}

// fn get_value(m: HashMap<String, String>, key: &str) -> Result<String, Box<dyn Error>> {
//     let value = match m.get(key) {
//         Some(val) => val,
//         None => {
//             return Err("x".into());
//         }
//     };
//     if value.is_empty() {
//         return Err("y".into());
//     }
//     Ok(value.to_string())
// }

pub fn run(c: &mut UploaderClient, file_path: &PathBuf, _file_size: usize, cfg: &HashMap<String, HashMap<String, String>>, s: &mut HashMap<String, HashMap<String, String>>) -> Result<String, Box<dyn Error>> {
    let mut token = utils::get_session_val(s, HOST,  "token");
    let mut root_folder = utils::get_session_val(s, HOST,  "root_folder");

    let gofile_cfg = utils::get_section_or_empty(cfg, HOST);
    let aio_upload = utils::get_config_val(&gofile_cfg, "aio_upload", false)?;

    if token.is_empty() {
        let account_meta = get_account_meta(c)?;
        token = format!("Bearer {}", account_meta.data.token);
        root_folder = account_meta.data.root_folder;
        utils::set_session_val(s, HOST, "token", &token);
        utils::set_session_val(s, HOST, "root_folder", &root_folder)
    }

    let server = get_server(c)?;
    let mut folder_id = utils::get_session_val(s, HOST, "folder_id");

    if folder_id.is_empty() {
        folder_id = create_folder(c, &token, root_folder.clone())?;
        utils::set_session_val(s, HOST, "folder_id", &folder_id);
    }

    if aio_upload != "y" {
        folder_id = create_folder(c, &token, root_folder)?;
    }

    let file_url = upload(c, &server, &token, folder_id, file_path)?;

    Ok(file_url)
}
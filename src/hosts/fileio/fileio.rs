use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;
use crate::client::UploaderClient;
use crate::hosts::fileio::structs::UploadResp;
use crate::hosts::sizes::Size;
use crate::hosts::utils::check_file_size_limit;

const UPLOAD_URL: &str = "https://file.io";
pub fn run(c: &mut UploaderClient, file_path: &PathBuf, file_size: usize, _cfg: &HashMap<String, HashMap<String, String>>, _s: &mut HashMap<String, HashMap<String, String>>) -> Result<String, Box<dyn Error>>{
    check_file_size_limit(file_size, Size::GB2)?;

    let resp = c.multipart_upload(UPLOAD_URL, "file".to_string(), file_path, None, None, None)?;
    let json_obj: UploadResp = resp.json()?;
    if !json_obj.success {
        return Err("api reported failure".into())
    }

    Ok(json_obj.link)
}

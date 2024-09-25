use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;
use std::error::Error;
use mime_guess::MimeGuess;
use reqwest::blocking::{Client, multipart, Response as ReqwestResp};
use reqwest::Error as ReqwestErr;
use reqwest::header::{CONTENT_TYPE, COOKIE, HeaderMap, HeaderValue, USER_AGENT};
use serde::Serialize;
use crate::utils;
use crate::client::structs::ProgressReader;

const CLIENT_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36";
pub struct UploaderClient {
    c: Client,
}

impl UploaderClient {
    pub fn new() -> Result<UploaderClient, ReqwestErr> {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static(CLIENT_USER_AGENT));

        let c = Client::builder()
            .default_headers(headers)
            .timeout(None)
            .build()?;

        let uploader_client = UploaderClient {
            c,
        };

        Ok(uploader_client)
    }

    pub fn do_get(&mut self, url: &str, headers: Option<HeaderMap>, cookies: Option<&HashMap<String, String>>) -> Result<ReqwestResp, Box<dyn Error>> {
        let mut req = self.c.get(url);

        if let Some(h) = headers {
            req = req.headers(h);
        }

        if let Some(cookies) = cookies {
            let cookie_header = cookies
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<String>>()
                .join("; ");
            req = req.header(COOKIE, cookie_header);
        }

        let resp = req.send()?;
        resp.error_for_status_ref()?;

        Ok(resp)
    }

    pub fn do_json_put<T: Serialize>(&mut self, url: &str, headers: Option<HeaderMap>, body: &T) -> Result<ReqwestResp, Box<dyn Error>> {
        let b = serde_json::to_string(body)?;

        let mut req = self.c.put(url)
            .header(CONTENT_TYPE, "application/json; charset=utf-8")
            .body(b);

        if let Some(h) = headers {
            req = req.headers(h);
        }

        let resp = req.send()?;
        resp.error_for_status_ref()?;

        Ok(resp)
    }

    pub fn do_post<T: Serialize>(&self, url: &str, headers: Option<HeaderMap>, body: Option<&T>) -> Result<ReqwestResp, Box<dyn Error>> {
        let mut req = self.c.post(url);

        if let Some(h) = headers {
            req = req.headers(h);
        }

        if let Some(b) = body {
            let body = serde_json::to_string(b)?;
            req = req.body(body)
                .header(CONTENT_TYPE, "application/json; charset=utf-8")
        } else {
            req = req.header(CONTENT_TYPE, "text/plain;charset=UTF-8");
        }

        let resp = req.send()?;

        resp.error_for_status_ref()?;
        Ok(resp)
    }

    pub fn multipart_upload(
        &mut self,
        url: &str,
        field_name: String,
        file_path: &PathBuf,
        headers: Option<HeaderMap>,
        fields: Option<Vec<(String, String)>>,
        cookies: Option<&HashMap<String, String>>
    ) -> Result<ReqwestResp, Box<dyn Error>> {
        let f = File::open(file_path)?;
        let file_size = f.metadata()?.len();

        let pr = ProgressReader::new(f, file_size)?;
        let fname = utils::get_fname_string_from_path(file_path)?;
        let file_mime = MimeGuess::from_path(fname.as_str())
            .first_or_octet_stream()
            .to_string();

        let mut form = multipart::Form::new()
            .part(field_name, multipart::Part::reader_with_length(pr, file_size)
            .mime_str(&file_mime)?
            .file_name(fname));

        if let Some(fields) = fields {
            for (name, value) in fields {
                form = form.text(name, value);
            }
        }

        let mut req = self.c.post(url)
            .multipart(form);

        if let Some(h) = headers {
            req = req.headers(h);
        }

        if let Some(cookies) = cookies {
            let cookie_header = cookies
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<String>>()
                .join("; ");
            req = req.header(COOKIE, cookie_header);
        }

        let resp = req.send()?;
        resp.error_for_status_ref()?;

        Ok(resp)
    }
}

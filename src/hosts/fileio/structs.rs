use serde::Deserialize;

#[derive(Deserialize)]
pub struct UploadResp {
    pub link: String,
    pub success: bool,
}
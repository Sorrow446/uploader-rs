use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct UploadResp {
    pub id: String,
}

#[derive(Serialize)]
pub struct CreateFolderPost {
    pub id: String,
}
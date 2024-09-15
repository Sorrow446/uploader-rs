use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Server {
    pub name: String,
}
#[derive(Deserialize)]
pub struct GetServersData {
    pub servers: Vec<Server>,
}

#[derive(Deserialize)]
pub struct GetServersResp {
    pub status: String,
    pub data: GetServersData,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateFolderPost {
    pub parent_folder_id: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FolderAttributesPut {
    pub attribute: String,
    pub attribute_value: String,
}

#[derive(Deserialize)]
pub struct CreateFolderData {
    pub id: String,
}

#[derive(Deserialize)]
pub struct CreateFolderResp {
    pub status: String,
    pub data: CreateFolderData,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountData {
    pub token: String,
    pub root_folder: String,
}
#[derive(Deserialize)]
pub struct AccountResp {
    pub status: String,
    pub data: AccountData,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadData {
    pub download_page: String,
}
#[derive(Deserialize)]
pub struct UploadResp {
    pub status: String,
    pub data: UploadData,
}
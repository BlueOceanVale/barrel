use serde::{Deserialize,Serialize};

#[derive(Serialize)]
pub struct FileInfo {
    pub name: String,
    pub extension: String,
}

#[derive(Deserialize)]
pub struct FileQuery {
    pub name: String,
}


#[derive(Serialize)]
pub struct ConvertResponse {
    pub output: String,
}
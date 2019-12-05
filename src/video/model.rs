use serde::*;
use yew::services::reader::FileData;

#[derive(Deserialize)]
pub struct Video {
    pub src: String,
}

#[derive(Clone)]
pub struct NewVideo {
    pub src: String,
}

impl NewVideo {
    pub fn build(file_data: FileData) -> Self {
        let src = format!(
            "data:image/gif;base64,{}",
            base64::encode(&file_data.content)
        );

        Self { src }
    }
}

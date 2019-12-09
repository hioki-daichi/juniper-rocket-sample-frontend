use serde::*;
use yew::services::reader::FileData;

#[derive(Deserialize, Debug)]
pub struct Video {
    pub src: String,
}

impl Video {
    pub fn new(file_data: FileData) -> Self {
        let src = format!(
            "data:video/mp4;base64,{}",
            base64::encode(&file_data.content)
        );

        Self { src }
    }
}

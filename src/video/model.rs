use serde::*;

#[derive(Deserialize, Debug)]
pub struct Video {
    pub src: String,
}

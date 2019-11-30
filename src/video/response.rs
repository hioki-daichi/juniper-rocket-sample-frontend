use crate::video::model::Video;
use serde::*;

#[derive(Deserialize)]
pub struct VideosResponse {
    pub videos: Vec<Video>,
}

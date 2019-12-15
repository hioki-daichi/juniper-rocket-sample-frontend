use crate::video::model::Video;
use serde::*;

#[derive(Deserialize)]
pub struct VideosResponse {
    pub videos: Vec<Video>,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct RegisterVideoResponse {
    pub registerVideo: Video,
}

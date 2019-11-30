use serde::*;

#[derive(Deserialize)]
pub struct ResponseData<T> {
    pub data: T,
}

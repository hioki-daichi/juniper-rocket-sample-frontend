use yew::format::Json;
use yew::services::fetch;

const API_ENDPOINT: &'static str = "http://localhost:8080/graphql";

pub fn build_request(
    data: &serde_json::value::Value,
) -> fetch::Request<Json<&serde_json::value::Value>> {
    fetch::Request::post(API_ENDPOINT)
        .header("Content-Type", "application/json")
        .body(Json(data))
        .expect("Failed to build request.")
}

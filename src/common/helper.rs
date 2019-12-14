use yew::format::Json;
use yew::services::fetch::Request;

const API_ENDPOINT: &'static str = "http://localhost:8080/graphql";

pub fn build_request(data: &serde_json::value::Value) -> Request<Json<&serde_json::value::Value>> {
    Request::post(API_ENDPOINT)
        .header("Content-Type", "application/json")
        .body(Json(data))
        .expect("Failed to build request.")
}

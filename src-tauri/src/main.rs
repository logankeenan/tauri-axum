#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use serde::{Serialize, Deserialize};
use axum::{response::{Html, Response}, routing::get, Router, Error, http};
use axum::body::{Body, Bytes};

use tower_service::Service;
use std::str;
use axum::handler::Handler;


#[derive(Serialize, Deserialize, Debug)]
struct PrimitiveHttpRequest {
    path: String,
    method: String,
    body: Option<String>,
}

fn create_axum_router() -> Router {
    let mut router: Router = Router::new()
        .route("/", get(index))
        .route("/mars", get(mars));
    router
}


#[tauri::command]
async fn make_http_request(request: PrimitiveHttpRequest) -> String {
    let mut router = create_axum_router();
    let uri = request.path;

    let request: http::Request<Body> = crate::http::Request::builder()
        .uri(uri)
        .body("".into())
        .unwrap();

    let mut response: Response = router.call(request).await.unwrap();

    // Get the response body as a string
    let data: Option<Result<Bytes, Error>> = http_body::Body::data(response.body_mut()).await;
    let result: Bytes = data.unwrap().unwrap();
    str::from_utf8(&*result).unwrap().to_string()
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![make_http_request])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


async fn index() -> Html<&'static str> {
    Html("<h1>Hello World from Axum!!!</h1><br/><a href=\"mars\">Hello Mars</a>")
}

async fn mars() -> Html<&'static str> {
    Html("<h1>Hello Mars from Axum!!!</h1></h1><br/><a href=\"/\">Hello World</a>")
}

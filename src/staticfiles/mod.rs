//! Static file serving

use axum::body::Body;
use axum::extract::Path;
use axum::http::header;
use axum::http::StatusCode;
use axum::response::Response;
use phf::phf_map;

static FILES: phf::Map<&'static str, &'static str> = phf_map! {
    "main.css" => include_str!("../../staticfiles/main.css")
};

pub(super) async fn staticfile(Path(filename): Path<String>) -> Response {
    match FILES.get(&filename) {
        Some(content) => Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "text/css")
            .body(Body::from(*content))
            .unwrap(),
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Not found"))
            .unwrap(),
    }
}

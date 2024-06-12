//! Index content

use askama::Template;
use axum::response::Html;

pub(super) async fn index() -> Html<String> {
    let template = IndexTemplate {};
    Html(template.render().unwrap())
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

use crate::HtmlTemplate;
use askama::Template;
use axum::response::IntoResponse;

pub async fn hello_world() -> impl IntoResponse {
    let template = HelloTemplate {
        name: "Glitter".to_string(),
    };
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "index.html")]
struct HelloTemplate {
    name: String,
}

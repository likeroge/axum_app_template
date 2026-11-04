use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

#[derive(Template, Default)]
#[template(path = "home.html")]
pub struct HomePage {}

pub async fn index() -> impl IntoResponse {
    let html = HomePage::default();
    match html.render() {
        Ok(result) => Html(result).into_response(),
        Err(e) => {
            eprintln!("Failed to render a template: {e:?}");

            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

#[derive(Template, Default)]
#[template(path = "home.html")]
pub struct HomePage {}

pub async fn home() -> impl IntoResponse {
    let html = HomePage::default();
    match html.render() {
        Ok(result) => Html(result).into_response(),
        Err(e) => {
            eprintln!("Failed to render a template: {e:?}");

            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

#[derive(Template, Default)]
#[template(path = "index.html")]
pub struct IndexPage {}

pub async fn index() -> impl IntoResponse {
    let html = IndexPage::default();
    match html.render() {
        Ok(result) => Html(result).into_response(),
        Err(e) => {
            eprintln!("Failed to render a template: {e:?}");

            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

#[derive(Template, Default)]
#[template(path = "about.html")]
pub struct AboutPage {}

pub async fn about() -> impl IntoResponse {
    let html = AboutPage::default();
    match html.render() {
        Ok(result) => Html(result).into_response(),
        Err(e) => {
            eprintln!("Failed to render a template: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

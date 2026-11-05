use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

#[derive(Template, Default)]
#[template(path = "create-user-form.html")]
pub struct CreateUserFormPage {}

pub async fn index() -> impl IntoResponse {
    let html = CreateUserFormPage::default();
    match html.render() {
        Ok(result) => Html(result).into_response(),
        Err(e) => {
            eprintln!("Failed to render a template: {e:?}");

            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

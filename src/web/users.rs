use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use sqlx::SqlitePool;

use crate::{domain::models::user::User, infrastructure::repos};

#[derive(Template, Default)]
#[template(path = "create_user_form.html")]
pub struct CreateUserFormPage {}

pub async fn create_form() -> impl IntoResponse {
    let html = CreateUserFormPage::default();
    match html.render() {
        Ok(result) => Html(result).into_response(),
        Err(e) => {
            eprintln!("Failed to render a template: {e:?}");

            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

#[derive(Template, Default)]
#[template(path = "users_list.html")]
pub struct UsersListPage {
    pub users: Vec<User>,
}

pub async fn user_list(State(pool): State<SqlitePool>) -> impl IntoResponse {
    let mut html = UsersListPage::default();

    match repos::users_repo::users_list(&pool).await {
        Ok(users) => {
            html.users = users;
            match html.render() {
                Ok(result) => Html(result).into_response(),
                Err(e) => {
                    eprintln!("Failed to render a template: {e:?}");
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            }
        }
        Err(err) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

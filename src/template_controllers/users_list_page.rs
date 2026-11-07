use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use sqlx::SqlitePool;

use crate::{domain::models::user::User, infrastructure::repos};

#[derive(Template, Default)]
#[template(path = "users-list.html")]
pub struct UsersListPage {
    pub users: Vec<User>,
}

pub async fn index(State(pool): State<SqlitePool>) -> impl IntoResponse {
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

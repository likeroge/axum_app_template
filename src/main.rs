use axum::{
    Router,
    http::{
        HeaderValue, Method,
        header::{ACCEPT, ACCESS_CONTROL_ALLOW_ORIGIN, AUTHORIZATION, CONTENT_TYPE},
    },
    routing::{get, post},
};
use tower_http::{
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
};

use crate::db::init_db;
mod api;
mod api_responses;
mod db;
mod domain;
mod infrastructure;
mod web;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();

    let static_files =
        ServeDir::new("./static").not_found_service(ServeFile::new("./static/index.html"));
    // ServeDir::new("./front/dist").not_found_service(ServeFile::new("./front/dist/index.html"));
    let cors = CorsLayer::new()
        .allow_origin("*".parse::<HeaderValue>().expect("Cannot parse"))
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::PATCH,
        ])
        .allow_headers([
            AUTHORIZATION,
            ACCEPT,
            CONTENT_TYPE,
            ACCESS_CONTROL_ALLOW_ORIGIN,
        ]);

    let pool = init_db().await.expect("Failed to init DB");

    // Передаём пул через state
    let app = Router::new()
        .route(
            "/sql",
            get(|| async { "Hello from Axum+SQLx+SQLite template!" }),
        )
        .route("/api/users", get(api::users::users_list))
        .route("/api/users/form/create", post(api::users::create_user_form))
        .route("/api/users/:name", get(api::users::users_list_with_name))
        // .route("/api/users/delete/:id", delete(api::users::delete_user))
        .route("/api/users/delete/:id", get(api::users::delete_user))
        .route("/", get(web::home::index))
        .route("/home", get(web::home::home))
        .route("/about", get(web::home::about))
        .route("/users", get(web::users::user_list))
        .route("/users/create", get(web::users::create_form))
        .fallback_service(static_files)
        .layer(cors)
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5005").await.unwrap();
    println!("Listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

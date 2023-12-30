use axum::{routing, Router, http::header::CONTENT_TYPE};
use api;
use sea_orm::{Database, DbErr};
use std::net::SocketAddr;
use axum::http::HeaderValue;
use tower_http::cors::CorsLayer;


const DATABASE_URL: &str = "sqlite:./db?mode=rwc";
#[tokio::main]
async fn main() -> Result<(), DbErr> {

    let conn = Database::connect(DATABASE_URL).await?;

    let app = Router::new()
        .route("/book/list", routing::get(api::list_books))
        .route("/book/create", routing::post(api::create_book))
        .layer(CorsLayer::new()
            .allow_origin(
                "http://127.0.0.1:8080".parse::<HeaderValue>().unwrap()
            )
            .allow_headers([CONTENT_TYPE])
        )
        .with_state(conn);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

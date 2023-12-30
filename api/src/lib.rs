use axum::{extract::State, http::StatusCode, response::Json};
use sea_orm::{ActiveModelTrait, DbConn, EntityTrait, Set, TryIntoModel};

pub mod entities;

use entities::{book, book::Entity as Book};


#[derive(serde::Deserialize)]
pub struct NewBook {
    title: String,
    is_owned: bool,
}

pub async fn create_book(
    State(conn): State<DbConn>,
    Json(new_book): Json<NewBook>,
) -> Result<Json<book::Model>, (StatusCode, String)> {
    book::ActiveModel {
        title: Set(new_book.title.to_owned()),
        is_owned: Set(new_book.is_owned.to_owned()),
        ..Default::default()
    }
        .save(&conn)
        .await
        .and_then(|model| model.try_into_model().map(|model| Json(model)))
        .map_err(internal_error)
}

pub async fn list_books(
    State(conn): State<DbConn>,
) -> Result<Json<Vec<book::Model>>, (StatusCode, String)> {
    Book::find()
        .all(&conn)
        .await
        .map(Json)
        .map_err(internal_error)
}

fn internal_error<E>(err: E) -> (StatusCode, String)
    where
        E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
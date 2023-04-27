use axum::{
    extract::{Query, Path},
    http::StatusCode,
};
use rusqlite::Connection;

use super::post::QueryParams;

pub async fn patch(Query(query): Query<QueryParams>, Path(id): Path<i32>) -> StatusCode {
    let conn = Connection::open("todos.db").unwrap();
    let mut statement = conn
        .prepare("update todos set todo=(?1) where id=(?2)")
        .unwrap();

    match statement.execute([(query.todo), (id.to_string())]) {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::BAD_REQUEST,
    }
}

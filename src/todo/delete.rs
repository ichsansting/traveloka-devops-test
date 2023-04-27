use axum::{
    extract::Path,
    http::StatusCode,
};
use rusqlite::Connection;

pub async fn delete(Path(id): Path<i32>) -> StatusCode {
    let conn = Connection::open("todos.db").unwrap();
    let mut statement = conn
        .prepare("delete from todos where id=(?1)")
        .unwrap();

    match statement.execute([(id.to_string())]) {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::BAD_REQUEST,
    }
}

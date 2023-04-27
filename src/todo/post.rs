use axum::{extract::Query, http::StatusCode};
use rusqlite::Connection;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryParams {
    pub todo: String,
}

pub async fn post(Query(query): Query<QueryParams>) -> StatusCode {
    let conn = Connection::open("todos.db").unwrap();
    let mut statement = conn
        .prepare("insert into todos (todo) values (?1)")
        .unwrap();

    match statement.execute([(query.todo)]) {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::BAD_REQUEST,
    }
}

struct Message {
    message: i32,
}

pub async fn get_last_id() -> String {
    let conn = Connection::open("todos.db").unwrap();
    let mut statement = conn.prepare("select * from todos where id = (select max(id) from todos);").unwrap();

    let id = statement
        .query_map((), |row| {
            Ok(Message {
                message: row.get(0).unwrap(),
            })
        })
        .unwrap();

    id.last().unwrap().unwrap().message.to_string()
}

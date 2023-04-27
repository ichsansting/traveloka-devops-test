use axum::{extract::Path, Json};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Todo {
    id: i32,
    todo: String,
}

pub async fn get_all() -> Json<Vec<Todo>> {
    let conn = Connection::open("todos.db").unwrap();

    let mut statement = conn.prepare("select * from todos").unwrap();
    let todos = statement
        .query_map((), |row| {
            Ok(Todo {
                id: row.get(0).unwrap(),
                todo: row.get(1).unwrap(),
            })
        })
        .unwrap();

    let mut vector = Vec::new();
    for todo in todos {
        vector.push(todo.unwrap());
    }

    Json(vector)
}

pub async fn get(Path(id): Path<i32>) -> Json<Todo> {
    let conn = Connection::open("todos.db").unwrap();

    let mut statement = conn.prepare("select * from todos where id = (?1)").unwrap();
    let todos = statement
        .query_map([id.to_string()], |row| {
            Ok(Todo {
                id: row.get(0).unwrap(),
                todo: row.get(1).unwrap(),
            })
        })
        .unwrap();

    Json(todos.last().unwrap().unwrap())
}

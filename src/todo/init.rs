use rusqlite::Connection;

pub fn init() {
    let conn = Connection::open("todos.db").unwrap();

    match conn.execute(
        "create table todos (
            id integer primary key,
            todo text not null
        )",
        (),
    ) {
        Ok(_) => insert_init_data(conn),
        Err(_) => (),
    }

    
}

fn insert_init_data(conn: Connection){
    let mut statement = conn
        .prepare("insert into todos (todo) values (?1)")
        .unwrap();
    statement.execute(["todo 1"]).unwrap();
    statement.execute(["todo 2"]).unwrap();
    statement.execute(["todo 3"]).unwrap();
    statement.execute(["todo 4"]).unwrap();
}
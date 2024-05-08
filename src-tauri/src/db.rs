use rusqlite::{Connection, Result};


pub fn get_connection () -> Result<Connection> {
    let conn = Connection::open("database.db");
    conn
}


pub fn create_users_table (connection: Result<&Connection, &rusqlite::Error>) {
    connection.unwrap().execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            password TEXT NOT NULL
        )",
        [],
    ).unwrap();
}
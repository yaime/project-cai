use rusqlite::{Connection, Result};
use std::sync::Mutex;

pub struct AppState {
    pub db: Mutex<Option<Connection>>,
}

pub fn init_db(path: &str) -> Result<Connection> {
    // In a real app, we would get the key from the user at runtime.
    // For now, we open the connection.
    // When using SQLCipher, we would call:
    // conn.pragma_update(None, "key", &key)?;
    let conn = Connection::open(path)?;
    Ok(conn)
}

use rusqlite::{Connection, Result};
use std::path::Path;
use std::sync::Mutex;

pub struct AppState {
    pub db: Mutex<Option<Connection>>,
}

const MIGRATIONS: &[(&str, &str)] = &[
    (
        "001_initial_schema",
        include_str!("../migrations/001_initial_schema.sql"),
    ),
    (
        "002_security_schema",
        include_str!("../migrations/002_security_schema.sql"),
    ),
    (
        "003_add_voucher_period_index",
        include_str!("../migrations/003_add_voucher_period_index.sql"),
    ),
    (
        "004_add_voucher_index",
        include_str!("../migrations/004_add_voucher_index.sql"),
    ),
];

pub fn init_db<P: AsRef<Path>>(path: P) -> Result<Connection> {
    let mut conn = Connection::open(path)?;

    // Enable foreign keys
    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    // Create migrations table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS _migrations (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            applied_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;

    let tx = conn.transaction()?;

    let mut applied_migrations = std::collections::HashSet::new();
    {
        let mut stmt = tx.prepare("SELECT name FROM _migrations")?;
        let mut rows = stmt.query([])?;
        while let Some(row) = rows.next()? {
            applied_migrations.insert(row.get::<_, String>(0)?);
        }
    }

    for (name, sql) in MIGRATIONS {
        if !applied_migrations.contains(*name) {
            tx.execute_batch(sql)?;
            tx.execute("INSERT INTO _migrations (name) VALUES (?1)", [name])?;
        }
    }

    tx.commit()?;

    // Check if sys_users is empty and create default admin
    let count: i32 = conn.query_row("SELECT count(*) FROM sys_users", [], |row| row.get(0))?;

    if count == 0 {
        let password_hash =
            crate::auth::hash_password("admin123").expect("Failed to hash default admin password");

        conn.execute(
            "INSERT INTO sys_users (username, password_hash, role_id) VALUES (?1, ?2, ?3)",
            ["admin", &password_hash, "ADMIN"],
        )?;
    }

    Ok(conn)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_db() {
        let conn = init_db(":memory:").expect("Failed to init db");

        // Check if tables exist
        let count: i32 = conn
            .query_row(
                "SELECT count(*) FROM sqlite_master WHERE type='table' AND name='sys_subjects'",
                [],
                |row| row.get(0),
            )
            .unwrap();

        assert_eq!(count, 1, "sys_subjects table should exist");

        // Check migrations table
        let mig_count: i32 = conn
            .query_row("SELECT count(*) FROM _migrations", [], |row| row.get(0))
            .unwrap();

        assert_eq!(mig_count, 4, "Should have 4 migrations applied");

        // Check if admin user exists
        let admin_count: i32 = conn
            .query_row(
                "SELECT count(*) FROM sys_users WHERE username = 'admin'",
                [],
                |row| row.get(0),
            )
            .unwrap();

        assert_eq!(admin_count, 1, "Admin user should be created");
    }
}

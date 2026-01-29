use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use std::path::Path;

pub struct AppState {
    pub db: SqlitePool,
}

pub async fn init_db<P: AsRef<Path>>(path: P) -> Result<SqlitePool, sqlx::Error> {
    let options = SqliteConnectOptions::new()
        .filename(path)
        .create_if_missing(true)
        .foreign_keys(true);

    let pool = SqlitePool::connect_with(options).await?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    Ok(pool)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init_db() {
        // Use a temporary file for testing to ensure connection persistence
        // (In-memory DBs are per-connection by default in sqlx/sqlite unless shared cache is used)
        let temp_dir = std::env::temp_dir();
        let db_name = format!("test_finance_{}.db", uuid::Uuid::new_v4());
        let db_path = temp_dir.join(&db_name);

        let pool = init_db(&db_path).await.expect("Failed to init db");

        // Check if tables exist
        let count: i32 = sqlx::query_scalar(
            "SELECT count(*) FROM sqlite_master WHERE type='table' AND name='sys_subjects'"
        )
        .fetch_one(&pool)
        .await
        .unwrap();

        assert_eq!(count, 1, "sys_subjects table should exist");

        // Check migrations table (sqlx uses _sqlx_migrations by default)
        let rows: Vec<(i64, String)> = sqlx::query_as(
            "SELECT version, description FROM _sqlx_migrations"
        )
        .fetch_all(&pool)
        .await
        .unwrap();

        assert_eq!(rows.len(), 4, "Should have 4 migrations applied");

        // Cleanup
        pool.close().await;
        let _ = std::fs::remove_file(db_path);
    }
}

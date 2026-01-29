use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use serde::Serialize;
use tauri::State;
use crate::db::AppState;

#[derive(Serialize)]
pub struct User {
    id: i32,
    username: String,
    role_id: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    user: User,
    token: String, // Just a dummy token or session ID for now
}

pub fn hash_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)
        .map_err(|e| e.to_string())?
        .to_string();
    Ok(password_hash)
}

pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, String> {
    let parsed_hash = PasswordHash::new(password_hash).map_err(|e| e.to_string())?;
    Ok(Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok())
}

#[tauri::command]
pub fn login(username: String, password: String, state: State<AppState>) -> Result<LoginResponse, String> {
    let conn_guard = state.db.lock().map_err(|e| e.to_string())?;
    let conn = conn_guard.as_ref().ok_or("Database not connected")?;

    let mut stmt = conn.prepare("SELECT id, username, password_hash, role_id FROM sys_users WHERE username = ?1")
        .map_err(|e| e.to_string())?;

    let user_result = stmt.query_row([&username], |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, String>(3)?,
        ))
    });

    match user_result {
        Ok((id, username, hash, role_id)) => {
            if verify_password(&password, &hash)? {
                Ok(LoginResponse {
                    user: User { id, username, role_id },
                    token: "session_token_placeholder".to_string(),
                })
            } else {
                Err("Invalid username or password".to_string())
            }
        },
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            Err("Invalid username or password".to_string())
        },
        Err(e) => {
            Err(format!("Database error: {}", e))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing() {
        let password = "my_secret_password";
        let hash = hash_password(password).unwrap();
        assert!(verify_password(password, &hash).unwrap());
        assert!(!verify_password("wrong_password", &hash).unwrap());
    }
}

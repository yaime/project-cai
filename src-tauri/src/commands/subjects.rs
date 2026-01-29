use crate::db::AppState;
use rusqlite::{Connection, Result as RusqliteResult};
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Subject {
    pub code: String,
    pub name: String,
    pub direction: String,
    #[serde(rename = "type")]
    pub subject_type: String,
    pub is_auxiliary: bool,
    pub parent_id: Option<String>,
}

// Logic functions
pub fn list_subjects(conn: &Connection) -> RusqliteResult<Vec<Subject>> {
    let mut stmt = conn.prepare("SELECT code, name, direction, type, is_auxiliary, parent_id FROM sys_subjects ORDER BY code")?;
    let subjects = stmt.query_map([], |row| {
        Ok(Subject {
            code: row.get(0)?,
            name: row.get(1)?,
            direction: row.get(2)?,
            subject_type: row.get(3)?,
            is_auxiliary: row.get(4)?,
            parent_id: row.get(5)?,
        })
    })?.collect();
    subjects
}

#[derive(Debug, Deserialize)]
pub struct CreateSubjectArgs {
    pub code: String,
    pub name: String,
    pub direction: String,
    #[serde(rename = "type")]
    pub subject_type: String,
    pub is_auxiliary: bool,
    pub parent_id: Option<String>,
}

pub fn insert_subject(conn: &Connection, args: &CreateSubjectArgs) -> RusqliteResult<()> {
    conn.execute(
        "INSERT INTO sys_subjects (code, name, direction, type, is_auxiliary, parent_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        (
            &args.code,
            &args.name,
            &args.direction,
            &args.subject_type,
            &args.is_auxiliary,
            &args.parent_id,
        ),
    )?;
    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct UpdateSubjectArgs {
    pub code: String,
    pub name: String,
    pub direction: String,
    #[serde(rename = "type")]
    pub subject_type: String,
    pub is_auxiliary: bool,
    pub parent_id: Option<String>,
}

pub fn modify_subject(conn: &Connection, args: &UpdateSubjectArgs) -> RusqliteResult<()> {
     conn.execute(
        "UPDATE sys_subjects SET name = ?1, direction = ?2, type = ?3, is_auxiliary = ?4, parent_id = ?5 WHERE code = ?6",
        (
            &args.name,
            &args.direction,
            &args.subject_type,
            &args.is_auxiliary,
            &args.parent_id,
            &args.code,
        ),
    )?;
    Ok(())
}

pub fn remove_subject(conn: &Connection, code: &str) -> RusqliteResult<()> {
    conn.execute("DELETE FROM sys_subjects WHERE code = ?1", [code])?;
    Ok(())
}

// Commands
#[tauri::command]
pub fn get_all_subjects(state: State<AppState>) -> Result<Vec<Subject>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let conn = db.as_ref().ok_or("Database not connected")?;
    list_subjects(conn).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_subject(state: State<AppState>, args: CreateSubjectArgs) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let conn = db.as_ref().ok_or("Database not connected")?;
    insert_subject(conn, &args).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_subject(state: State<AppState>, args: UpdateSubjectArgs) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let conn = db.as_ref().ok_or("Database not connected")?;
    modify_subject(conn, &args).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_subject(state: State<AppState>, code: String) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let conn = db.as_ref().ok_or("Database not connected")?;
    remove_subject(conn, &code).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_db;

    #[test]
    fn test_subjects_logic() {
        let conn = init_db(":memory:").expect("Failed to init db");

        // Create
        let args = CreateSubjectArgs {
            code: "1001".to_string(),
            name: "Cash".to_string(),
            direction: "Debit".to_string(),
            subject_type: "Asset".to_string(),
            is_auxiliary: false,
            parent_id: None,
        };
        insert_subject(&conn, &args).expect("Failed to create subject");

        // Get
        let subjects = list_subjects(&conn).expect("Failed to list subjects");
        assert_eq!(subjects.len(), 1);
        assert_eq!(subjects[0].code, "1001");

        // Update
        let update_args = UpdateSubjectArgs {
            code: "1001".to_string(),
            name: "Cash in Hand".to_string(),
            direction: "Debit".to_string(),
            subject_type: "Asset".to_string(),
            is_auxiliary: false,
            parent_id: None,
        };
        modify_subject(&conn, &update_args).expect("Failed to update");
        let subjects = list_subjects(&conn).expect("Failed to list subjects");
        assert_eq!(subjects[0].name, "Cash in Hand");

        // Delete
        remove_subject(&conn, "1001").expect("Failed to delete");
         let subjects = list_subjects(&conn).expect("Failed to list subjects");
        assert_eq!(subjects.len(), 0);
    }
}

use tauri::State;
use crate::db::AppState;
use serde::Deserialize;
use rusqlite::Connection;

#[derive(Deserialize, Clone)]
pub struct VoucherEntryData {
    pub summary: Option<String>,
    pub subject_code: String,
    pub debit_amount: i64,
    pub credit_amount: i64,
}

#[derive(Deserialize, Clone)]
pub struct VoucherData {
    pub voucher_date: String,
    pub voucher_period: i32,
    pub voucher_word: String,
    pub entries: Vec<VoucherEntryData>,
}

fn save_voucher_impl(conn: &mut Connection, voucher: VoucherData) -> Result<String, String> {
    // Validation
    let total_debit: i64 = voucher.entries.iter().map(|e| e.debit_amount).sum();
    let total_credit: i64 = voucher.entries.iter().map(|e| e.credit_amount).sum();

    if total_debit != total_credit {
        return Err(format!("Voucher is unbalanced: Debit {}, Credit {}", total_debit, total_credit));
    }

    let tx = conn.transaction().map_err(|e| e.to_string())?;

    // Get next voucher number
    let next_number: i32 = {
        let mut stmt = tx.prepare("SELECT MAX(voucher_number) FROM t_vouchers WHERE voucher_period = ?1").map_err(|e| e.to_string())?;
        let max_num: Option<i32> = stmt.query_row([voucher.voucher_period], |row| row.get(0)).unwrap_or(None);
        max_num.unwrap_or(0) + 1
    };

    // Insert Header
    tx.execute(
        "INSERT INTO t_vouchers (voucher_date, voucher_period, voucher_word, voucher_number) VALUES (?1, ?2, ?3, ?4)",
        (
            &voucher.voucher_date,
            voucher.voucher_period,
            &voucher.voucher_word,
            next_number,
        ),
    ).map_err(|e| e.to_string())?;

    let voucher_id = tx.last_insert_rowid();

    // Insert Entries
    for entry in voucher.entries {
        tx.execute(
            "INSERT INTO t_voucher_entries (voucher_id, summary, subject_code, debit_amount, credit_amount) VALUES (?1, ?2, ?3, ?4, ?5)",
            (
                voucher_id,
                &entry.summary,
                &entry.subject_code,
                entry.debit_amount,
                entry.credit_amount,
            ),
        ).map_err(|e| e.to_string())?;
    }

    tx.commit().map_err(|e| e.to_string())?;

    Ok(format!("Voucher saved with number {}", next_number))
}

#[tauri::command]
pub fn save_voucher(state: State<AppState>, voucher: VoucherData) -> Result<String, String> {
    let mut db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let conn = db_guard.as_mut().ok_or("Database not connected")?;
    save_voucher_impl(conn, voucher)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_db;

    #[test]
    fn test_save_voucher_success() {
        let mut conn = init_db(":memory:").expect("Failed to init db");

        // Need to insert some subjects first due to FK constraint
        conn.execute("INSERT INTO sys_subjects (code, name, direction, type) VALUES ('1001', 'Cash', 'Debit', 'Asset')", []).unwrap();
        conn.execute("INSERT INTO sys_subjects (code, name, direction, type) VALUES ('2001', 'Loan', 'Credit', 'Liability')", []).unwrap();

        let voucher = VoucherData {
            voucher_date: "2023-10-01".to_string(),
            voucher_period: 202310,
            voucher_word: "记".to_string(),
            entries: vec![
                VoucherEntryData {
                    summary: Some("Test".to_string()),
                    subject_code: "1001".to_string(),
                    debit_amount: 1000,
                    credit_amount: 0,
                },
                VoucherEntryData {
                    summary: Some("Test".to_string()),
                    subject_code: "2001".to_string(),
                    debit_amount: 0,
                    credit_amount: 1000,
                },
            ],
        };

        let result = save_voucher_impl(&mut conn, voucher.clone());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Voucher saved with number 1");

        // Save again to check sequential numbering
        let result2 = save_voucher_impl(&mut conn, voucher);
        assert!(result2.is_ok());
        assert_eq!(result2.unwrap(), "Voucher saved with number 2");
    }

    #[test]
    fn test_save_voucher_unbalanced() {
        let mut conn = init_db(":memory:").expect("Failed to init db");

        conn.execute("INSERT INTO sys_subjects (code, name, direction, type) VALUES ('1001', 'Cash', 'Debit', 'Asset')", []).unwrap();

        let voucher = VoucherData {
            voucher_date: "2023-10-01".to_string(),
            voucher_period: 202310,
            voucher_word: "记".to_string(),
            entries: vec![
                VoucherEntryData {
                    summary: Some("Test".to_string()),
                    subject_code: "1001".to_string(),
                    debit_amount: 1000,
                    credit_amount: 0,
                },
            ],
        };

        let result = save_voucher_impl(&mut conn, voucher);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Voucher is unbalanced"));

        // Ensure nothing was inserted
        let count: i32 = conn.query_row("SELECT count(*) FROM t_vouchers", [], |r| r.get(0)).unwrap();
        assert_eq!(count, 0);
    }
}

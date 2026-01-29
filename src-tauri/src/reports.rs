use crate::db::AppState;
use rusqlite::Connection;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use tauri::State;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TrialBalanceItem {
    pub code: String,
    pub name: String,
    pub opening_balance: i64,
    pub current_debit: i64,
    pub current_credit: i64,
    pub ending_balance: i64,
}

struct Subject {
    code: String,
    name: String,
    direction: String,
    parent_id: Option<String>,
}

struct EntryStats {
    opening_debit: i64,
    opening_credit: i64,
    current_debit: i64,
    current_credit: i64,
}

#[tauri::command]
pub fn get_trial_balance(
    period: i32,
    state: State<AppState>,
) -> Result<Vec<TrialBalanceItem>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let conn = db.as_ref().ok_or("Database not connected")?;

    compute_trial_balance(conn, period).map_err(|e| e.to_string())
}

fn compute_trial_balance(conn: &Connection, period: i32) -> rusqlite::Result<Vec<TrialBalanceItem>> {
    // 1. Fetch all subjects
    let mut stmt = conn.prepare("SELECT code, name, direction, parent_id FROM sys_subjects")?;
    let subjects_iter = stmt.query_map([], |row| {
        Ok(Subject {
            code: row.get(0)?,
            name: row.get(1)?,
            direction: row.get(2)?,
            parent_id: row.get(3)?,
        })
    })?;

    let mut subjects = HashMap::new();
    for s in subjects_iter {
        let s = s?;
        subjects.insert(s.code.clone(), s);
    }

    // 2. Fetch sums
    // Opening: period < requested_period
    let mut stmt = conn.prepare(
        "SELECT e.subject_code, SUM(e.debit_amount), SUM(e.credit_amount)
         FROM t_voucher_entries e
         JOIN t_vouchers v ON e.voucher_id = v.id
         WHERE v.voucher_period < ?1
         GROUP BY e.subject_code"
    )?;
    let opening_iter = stmt.query_map([period], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?, row.get::<_, i64>(2)?))
    })?;

    // Current: period == requested_period
    let mut stmt = conn.prepare(
        "SELECT e.subject_code, SUM(e.debit_amount), SUM(e.credit_amount)
         FROM t_voucher_entries e
         JOIN t_vouchers v ON e.voucher_id = v.id
         WHERE v.voucher_period = ?1
         GROUP BY e.subject_code"
    )?;
    let current_iter = stmt.query_map([period], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?, row.get::<_, i64>(2)?))
    })?;

    let mut stats: HashMap<String, EntryStats> = HashMap::new();

    // Init stats for all subjects to 0
    for code in subjects.keys() {
        stats.insert(code.clone(), EntryStats {
            opening_debit: 0,
            opening_credit: 0,
            current_debit: 0,
            current_credit: 0,
        });
    }

    for res in opening_iter {
        let (code, deb, cred) = res?;
        if let Some(s) = stats.get_mut(&code) {
            s.opening_debit = deb;
            s.opening_credit = cred;
        }
    }

    for res in current_iter {
        let (code, deb, cred) = res?;
        if let Some(s) = stats.get_mut(&code) {
            s.current_debit = deb;
            s.current_credit = cred;
        }
    }

    // 3. Aggregate (Rollup)
    // Process from longest code to shortest (children first)
    let mut sorted_codes: Vec<String> = subjects.keys().cloned().collect();
    sorted_codes.sort_by_key(|k| std::cmp::Reverse(k.len()));

    for code in sorted_codes {
        if let Some(subject) = subjects.get(&code) {
            if let Some(parent_id) = &subject.parent_id {
                // Get current subject's stats (copy them)
                let (od, oc, cd, cc) = if let Some(s) = stats.get(&code) {
                    (s.opening_debit, s.opening_credit, s.current_debit, s.current_credit)
                } else {
                    (0, 0, 0, 0)
                };

                // Add to parent
                if let Some(parent_stats) = stats.get_mut(parent_id) {
                    parent_stats.opening_debit += od;
                    parent_stats.opening_credit += oc;
                    parent_stats.current_debit += cd;
                    parent_stats.current_credit += cc;
                }
            }
        }
    }

    // 4. Calculate final balances
    let mut results = Vec::new();
    for code in subjects.keys() {
        let subject = &subjects[code];
        let st = &stats[code];

        let (opening_balance, ending_balance) = if subject.direction == "Debit" {
            let ob = st.opening_debit - st.opening_credit;
            let eb = ob + st.current_debit - st.current_credit;
            (ob, eb)
        } else {
            let ob = st.opening_credit - st.opening_debit;
            let eb = ob + st.current_credit - st.current_debit;
            (ob, eb)
        };

        results.push(TrialBalanceItem {
            code: code.clone(),
            name: subject.name.clone(),
            opening_balance,
            current_debit: st.current_debit,
            current_credit: st.current_credit,
            ending_balance,
        });
    }

    results.sort_by(|a, b| a.code.cmp(&b.code));
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_db;

    #[test]
    fn test_trial_balance_aggregation() {
        let conn = init_db(":memory:").unwrap();

        // Setup Subjects
        conn.execute("INSERT INTO sys_subjects (code, name, direction, type, parent_id) VALUES
            ('1001', 'Cash', 'Debit', 'Asset', NULL),
            ('100101', 'Petty Cash', 'Debit', 'Asset', '1001'),
            ('1002', 'Bank', 'Debit', 'Asset', NULL)", []).unwrap();

        // Setup Vouchers
        // Period 202309 (Previous)
        conn.execute("INSERT INTO t_vouchers (voucher_date, voucher_period, voucher_number) VALUES ('2023-09-01', 202309, 1)", []).unwrap();
        let v1_id = conn.last_insert_rowid();

        // 100101 Debit 1000 (Opening)
        conn.execute("INSERT INTO t_voucher_entries (voucher_id, subject_code, debit_amount, credit_amount) VALUES (?1, '100101', 1000, 0)", [v1_id]).unwrap();

        // Period 202310 (Current)
        conn.execute("INSERT INTO t_vouchers (voucher_date, voucher_period, voucher_number) VALUES ('2023-10-01', 202310, 2)", []).unwrap();
        let v2_id = conn.last_insert_rowid();

        // 100101 Debit 500 (Current)
        conn.execute("INSERT INTO t_voucher_entries (voucher_id, subject_code, debit_amount, credit_amount) VALUES (?1, '100101', 500, 0)", [v2_id]).unwrap();

        // 1002 Debit 200 (Current)
        conn.execute("INSERT INTO t_voucher_entries (voucher_id, subject_code, debit_amount, credit_amount) VALUES (?1, '1002', 200, 0)", [v2_id]).unwrap();


        // Run Calculation for 202310
        let result = compute_trial_balance(&conn, 202310).unwrap();

        // Verify 100101 (Child)
        let cash_child = result.iter().find(|i| i.code == "100101").unwrap();
        assert_eq!(cash_child.opening_balance, 1000); // From 202309
        assert_eq!(cash_child.current_debit, 500);    // From 202310
        assert_eq!(cash_child.ending_balance, 1500);

        // Verify 1001 (Parent) - Should Aggregate
        let cash_parent = result.iter().find(|i| i.code == "1001").unwrap();
        assert_eq!(cash_parent.opening_balance, 1000); // Aggregated from 100101
        assert_eq!(cash_parent.current_debit, 500);    // Aggregated from 100101
        assert_eq!(cash_parent.ending_balance, 1500);

        // Verify 1002
        let bank = result.iter().find(|i| i.code == "1002").unwrap();
        assert_eq!(bank.opening_balance, 0);
        assert_eq!(bank.current_debit, 200);
        assert_eq!(bank.ending_balance, 200);
    }
}

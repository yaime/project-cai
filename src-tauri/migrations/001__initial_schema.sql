-- sys_subjects: Chart of Accounts
CREATE TABLE sys_subjects (
    code TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    direction TEXT NOT NULL CHECK(direction IN ('Debit', 'Credit')),
    type TEXT NOT NULL CHECK(type IN ('Asset', 'Liability', 'Equity', 'Cost', 'ProfitLoss')),
    is_auxiliary BOOLEAN NOT NULL DEFAULT 0,
    parent_id TEXT,
    FOREIGN KEY(parent_id) REFERENCES sys_subjects(code)
);

CREATE INDEX idx_sys_subjects_parent_id ON sys_subjects(parent_id);

-- t_vouchers: Voucher Header
CREATE TABLE t_vouchers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    voucher_date TEXT NOT NULL, -- ISO8601 YYYY-MM-DD
    voucher_period INTEGER NOT NULL, -- YYYYMM
    voucher_word TEXT NOT NULL DEFAULT '记',
    voucher_number INTEGER NOT NULL,
    attachment_count INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- t_voucher_entries: Voucher Lines
CREATE TABLE t_voucher_entries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    voucher_id INTEGER NOT NULL,
    summary TEXT,
    subject_code TEXT NOT NULL,
    debit_amount INTEGER DEFAULT 0, -- Stored in CENTS (fen)
    credit_amount INTEGER DEFAULT 0, -- Stored in CENTS (fen)
    FOREIGN KEY(voucher_id) REFERENCES t_vouchers(id) ON DELETE CASCADE,
    FOREIGN KEY(subject_code) REFERENCES sys_subjects(code)
);

-- t_auxiliary_balance: Auxiliary Dimensions
CREATE TABLE t_auxiliary_balance (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    subject_code TEXT NOT NULL,
    aux_type TEXT NOT NULL, -- e.g., 'Customer', 'Vendor'
    aux_id TEXT NOT NULL,
    balance INTEGER DEFAULT 0,
    FOREIGN KEY(subject_code) REFERENCES sys_subjects(code)
);

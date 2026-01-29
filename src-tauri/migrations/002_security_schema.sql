-- sys_roles: RBAC Roles
CREATE TABLE sys_roles (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    permissions TEXT -- JSON string of permissions
);

-- Insert Predefined Roles
INSERT INTO sys_roles (id, name, description) VALUES
('ADMIN', 'Administrator', 'System settings, user management'),
('ACCOUNTANT', 'Accountant', 'Create vouchers, view reports'),
('CASHIER', 'Cashier', 'Cash/Bank related vouchers only'),
('MANAGER', 'Manager', 'Approve vouchers (Audit)');

-- sys_users: Users
CREATE TABLE sys_users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    role_id TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(role_id) REFERENCES sys_roles(id)
);

-- sys_logs: Audit Trail
CREATE TABLE sys_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER,
    action TEXT NOT NULL, -- e.g. "INSERT t_vouchers id=10"
    details TEXT, -- JSON details
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(user_id) REFERENCES sys_users(id)
);

-- Index for performance
CREATE INDEX idx_sys_logs_user_id ON sys_logs(user_id);

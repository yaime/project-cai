-- Index for performance on subject_code lookups
CREATE INDEX idx_voucher_entries_subject_code ON t_voucher_entries(subject_code);

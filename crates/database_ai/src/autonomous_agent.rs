use database_core::{StatementType, classify_statement};

/// Controls which SQL operations an autonomous agent is permitted to execute.
///
/// Read-only operations (SELECT, EXPLAIN) are allowed by default.
/// All write operations require explicit opt-in to prevent unintended mutations.
pub struct ActionAllowlist {
    pub allow_select: bool,
    pub allow_insert: bool,
    pub allow_update: bool,
    pub allow_delete: bool,
    pub allow_ddl: bool,
    pub allow_explain: bool,
    pub max_rows_affected: usize,
    pub require_confirmation: bool,
}

impl Default for ActionAllowlist {
    fn default() -> Self {
        Self {
            allow_select: true,
            allow_insert: false,
            allow_update: false,
            allow_delete: false,
            allow_ddl: false,
            allow_explain: true,
            max_rows_affected: 100,
            require_confirmation: true,
        }
    }
}

impl ActionAllowlist {
    /// Creates an allowlist that permits only safe read-only operations.
    pub fn read_only() -> Self {
        Self::default()
    }

    /// Checks whether the given SQL statement is permitted by this allowlist.
    ///
    /// Returns `Ok(())` if execution is allowed, or `Err` with a description
    /// of why the operation is not permitted.
    pub fn check_permission(&self, sql: &str) -> Result<(), String> {
        let statement_type = classify_statement(sql);
        match statement_type {
            StatementType::ReadOnly => {
                if self.allow_select {
                    Ok(())
                } else {
                    Err("SELECT queries are not permitted by the current allowlist".to_string())
                }
            }
            StatementType::Insert => {
                if self.allow_insert {
                    Ok(())
                } else {
                    Err(
                        "INSERT statements are not permitted. Enable allow_insert to run this operation.".to_string()
                    )
                }
            }
            StatementType::Update => {
                if self.allow_update {
                    Ok(())
                } else {
                    Err(
                        "UPDATE statements are not permitted. Enable allow_update to run this operation.".to_string()
                    )
                }
            }
            StatementType::Delete => {
                if self.allow_delete {
                    Ok(())
                } else {
                    Err(
                        "DELETE statements are not permitted. Enable allow_delete to run this operation.".to_string()
                    )
                }
            }
            StatementType::Ddl => {
                if self.allow_ddl {
                    Ok(())
                } else {
                    Err(
                        "DDL statements (CREATE/ALTER/DROP) are not permitted. Enable allow_ddl to run this operation.".to_string()
                    )
                }
            }
            StatementType::Dcl => Err(
                "DCL statements (GRANT/REVOKE) are not permitted for autonomous agents.".to_string(),
            ),
            StatementType::Transaction => Err(
                "Transaction control statements are not permitted for autonomous agents."
                    .to_string(),
            ),
            StatementType::Unknown => Err(
                "Unknown statement type. Only recognized SQL statements are permitted.".to_string(),
            ),
        }
    }

    /// Returns whether a write operation requires user confirmation before proceeding.
    pub fn requires_confirmation_for(&self, sql: &str) -> bool {
        if !self.require_confirmation {
            return false;
        }
        let statement_type = classify_statement(sql);
        !matches!(statement_type, StatementType::ReadOnly)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_allows_select() {
        let allowlist = ActionAllowlist::default();
        assert!(allowlist.check_permission("SELECT * FROM users").is_ok());
    }

    #[test]
    fn test_default_blocks_insert() {
        let allowlist = ActionAllowlist::default();
        assert!(
            allowlist
                .check_permission("INSERT INTO users VALUES (1)")
                .is_err()
        );
    }

    #[test]
    fn test_default_blocks_update() {
        let allowlist = ActionAllowlist::default();
        assert!(
            allowlist
                .check_permission("UPDATE users SET name = 'x'")
                .is_err()
        );
    }

    #[test]
    fn test_default_blocks_delete() {
        let allowlist = ActionAllowlist::default();
        assert!(
            allowlist
                .check_permission("DELETE FROM users WHERE id = 1")
                .is_err()
        );
    }

    #[test]
    fn test_default_blocks_ddl() {
        let allowlist = ActionAllowlist::default();
        assert!(
            allowlist
                .check_permission("CREATE TABLE foo (id INT)")
                .is_err()
        );
    }

    #[test]
    fn test_default_allows_explain() {
        let allowlist = ActionAllowlist::default();
        assert!(
            allowlist
                .check_permission("EXPLAIN SELECT * FROM users")
                .is_ok()
        );
    }

    #[test]
    fn test_insert_allowed_when_enabled() {
        let allowlist = ActionAllowlist {
            allow_insert: true,
            ..ActionAllowlist::default()
        };
        assert!(
            allowlist
                .check_permission("INSERT INTO t VALUES (1)")
                .is_ok()
        );
    }

    #[test]
    fn test_requires_confirmation_for_write() {
        let allowlist = ActionAllowlist::default();
        assert!(allowlist.requires_confirmation_for("INSERT INTO t VALUES (1)"));
        assert!(!allowlist.requires_confirmation_for("SELECT 1"));
    }

    #[test]
    fn test_no_confirmation_when_disabled() {
        let allowlist = ActionAllowlist {
            require_confirmation: false,
            ..ActionAllowlist::default()
        };
        assert!(!allowlist.requires_confirmation_for("INSERT INTO t VALUES (1)"));
    }
}

use std::time::Duration;

#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Connection failed to {host}:{port}: {cause}")]
    ConnectionFailed {
        host: String,
        port: u16,
        cause: String,
    },

    #[error("Authentication failed for user '{user}'")]
    AuthenticationFailed { user: String },

    #[error("Query timed out after {duration:?}: {sql_preview}")]
    QueryTimeout {
        sql_preview: String,
        duration: Duration,
    },

    #[error("Query failed: {db_error}\nSQL: {sql_preview}")]
    QueryFailed {
        sql_preview: String,
        db_error: String,
        position: Option<usize>,
    },

    #[error("Connection lost (in transaction: {was_in_transaction})")]
    ConnectionLost { was_in_transaction: bool },

    #[error("Query cancelled")]
    Cancelled,

    #[error("Read-only violation: {statement_type} statements are not allowed")]
    ReadOnlyViolation { statement_type: String },

    #[error("SSL/TLS error: {cause}")]
    SslError { cause: String },

    #[error("SSH tunnel to {host}:{port} failed: {cause}")]
    SshTunnelFailed {
        host: String,
        port: u16,
        cause: String,
    },

    #[error("LOB size exceeded for column '{column}': {size} bytes (max {max_size})")]
    LobSizeExceeded {
        column: String,
        size: usize,
        max_size: usize,
    },

    #[error("Driver not found for type '{driver_type}'")]
    DriverNotFound { driver_type: String },
}

impl DatabaseError {
    pub fn sql_preview(sql: &str) -> String {
        const MAX_PREVIEW_LEN: usize = 120;
        if sql.len() <= MAX_PREVIEW_LEN {
            sql.to_string()
        } else {
            format!("{}...", &sql[..MAX_PREVIEW_LEN])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sql_preview_short() {
        let sql = "SELECT * FROM users";
        assert_eq!(DatabaseError::sql_preview(sql), sql);
    }

    #[test]
    fn test_sql_preview_long() {
        let sql = "x".repeat(200);
        let preview = DatabaseError::sql_preview(&sql);
        assert_eq!(preview.len(), 123); // 120 + "..."
        assert!(preview.ends_with("..."));
    }

    #[test]
    fn test_error_display() {
        let error = DatabaseError::ConnectionFailed {
            host: "localhost".to_string(),
            port: 5432,
            cause: "refused".to_string(),
        };
        assert_eq!(
            error.to_string(),
            "Connection failed to localhost:5432: refused"
        );
    }

    #[test]
    fn test_cancelled_display() {
        let error = DatabaseError::Cancelled;
        assert_eq!(error.to_string(), "Query cancelled");
    }

    #[test]
    fn test_read_only_violation_display() {
        let error = DatabaseError::ReadOnlyViolation {
            statement_type: "INSERT".to_string(),
        };
        assert_eq!(
            error.to_string(),
            "Read-only violation: INSERT statements are not allowed"
        );
    }
}

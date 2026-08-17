use crate::{ConnAcquireErr, ConnectOptions, DbErr, RuntimeErr};

/// Converts an [sqlx::error] execution error to a [DbErr]
pub fn sqlx_error_to_exec_err(err: sqlx::Error) -> DbErr {
    DbErr::Exec(RuntimeErr::SqlxError(err))
}

/// Converts an [sqlx::error] query error to a [DbErr]
pub fn sqlx_error_to_query_err(err: sqlx::Error) -> DbErr {
    DbErr::Query(RuntimeErr::SqlxError(err))
}

/// Converts an [sqlx::error] connection error to a [DbErr]
pub fn sqlx_error_to_conn_err(err: sqlx::Error) -> DbErr {
    DbErr::Conn(RuntimeErr::SqlxError(err))
}

/// Converts an [sqlx::error] error to a [DbErr]
pub fn sqlx_map_err_ignore_not_found<T: std::fmt::Debug>(
    err: Result<Option<T>, sqlx::Error>,
) -> Result<Option<T>, DbErr> {
    if let Err(sqlx::Error::RowNotFound) = err {
        Ok(None)
    } else {
        err.map_err(sqlx_error_to_query_err)
    }
}

/// Converts an [sqlx::error] error to a [DbErr]
pub fn sqlx_conn_acquire_err(sqlx_err: sqlx::Error) -> DbErr {
    match sqlx_err {
        sqlx::Error::PoolTimedOut => DbErr::ConnectionAcquire(ConnAcquireErr::Timeout),
        sqlx::Error::PoolClosed => DbErr::ConnectionAcquire(ConnAcquireErr::ConnectionClosed),
        _ => DbErr::Conn(RuntimeErr::SqlxError(sqlx_err)),
    }
}

impl ConnectOptions {
    /// Convert [ConnectOptions] into [sqlx::pool::PoolOptions]
    pub fn sqlx_pool_options<DB>(self) -> sqlx::pool::PoolOptions<DB>
    where
        DB: sqlx::Database,
    {
        let mut opt = sqlx::pool::PoolOptions::new();
        if let Some(max_connections) = self.max_connections {
            opt = opt.max_connections(max_connections);
        }
        if let Some(min_connections) = self.min_connections {
            opt = opt.min_connections(min_connections);
        }
        if let Some(connect_timeout) = self.connect_timeout {
            opt = opt.acquire_timeout(connect_timeout);
        }
        if let Some(idle_timeout) = self.idle_timeout {
            opt = opt.idle_timeout(Some(idle_timeout));
        }
        if let Some(acquire_timeout) = self.acquire_timeout {
            opt = opt.acquire_timeout(acquire_timeout);
        }
        if let Some(max_lifetime) = self.max_lifetime {
            opt = opt.max_lifetime(Some(max_lifetime));
        }
        opt = opt.test_before_acquire(self.test_before_acquire);
        opt
    }
}

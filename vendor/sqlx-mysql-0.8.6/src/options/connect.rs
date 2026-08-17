use crate::connection::ConnectOptions;
use crate::error::Error;
use crate::executor::Executor;
use crate::{MySqlConnectOptions, MySqlConnection};
use futures_core::future::BoxFuture;
use log::LevelFilter;
use sqlx_core::Url;
use std::time::Duration;

impl ConnectOptions for MySqlConnectOptions {
    type Connection = MySqlConnection;

    fn from_url(url: &Url) -> Result<Self, Error> {
        Self::parse_from_url(url)
    }

    fn to_url_lossy(&self) -> Url {
        self.build_url()
    }

    fn connect(&self) -> BoxFuture<'_, Result<Self::Connection, Error>>
    where
        Self::Connection: Sized,
    {
        Box::pin(async move {
            let mut conn = MySqlConnection::establish(self).await?;

            // After the connection is established, we initialize by configuring a few
            // connection parameters

            // https://mariadb.com/kb/en/sql-mode/

            // PIPES_AS_CONCAT - Allows using the pipe character (ASCII 124) as string concatenation operator.
            //                   This means that "A" || "B" can be used in place of CONCAT("A", "B").

            // NO_ENGINE_SUBSTITUTION - If not set, if the available storage engine specified by a CREATE TABLE is
            //                          not available, a warning is given and the default storage
            //                          engine is used instead.

            // NO_ZERO_DATE - Don't allow '0000-00-00'. This is invalid in Rust.

            // NO_ZERO_IN_DATE - Don't allow 'YYYY-00-00'. This is invalid in Rust.

            // --

            // Setting the time zone allows us to assume that the output
            // from a TIMESTAMP field is UTC

            // --

            // https://mathiasbynens.be/notes/mysql-utf8mb4

            let mut sql_mode = Vec::new();
            if self.pipes_as_concat {
                sql_mode.push(r#"PIPES_AS_CONCAT"#);
            }
            if self.no_engine_substitution {
                sql_mode.push(r#"NO_ENGINE_SUBSTITUTION"#);
            }

            let mut options = Vec::new();
            if !sql_mode.is_empty() {
                options.push(format!(
                    r#"sql_mode=(SELECT CONCAT(@@sql_mode, ',{}'))"#,
                    sql_mode.join(",")
                ));
            }
            if let Some(timezone) = &self.timezone {
                options.push(format!(r#"time_zone='{}'"#, timezone));
            }
            if self.set_names {
                options.push(format!(
                    r#"NAMES {} COLLATE {}"#,
                    conn.inner.stream.charset.as_str(),
                    conn.inner.stream.collation.as_str()
                ))
            }

            if !options.is_empty() {
                conn.execute(&*format!(r#"SET {};"#, options.join(",")))
                    .await?;
            }

            Ok(conn)
        })
    }

    fn log_statements(mut self, level: LevelFilter) -> Self {
        self.log_settings.log_statements(level);
        self
    }

    fn log_slow_statements(mut self, level: LevelFilter, duration: Duration) -> Self {
        self.log_settings.log_slow_statements(level, duration);
        self
    }
}

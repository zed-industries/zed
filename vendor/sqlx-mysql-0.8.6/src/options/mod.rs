use std::path::{Path, PathBuf};

mod connect;
mod parse;
mod ssl_mode;

use crate::{connection::LogSettings, net::tls::CertificateInput};
pub use ssl_mode::MySqlSslMode;

/// Options and flags which can be used to configure a MySQL connection.
///
/// A value of `MySqlConnectOptions` can be parsed from a connection URL,
/// as described by [MySQL](https://dev.mysql.com/doc/connector-j/8.0/en/connector-j-reference-jdbc-url-format.html).
///
/// The generic format of the connection URL:
///
/// ```text
/// mysql://[host][/database][?properties]
/// ```
///
/// This type also implements [`FromStr`][std::str::FromStr] so you can parse it from a string
/// containing a connection URL and then further adjust options if necessary (see example below).
///
/// ## Properties
///
/// |Parameter|Default|Description|
/// |---------|-------|-----------|
/// | `ssl-mode` | `PREFERRED` | Determines whether or with what priority a secure SSL TCP/IP connection will be negotiated. See [`MySqlSslMode`]. |
/// | `ssl-ca` | `None` | Sets the name of a file containing a list of trusted SSL Certificate Authorities. |
/// | `statement-cache-capacity` | `100` | The maximum number of prepared statements stored in the cache. Set to `0` to disable. |
/// | `socket` | `None` | Path to the unix domain socket, which will be used instead of TCP if set. |
///
/// # Example
///
/// ```rust,no_run
/// # async fn example() -> sqlx::Result<()> {
/// use sqlx::{Connection, ConnectOptions};
/// use sqlx::mysql::{MySqlConnectOptions, MySqlConnection, MySqlPool, MySqlSslMode};
///
/// // URL connection string
/// let conn = MySqlConnection::connect("mysql://root:password@localhost/db").await?;
///
/// // Manually-constructed options
/// let conn = MySqlConnectOptions::new()
///     .host("localhost")
///     .username("root")
///     .password("password")
///     .database("db")
///     .connect().await?;
///
/// // Modifying options parsed from a string
/// let mut opts: MySqlConnectOptions = "mysql://root:password@localhost/db".parse()?;
///
/// // Change the log verbosity level for queries.
/// // Information about SQL queries is logged at `DEBUG` level by default.
/// opts = opts.log_statements(log::LevelFilter::Trace);
///
/// let pool = MySqlPool::connect_with(opts).await?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct MySqlConnectOptions {
    pub(crate) host: String,
    pub(crate) port: u16,
    pub(crate) socket: Option<PathBuf>,
    pub(crate) username: String,
    pub(crate) password: Option<String>,
    pub(crate) database: Option<String>,
    pub(crate) ssl_mode: MySqlSslMode,
    pub(crate) ssl_ca: Option<CertificateInput>,
    pub(crate) ssl_client_cert: Option<CertificateInput>,
    pub(crate) ssl_client_key: Option<CertificateInput>,
    pub(crate) statement_cache_capacity: usize,
    pub(crate) charset: String,
    pub(crate) collation: Option<String>,
    pub(crate) log_settings: LogSettings,
    pub(crate) pipes_as_concat: bool,
    pub(crate) enable_cleartext_plugin: bool,
    pub(crate) no_engine_substitution: bool,
    pub(crate) timezone: Option<String>,
    pub(crate) set_names: bool,
}

impl Default for MySqlConnectOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl MySqlConnectOptions {
    /// Creates a new, default set of options ready for configuration
    pub fn new() -> Self {
        Self {
            port: 3306,
            host: String::from("localhost"),
            socket: None,
            username: String::from("root"),
            password: None,
            database: None,
            charset: String::from("utf8mb4"),
            collation: None,
            ssl_mode: MySqlSslMode::Preferred,
            ssl_ca: None,
            ssl_client_cert: None,
            ssl_client_key: None,
            statement_cache_capacity: 100,
            log_settings: Default::default(),
            pipes_as_concat: true,
            enable_cleartext_plugin: false,
            no_engine_substitution: true,
            timezone: Some(String::from("+00:00")),
            set_names: true,
        }
    }

    /// Sets the name of the host to connect to.
    ///
    /// The default behavior when the host is not specified,
    /// is to connect to localhost.
    pub fn host(mut self, host: &str) -> Self {
        host.clone_into(&mut self.host);
        self
    }

    /// Sets the port to connect to at the server host.
    ///
    /// The default port for MySQL is `3306`.
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Pass a path to a Unix socket. This changes the connection stream from
    /// TCP to UDS.
    ///
    /// By default set to `None`.
    pub fn socket(mut self, path: impl AsRef<Path>) -> Self {
        self.socket = Some(path.as_ref().to_path_buf());
        self
    }

    /// Sets the username to connect as.
    pub fn username(mut self, username: &str) -> Self {
        username.clone_into(&mut self.username);
        self
    }

    /// Sets the password to connect with.
    pub fn password(mut self, password: &str) -> Self {
        self.password = Some(password.to_owned());
        self
    }

    /// Sets the database name.
    pub fn database(mut self, database: &str) -> Self {
        self.database = Some(database.to_owned());
        self
    }

    /// Sets whether or with what priority a secure SSL TCP/IP connection will be negotiated
    /// with the server.
    ///
    /// By default, the SSL mode is [`Preferred`](MySqlSslMode::Preferred), and the client will
    /// first attempt an SSL connection but fallback to a non-SSL connection on failure.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use sqlx_mysql::{MySqlSslMode, MySqlConnectOptions};
    /// let options = MySqlConnectOptions::new()
    ///     .ssl_mode(MySqlSslMode::Required);
    /// ```
    pub fn ssl_mode(mut self, mode: MySqlSslMode) -> Self {
        self.ssl_mode = mode;
        self
    }

    /// Sets the name of a file containing a list of trusted SSL Certificate Authorities.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use sqlx_mysql::{MySqlSslMode, MySqlConnectOptions};
    /// let options = MySqlConnectOptions::new()
    ///     .ssl_mode(MySqlSslMode::VerifyCa)
    ///     .ssl_ca("path/to/ca.crt");
    /// ```
    pub fn ssl_ca(mut self, file_name: impl AsRef<Path>) -> Self {
        self.ssl_ca = Some(CertificateInput::File(file_name.as_ref().to_owned()));
        self
    }

    /// Sets PEM encoded list of trusted SSL Certificate Authorities.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use sqlx_mysql::{MySqlSslMode, MySqlConnectOptions};
    /// let options = MySqlConnectOptions::new()
    ///     .ssl_mode(MySqlSslMode::VerifyCa)
    ///     .ssl_ca_from_pem(vec![]);
    /// ```
    pub fn ssl_ca_from_pem(mut self, pem_certificate: Vec<u8>) -> Self {
        self.ssl_ca = Some(CertificateInput::Inline(pem_certificate));
        self
    }

    /// Sets the name of a file containing SSL client certificate.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use sqlx_mysql::{MySqlSslMode, MySqlConnectOptions};
    /// let options = MySqlConnectOptions::new()
    ///     .ssl_mode(MySqlSslMode::VerifyCa)
    ///     .ssl_client_cert("path/to/client.crt");
    /// ```
    pub fn ssl_client_cert(mut self, cert: impl AsRef<Path>) -> Self {
        self.ssl_client_cert = Some(CertificateInput::File(cert.as_ref().to_path_buf()));
        self
    }

    /// Sets the SSL client certificate as a PEM-encoded byte slice.
    ///
    /// This should be an ASCII-encoded blob that starts with `-----BEGIN CERTIFICATE-----`.
    ///
    /// # Example
    /// Note: embedding SSL certificates and keys in the binary is not advised.
    /// This is for illustration purposes only.
    ///
    /// ```rust
    /// # use sqlx_mysql::{MySqlSslMode, MySqlConnectOptions};
    ///
    /// const CERT: &[u8] = b"\
    /// -----BEGIN CERTIFICATE-----
    /// <Certificate data here.>
    /// -----END CERTIFICATE-----";
    ///
    /// let options = MySqlConnectOptions::new()
    ///     .ssl_mode(MySqlSslMode::VerifyCa)
    ///     .ssl_client_cert_from_pem(CERT);
    /// ```
    pub fn ssl_client_cert_from_pem(mut self, cert: impl AsRef<[u8]>) -> Self {
        self.ssl_client_cert = Some(CertificateInput::Inline(cert.as_ref().to_vec()));
        self
    }

    /// Sets the name of a file containing SSL client key.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use sqlx_mysql::{MySqlSslMode, MySqlConnectOptions};
    /// let options = MySqlConnectOptions::new()
    ///     .ssl_mode(MySqlSslMode::VerifyCa)
    ///     .ssl_client_key("path/to/client.key");
    /// ```
    pub fn ssl_client_key(mut self, key: impl AsRef<Path>) -> Self {
        self.ssl_client_key = Some(CertificateInput::File(key.as_ref().to_path_buf()));
        self
    }

    /// Sets the SSL client key as a PEM-encoded byte slice.
    ///
    /// This should be an ASCII-encoded blob that starts with `-----BEGIN PRIVATE KEY-----`.
    ///
    /// # Example
    /// Note: embedding SSL certificates and keys in the binary is not advised.
    /// This is for illustration purposes only.
    ///
    /// ```rust
    /// # use sqlx_mysql::{MySqlSslMode, MySqlConnectOptions};
    ///
    /// const KEY: &[u8] = b"\
    /// -----BEGIN PRIVATE KEY-----
    /// <Private key data here.>
    /// -----END PRIVATE KEY-----";
    ///
    /// let options = MySqlConnectOptions::new()
    ///     .ssl_mode(MySqlSslMode::VerifyCa)
    ///     .ssl_client_key_from_pem(KEY);
    /// ```
    pub fn ssl_client_key_from_pem(mut self, key: impl AsRef<[u8]>) -> Self {
        self.ssl_client_key = Some(CertificateInput::Inline(key.as_ref().to_vec()));
        self
    }

    /// Sets the capacity of the connection's statement cache in a number of stored
    /// distinct statements. Caching is handled using LRU, meaning when the
    /// amount of queries hits the defined limit, the oldest statement will get
    /// dropped.
    ///
    /// The default cache capacity is 100 statements.
    pub fn statement_cache_capacity(mut self, capacity: usize) -> Self {
        self.statement_cache_capacity = capacity;
        self
    }

    /// Sets the character set for the connection.
    ///
    /// The default character set is `utf8mb4`. This is supported from MySQL 5.5.3.
    /// If you need to connect to an older version, we recommend you to change this to `utf8`.
    pub fn charset(mut self, charset: &str) -> Self {
        charset.clone_into(&mut self.charset);
        self
    }

    /// Sets the collation for the connection.
    ///
    /// The default collation is derived from the `charset`. Normally, you should only have to set
    /// the `charset`.
    pub fn collation(mut self, collation: &str) -> Self {
        self.collation = Some(collation.to_owned());
        self
    }

    /// Sets the flag that enables or disables the `PIPES_AS_CONCAT` connection setting
    ///
    /// The default value is set to true, but some MySql databases such as PlanetScale
    /// error out with this connection setting so it needs to be set false in such
    /// cases.
    pub fn pipes_as_concat(mut self, flag_val: bool) -> Self {
        self.pipes_as_concat = flag_val;
        self
    }

    /// Enables mysql_clear_password plugin support.
    ///
    /// Security Note:
    /// Sending passwords as cleartext may be a security problem in some
    /// configurations. Without additional defensive configuration like
    /// ssl-mode=VERIFY_IDENTITY, an attacker can compromise a router
    /// and trick the application into divulging its credentials.
    ///
    /// It is strongly recommended to set `.ssl_mode` to `Required`,
    /// `VerifyCa`, or `VerifyIdentity` when enabling cleartext plugin.
    pub fn enable_cleartext_plugin(mut self, flag_val: bool) -> Self {
        self.enable_cleartext_plugin = flag_val;
        self
    }

    #[deprecated = "renamed to .no_engine_substitution()"]
    pub fn no_engine_subsitution(self, flag_val: bool) -> Self {
        self.no_engine_substitution(flag_val)
    }

    /// Flag that enables or disables the `NO_ENGINE_SUBSTITUTION` sql_mode setting after
    /// connection.
    ///
    /// If not set, if the available storage engine specified by a `CREATE TABLE` is not available,
    /// a warning is given and the default storage engine is used instead.
    ///
    /// By default, this is `true` (`NO_ENGINE_SUBSTITUTION` is passed, forbidding engine
    /// substitution).
    ///
    /// <https://mariadb.com/kb/en/sql-mode/>
    pub fn no_engine_substitution(mut self, flag_val: bool) -> Self {
        self.no_engine_substitution = flag_val;
        self
    }

    /// If `Some`, sets the `time_zone` option to the given string after connecting to the database.
    ///
    /// If `None`, no `time_zone` parameter is sent; the server timezone will be used instead.
    ///
    /// Defaults to `Some(String::from("+00:00"))` to ensure all timestamps are in UTC.
    ///
    /// ### Warning
    /// Changing this setting from its default will apply an unexpected skew to any
    /// `time::OffsetDateTime` or `chrono::DateTime<Utc>` value, whether passed as a parameter or
    /// decoded as a result. `TIMESTAMP` values are not encoded with their UTC offset in the MySQL
    /// protocol, so encoding and decoding of these types assumes the server timezone is *always*
    /// UTC.
    ///
    /// If you are changing this option, ensure your application only uses
    /// `time::PrimitiveDateTime` or `chrono::NaiveDateTime` and that it does not assume these
    /// timestamps can be placed on a real timeline without applying the proper offset.
    pub fn timezone(mut self, value: impl Into<Option<String>>) -> Self {
        self.timezone = value.into();
        self
    }

    /// If enabled, `SET NAMES '{charset}' COLLATE '{collation}'` is passed with the values of
    /// [`.charset()`] and [`.collation()`] after connecting to the database.
    ///
    /// This ensures the connection uses the specified character set and collation.
    ///
    /// Enabled by default.
    ///
    /// ### Warning
    /// If this is disabled and the default charset is not binary-compatible with UTF-8, query
    /// strings, column names and string values will likely not decode (or encode) correctly, which
    /// may result in unexpected errors or garbage outputs at runtime.
    ///
    /// For proper functioning, you *must* ensure the server is using a binary-compatible charset,
    /// such as ASCII or Latin-1 (ISO 8859-1), and that you do not pass any strings containing
    /// codepoints not supported by said charset.
    ///
    /// Instead of disabling this, you may also consider setting [`.charset()`] to a charset that
    /// is supported by your MySQL or MariaDB server version and compatible with UTF-8.
    pub fn set_names(mut self, flag_val: bool) -> Self {
        self.set_names = flag_val;
        self
    }
}

impl MySqlConnectOptions {
    /// Get the current host.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use sqlx_mysql::MySqlConnectOptions;
    /// let options = MySqlConnectOptions::new()
    ///     .host("127.0.0.1");
    /// assert_eq!(options.get_host(), "127.0.0.1");
    /// ```
    pub fn get_host(&self) -> &str {
        &self.host
    }

    /// Get the server's port.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use sqlx_mysql::MySqlConnectOptions;
    /// let options = MySqlConnectOptions::new()
    ///     .port(6543);
    /// assert_eq!(options.get_port(), 6543);
    /// ```
    pub fn get_port(&self) -> u16 {
        self.port
    }

    /// Get the socket path.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use sqlx_mysql::MySqlConnectOptions;
    /// let options = MySqlConnectOptions::new()
    ///     .socket("/tmp");
    /// assert!(options.get_socket().is_some());
    /// ```
    pub fn get_socket(&self) -> Option<&PathBuf> {
        self.socket.as_ref()
    }

    /// Get the current username.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use sqlx_mysql::MySqlConnectOptions;
    /// let options = MySqlConnectOptions::new()
    ///     .username("foo");
    /// assert_eq!(options.get_username(), "foo");
    /// ```
    pub fn get_username(&self) -> &str {
        &self.username
    }

    /// Get the current database name.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use sqlx_mysql::MySqlConnectOptions;
    /// let options = MySqlConnectOptions::new()
    ///     .database("postgres");
    /// assert!(options.get_database().is_some());
    /// ```
    pub fn get_database(&self) -> Option<&str> {
        self.database.as_deref()
    }

    /// Get the SSL mode.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use sqlx_mysql::{MySqlConnectOptions, MySqlSslMode};
    /// let options = MySqlConnectOptions::new();
    /// assert!(matches!(options.get_ssl_mode(), MySqlSslMode::Preferred));
    /// ```
    pub fn get_ssl_mode(&self) -> MySqlSslMode {
        self.ssl_mode
    }

    /// Get the server charset.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use sqlx_mysql::MySqlConnectOptions;
    /// let options = MySqlConnectOptions::new();
    /// assert_eq!(options.get_charset(), "utf8mb4");
    /// ```
    pub fn get_charset(&self) -> &str {
        &self.charset
    }

    /// Get the server collation.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use sqlx_mysql::MySqlConnectOptions;
    /// let options = MySqlConnectOptions::new()
    ///     .collation("collation");
    /// assert!(options.get_collation().is_some());
    /// ```
    pub fn get_collation(&self) -> Option<&str> {
        self.collation.as_deref()
    }
}

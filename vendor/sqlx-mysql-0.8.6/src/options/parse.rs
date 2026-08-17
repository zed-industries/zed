use std::str::FromStr;

use percent_encoding::{percent_decode_str, utf8_percent_encode, NON_ALPHANUMERIC};
use sqlx_core::Url;

use crate::{error::Error, MySqlSslMode};

use super::MySqlConnectOptions;

impl MySqlConnectOptions {
    pub(crate) fn parse_from_url(url: &Url) -> Result<Self, Error> {
        let mut options = Self::new();

        if let Some(host) = url.host_str() {
            options = options.host(host);
        }

        if let Some(port) = url.port() {
            options = options.port(port);
        }

        let username = url.username();
        if !username.is_empty() {
            options = options.username(
                &percent_decode_str(username)
                    .decode_utf8()
                    .map_err(Error::config)?,
            );
        }

        if let Some(password) = url.password() {
            options = options.password(
                &percent_decode_str(password)
                    .decode_utf8()
                    .map_err(Error::config)?,
            );
        }

        let path = url.path().trim_start_matches('/');
        if !path.is_empty() {
            options = options.database(
                &percent_decode_str(path)
                    .decode_utf8()
                    .map_err(Error::config)?,
            );
        }

        for (key, value) in url.query_pairs().into_iter() {
            match &*key {
                "sslmode" | "ssl-mode" => {
                    options = options.ssl_mode(value.parse().map_err(Error::config)?);
                }

                "sslca" | "ssl-ca" => {
                    options = options.ssl_ca(&*value);
                }

                "charset" => {
                    options = options.charset(&value);
                }

                "collation" => {
                    options = options.collation(&value);
                }

                "sslcert" | "ssl-cert" => options = options.ssl_client_cert(&*value),

                "sslkey" | "ssl-key" => options = options.ssl_client_key(&*value),

                "statement-cache-capacity" => {
                    options =
                        options.statement_cache_capacity(value.parse().map_err(Error::config)?);
                }

                "socket" => {
                    options = options.socket(&*value);
                }

                "timezone" | "time-zone" => {
                    options = options.timezone(Some(value.to_string()));
                }

                _ => {}
            }
        }

        Ok(options)
    }

    pub(crate) fn build_url(&self) -> Url {
        let mut url = Url::parse(&format!(
            "mysql://{}@{}:{}",
            self.username, self.host, self.port
        ))
        .expect("BUG: generated un-parseable URL");

        if let Some(password) = &self.password {
            let password = utf8_percent_encode(password, NON_ALPHANUMERIC).to_string();
            let _ = url.set_password(Some(&password));
        }

        if let Some(database) = &self.database {
            url.set_path(database);
        }

        let ssl_mode = match self.ssl_mode {
            MySqlSslMode::Disabled => "DISABLED",
            MySqlSslMode::Preferred => "PREFERRED",
            MySqlSslMode::Required => "REQUIRED",
            MySqlSslMode::VerifyCa => "VERIFY_CA",
            MySqlSslMode::VerifyIdentity => "VERIFY_IDENTITY",
        };
        url.query_pairs_mut().append_pair("ssl-mode", ssl_mode);

        if let Some(ssl_ca) = &self.ssl_ca {
            url.query_pairs_mut()
                .append_pair("ssl-ca", &ssl_ca.to_string());
        }

        url.query_pairs_mut().append_pair("charset", &self.charset);

        if let Some(collation) = &self.collation {
            url.query_pairs_mut().append_pair("charset", collation);
        }

        if let Some(ssl_client_cert) = &self.ssl_client_cert {
            url.query_pairs_mut()
                .append_pair("ssl-cert", &ssl_client_cert.to_string());
        }

        if let Some(ssl_client_key) = &self.ssl_client_key {
            url.query_pairs_mut()
                .append_pair("ssl-key", &ssl_client_key.to_string());
        }

        url.query_pairs_mut().append_pair(
            "statement-cache-capacity",
            &self.statement_cache_capacity.to_string(),
        );

        if let Some(socket) = &self.socket {
            url.query_pairs_mut()
                .append_pair("socket", &socket.to_string_lossy());
        }

        url
    }
}

impl FromStr for MySqlConnectOptions {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        let url: Url = s.parse().map_err(Error::config)?;
        Self::parse_from_url(&url)
    }
}

#[test]
fn it_parses_username_with_at_sign_correctly() {
    let url = "mysql://user@hostname:password@hostname:5432/database";
    let opts = MySqlConnectOptions::from_str(url).unwrap();

    assert_eq!("user@hostname", &opts.username);
}

#[test]
fn it_parses_password_with_non_ascii_chars_correctly() {
    let url = "mysql://username:p@ssw0rd@hostname:5432/database";
    let opts = MySqlConnectOptions::from_str(url).unwrap();

    assert_eq!(Some("p@ssw0rd".into()), opts.password);
}

#[test]
fn it_returns_the_parsed_url() {
    let url = "mysql://username:p@ssw0rd@hostname:3306/database";
    let opts = MySqlConnectOptions::from_str(url).unwrap();

    let mut expected_url = Url::parse(url).unwrap();
    // MySqlConnectOptions defaults
    let query_string = "ssl-mode=PREFERRED&charset=utf8mb4&statement-cache-capacity=100";
    expected_url.set_query(Some(query_string));

    assert_eq!(expected_url, opts.build_url());
}

#[test]
fn it_parses_timezone() {
    let opts: MySqlConnectOptions = "mysql://user:password@hostname/database?timezone=%2B08:00"
        .parse()
        .unwrap();
    assert_eq!(opts.timezone.as_deref(), Some("+08:00"));

    let opts: MySqlConnectOptions = "mysql://user:password@hostname/database?time-zone=%2B08:00"
        .parse()
        .unwrap();
    assert_eq!(opts.timezone.as_deref(), Some("+08:00"));
}

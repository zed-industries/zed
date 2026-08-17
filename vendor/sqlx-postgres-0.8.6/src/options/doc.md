Options and flags which can be used to configure a PostgreSQL connection.

A value of `PgConnectOptions` can be parsed from a connection URL,
as described by [libpq][libpq-connstring].

The general form for a connection URL is:

```text
postgresql://[user[:password]@][host][:port][/dbname][?param1=value1&...]
```

The URL scheme designator can be either `postgresql://` or `postgres://`.
Each of the URL parts is optional. For defaults, see the next section.

This type also implements [`FromStr`][std::str::FromStr] so you can parse it from a string
containing a connection URL and then further adjust options if necessary (see example below).

Note that characters not allowed in URLs must be [percent-encoded].

# Parameters

This API accepts many of the same parameters as [libpq][libpq-params];
if a parameter is not passed in via URL, it is populated by reading
[environment variables][libpq-envars] or choosing customary defaults.

| Parameter          | Environment Variable | Default / Remarks                                           |
|--------------------|----------------------|-------------------------------------------------------------|
| `user`             | `PGUSER`             | The `whoami` of the currently running process.              |
| `password`         | `PGPASSWORD`         | Read from [`passfile`], if it exists.                       |
| [`passfile`]       | `PGPASSFILE`         | `~/.pgpass` or `%APPDATA%\postgresql\pgpass.conf` (Windows) |
| `host`             | `PGHOST`             | See [Note: Default Host](#note-default-host).               |
| `hostaddr`         | `PGHOSTADDR`         | See [Note: Default Host](#note-default-host).               |
| `port`             | `PGPORT`             | `5432`                                                      |
| `dbname`           | `PGDATABASE`         | Unset; defaults to the username server-side.                |
| `sslmode`          | `PGSSLMODE`          | `prefer`. See [`PgSslMode`] for details.                    |
| `sslrootcert`      | `PGSSLROOTCERT`      | Unset. See [Note: SSL](#note-ssl).                          |
| `sslcert`          | `PGSSLCERT`          | Unset. See [Note: SSL](#note-ssl).                          |
| `sslkey`           | `PGSSLKEY`           | Unset. See [Note: SSL](#note-ssl).                          |
| `options`          | `PGOPTIONS`          | Unset.                                                      |
| `application_name` | `PGAPPNAME`          | Unset.                                                      |

[`passfile`] handling may be bypassed using [`PgConnectOptions::new_without_pgpass()`].

## SQLx-Specific
SQLx also parses some bespoke parameters. These are _not_ configurable by environment variable.
Instead, the name is linked to the method to set the value.

| Parameter                                                    | Default                       |
|--------------------------------------------------------------|-------------------------------|
| [`statement-cache-capacity`][Self::statement_cache_capacity] | `100`                         |

# Example URLs
```text
postgresql://
postgresql://:5433
postgresql://localhost
postgresql://localhost:5433
postgresql://localhost/mydb
postgresql://user@localhost
postgresql://user:secret@localhost
postgresql://user:correct%20horse%20battery%20staple@localhost
postgresql://localhost?dbname=mydb&user=postgres&password=postgres
```

See also [Note: Unix Domain Sockets](#note-unix-domain-sockets) below.

# Note: Default Host
If the connection URL does not contain a hostname and `PGHOST` is not set,
this constructor looks for an open Unix domain socket in one of a few standard locations
(configured when Postgres is built):

* `/var/run/postgresql/.s.PGSQL.{port}` (Debian)
* `/private/tmp/.s.PGSQL.{port}` (macOS when installed through Homebrew)
* `/tmp/.s.PGSQL.{port}` (default otherwise)

This depends on the value of `port` being correct.
If Postgres is using a port other than the default (`5432`), `port` must be set.

If no Unix domain socket is found, `localhost` is assumed.

Note: this description is updated on a best-effort basis.
See `default_host()` in the same source file as this method for the current behavior.

# Note: SSL
## Root Certs
If `sslrootcert` is not set, the default root certificates used depends on Cargo features:

* If `tls-native-tls` is enabled, the system root certificates are used.
* If `tls-rustls-native-roots` is enabled, the system root certificates are used.
* Otherwise, TLS roots are populated using the [`webpki-roots`] crate.

## Environment Variables
Unlike with `libpq`, the following environment variables may be _either_
a path to a file _or_ a string value containing a [PEM-encoded value][rfc7468]:

* `PGSSLROOTCERT`
* `PGSSLCERT`
* `PGSSLKEY`

If the string begins with the standard `-----BEGIN <CERTIFICATE | PRIVATE KEY>-----` header
and ends with the standard `-----END <CERTIFICATE | PRIVATE KEY>-----` footer,
it is parsed directly.

This behavior is _only_ implemented for the environment variables, not the URL parameters.

Note: passing the SSL private key via environment variable may be a security risk.

# Note: Unix Domain Sockets
If you want to connect to Postgres over a Unix domain socket, you can pass the path
to the _directory_ containing the socket as the `host` parameter.

The final path to the socket will be `{host}/.s.PGSQL.{port}` as is standard for Postgres.

If you're passing the domain socket path as the host segment of the URL, forward slashes
in the path must be [percent-encoded] (replacing `/` with `%2F`), e.g.:

```text
postgres://%2Fvar%2Frun%2Fpostgresql/dbname

Different port:
postgres://%2Fvar%2Frun%2Fpostgresql:5433/dbname

With username and password:
postgres://user:password@%2Fvar%2Frun%2Fpostgresql/dbname

With username and password, and different port:
postgres://user:password@%2Fvar%2Frun%2Fpostgresql:5432/dbname
```

Instead, the hostname can be passed in the query segment of the URL,
which does not require forward-slashes to be percent-encoded
(however, [other characters are][percent-encoded]):

```text
postgres:dbname?host=/var/run/postgresql

Different port:
postgres://:5433/dbname?host=/var/run/postgresql

With username and password:
postgres://user:password@/dbname?host=/var/run/postgresql

With username and password, and different port:
postgres://user:password@:5433/dbname?host=/var/run/postgresql
```

# Example

```rust,no_run
use sqlx::{Connection, ConnectOptions};
use sqlx::postgres::{PgConnectOptions, PgConnection, PgPool, PgSslMode};

# async fn example() -> sqlx::Result<()> {
// URL connection string
let conn = PgConnection::connect("postgres://localhost/mydb").await?;

// Manually-constructed options
let conn = PgConnectOptions::new()
    .host("secret-host")
    .port(2525)
    .username("secret-user")
    .password("secret-password")
    .ssl_mode(PgSslMode::Require)
    .connect()
    .await?;

// Modifying options parsed from a string
let mut opts: PgConnectOptions = "postgres://localhost/mydb".parse()?;

// Change the log verbosity level for queries.
// Information about SQL queries is logged at `DEBUG` level by default.
opts = opts.log_statements(log::LevelFilter::Trace);

let pool = PgPool::connect_with(opts).await?;
# Ok(())
# }
```

[percent-encoded]: https://developer.mozilla.org/en-US/docs/Glossary/Percent-encoding
[`passfile`]: https://www.postgresql.org/docs/current/libpq-pgpass.html
[libpq-connstring]: https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-CONNSTRING
[libpq-params]: https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PARAMKEYWORDS
[libpq-envars]: https://www.postgresql.org/docs/current/libpq-envars.html
[rfc7468]: https://datatracker.ietf.org/doc/html/rfc7468
[`webpki-roots`]: https://docs.rs/webpki-roots
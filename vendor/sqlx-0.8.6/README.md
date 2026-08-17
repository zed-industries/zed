<h1 align="center">SQLx</h1>
<div align="center">
 <strong>
   🧰 The Rust SQL Toolkit
 </strong>
</div>

<br />

<div align="center">
  <!-- Github Actions -->
  <a href="https://github.com/launchbadge/sqlx/actions/workflows/sqlx.yml?query=branch%3Amain">
    <img src="https://img.shields.io/github/actions/workflow/status/launchbadge/sqlx/sqlx.yml?branch=main&style=flat-square" alt="actions status" /></a>
  <!-- Version -->
  <a href="https://crates.io/crates/sqlx">
    <img src="https://img.shields.io/crates/v/sqlx.svg?style=flat-square"
    alt="Crates.io version" /></a>
  <!-- Discord -->
  <a href="https://discord.gg/uuruzJ7">
  <img src="https://img.shields.io/discord/665528275556106240?style=flat-square" alt="chat" /></a>
  <!-- Docs -->
  <a href="https://docs.rs/sqlx">
  <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/sqlx">
    <img src="https://img.shields.io/crates/d/sqlx.svg?style=flat-square" alt="Download" />
  </a>
</div>

<div align="center">
  <h4>
    <a href="#install">
      Install
    </a>
    <span> | </span>
    <a href="#usage">
      Usage
    </a>
    <span> | </span>
    <a href="https://docs.rs/sqlx">
      Docs
    </a>
    <span> | </span>
    <a href="https://github.com/launchbadge/sqlx/wiki/Ecosystem">
      Ecosystem
    </a>    
    <span> | </span>
    <a href="https://discord.gg/uuruzJ7">
      Discord
    </a>
  </h4>
</div>

<br />

<div align="center">
  <small>Built with ❤️ by <a href="https://launchbadge.com">The LaunchBadge team</a></small>
</div>

<br />

<div align="center">
    <h5>Have a question? Be sure to <a href="FAQ.md">check the FAQ first!</a></h5>
</div>

<br />

SQLx is an async, pure Rust<sub>†</sub> SQL crate featuring compile-time checked queries without a DSL.

-   **Truly Asynchronous**. Built from the ground-up using async/await for maximum concurrency.

-   **Compile-time checked queries** (if you want). See [SQLx is not an ORM](#sqlx-is-not-an-orm).

-   **Database Agnostic**. Support for [PostgreSQL], [MySQL], [MariaDB], [SQLite].
    -   [MSSQL] was supported prior to version 0.7, but has been removed pending a full rewrite of the driver as part of our [SQLx Pro initiative].

-   **Pure Rust**. The Postgres and MySQL/MariaDB drivers are written in pure Rust using **zero** unsafe<sub>††</sub> code.

-   **Runtime Agnostic**. Works on different runtimes ([`async-std`] / [`tokio`] / [`actix`]) and TLS backends ([`native-tls`], [`rustls`]).

<small><small>

† The SQLite driver uses the libsqlite3 C library as SQLite is an embedded database (the only way
we could be pure Rust for SQLite is by porting _all_ of SQLite to Rust).

†† SQLx uses `#![forbid(unsafe_code)]` unless the `sqlite` feature is enabled.
The SQLite driver directly invokes the SQLite3 API via `libsqlite3-sys`, which requires `unsafe`.

</small></small>

[postgresql]: http://postgresql.org/
[sqlite]: https://sqlite.org/
[mysql]: https://www.mysql.com/
[mariadb]: https://www.mariadb.org/
[mssql]: https://www.microsoft.com/en-us/sql-server
[SQLx Pro initiative]: https://github.com/launchbadge/sqlx/discussions/1616

---

-   Cross-platform. Being native Rust, SQLx will compile anywhere Rust is supported.

-   Built-in connection pooling with `sqlx::Pool`.

-   Row streaming. Data is read asynchronously from the database and decoded on demand.

-   Automatic statement preparation and caching. When using the high-level query API (`sqlx::query`), statements are
    prepared and cached per connection.

-   Simple (unprepared) query execution including fetching results into the same `Row` types used by
    the high-level API. Supports batch execution and returns results from all statements.

-   Transport Layer Security (TLS) where supported ([MySQL], [MariaDB] and [PostgreSQL]).

-   Asynchronous notifications using `LISTEN` and `NOTIFY` for [PostgreSQL].

-   Nested transactions with support for save points.

-   `Any` database driver for changing the database driver at runtime. An `AnyPool` connects to the driver indicated by the URL scheme.

## Install

SQLx is compatible with the [`async-std`], [`tokio`], and [`actix`] runtimes; and, the [`native-tls`] and [`rustls`] TLS backends. When adding the dependency, you must choose a runtime feature that is `runtime` + `tls`.

[`async-std`]: https://github.com/async-rs/async-std
[`tokio`]: https://github.com/tokio-rs/tokio
[`actix`]: https://github.com/actix/actix-net
[`native-tls`]: https://crates.io/crates/native-tls
[`rustls`]: https://crates.io/crates/rustls

```toml
# Cargo.toml
[dependencies]
# PICK ONE OF THE FOLLOWING:

# tokio (no TLS)
sqlx = { version = "0.8", features = [ "runtime-tokio" ] }
# tokio + native-tls
sqlx = { version = "0.8", features = [ "runtime-tokio", "tls-native-tls" ] }
# tokio + rustls with ring and WebPKI CA certificates
sqlx = { version = "0.8", features = [ "runtime-tokio", "tls-rustls-ring-webpki" ] }
# tokio + rustls with ring and platform's native CA certificates
sqlx = { version = "0.8", features = [ "runtime-tokio", "tls-rustls-ring-native-roots" ] }
# tokio + rustls with aws-lc-rs
sqlx = { version = "0.8", features = [ "runtime-tokio", "tls-rustls-aws-lc-rs" ] }

# async-std (no TLS)
sqlx = { version = "0.8", features = [ "runtime-async-std" ] }
# async-std + native-tls
sqlx = { version = "0.8", features = [ "runtime-async-std", "tls-native-tls" ] }
# async-std + rustls with ring and WebPKI CA certificates
sqlx = { version = "0.8", features = [ "runtime-async-std", "tls-rustls-ring-webpki" ] }
# async-std + rustls with ring and platform's native CA certificates
sqlx = { version = "0.8", features = [ "runtime-async-std", "tls-rustls-ring-native-roots" ] }
# async-std + rustls with aws-lc-rs
sqlx = { version = "0.8", features = [ "runtime-async-std", "tls-rustls-aws-lc-rs" ] }
```

#### Cargo Feature Flags

For backward-compatibility reasons, the runtime and TLS features can either be chosen together as a single feature,
or separately.

For forward compatibility, you should use the separate runtime and TLS features as the combination features may
be removed in the future.

-   `runtime-async-std`: Use the `async-std` runtime without enabling a TLS backend.

-   `runtime-async-std-native-tls`: Use the `async-std` runtime and `native-tls` TLS backend (SOFT-DEPRECATED).

-   `runtime-async-std-rustls`: Use the `async-std` runtime and `rustls` TLS backend (SOFT-DEPRECATED).

-   `runtime-tokio`: Use the `tokio` runtime without enabling a TLS backend.

-   `runtime-tokio-native-tls`: Use the `tokio` runtime and `native-tls` TLS backend (SOFT-DEPRECATED).

-   `runtime-tokio-rustls`: Use the `tokio` runtime and `rustls` TLS backend (SOFT-DEPRECATED).

    - Actix-web is fully compatible with Tokio and so a separate runtime feature is no longer needed.

-   `tls-native-tls`: Use the `native-tls` TLS backend (OpenSSL on *nix, SChannel on Windows, Secure Transport on macOS).

-   `tls-rustls`: Use the `rustls` TLS backend (cross-platform backend, only supports TLS 1.2 and 1.3).

-   `postgres`: Add support for the Postgres database server.

-   `mysql`: Add support for the MySQL/MariaDB database server.

-   `mssql`: Add support for the MSSQL database server.

-   `sqlite`: Add support for the self-contained [SQLite](https://sqlite.org/) database engine with SQLite bundled and statically-linked.

-   `sqlite-unbundled`: The same as above (`sqlite`), but link SQLite from the system instead of the bundled version.
    * Allows updating SQLite independently of SQLx or using forked versions.
    * You must have SQLite installed on the system or provide a path to the library at build time.
       See [the `rusqlite` README](https://github.com/rusqlite/rusqlite?tab=readme-ov-file#notes-on-building-rusqlite-and-libsqlite3-sys) for details.
    * May result in link errors if the SQLite version is too old. Version `3.20.0` or newer is recommended.
    * Can increase build time due to the use of bindgen.

-   `sqlite-preupdate-hook`: enables SQLite's [preupdate hook](https://sqlite.org/c3ref/preupdate_count.html) API.
    * Exposed as a separate feature because it's generally not enabled by default.
    * Using this feature with `sqlite-unbundled` may cause linker failures if the system SQLite version does not support it.

-   `any`: Add support for the `Any` database driver, which can proxy to a database driver at runtime.

-   `derive`: Add support for the derive family macros, those are `FromRow`, `Type`, `Encode`, `Decode`.

-   `macros`: Add support for the `query*!` macros, which allows compile-time checked queries.

-   `migrate`: Add support for the migration management and `migrate!` macro, which allow compile-time embedded migrations.

-   `uuid`: Add support for UUID.

-   `chrono`: Add support for date and time types from `chrono`.

-   `time`: Add support for date and time types from `time` crate (alternative to `chrono`, which is preferred by `query!` macro, if both enabled)

-   `bstr`: Add support for `bstr::BString`.

-   `bigdecimal`: Add support for `NUMERIC` using the `bigdecimal` crate.

-   `rust_decimal`: Add support for `NUMERIC` using the `rust_decimal` crate.

-   `ipnet`: Add support for `INET` and `CIDR` (in postgres) using the `ipnet` crate.

-   `ipnetwork`: Add support for `INET` and `CIDR` (in postgres) using the `ipnetwork` crate.

-   `json`: Add support for `JSON` and `JSONB` (in postgres) using the `serde_json` crate.

-   Offline mode is now always enabled. See [sqlx-cli/README.md][readme-offline].

[readme-offline]: sqlx-cli/README.md#enable-building-in-offline-mode-with-query

## SQLx is not an ORM!

SQLx supports **compile-time checked queries**. It does not, however, do this by providing a Rust
API or DSL (domain-specific language) for building queries. Instead, it provides macros that take
regular SQL as input and ensure that it is valid for your database. The way this works is that
SQLx connects to your development DB at compile time to have the database itself verify (and return
some info on) your SQL queries. This has some potentially surprising implications:

- Since SQLx never has to parse the SQL string itself, any syntax that the development DB accepts
  can be used (including things added by database extensions)
- Due to the different amount of information databases let you retrieve about queries, the extent of
  SQL verification you get from the query macros depends on the database

**If you are looking for an (asynchronous) ORM,** you can check out our new [Ecosystem wiki page](https://github.com/launchbadge/sqlx/wiki/Ecosystem#orms)!

[`ormx`]: https://crates.io/crates/ormx
[`SeaORM`]: https://github.com/SeaQL/sea-orm
## Usage

See the `examples/` folder for more in-depth usage.

### Quickstart

```rust
use sqlx::postgres::PgPoolOptions;
// use sqlx::mysql::MySqlPoolOptions;
// etc.

#[async_std::main] // Requires the `attributes` feature of `async-std`
// or #[tokio::main]
// or #[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
    // Create a connection pool
    //  for MySQL/MariaDB, use MySqlPoolOptions::new()
    //  for SQLite, use SqlitePoolOptions::new()
    //  etc.
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost/test").await?;

    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL/MariaDB)
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool).await?;

    assert_eq!(row.0, 150);

    Ok(())
}
```


### Connecting

A single connection can be established using any of the database connection types and calling `connect()`.

```rust
use sqlx::Connection;

let conn = SqliteConnection::connect("sqlite::memory:").await?;
```

Generally, you will want to instead create a connection pool (`sqlx::Pool`) for the application to
regulate how many server-side connections it's using.

```rust
let pool = MySqlPool::connect("mysql://user:pass@host/database").await?;
```

### Querying

In SQL, queries can be separated into prepared (parameterized) or unprepared (simple). Prepared queries have their
query plan _cached_, use a binary mode of communication (lower bandwidth and faster decoding), and utilize parameters
to avoid SQL injection. Unprepared queries are simple and intended only for use where a prepared statement
will not work, such as various database commands (e.g., `PRAGMA` or `SET` or `BEGIN`).

SQLx supports all operations with both types of queries. In SQLx, a `&str` is treated as an unprepared query,
and a `Query` or `QueryAs` struct is treated as a prepared query.

```rust
// low-level, Executor trait
conn.execute("BEGIN").await?; // unprepared, simple query
conn.execute(sqlx::query("DELETE FROM table")).await?; // prepared, cached query
```

We should prefer to use the high-level `query` interface whenever possible. To make this easier, there are finalizers
on the type to avoid the need to wrap with an executor.

```rust
sqlx::query("DELETE FROM table").execute(&mut conn).await?;
sqlx::query("DELETE FROM table").execute(&pool).await?;
```

The `execute` query finalizer returns the number of affected rows, if any, and drops all received results.
In addition, there are `fetch`, `fetch_one`, `fetch_optional`, and `fetch_all` to receive results.

The `Query` type returned from `sqlx::query` will return `Row<'conn>` from the database. Column values can be accessed
by ordinal or by name with `row.get()`. As the `Row` retains an immutable borrow on the connection, only one
`Row` may exist at a time.

The `fetch` query finalizer returns a stream-like type that iterates through the rows in the result sets.

```rust
// provides `try_next`
use futures::TryStreamExt;
// provides `try_get`
use sqlx::Row;

let mut rows = sqlx::query("SELECT * FROM users WHERE email = ?")
    .bind(email)
    .fetch(&mut conn);

while let Some(row) = rows.try_next().await? {
    // map the row into a user-defined domain type
    let email: &str = row.try_get("email")?;
}
```

To assist with mapping the row into a domain type, one of two idioms may be used:

```rust
let mut stream = sqlx::query("SELECT * FROM users")
    .map(|row: PgRow| {
        // map the row into a user-defined domain type
    })
    .fetch(&mut conn);
```

```rust
#[derive(sqlx::FromRow)]
struct User { name: String, id: i64 }

let mut stream = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ? OR name = ?")
    .bind(user_email)
    .bind(user_name)
    .fetch(&mut conn);
```

Instead of a stream of results, we can use `fetch_one` or `fetch_optional` to request one required or optional result
from the database.

### Compile-time verification

We can use the macro, `sqlx::query!` to achieve compile-time syntactic and semantic verification of the SQL, with
an output to an anonymous record type where each SQL column is a Rust field (using raw identifiers where needed).

```rust
let countries = sqlx::query!(
        "
SELECT country, COUNT(*) as count
FROM users
GROUP BY country
WHERE organization = ?
        ",
        organization
    )
    .fetch_all(&pool) // -> Vec<{ country: String, count: i64 }>
    .await?;

// countries[0].country
// countries[0].count
```

Differences from `query()`:

-   The input (or bind) parameters must be given all at once (and they are compile-time validated to be
    the right number and the right type).

-   The output type is an anonymous record. In the above example the type would be similar to:

    ```rust
    { country: String, count: i64 }
    ```

-   The `DATABASE_URL` environment variable must be set at build time to a database which it can prepare
    queries against; the database does not have to contain any data but must be the same
    kind (MySQL, Postgres, etc.) and have the same schema as the database you will be connecting to at runtime.

    For convenience, you can use [a `.env` file][dotenv]<sup>1</sup> to set DATABASE_URL so that you don't have to pass it every time:

    ```
    DATABASE_URL=mysql://localhost/my_database
    ```

[dotenv]: https://github.com/dotenv-rs/dotenv#examples

The biggest downside to `query!()` is that the output type cannot be named (due to Rust not
officially supporting anonymous records). To address that, there is a `query_as!()` macro that is
mostly identical except that you can name the output type.

```rust
// no traits are needed
struct Country { country: String, count: i64 }

let countries = sqlx::query_as!(Country,
        "
SELECT country, COUNT(*) as count
FROM users
GROUP BY country
WHERE organization = ?
        ",
        organization
    )
    .fetch_all(&pool) // -> Vec<Country>
    .await?;

// countries[0].country
// countries[0].count
```

To avoid the need of having a development database around to compile the project even when no
modifications (to the database-accessing parts of the code) are done, you can enable "offline mode"
to cache the results of the SQL query analysis using the `sqlx` command-line tool. See
[sqlx-cli/README.md](./sqlx-cli/README.md#enable-building-in-offline-mode-with-query).

Compile-time verified queries do quite a bit of work at compile time. Incremental actions like
`cargo check` and `cargo build` can be significantly faster when using an optimized build by
putting the following in your `Cargo.toml` (More information in the
[Profiles section](https://doc.rust-lang.org/cargo/reference/profiles.html) of The Cargo Book)

```toml
[profile.dev.package.sqlx-macros]
opt-level = 3
```

<sup>1</sup> The `dotenv` crate itself appears abandoned as of [December 2021](https://github.com/dotenv-rs/dotenv/issues/74)
so we now use the `dotenvy` crate instead. The file format is the same.

## Safety

This crate uses `#![forbid(unsafe_code)]` to ensure everything is implemented in 100% Safe Rust.

If the `sqlite` feature is enabled, this is downgraded to `#![deny(unsafe_code)]` with `#![allow(unsafe_code)]` on the
`sqlx::sqlite` module. There are several places where we interact with the C SQLite API. We try to document each call for the invariants we're assuming. We absolutely welcome auditing of, and feedback on, our unsafe code usage.

## License

Licensed under either of

-   Apache License, Version 2.0
    ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
-   MIT license
    ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any Contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## 0.32.7 - 2025-08-06

### Enhancements

* Added `ValueType::is_option`

### Bug Fixes

* Fix incorrect casting of `ChronoDateTimeWithTimeZone` in `Value::Array` https://github.com/SeaQL/sea-query/pull/933
* Add missing parenthesis to `WINDOW` clause https://github.com/SeaQL/sea-query/pull/919
```sql
SELECT .. OVER "w" FROM "character" WINDOW "w" AS (PARTITION BY "ww")
```
* Fix serializing iden as a value in `ALTER TYPE ... RENAME TO ...` statements https://github.com/SeaQL/sea-query/pull/924
```sql
ALTER TYPE "font" RENAME TO "typeface"
```
* Fixed the issue where milliseconds were truncated when formatting `Value::Constant` https://github.com/SeaQL/sea-query/pull/929
```sql
'2025-01-01 00:00:00.000000'
                    ^^^^^^^
```

## 0.32.6 - 2025-05-27

### Enhancements

* impl `From<Condition>` and `From<ConditionExpression>` for `SimpleExpr` https://github.com/SeaQL/sea-query/pull/886

## 0.32.5 - 2025-05-07

### New features

* Support for creating functional indexes in PostgreSQL and MySQL https://github.com/SeaQL/sea-query/pull/869

### Enhancements

* Make `RcOrArc` a documented type alias instead of a direct reexport https://github.com/SeaQL/sea-query/pull/875
* Impl `Iden` for `&'static str` (don't wrap strings in `Alias::new`) https://github.com/SeaQL/sea-query/pull/882

## 0.32.4 - 2025-04-17

### New Features

* Added support for temporary tables https://github.com/SeaQL/sea-query/pull/878
```rust
let statement = Table::create()
    .table(Font::Table)
    .temporary()
    .col(
        ColumnDef::new(Font::Id)
            .integer()
            .not_null()
            .primary_key()
            .auto_increment()
    )
    .col(ColumnDef::new(Font::Name).string().not_null())
    .take();

assert_eq!(
    statement.to_string(MysqlQueryBuilder),
    [
        "CREATE TEMPORARY TABLE `font` (",
        "`id` int NOT NULL PRIMARY KEY AUTO_INCREMENT,",
        "`name` varchar(255) NOT NULL",
        ")",
    ]
    .join(" ")
);
```
* Added `Value::dummy_value`
```rust
use sea_query::Value;
let v = Value::Int(None);
let n = v.dummy_value();
assert_eq!(n, Value::Int(Some(0)));
```

### Bug Fixes

* Quote type properly in `AsEnum` casting https://github.com/SeaQL/sea-query/pull/880
```rust
let query = Query::select()
    .expr(Expr::col(Char::FontSize).as_enum(TextArray))
    .from(Char::Table)
    .to_owned();

assert_eq!(
    query.to_string(PostgresQueryBuilder),
    r#"SELECT CAST("font_size" AS "text"[]) FROM "character""#
);
```

## 0.32.3 - 2025-03-16

### New Features

* Support `Update FROM ..` https://github.com/SeaQL/sea-query/pull/861
```rust
let query = Query::update()
    .table(Glyph::Table)
    .value(Glyph::Tokens, Expr::column((Char::Table, Char::Character)))
    .from(Char::Table)
    .cond_where(
        Expr::col((Glyph::Table, Glyph::Image))
            .eq(Expr::col((Char::Table, Char::UserData))),
    )
    .to_owned();

assert_eq!(
    query.to_string(PostgresQueryBuilder),
    r#"UPDATE "glyph" SET "tokens" = "character"."character" FROM "character" WHERE "glyph"."image" = "character"."user_data""#
);
assert_eq!(
    query.to_string(SqliteQueryBuilder),
    r#"UPDATE "glyph" SET "tokens" = "character"."character" FROM "character" WHERE "glyph"."image" = "character"."user_data""#
);
```
* Support `TABLESAMPLE` (Postgres) https://github.com/SeaQL/sea-query/pull/865
```rust
use sea_query::extension::postgres::PostgresSelectStatementExt;

let query = Query::select()
    .columns([Glyph::Image])
    .from(Glyph::Table)
    .table_sample(SampleMethod::SYSTEM, 50.0, None)
    .to_owned();

assert_eq!(
    query.to_string(PostgresQueryBuilder),
    r#"SELECT "image" FROM "glyph" TABLESAMPLE SYSTEM (50)"#
);
```
* Support `ALTER COLUMN USING ..` (Postgres) https://github.com/SeaQL/sea-query/pull/848
```rust
let table = Table::alter()
    .table(Char::Table)
    .modify_column(
        ColumnDef::new(Char::Id)
            .integer()
            .using(Expr::col(Char::Id).cast_as(Alias::new("integer"))),
    )
    .to_owned();

assert_eq!(
    table.to_string(PostgresQueryBuilder),
    [
        r#"ALTER TABLE "character""#,
        r#"ALTER COLUMN "id" TYPE integer USING CAST("id" AS integer)"#,
    ]
    .join(" ")
);
```

### House Keeping

* Updated `ordered-float` to `4`
* Updated `thiserror` to `2`

## 0.32.2 - 2025-02-18

### New Features

* Added `with_cte` to use `WITH` clauses in all statements https://github.com/SeaQL/sea-query/pull/859
```rust
let select = SelectStatement::new()
    .columns([Glyph::Id, Glyph::Image, Glyph::Aspect])
    .from(Glyph::Table)
    .to_owned();
let cte = CommonTableExpression::new()
    .query(select)
    .table_name(Alias::new("cte"))
    .to_owned();
let select = SelectStatement::new()
    .columns([Glyph::Id, Glyph::Image, Glyph::Aspect])
    .from(Alias::new("cte"))
    .with_cte(cte)
    .to_owned();
assert_eq!(
    select.to_string(PostgresQueryBuilder),
    [
        r#"WITH "cte" AS"#,
        r#"(SELECT "id", "image", "aspect""#,
        r#"FROM "glyph")"#,
        r#"SELECT "id", "image", "aspect" FROM "cte""#,
    ]
    .join(" ")
);
```

### Enhancements

* Added `Expr::column` https://github.com/SeaQL/sea-query/pull/852
* Added Postgres function `DATE_TRUNC` https://github.com/SeaQL/sea-query/pull/825
* Added `INCLUDE` clause for Postgres BTree index https://github.com/SeaQL/sea-query/pull/826

### Bug Fixes

* Write empty Postgres array as '{}' https://github.com/SeaQL/sea-query/pull/854

## 0.32.1 - 2024-12-01

### New Features

* Added `Value::as_null`
```rust
let v = Value::Int(Some(2));
let n = v.as_null();

assert_eq!(n, Value::Int(None));
```
* Added bitwise and/or operators (`bit_and`, `bit_or`) https://github.com/SeaQL/sea-query/pull/841
```rust
let query = Query::select()
    .expr(1.bit_and(2).eq(3))
    .to_owned();

assert_eq!(
    query.to_string(PostgresQueryBuilder),
    r#"SELECT (1 & 2) = 3"#
);
```

### Enhancements

* Added `GREATEST` & `LEAST` function https://github.com/SeaQL/sea-query/pull/844
* Added `ValueType::enum_type_name()` https://github.com/SeaQL/sea-query/pull/836
* Removed "one common table" restriction on recursive CTE https://github.com/SeaQL/sea-query/pull/835

### House keeping

* Remove unnecessary string hashes https://github.com/SeaQL/sea-query/pull/815

## 0.32.0 - 2024-10-17

### Releases

#### 2024-08-09

+ `sea-query`/`0.32.0-rc.1`
+ `sea-query-binder`/`0.7.0-rc.1`
+ `sea-query-binder`/`0.7.0-rc.2`
+ `sea-query-rusqlite`/`0.7.0-rc.1`
+ `sea-query-postgres`/`0.5.0-rc.1`

#### 2024-10-05

+ `sea-query`/`0.32.0-rc.2`
+ `sea-query-attr`/`0.1.3`
+ `sea-query-derive`/`0.4.2`
+ `sea-query-rusqlite`/`0.7.0-rc.2`

### New Features

* Construct Postgres query with vector extension https://github.com/SeaQL/sea-query/pull/774
    * Added `postgres-vector` feature flag
    * Added `Value::Vector`, `ColumnType::Vector`, `ColumnDef::vector()`, `PgBinOper::EuclideanDistance`, `PgBinOper::NegativeInnerProduct` and `PgBinOper::CosineDistance`
    ```rust
    assert_eq!(
        Query::select()
            .columns([Char::Character])
            .from(Char::Table)
            .and_where(
                Expr::col(Char::Character).eq(Expr::val(pgvector::Vector::from(vec![1.0, 2.0])))
            )
            .to_string(PostgresQueryBuilder),
        r#"SELECT "character" FROM "character" WHERE "character" = '[1,2]'"#
    );
    ```
* Added `ExprTrait` to unify `Expr` and `SimpleExpr` methods https://github.com/SeaQL/sea-query/pull/791
* Support partial index `CREATE INDEX .. WHERE ..` https://github.com/SeaQL/sea-query/pull/478

### Enhancements

* Replace `Educe` with manual implementations https://github.com/SeaQL/sea-query/pull/817

#### `sea-query-derive`

* Merged `#[enum_def]` into `sea-query-derive`
* `#[enum_def]` now impl additional `IdenStatic` and `AsRef<str>` https://github.com/SeaQL/sea-query/pull/769

#### `sea-query-attr`

* Updated `syn`, `heck` and `darling`
* `sea-query-attr` is now deprecated

### Upgrades

* Upgrade `sqlx` to `0.8` https://github.com/SeaQL/sea-query/pull/798
* Upgrade `bigdecimal` to `0.4` https://github.com/SeaQL/sea-query/pull/798
* Upgrade `rusqlite` to `0.32` https://github.com/SeaQL/sea-query/pull/802

## 0.31.1 - 2024-10-05

### Enhancements

* Derive `Eq`, `Ord`, `Hash` for `Alias` https://github.com/SeaQL/sea-query/pull/818
* Added `Func::md5` function https://github.com/SeaQL/sea-query/pull/786
* Added Postgres Json functions `JSON_BUILD_OBJECT` and `JSON_AGG` https://github.com/SeaQL/sea-query/pull/787
* Added Postgres function `ARRAY_AGG` https://github.com/SeaQL/sea-query/pull/846
* Added `Func::cast_as_quoted` https://github.com/SeaQL/sea-query/pull/789
* Added `IF NOT EXISTS` to `ALTER TYPE ADD VALUE` https://github.com/SeaQL/sea-query/pull/803

## 0.31.0 - 2024-08-02

### Versions

+ `sea-query`/`0.31.0-rc.1`: 2024-01-31
+ `sea-query`/`0.31.0-rc.4`: 2024-02-02
+ `sea-query`/`0.31.0-rc.5`: 2024-04-14
+ `sea-query`/`0.31.0-rc.6`: 2024-05-03
+ `sea-query`/`0.31.0-rc.7`: 2024-06-02
+ `sea-query`/`0.31.0-rc.8`: 2024-06-19
+ `sea-query-binder`/`0.6.0-rc.1`: 2024-01-31
+ `sea-query-binder`/`0.6.0-rc.2`: 2024-04-14
+ `sea-query-binder`/`0.6.0-rc.3`: 2024-06-19
+ `sea-query-binder`/`0.6.0-rc.4`: 2024-06-25
+ `sea-query-binder`/`0.6.0`: 2024-08-02
+ `sea-query-rusqlite`/`0.6.0-rc.1`: 2024-02-19
+ `sea-query-rusqlite`/`0.6.0`: 2024-08-02
+ `sea-query-attr`/`0.1.2`: 2024-04-14
+ `sea-query-diesel`/`0.2.0`: 2024-08-02

### New Features

* Added `table_name` attribute to `enum_def` macro https://github.com/SeaQL/sea-query/pull/759
* Added `ColumnType::Blob` https://github.com/SeaQL/sea-query/pull/777

### Breaking Changes

* Rework SQLite type mapping https://github.com/SeaQL/sea-query/pull/735
```rust
assert_eq!(
    Table::create()
        .table(Alias::new("strange"))
        .col(ColumnDef::new(Alias::new("id")).integer().not_null().auto_increment().primary_key())
        .col(ColumnDef::new(Alias::new("int1")).integer())
        .col(ColumnDef::new(Alias::new("int2")).tiny_integer())
        .col(ColumnDef::new(Alias::new("int3")).small_integer())
        .col(ColumnDef::new(Alias::new("int4")).big_integer())
        .col(ColumnDef::new(Alias::new("string1")).string())
        .col(ColumnDef::new(Alias::new("string2")).string_len(24))
        .col(ColumnDef::new(Alias::new("char1")).char())
        .col(ColumnDef::new(Alias::new("char2")).char_len(24))
        .col(ColumnDef::new(Alias::new("text_col")).text())
        .col(ColumnDef::new(Alias::new("json_col")).json())
        .col(ColumnDef::new(Alias::new("uuid_col")).uuid())
        .col(ColumnDef::new(Alias::new("decimal1")).decimal())
        .col(ColumnDef::new(Alias::new("decimal2")).decimal_len(12, 4))
        .col(ColumnDef::new(Alias::new("money1")).money())
        .col(ColumnDef::new(Alias::new("money2")).money_len(12, 4))
        .col(ColumnDef::new(Alias::new("float_col")).float())
        .col(ColumnDef::new(Alias::new("double_col")).double())
        .col(ColumnDef::new(Alias::new("date_col")).date())
        .col(ColumnDef::new(Alias::new("time_col")).time())
        .col(ColumnDef::new(Alias::new("datetime_col")).date_time())
        .col(ColumnDef::new(Alias::new("boolean_col")).boolean())
        .col(ColumnDef::new(Alias::new("binary2")).binary_len(1024))
        .col(ColumnDef::new(Alias::new("binary3")).var_binary(1024))
        .col(ColumnDef::new(Alias::new("binary4")).blob())
        .to_string(SqliteQueryBuilder),
    [
        r#"CREATE TABLE "strange" ( "id" integer NOT NULL PRIMARY KEY AUTOINCREMENT,"#,
            r#""int1" integer,"#,
            r#""int2" tinyint,"#,
            r#""int3" smallint,"#,
            r#""int4" bigint,"#,
            r#""string1" varchar,"#,
            r#""string2" varchar(24),"#,
            r#""char1" char,"#,
            r#""char2" char(24),"#,
            r#""text_col" text,"#,
            r#""json_col" json_text,"#,
            r#""uuid_col" uuid_text,"#,
            r#""decimal1" real,"#,
            r#""decimal2" real(12, 4),"#,
            r#""money1" real_money,"#,
            r#""money2" real_money(12, 4),"#,
            r#""float_col" float,"#,
            r#""double_col" double,"#,
            r#""date_col" date_text,"#,
            r#""time_col" time_text,"#,
            r#""datetime_col" datetime_text,"#,
            r#""boolean_col" boolean,"#,
            r#""binary2" blob(1024),"#,
            r#""binary3" varbinary_blob(1024),"#,
            r#""binary4" blob"#,
        r#")"#,
    ]
    .join(" ")
);
```
* MySQL money type maps to decimal
* MySQL blob types moved to `sea_query::extension::mysql::MySqlType`; `ColumnDef::blob()` now takes no parameters
```rust
assert_eq!(
    Table::create()
        .table(BinaryType::Table)
        .col(ColumnDef::new(BinaryType::BinaryLen).binary_len(32))
        .col(ColumnDef::new(BinaryType::Binary).binary())
        .col(ColumnDef::new(BinaryType::Blob).blob())
        .col(ColumnDef::new(BinaryType::TinyBlob).custom(MySqlType::TinyBlob))
        .col(ColumnDef::new(BinaryType::MediumBlob).custom(MySqlType::MediumBlob))
        .col(ColumnDef::new(BinaryType::LongBlob).custom(MySqlType::LongBlob))
        .to_string(MysqlQueryBuilder),
    [
        "CREATE TABLE `binary_type` (",
            "`binlen` binary(32),",
            "`bin` binary(1),",
            "`b` blob,",
            "`tb` tinyblob,",
            "`mb` mediumblob,",
            "`lb` longblob",
        ")",
    ]
    .join(" ")
);
```
* `ColumnDef::binary()` set column type as binary with default length of 1
* Removed `BlobSize` enum
* Added `StringLen` to represent length of var-char/binary
```rust
/// Length for var-char/binary; default to 255
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum StringLen {
    /// String size
    N(u32),
    Max,
    #[default]
    None,
}
```
* `ValueType::columntype()` of `Vec<u8>` maps to `VarBinary(StringLen::None)`
* `ValueType::columntype()` of `String` maps to `String(StringLen::None)`
* `ColumnType::Bit` maps to `bit` for Postgres
* `ColumnType::Binary` and `ColumnType::VarBinary` map to `bytea` for Postgres
* `Value::Decimal` and `Value::BigDecimal` bind as `real` for SQLite
* `ColumnType::Year(Option<MySqlYear>)` changed to `ColumnType::Year`

### Enhancements

* Added `IntoColumnDef` trait, allowing `&mut ColumnDef` / `ColumnDef` as argument
* Added `ColumnType::string()` and `ColumnType::var_binary()` as shim for old API
* Added `ON DUPLICATE KEY DO NOTHING` polyfill for MySQL https://github.com/SeaQL/sea-query/pull/765
* Added non-TLS runtime https://github.com/SeaQL/sea-query/pull/783

### House keeping

* Added `ColumnType` mapping documentation
* Replace `derivative` with `educe` https://github.com/SeaQL/sea-query/pull/763

### Upgrades

* Upgrade `rusqlite` to `0.31` https://github.com/SeaQL/sea-query/pull/755
* Upgrade `time` to `0.3.36` https://github.com/SeaQL/sea-query/pull/788

## 0.30.8 - Pending

### Enhancements

* Added `InsertStatement::values_from_panic` https://github.com/SeaQL/sea-query/pull/739

## 0.30.7 - 2024-01-12

### Enhancements

* Added `SelectStatement::apply` https://github.com/SeaQL/sea-query/pull/730

### House keeping

* Slight refactors and documentation update

## 0.30.6 - 2024-01-01

### House keeping

* Fix clippy warnings on Rust 1.75 https://github.com/SeaQL/sea-query/pull/729

## `sea-query-rusqlite` 0.5.0 - 2023-12-29

* Upgrade `rusqlite` to `0.30` https://github.com/SeaQL/sea-query/pull/728

## 0.30.5 - 2023-12-14

### New Features

* Added feature flag `option-more-parentheses` to have more parentheses in expressions https://github.com/SeaQL/sea-query/pull/723
* Added feature flag `option-sqlite-exact-column-type` to only use `integer` for SQLite
* Support `COUNT(DISTINCT "column")` https://github.com/SeaQL/sea-query/pull/700
* Support index hints for MySQL (via `extension::mysql::MySqlSelectStatementExt`) https://github.com/SeaQL/sea-query/pull/636
* Support expressions for `ON CONFLICT` targets https://github.com/SeaQL/sea-query/pull/692

### Enhancements

* Add `from_clear` to allow emptying current from tables in select statement https://github.com/SeaQL/sea-query/pull/716

### Breaking Changes

* Caution: do not use the `--all-features` param in Cargo. If you want to enable all features, use the `all-features` feature flag instead.

## 0.30.4 - 2023-12-01

### Enhancements

* Impl `QueryStatementWriter` as inherent methods for `WithQuery`

## 0.30.3 - 2023-11-22

### New Features

* Added `LTree` column type https://github.com/SeaQL/sea-query/pull/604
* Improved parenthesis omission logic https://github.com/SeaQL/sea-query/pull/675

### Bug Fixes

* Fixed `BIGINT PRIMARY KEY AUTOINCREMENT` for SQLite https://github.com/SeaQL/sea-query/issues/689

### Upgrades

* Upgrade `chrono` to `0.4.27`

### Breaking changes

* Removed `ToTokens` for `PgInterval` https://github.com/SeaQL/sea-query/pull/710

### `sea-query-derive` 0.4.1 - 2023-10-19

* Upgrade `syn` to `2`

## 0.30.2 - 2023-09-23

+ [`sea-query-diesel`/`0.1.0`](https://crates.io/crates/sea-query-diesel/0.1.0)

### Bug Fixes

* Fixed incorrect behavior when adding an autoincrement column for Postgres https://github.com/SeaQL/sea-query/pull/697

### Enhancements

* Make `ValueTuple` hashable

## 0.30.1 - 2023-08-25

### Versions

+ [`sea-query-postgres`/`0.4.0`](https://crates.io/crates/sea-query-postgres/0.4.0)
+ [`sea-query-rusqlite`/`0.4.0`](https://crates.io/crates/sea-query-rusqlite/0.4.0)
+ [`sea-query-rbatis`/`0.1.0`](https://crates.io/crates/sea-query-rbatis/0.1.0)

### New Features

* Added `Func::round` and `Func::round_with_precision` https://github.com/SeaQL/sea-query/pull/671

### Enhancements

* Added some getters to `FunctionCall` https://github.com/SeaQL/sea-query/pull/677

### Bug Fixes

* Fixed bytea literal syntax for Postgres https://github.com/SeaQL/sea-query/pull/666
* Fixed issues with semantics of parenthesis removal https://github.com/SeaQL/sea-query/pull/675
* Fixed behaviour in `FunctionCall` when calling `arg` multiple times https://github.com/SeaQL/sea-query/pull/687

### Breaking changes

* As part of https://github.com/SeaQL/sea-query/pull/687, calling `FunctionCall::arg` multiple times will append the arguments instead of replacing old values

## 0.30.0 - 2023-07-20

This is a small (but major) upgrade, the only changes are:

* Upgrade [SQLx to `0.7`](https://github.com/launchbadge/sqlx/blob/main/CHANGELOG.md#071---2023-07-14) https://github.com/SeaQL/sea-query/pull/652
* Upgrade ipnetwork to `0.20`

### Versions

+ [`sea-query-binder`/`0.5.0`](https://crates.io/crates/sea-query-binder/0.5.0)

### Notes

`sea-query 0.29` has a number of breaking changes, so it might be easier for you to first upgrade to `0.29`, then upgrade `sqlx` by bumping to `0.30`.

## 0.29.1 - 2023-07-12

### Versions

+ `sea-query`/`0.29.0-rc.1`: 2023-03-22
+ `sea-query`/`0.29.0` (Yanked)
+ `sea-query`/`0.29.1`: 2023-07-12
+ [`sea-query-binder`/`0.4.0`](https://crates.io/crates/sea-query-binder/0.4.0)
+ [`sea-query-postgres`/`0.3.0`](https://crates.io/crates/sea-query-postgres/0.3.0)
+ [`sea-query-rusqlite`/`0.3.0`](https://crates.io/crates/sea-query-rusqlite/0.3.0)

### New Features

* Added `ValueTuple::Many` for tuple with length up to 12 https://github.com/SeaQL/sea-query/pull/564
* Added `CREATE TABLE CHECK` constraints https://github.com/SeaQL/sea-query/pull/567
* Added support generated column spec https://github.com/SeaQL/sea-query/pull/581
* Added `BIT_AND`, `BIT_OR` functions https://github.com/SeaQL/sea-query/pull/582
* Added implementation `SqlxBinder`, `RusqliteBinder` and `PostgresBinder` for `WithQuery` https://github.com/SeaQL/sea-query/pull/580
* Added new type `Asteriks` https://github.com/SeaQL/sea-query/pull/596
* Added `IF NOT EXISTS` for `DROP INDEX` in Postgres and Sqlite https://github.com/SeaQL/sea-query/pull/610
* Added `->` and `->>` operators for Postgres https://github.com/SeaQL/sea-query/pull/617
* Added `TableCreateStatement::set_extra` and `TableCreateStatement::get_extra` https://github.com/SeaQL/sea-query/pull/611
* Added `TableCreateStatement::comment` and `ColumnDef::comment` for MySQL comments https://github.com/SeaQL/sea-query/pull/622
* Added `PgExpr::get_json_field` and `PgExpr::cast_json_field` methods for constructing Postgres JSON expressions https://github.com/SeaQL/sea-query/pull/630
* Added `PgBinOper::Regex` and `PgBinOper::RegexCaseInsensitive` for Postgres Regex operators
* Added `BinOper::Custom` for defining custom binary operators
* Added `GLOB` operator for Sqlite https://github.com/SeaQL/sea-query/pull/651/
* Added `CREATE or DROP EXTENSION` statements for Postgres https://github.com/SeaQL/sea-query/pull/616
* Added a feature flag `hashable-value`, which will `impl Hash for Value`; when enabled, `Value::Float(NaN) == Value::Float(NaN)` would be true https://github.com/SeaQL/sea-query/pull/598
* Added `PgBinOper::Overlap` for Postgres operators https://github.com/SeaQL/sea-query/pull/653

### Enhancements

* Implemented `PartialEq` for `DynIden`, `SimpleExpr` and related types https://github.com/SeaQL/sea-query/pull/620

### Breaking changes

* Removed variants `Four, Five, Six` from `enum ValueTuple` as part of https://github.com/SeaQL/sea-query/pull/564
* Removed `Expr::tbl`, `Expr::greater_than`, `Expr::greater_or_equal`, `Expr::less_than`, `Expr::less_or_equal`, `Expr::into_simple_expr` https://github.com/SeaQL/sea-query/pull/551
* Removed `SimpleExpr::equals` and `SimpleExpr::not_equals` https://github.com/SeaQL/sea-query/pull/551
* Removed `InsertStatement::exprs`, `InsertStatement::exprs_panic` https://github.com/SeaQL/sea-query/pull/551
* Removed `OnConflict::update_value`, `OnConflict::update_values`, `OnConflict::update_expr`, `OnConflict::update_exprs` https://github.com/SeaQL/sea-query/pull/551
* Removed `UpdateStatement::exprs`, `UpdateStatement::col_expr`, `UpdateStatement::value_expr` https://github.com/SeaQL/sea-query/pull/551
* `BigInteger` now maps to `bigint` instead of `integer` on SQLite https://github.com/SeaQL/sea-query/pull/556
* `Table::truncate` now panic for Sqlite https://github.com/SeaQL/sea-query/pull/590
* Deprecated `Expr::asteriks` and `Expr::table_asteriks` https://github.com/SeaQL/sea-query/pull/596
* `Expr::cust`, `Expr::cust_with_values`, `Expr::cust_with_expr`, `Expr::cust_with_exprs`, `TableForeignKey::name`, `ForeignKeyCreateStatement::name`, `ForeignKeyDropStatement::name`, `TableIndex::name`, `IndexCreateStatement::name`, `IndexDropStatement::name`, `SqlWriterValues::new`, `ColumnType::custom`, `TableCreateStatement::engine`, `TableCreateStatement::collate`, `TableCreateStatement::character_set`, `TableRef::new`, `LikeExpr::str` now accept `T: Into<String>` https://github.com/SeaQL/sea-query/pull/594
* `OnConflict::values` and `OnConflict::update_columns` will append the new values keeping the old values intact instead of erasing them https://github.com/SeaQL/sea-query/pull/609
* As part of https://github.com/SeaQL/sea-query/pull/620, `SeaRc` now becomes a wrapper type.
If you used `SeaRc` for something other than `dyn Iden`, you now have to use `RcOrArc`.
However be reminded that it is not an intended use of the API anyway.
```rust
// new definition
struct SeaRc<I>(RcOrArc<I>);
// remains unchanged
type DynIden = SeaRc<dyn Iden>;

// if you did:
let _: DynIden = Rc::new(Alias::new("char"));
// replace with:
let _: DynIden = SeaRc::new(Alias::new("char"));
```
* Added new type `Quote` and changed the `Iden` trait:
```rust
struct Quote(pub(crate) u8, pub(crate) u8);

trait Iden {
    // then:
    fn prepare(&self, s: &mut dyn fmt::Write, q: char);
    // now:
    fn prepare(&self, s: &mut dyn fmt::Write, q: Quote);

    // then:
    fn quoted(&self, q: char) -> String;
    // now:
    fn quoted(&self, q: Quote) -> String;
}
```

### House keeping

* Elided unnecessary lifetimes https://github.com/SeaQL/sea-query/pull/552
* Changed all `version = "^x.y.z"` into `version = "x.y.z"` in all Cargo.toml https://github.com/SeaQL/sea-query/pull/547/
* Disabled default features and enable only the needed ones https://github.com/SeaQL/sea-query/pull/547/
* `tests_cfg` module is available only if you enabled `tests-cfg` feature https://github.com/SeaQL/sea-query/pull/584
* Removed hard coded quotes https://github.com/SeaQL/sea-query/pull/613
* Enabled required `syn` v1 features https://github.com/SeaQL/sea-query/pull/624
* Fix macro hygiene (`any!` / `all!`) https://github.com/SeaQL/sea-query/pull/639 https://github.com/SeaQL/sea-query/pull/640

### Bug fixes

* `ALTER TABLE` now panic if has multiple column for Sqlite https://github.com/SeaQL/sea-query/pull/595
* Fixed alter primary key column statements for Postgres https://github.com/SeaQL/sea-query/pull/646

## 0.28.5 - 2023-05-11

* Added implementation `SqlxBinder`, `RusqliteBinder` and `PostgresBinder` for `WithQuery` https://github.com/SeaQL/sea-query/pull/580
    * `sea-query-binder` `0.3.1`
    * `sea-query-postgres` `0.2.1`
    * `sea-query-rusqlite` `0.2.1`

## 0.28.4 - 2023-04-11

### Bug fixes

* Fix quoted string bug while inserting array of strings to Postgres https://github.com/SeaQL/sea-query/pull/576
* Added comma if multiple names are passed to `TypeDropStatement` https://github.com/SeaQL/sea-query/pull/623

## 0.28.3 - 2023-01-18

### Enhancements

* Added getter for the `UpdateStatement::values` field https://github.com/SeaQL/sea-query/pull/578
* Implements `PartialEq` for `ColumnType` https://github.com/SeaQL/sea-query/pull/579
* Added helper function to construct `ColumnType::Custom` https://github.com/SeaQL/sea-query/pull/579

## 0.28.2 - 2023-01-04

### Enhancements

* Added `Cow<str>` conversion to `Value` https://github.com/SeaQL/sea-query/pull/550
* Added convert various `UUID` defined in `uuid::fmt` module into `sea_query::Value::Uuid` https://github.com/SeaQL/sea-query/pull/563

## 0.28.1 - 2022-12-29

### Bug fixes

* Fixes Postgres `GEN_RANDOM_UUID` https://github.com/SeaQL/sea-query/issues/568

## 0.28.0 - 2022-12-09

### New Features

* New struct `FunctionCall` which hold function and arguments https://github.com/SeaQL/sea-query/pull/475
* New trait `IdenStatic` with method `fn as_str(&self) -> &'static str` https://github.com/SeaQL/sea-query/pull/508
* New traits `PgExpr` and `SqliteExpr` for custom expressions https://github.com/SeaQL/sea-query/pull/519
* Support `BigDecimal`, `IpNetwork` and `MacAddress` for `sea-query-postgres` https://github.com/SeaQL/sea-query/pull/503

### API Additions

* Added `SelectStatement::from_function` https://github.com/SeaQL/sea-query/pull/475
* Added binary operators from the Postgres `pg_trgm` extension https://github.com/SeaQL/sea-query/pull/486
* Added `ILIKE` and `NOT ILIKE` operators https://github.com/SeaQL/sea-query/pull/473
* Added the `mul` and `div` methods for `SimpleExpr` https://github.com/SeaQL/sea-query/pull/510
* Added the `MATCH`, `->` and `->>` operators for SQLite https://github.com/SeaQL/sea-query/pull/513
* Added the `FULL OUTER JOIN` https://github.com/SeaQL/sea-query/pull/497
* Added `PgFunc::get_random_uuid` https://github.com/SeaQL/sea-query/pull/530
* Added `SimpleExpr::eq`, `SimpleExpr::ne`, `Expr::not_equals` https://github.com/SeaQL/sea-query/pull/528
* Added `PgFunc::starts_with` https://github.com/SeaQL/sea-query/pull/529
* Added `Expr::custom_keyword` and `SimpleExpr::not` https://github.com/SeaQL/sea-query/pull/535
* Added `SimpleExpr::like`, `SimpleExpr::not_like` and `Expr::cast_as` https://github.com/SeaQL/sea-query/pull/539
* Added support for `NULLS NOT DISTINCT` clause for Postgres https://github.com/SeaQL/sea-query/pull/532
* Added `Expr::cust_with_expr` and `Expr::cust_with_exprs` https://github.com/SeaQL/sea-query/pull/531
* Added support for converting `&String` to Value https://github.com/SeaQL/sea-query/issues/537

### Enhancements

* Made `value::with_array` module public and therefore making `NotU8` trait public https://github.com/SeaQL/sea-query/pull/511
* Drop the `Sized` requirement on implementers of `SchemaBuilders` https://github.com/SeaQL/sea-query/pull/524

### Bug fixes

* Wrap unions into parenthesis https://github.com/SeaQL/sea-query/pull/498
* Syntax error on empty condition https://github.com/SeaQL/sea-query/pull/505
```rust

// given
let (statement, values) = sea_query::Query::select()
    .column(Glyph::Id)
    .from(Glyph::Table)
    .cond_where(Cond::any()
        .add(Cond::all()) // empty all() => TRUE
        .add(Cond::any()) // empty any() => FALSE
    )
    .build(sea_query::MysqlQueryBuilder);

// old behavior
assert_eq!(statement, r#"SELECT `id` FROM `glyph`"#);

// new behavior
assert_eq!(
    statement,
    r#"SELECT `id` FROM `glyph` WHERE (TRUE) OR (FALSE)"#
);
```

### Breaking changes

* MSRV is up to 1.62 https://github.com/SeaQL/sea-query/pull/535
* `ColumnType::Array` definition changed from `Array(SeaRc<Box<ColumnType>>)` to `Array(SeaRc<ColumnType>)` https://github.com/SeaQL/sea-query/pull/492
* `Func::*` now returns `FunctionCall` instead of `SimpleExpr` https://github.com/SeaQL/sea-query/pull/475
* `Func::coalesce` now accepts `IntoIterator<Item = SimpleExpr>` instead of `IntoIterator<Item = Into<SimpleExpr>` https://github.com/SeaQL/sea-query/pull/475
* Removed `Expr::arg` and `Expr::args` - these functions are no longer needed https://github.com/SeaQL/sea-query/pull/475
* Moved all Postgres specific operators to `PgBinOper` https://github.com/SeaQL/sea-query/pull/507
* `Expr::value`, `Expr::gt`, `Expr::gte`, `Expr::lt`, `Expr::lte`, `Expr::add`, `Expr::div`, `Expr::sub`, `Expr::modulo`, `Expr::left_shift`, `Expr::right_shift`, `Expr::between`, `Expr::not_between`, `Expr::is`, `Expr::is_not`, `Expr::if_null` now accepts `Into<SimpleExpr>` instead of `Into<Value>` https://github.com/SeaQL/sea-query/pull/476
* `Expr::is_in`, `Expr::is_not_in` now accepts `Into<SimpleExpr>` instead of `Into<Value>` and convert it to `SimpleExpr::Tuple` instead of `SimpleExpr::Values` https://github.com/SeaQL/sea-query/pull/476
* `Expr::expr` now accepts `Into<SimpleExpr>` instead of `SimpleExpr` https://github.com/SeaQL/sea-query/pull/475
* Moved `Expr::ilike`, `Expr::not_ilike`, `Expr::matches`, `Expr::contains`, `Expr::contained`, `Expr::concatenate`, `Expr::concat`, `SimpleExpr::concatenate` and `SimpleExpr::concat` to new trait `PgExpr` https://github.com/SeaQL/sea-query/pull/519
* `Expr::equals` now accepts `C: IntoColumnRef` instead of `T: IntoIden, C: IntoIden` https://github.com/SeaQL/sea-query/pull/528
* Removed integer and date time column types' display length / precision option https://github.com/SeaQL/sea-query/pull/525

### Deprecations

* Deprecated `Expr::greater_than`, `Expr::greater_or_equal`, `Expr::less_than` and `Expr::less_or_equal` https://github.com/SeaQL/sea-query/pull/476
* Deprecated `SimpleExpr::equals`, `SimpleExpr::not_equals` https://github.com/SeaQL/sea-query/pull/528
* Deprecated `Expr::tbl`, please use `Expr::col` with a tuple https://github.com/SeaQL/sea-query/pull/540

### House keeping

* Replace `impl Default` with `#[derive(Default)]` https://github.com/SeaQL/sea-query/pull/535
* Exclude `sqlx` default features https://github.com/SeaQL/sea-query/pull/543
* Use `dtolnay/rust-toolchain` instead of `actions-rs/toolchain` in `CI` https://github.com/SeaQL/sea-query/pull/544

## 0.27.2 - 2022-11-14

* Made `value::with_array` module public and therefore making `NotU8` trait public https://github.com/SeaQL/sea-query/pull/511

## sea-query-binder 0.2.2

* Enable SQLx features only if SQLx optional dependency is enabled https://github.com/SeaQL/sea-query/pull/517

## 0.27.1 - 2022-10-18

* Fix consecutive spacing on schema statements https://github.com/SeaQL/sea-query/pull/481
* SQLite bind `rust_decimal` & `bigdecimal` as f64 https://github.com/SeaQL/sea-query/pull/480

## 0.27.0 - 2022-10-16

### New Features

* Support `CROSS JOIN` https://github.com/SeaQL/sea-query/pull/376
* We are going through series of changes to how database drivers work
(https://github.com/SeaQL/sea-query/pull/416, https://github.com/SeaQL/sea-query/pull/423):
	1. `sea-query-binder` is now the recommended way (trait based) of working with SQLx, replacing `sea-query-driver` (macro based) https://github.com/SeaQL/sea-query/pull/434
	2. `sea-query-binder` is now a separate dependency, instead of integrated with `sea-query` https://github.com/SeaQL/sea-query/pull/432
	3. `rusqlite` support is moved to `sea-query-rusqlite` https://github.com/SeaQL/sea-query/pull/422
	4. `postgres` support is moved to `sea-query-postgres` https://github.com/SeaQL/sea-query/pull/433
* Added sub-query operators: `EXISTS`, `ALL`, `ANY`, `SOME` https://github.com/SeaQL/sea-query/pull/379
* Added support to `ON CONFLICT WHERE` https://github.com/SeaQL/sea-query/pull/447
* Added support `DROP COLUMN` for SQLite https://github.com/SeaQL/sea-query/pull/455
* Added `YEAR`, `BIT` and `VARBIT` types https://github.com/SeaQL/sea-query/pull/466
* Added support one dimension Postgres array for SQLx https://github.com/SeaQL/sea-query/pull/467

### Enhancements

* Handle Postgres schema name for schema statements https://github.com/SeaQL/sea-query/pull/385
* Added `%`, `<<` and `>>` binary operators https://github.com/SeaQL/sea-query/pull/419
* Added `RAND` function https://github.com/SeaQL/sea-query/pull/430
* Implements `Display` for `Value` https://github.com/SeaQL/sea-query/pull/425
* Added `INTERSECT` and `EXCEPT` to `UnionType` https://github.com/SeaQL/sea-query/pull/438
* Added `OnConflict::value` and `OnConflict::values` https://github.com/SeaQL/sea-query/issues/451
* `ColumnDef::default` now accepts both `Value` and `SimpleExpr` https://github.com/SeaQL/sea-query/pull/436
* `OrderedStatement::order_by_customs`, `OrderedStatement::order_by_columns`, `OverStatement::partition_by_customs`, `OverStatement::partition_by_columns` now accepts `IntoIterator<Item = T>` instead of `Vec<T>` https://github.com/SeaQL/sea-query/pull/448
* `Expr::case`, `CaseStatement::case` and `CaseStatement::finally` now accepts `Into<SimpleExpr>` instead of `Into<Expr>` https://github.com/SeaQL/sea-query/pull/460
* `UpdateStatement::value` now accept `Into<SimpleExpr>` instead of `Into<Value>` https://github.com/SeaQL/sea-query/pull/460
* `TableAlterStatement::rename_column`, `TableAlterStatement::drop_column`, `ColumnDef::new`, `ColumnDef::new_with_type` now accepts `IntoIden` instead of `Iden` https://github.com/SeaQL/sea-query/pull/472

### Bug Fixes

* `distinct_on` properly handles `ColumnRef` https://github.com/SeaQL/sea-query/pull/450
* Removed `ON` for `DROP INDEX` for SQLite https://github.com/SeaQL/sea-query/pull/462
* Change datetime string format to include microseconds https://github.com/SeaQL/sea-query/pull/468
* `ALTER TABLE` for PosgreSQL with `UNIQUE` constraint https://github.com/SeaQL/sea-query/pull/472

### Breaking changes

* Changed `in_tuples` interface to accept `IntoValueTuple` https://github.com/SeaQL/sea-query/pull/386
* Removed deprecated methods (`or_where`, `or_having`, `table_column` etc) https://github.com/SeaQL/sea-query/pull/380
* **Changed `cond_where` chaining semantics** https://github.com/SeaQL/sea-query/pull/417
```rust
// Before: will extend current Condition
assert_eq!(
    Query::select()
        .cond_where(any![Expr::col(Glyph::Id).eq(1), Expr::col(Glyph::Id).eq(2)])
        .cond_where(Expr::col(Glyph::Id).eq(3))
        .to_owned()
        .to_string(PostgresQueryBuilder),
    r#"SELECT WHERE "id" = 1 OR "id" = 2 OR "id" = 3"#
);
// Before: confusing, since it depends on the order of invocation:
assert_eq!(
    Query::select()
        .cond_where(Expr::col(Glyph::Id).eq(3))
        .cond_where(any![Expr::col(Glyph::Id).eq(1), Expr::col(Glyph::Id).eq(2)])
        .to_owned()
        .to_string(PostgresQueryBuilder),
    r#"SELECT WHERE "id" = 3 AND ("id" = 1 OR "id" = 2)"#
);
// Now: will always conjoin with `AND`
assert_eq!(
    Query::select()
        .cond_where(Expr::col(Glyph::Id).eq(1))
        .cond_where(any![Expr::col(Glyph::Id).eq(2), Expr::col(Glyph::Id).eq(3)])
        .to_owned()
        .to_string(PostgresQueryBuilder),
    r#"SELECT WHERE "id" = 1 AND ("id" = 2 OR "id" = 3)"#
);
// Now: so they are now equivalent
assert_eq!(
    Query::select()
        .cond_where(any![Expr::col(Glyph::Id).eq(2), Expr::col(Glyph::Id).eq(3)])
        .cond_where(Expr::col(Glyph::Id).eq(1))
        .to_owned()
        .to_string(PostgresQueryBuilder),
    r#"SELECT WHERE ("id" = 2 OR "id" = 3) AND "id" = 1"#
);
```
* `CURRENT_TIMESTAMP` changed from being a function to keyword https://github.com/SeaQL/sea-query/pull/441
* Update SQLite `boolean` type from `integer` to `boolean` https://github.com/SeaQL/sea-query/pull/400
* Changed type of `ColumnType::Enum` from `(String, Vec<String>)` to: https://github.com/SeaQL/sea-query/pull/435
```rust
Enum {
    name: DynIden,
    variants: Vec<DynIden>,
}
```
* Deprecated `InsertStatement::exprs`, `InsertStatement::exprs_panic`, `OnConflict::update_value`, `OnConflict::update_values`, `OnConflict::update_expr`, `OnConflict::update_exprs`, `UpdateStatement::col_expr`, `UpdateStatement::value_expr`, `UpdateStatement::exprs` https://github.com/SeaQL/sea-query/pull/460
* `InsertStatement::values`, `UpdateStatement::values` now accepts `IntoIterator<Item = SimpleExpr>` instead of `IntoIterator<Item = Value>` https://github.com/SeaQL/sea-query/pull/460
* Use native api from SQLx for SQLite to work with `time` https://github.com/SeaQL/sea-query/pull/412

### House keeping

* Cleanup `IndexBuilder` trait methods https://github.com/SeaQL/sea-query/pull/426
* Introduce `SqlWriter` trait https://github.com/SeaQL/sea-query/pull/436
* Remove unneeded `vec!` from examples https://github.com/SeaQL/sea-query/pull/448

### Upgrades

* Upgrade `sqlx` driver to 0.6.1

## 0.26.4 - 2022-10-13

### New Features

* Added support `DROP COLUMN` for SQLite https://github.com/SeaQL/sea-query/pull/455

### Bug Fixes

* Removed `ON` for `DROP INDEX` for SQLite https://github.com/SeaQL/sea-query/pull/462
* Changed datetime display format to include microseconds https://github.com/SeaQL/sea-query/pull/468

## 0.26.3 - 2022-08-18

### Bug Fixes

* `DROP NOT NULL` for Postgres `ALTER COLUMN` https://github.com/SeaQL/sea-query/pull/394

### House keeping

* Exclude `chrono` default-features https://github.com/SeaQL/sea-query/pull/410
* Fix clippy warnings https://github.com/SeaQL/sea-query/pull/415

## 0.26.2 - 2022-07-21

### Bug Fixes

* Rename `postgres-*` features to `with-*` on `postgres` driver https://github.com/SeaQL/sea-query/pull/377

## 0.26.0 - 2022-07-02

### New Features

* Add support for `VALUES` lists https://github.com/SeaQL/sea-query/pull/351
* Introduce `sea-query-binder` https://github.com/SeaQL/sea-query/pull/275
* Convert from `IpNetwork` and `MacAddress` to `Value` https://github.com/SeaQL/sea-query/pull/364

### Enhancements

* Move `escape` and `unescape` string to backend https://github.com/SeaQL/sea-query/pull/306
* `LIKE ESCAPE` support https://github.com/SeaQL/sea-query/pull/352 #353)
* `clear_order_by` for `OrderedStatement`
* Add method to make a column nullable https://github.com/SeaQL/sea-query/pull/365
* Add `is` & `is_not` to Expr https://github.com/SeaQL/sea-query/pull/348
* Add `CURRENT_TIMESTAMP` function https://github.com/SeaQL/sea-query/pull/349
* Add `in_tuples` method to Expr https://github.com/SeaQL/sea-query/pull/345

### Upgrades

* Upgrade `uuid` to 1.0
* Upgrade `time` to 0.3
* Upgrade `ipnetwork` to 0.19
* Upgrade `bigdecimal` to 0.3
* Upgrade `sqlx` driver to 0.6

### Breaking changes

* As part of #306, the standalone functions `escape_string` and `unescape_string` are removed, and becomes backend specific. So now, you have to:

```rust
use sea_query::EscapeBuilder;

let string: String = MySqlQueryBuilder.escape_string(r#" "abc" "#);
let string: String = MysqlQueryBuilder.unescape_string(r#" \"abc\" "#);
```

* Replace `Value::Ipv4Network` and `Value::Ipv6Network`  to `Value::IpNetwork` https://github.com/SeaQL/sea-query/pull/364

* Remove some redundant feature flags `postgres-chrono`, `postgres-json`, `postgres-uuid`, `postgres-time`. Use the `with-*` equivalence

**Full Changelog**: https://github.com/SeaQL/sea-query/compare/0.25.0...0.26.0

## 0.25.2 - 2022-07-01

### New features

* Introduce `sea-query-binder` https://github.com/SeaQL/sea-query/pull/275

### Enhancements

* Add method to make a column nullable https://github.com/SeaQL/sea-query/pull/365
* Add `is` & `is_not` to Expr https://github.com/SeaQL/sea-query/pull/348
* Add `CURRENT_TIMESTAMP` function https://github.com/SeaQL/sea-query/pull/349

## 0.25.1 - 2022-06-26

### Enhancements

* `clear_order_by` for `OrderedStatement`

## 0.25.0 - 2022-05-28

### New Features

* CASE WHEN statement support https://github.com/SeaQL/sea-query/pull/304
* Add support for Ip(4,6)Network and MacAddress https://github.com/SeaQL/sea-query/pull/309
* [sea-query-attr] macro for deriving `Iden` enum from struct https://github.com/SeaQL/sea-query/pull/300
* Add ability to alter foreign keys https://github.com/SeaQL/sea-query/pull/299
* Select `DISTINCT ON` https://github.com/SeaQL/sea-query/pull/313

### Enhancements

* Insert Default https://github.com/SeaQL/sea-query/pull/266
* Make `sea-query-driver` an optional dependency https://github.com/SeaQL/sea-query/pull/324
* Add `ABS` function https://github.com/SeaQL/sea-query/pull/334
* Support `IF NOT EXISTS` when create index https://github.com/SeaQL/sea-query/pull/332
* Support different `blob` types in MySQL https://github.com/SeaQL/sea-query/pull/314
* Add `VarBinary` column type https://github.com/SeaQL/sea-query/pull/331
* Returning expression supporting `SimpleExpr` https://github.com/SeaQL/sea-query/pull/335

### Bug fixes

* Fix arguments when nesting custom expressions https://github.com/SeaQL/sea-query/pull/333
* Fix clippy warnings for manual map https://github.com/SeaQL/sea-query/pull/337

### Breaking Changes

* Introducing a dedicated `ReturningClause` instead of reusing `Select` on `returning`: https://github.com/SeaQL/sea-query/pull/317

```rust
.returning(Query::select().column(Glyph::Id).take()) // before
.returning(Query::returning().columns([Glyph::Id])) // now
```

* In #333, the custom expression API changed for Postgres, users should change their placeholder from `?` to Postgres's `$N`

```rust
let query = Query::select()
    .columns([Char::Character, Char::SizeW, Char::SizeH])
    .from(Char::Table)
    .and_where(Expr::col(Char::Id).eq(1))
    .and_where(Expr::cust_with_values("6 = $2 * $1", vec![3, 2]).into())
    .to_owned();

assert_eq!(
    query.to_string(PostgresQueryBuilder),
    r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "id" = 1 AND 6 = 2 * 3"#
);
```

As a side effect, `??` is no longer needed for escaping `?`

```rust
let query = Query::select()
    .expr(Expr::cust_with_values(
        "data @? ($1::JSONPATH)",
        vec!["hello"],
    ))
    .to_owned();

assert_eq!(
    query.to_string(PostgresQueryBuilder),
    r#"SELECT data @? ('hello'::JSONPATH)"#
);
```

* In #314, `ColumnType`'s `Binary(Option<u32>)` changed to `Binary(BlobSize)`, so if you used `Binary(None)` before, you should change to `Binary(BlobSize::Blob(None))`

**Full Changelog**: https://github.com/SeaQL/sea-query/compare/0.24.0...0.25.0

## 0.24.6 - 2022-05-12

* Make `sea-query-driver` an optional dependency https://github.com/SeaQL/sea-query/pull/324

## 0.24.5 - 2022-05-09

* Insert `or_default_values` https://github.com/SeaQL/sea-query/pull/266

## 0.24.4 - 2022-04-26

* update sea-query-driver

## 0.24.3 - 2022-04-26

### Bug fixes

* Fix MySQL index create statement https://github.com/SeaQL/sea-query/pull/308

### Enhancements

* Add length check on condition array https://github.com/SeaQL/sea-query/pull/307

## 0.24.2 - 2022-04-18

### Bug fixes

* Fixed https://github.com/SeaQL/sea-query/issues/303 driver breakage in 0.24.0

Notes: 0.24.0 & 0.24.1 were yanked

## 0.24.1 - 2022-04-15

### Enhancements

* #295 Add parameter for SQLx path to proc-macro https://github.com/SeaQL/sea-query/pull/297

### Bug fixes

* CTE optional columns https://github.com/SeaQL/sea-query/pull/301

## 0.24.0 - 2022-04-05

### New Features

* Add `LOWER` and `UPPER` func https://github.com/SeaQL/sea-query/pull/276
* Insert `ON CONFLICT` support https://github.com/SeaQL/sea-query/pull/279
* #174 Add support for `WINDOWS` statement https://github.com/SeaQL/sea-query/pull/271
* #142 full support lock in select https://github.com/SeaQL/sea-query/pull/289
* #269 add support for postgres `ANY`, `SOME`, `ALL` https://github.com/SeaQL/sea-query/pull/283

### Enhancements

* Add support for multiple `ALTER` operations https://github.com/SeaQL/sea-query/pull/277
* #229 add column if not exists https://github.com/SeaQL/sea-query/pull/278
* #255 Add support to CommonTableExpression columns method https://github.com/SeaQL/sea-query/pull/284
* #280 Rewrite drivers using proc-macro https://github.com/SeaQL/sea-query/pull/292

### Bug fixes

* #285 Fix timestamp_with_time_zone_len https://github.com/SeaQL/sea-query/pull/286

### Breaking changes

* The enum variants for `LockType` were renamed: `Exclusive` -> `Update` and `Shared` -> `Share`
* As part of #283, the drivers are split to the `sea-query-driver` crate
    1. Remove methods `Value::is_json` and `Value::as_ref_json` when feature: **with-json** is disabled
    2. Remove methods `Value::is_time_*` and `Value::as_ref_time_*` when feature: **with-time** is disabled
    3. Remove methods `Value::is_chrono_*` and `Value::as_ref_chrono*` when feature: **with-chrono** is disabled
    4. Remove methods `Value::is_decimal`, `Value::as_ref_decimal` and `Value::decimal_to_f64` when feature: **with-rust_decimal** is disabled
    5. Remove methods `Value::is_big_decimal`, `Value::as_ref_big_decimal` and `Value::big_decimal_to_f64` when feature: **with-bigdecimal** is disabled
    6. Remove methods `Value::is_uuid` and `Value::as_ref_uuid` when feature: **with-uuid** is disabled
    7. Remove methods `Value::is_array` and `Value::as_ref_array` when feature: **postgres-array** is disabled

**Full Changelog**: https://github.com/SeaQL/sea-query/compare/0.23.0...0.24.0

## 0.23.0 - 2022-03-15

### New Features

* Supports `time` in addition to `chrono` https://github.com/SeaQL/sea-query/pull/267

### Enhancements

* Allow for trailing commas in any and all macros https://github.com/SeaQL/sea-query/pull/270

### Bug fixes

* Fix UNIQUE table index expression syntax for sqlite https://github.com/SeaQL/sea-query/pull/227

### Breaking changes

In order to co-exist with the `time` crate, `Date`, `Time`, `DateTime` etc are renamed to `ChronoDate`, `ChronoTime`, `ChronoDateTime`. In addition, new variants `TimeDate`, `TimeTime`, `TimeDateTime` and so on are introduced to `Value`.

## 0.22.0 - 2022-02-26

### New Features

* Support multiple tables in the select from by @Sytten in https://github.com/SeaQL/sea-query/pull/261
* Add support for replace insert by @Sytten in https://github.com/SeaQL/sea-query/pull/262
* Add `ColumnType` unsigned integer types by @billy1624 in https://github.com/SeaQL/sea-query/pull/211

**Full Changelog**: https://github.com/SeaQL/sea-query/compare/0.21.0...0.22.0

## 0.21.0 - 2022-02-01

### Breaking Changes

* Use double quotes for quoting identifiers for SQLite by @SpyrosRoum in https://github.com/SeaQL/sea-query/pull/221

### New Features

* Implement `RETURNING` for SQLite by @SpyrosRoum in https://github.com/SeaQL/sea-query/pull/194
* Support 'NULLS LAST' and 'NULLS FIRST' by @qyihua in https://github.com/SeaQL/sea-query/pull/210
* [join-lateral]  by @rex-remind101 in https://github.com/SeaQL/sea-query/pull/224
* Insert from select by @05storm26 in https://github.com/SeaQL/sea-query/pull/238
* Add Expr::asterisk() and Expr::tbl_asterisk(table: DynIden) methods - Fix #217 by @RomainMazB in https://github.com/SeaQL/sea-query/pull/219

### Enhancements

* Implement ToTokens for IntervalField by @autarch in https://github.com/SeaQL/sea-query/pull/195
* Implemented 'Array' type for Postgres. by @kev0960 in https://github.com/SeaQL/sea-query/pull/205
* Add `Value::DateTimeLocal` by @billy1624 in https://github.com/SeaQL/sea-query/pull/249
* Add `ColumnRef::SchemaTableColumn` by @billy1624 in https://github.com/SeaQL/sea-query/pull/206
* Datetime utc by @tyt2y3 in https://github.com/SeaQL/sea-query/pull/241
* Support the use of chrono::DateTime<Utc> using the type alias DateTim… by @charleschege in https://github.com/SeaQL/sea-query/pull/222

### Bug Fixes

* Fix Postgres `ColumnType::TinyInteger` mapping by @billy1624 in https://github.com/SeaQL/sea-query/pull/207
* PR without clippy warmings in file changed tab by @billy1624 in https://github.com/SeaQL/sea-query/pull/212

**Full Changelog**: https://github.com/SeaQL/sea-query/compare/0.20.0...0.21.0

## 0.20.0 - 2021-12-11

### Merged PRs

* Add `TableRef::DatabaseSchemaTable` by @billy1624 in https://github.com/SeaQL/sea-query/pull/193

**Full Changelog**: https://github.com/SeaQL/sea-query/compare/0.19.4...0.20.0

## 0.19.4 - 2021-12-11

### Merged PRs

* Binding `DateTime<FixedOffset>` for SQLx MySQL & SQLite by @billy1624 in https://github.com/SeaQL/sea-query/pull/197

**Full Changelog**: https://github.com/SeaQL/sea-query/compare/0.19.2...0.19.4

## 0.19.2 - 2021-12-04

### Merged PRs

* Impl `ValueTuple` Up to Six by @billy1624 in https://github.com/SeaQL/sea-query/pull/200
* Basic Benchmark by @tyt2y3 in https://github.com/SeaQL/sea-query/pull/192

**Full Changelog**: https://github.com/SeaQL/sea-query/compare/0.19.1...0.19.2

## 0.19.1 - 2021-11-25

### Merged PRs
* `driver/postgres` handle non-exhaustive `Value` by @billy1624 in https://github.com/SeaQL/sea-query/pull/191

**Full Changelog**: https://github.com/SeaQL/sea-query/compare/0.19.0...0.19.1

## 0.19.0 - 2021-11-19

### Merged PRs
* `TableCreateStatement` and `TableDropStatement` takes any `IntoTableRef` table name. by @josh-codes in https://github.com/SeaQL/sea-query/pull/186
* Add `ColumnType::Enum` by @billy1624 in https://github.com/SeaQL/sea-query/pull/188
* Update to Rust Edition 2021 by @billy1624 in https://github.com/SeaQL/sea-query/pull/189

**Full Changelog**: https://github.com/SeaQL/sea-query/compare/0.18.2...0.19.0

## 0.18.2 - 2021-11-04

### Merged PRs
* Rename "where" keywords in `SelectStatement` to suppress IDEA warnings by @baoyachi in https://github.com/SeaQL/sea-query/pull/166
* Add binary method to expr by @Progdrasil in https://github.com/SeaQL/sea-query/pull/173
* Cast expression as custom type by @billy1624 in https://github.com/SeaQL/sea-query/pull/170
* Support tuple expression by @shuoli84 in https://github.com/SeaQL/sea-query/pull/178

**Full Changelog**: https://github.com/SeaQL/sea-query/compare/0.18.1...0.18.2

## 0.18.1 - 2021-10-26

+ [[#169]] Add support for Postgres interval type
+ [[#171]] Fix bug in `Condition::add` where Condition negation is ignored

[#169]: https://github.com/SeaQL/sea-query/pull/169
[#171]: https://github.com/SeaQL/sea-query/pull/171

## 0.18.0 - 2021-10-15

+ [[#159]] Add `ValueType::column_type`
+ [[#160]] Add `FromValueTuple` trait

[#159]: https://github.com/SeaQL/sea-query/pull/159
[#160]: https://github.com/SeaQL/sea-query/pull/160

## 0.17.3 - 2021-10-26

+ [[#171]] Fix bug in `Condition::add` where Condition negation is ignored

[#171]: https://github.com/SeaQL/sea-query/pull/171

## 0.17.2 - 2021-10-15

+ [[#164]] Revert "Fix SQLite `chrono::NaiveTime` binding"

[#164]: https://github.com/SeaQL/sea-query/pull/164

## 0.17.1 - 2021-10-12 (yanked)

## 0.17.0 - 2021-10-06

+ [[#157]] Fix binding nullable custom types on db drivers

The `as_ref_*` methods on `Value` are changed:

```rust
pub fn as_ref_json(&self) -> &Json;
```

Is now

```rust
pub fn as_ref_json(&self) -> Option<&Json>;
```

[#157]: https://github.com/SeaQL/sea-query/pull/157

## 0.16.6 - 2021-10-26

+ [[#171]] Fix bug in `Condition::add` where Condition negation is ignored

[#171]: https://github.com/SeaQL/sea-query/pull/171

## 0.16.5 - 2021-09-30

+ [[#145]] Add Condition::not
+ [[#149]] Fix SQLite `chrono::NaiveTime` binding

[#145]: https://github.com/SeaQL/sea-query/pull/145
[#149]: https://github.com/SeaQL/sea-query/pull/149

## 0.16.4 - 2021-09-26

+ Fix table drop options for SQLite
+ Add `IndexCreateStatement::is_unique_key()`

## 0.16.3 - 2021-09-17

+ [[#131]] `CAST AS` expression
+ [[#131]] `InsertStatement` accepts `SimpleExpr`
+ [[#137]] SQLx Postgres driver bind `DateTime<FixedOffset>`

[#131]: https://github.com/SeaQL/sea-query/issues/131
[#137]: https://github.com/SeaQL/sea-query/pull/137

## 0.16.2 - 2021-09-15

+ [[#120]] Support `RETURNING` for `DeleteStatement`
+ [[#128]] Support `UNION` clause for `SelectStatement`

[#120]: https://github.com/SeaQL/sea-query/issues/120
[#128]: https://github.com/SeaQL/sea-query/pull/128

## 0.16.1 - 2021-09-10

+ [[#129]] MySql `ColumnType::Binary(None)` maps to "blob"

[#129]: https://github.com/SeaQL/sea-query/pull/129

## 0.16.0 - 2021-09-02

+ [[#112]] Introduce `Nullable` trait to permit custom `Option<T>`
+ [[#113]] `ValueType` trait should have a non-panic-ing method
+ [[#114]] `ValueType` revamp

    1. Remove `ValueTypeDefault`
    1. Change `type_name` to return `String`

+ [[#115]] Postgres concatenate operator (`||`)
+ [[#117]] Lock support (`FOR SHARE`, `FOR UPDATE`) for SELECT statement

[#112]: https://github.com/SeaQL/sea-query/pull/112
[#113]: https://github.com/SeaQL/sea-query/pull/113
[#114]: https://github.com/SeaQL/sea-query/pull/114
[#115]: https://github.com/SeaQL/sea-query/pull/115
[#117]: https://github.com/SeaQL/sea-query/pull/117

## 0.15.0 - 2021-08-21

+ [[#107]] Revamp `Value` to typed null value
+ Added `BigDecimal` support

The `Value::Null` variant is removed. You have to use a specific variant with a `None`.

Before:

```rust
Query::insert()
    .values_panic(vec![
        Value::Null,
        2.1345.into(),
    ])
```

After:

```rust
Query::insert()
    .values_panic(vec![
        Value::String(None),
        2.1345.into(),
    ])
```

Since we cannot handle the generic `Null` value on JSON, we removed the `json` method on `InsertStatement` and `UpdateStatement`. The following NO LONGER WORKS:

```rust
let query = Query::insert()
    .into_table(Glyph::Table)
    .json(json!({
        "aspect": 2.1345,
        "image": "24B",
    }));
```

```rust
let query = Query::update()
    .table(Glyph::Table)
    .json(json!({
        "aspect": 2.1345,
        "image": "235m",
    }));
```

In addition, if you constructed `Value` manually before (instead of using `into()` which is unaffected), you have to wrap them in an `Option`:

Before:

```rust
let (sql, values) = query.build(PostgresQueryBuilder);
assert_eq!(
	values,
	Values(vec![Value::String(Box::new("A".to_owned())), Value::Int(1), Value::Int(2), Value::Int(3)]))
);
```

After:

```rust
let (sql, values) = query.build(PostgresQueryBuilder);
assert_eq!(
	values,
	Values(vec![Value::String(Some(Box::new("A".to_owned()))), Value::Int(Some(1)), Value::Int(Some(2)), Value::Int(Some(3))]))
);
```

[#107]: https://github.com/SeaQL/sea-query/pull/107

## 0.14.1 - 2021-08-15

+ [[#87]] Fix inconsistent Ownership of self in Builder APIs
+ [[#105]] Use Arc for SeaRc with feature flag thread-safe

[#87]: https://github.com/SeaQL/sea-query/pull/87
[#105]: https://github.com/SeaQL/sea-query/pull/105

## 0.12.12 - 2021-08-14

+ [[#98]] Support Postgres full text search

[#98]: https://github.com/SeaQL/sea-query/pull/98

## 0.12.11 - 2021-08-13

+ Support SeaORM

## 0.12.10 - 2021-08-11

+ [[#89]] flattening iden enums in derive macro

[#89]: https://github.com/SeaQL/sea-query/pull/87

## 0.12.9 - 2021-08-08

+ [[#77]] Postgres `binary` type
+ [[#81]] example for CockroachDB
+ [[#84]] Fix Postgres constraint keywords
+ [[#75]] `DateTimeWithTimeZone` value type and `TimestampWithTimeZone` column type

[#77]: https://github.com/SeaQL/sea-query/pull/77
[#81]: https://github.com/SeaQL/sea-query/pull/81
[#84]: https://github.com/SeaQL/sea-query/pull/84
[#75]: https://github.com/SeaQL/sea-query/pull/75

## 0.12.8 - 2021-07-24

+ Fix Postgres `datetime` column type mapping
+ `Uuid` in schema builder

## 0.12.7 - 2021-07-13

+ `cust_with_values` allow escape `?` using `??`

## 0.12.6 - 2021-07-07

+ Fixed build error for `sqlx-sqlite`

## 0.12.5 - 2021-07-07

+ Support `Decimal` from rust_decimal

## 0.12.4 - 2021-06-23

+ Added `returning` for update statement

## 0.12.3 - 2021-06-19

+ Added `type_name` for ValueType
+ `Values` derive `Clone`
+ Added `Update::col_expr`
+ Type def Rc as `SeaRc`
+ getters for schema statements

## 0.12.2 - 2021-06-04

+ Fixed `and_where_option`
+ Added `Condition::add_option`

## 0.12.1 - 2021-06-03

+ Added `not_in_subquery`

## 0.12.0 - 2021-05-31

+ Unify `cond_where` and `and_where`. Note: will panic if calling `or_where` after `and_where`.

## 0.11.1 - 2021-05-22

+ Updated Readme

## 0.11.0 - 2021-05-19

+ Added APIs to support ORM
+ Backend and internal refactoring
+ Introduced `QueryStatementBuilder` and `SchemaStatementBuilder` traits
+ Introduced `ConditionalStatement` and `OrderedStatement` traits
+ Introduced any/all style conditions for `cond_where` and `cond_having`

## 0.10.6 - 2021-05-04

+ Postgres `ALTER TYPE` statements for `ENUM`

## 0.10.5 - 2021-05-02

+ Updated documentation

## 0.10.4 - 2021-05-02

+ `returning()` expression for Postgres insert statements
+ Remove redundant index name in foreign key expression of MySQL

## 0.10.3 - 2021-04-30

+ custom `Error` type
+ Empty value list for IN
+ Index prefix and `IndexOrder`

## 0.10.2 - 2021-04-27

+ Foreign key API `from` and `to`
+ Fix foreign key bug in `ON UPDATE`

## 0.10.1 - 2021-04-25

+ Added `index_type()` (`FullText` and `Hash`)
+ Added `unique()` to `Index`
+ Support composite primary key

## 0.10.0 - 2021-04-23

+ Use `IntoIterator` trait instead of `Vec` on most APIs
+ UUID support in `Value`
+ Rusqlite support

## 0.9.6 - 2021-04-18

+ Rename `create_if_not_exists` to `if_not_exists`
+ Remove `partition_option` from `TableAlterStatement`
+ Added `ColumnDef::extra()`

## 0.9.5 - 2021-04-17

+ Added `SchemaStatement`

## 0.9.4 - 2021-04-13

+ Fixed `DateTime` quoting bugs

## 0.9.3 - 2021-04-08

+ Update sea-query-derive to 0.1.2

## 0.9.2 - 2021-04-05

+ derive supporting enum tuple variant and custom method

## 0.9.1 - 2021-03-30

+ updated docs

## 0.9.0 - 2021-03-29

+ Introduced `IntoColumnRef` trait to consolidate `column` and `table.column`
+ Introduced `IntoTableRef` trait to consolidate `table` and `schema.table`
+ Introduced `IntoIden` trait to remove `*_dyn` methods

## 0.8.5 - 2021-03-29

+ added `into_simple_expr()`

## 0.8.4 - 2021-03-24

+ Fixing `IS NULL`

## 0.8.3 - 2021-03-23

+ derive `Debug` on most structs

## 0.8.2 - 2021-03-23

+ Added `unescape_string`

## 0.8.1 - 2021-03-23

+ Improve documentation

## 0.8.0 - 2021-03-14

+ `json` support behind features
+ backend as features (`backend-mysql`, `backend-postgres`, `backend-sqlite`)
+ added `from_schema()`, `from_schema_as()`

## 0.7.0 - 2021-03-06

+ Revamp `Value`
+ `build()` API change
+ `postgres` driver support
+ `json` and `chrono` support

## 0.6.1 - 2021-03-05

+ Added `join_as`
+ Deprecated `expr_alias`, `from_alias`

## 0.6.0 - 2021-02-20

+ Custom expression with parameters `Expr::cust_with_values()`
+ Custom function call `Func::cust()`
+ Postgres enum `Type::create().as_enum()`

## 0.5.0 - 2021-02-09

+ derive macro `#[derive(Iden)]`

## 0.4.0 - 2021-02-02

+ Added JSON binary column type `ColumnDef::json_binary()`
+ Custom column type `ColumnDef::custom()`

## 0.3.0 - 2020-12-29



## 0.2.0 - 2020-12-26



## 0.1.0 - 2020-12-16

Publish to crate.io

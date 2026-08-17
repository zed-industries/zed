<div align="center">

  <img src="https://raw.githubusercontent.com/SeaQL/sea-query/master/docs/SeaQuery logo.png" width="280" alt="SeaQuery logo"/>

  <p>
    <strong>🔱 A dynamic query builder for MySQL, Postgres and SQLite</strong>
  </p>

  [![crate](https://img.shields.io/crates/v/sea-query.svg)](https://crates.io/crates/sea-query)
  [![docs](https://docs.rs/sea-query/badge.svg)](https://docs.rs/sea-query)
  [![build status](https://github.com/SeaQL/sea-query/actions/workflows/rust.yml/badge.svg)](https://github.com/SeaQL/sea-query/actions/workflows/rust.yml)

</div>

## SeaQuery

SeaQuery is a query builder to help you construct dynamic SQL queries in Rust.
You can construct expressions, queries and schema as abstract syntax trees using an ergonomic API.
We support MySQL, Postgres and SQLite behind a common interface that aligns their behaviour where appropriate.

We provide integration for [SQLx](https://crates.io/crates/sqlx),
[postgres](https://crates.io/crates/postgres) and [rusqlite](https://crates.io/crates/rusqlite).
See [examples](https://github.com/SeaQL/sea-query/blob/master/examples) for usage.

SeaQuery is the foundation of [SeaORM](https://github.com/SeaQL/sea-orm), an async & dynamic ORM for Rust.

[![GitHub stars](https://img.shields.io/github/stars/SeaQL/sea-query.svg?style=social&label=Star&maxAge=1)](https://github.com/SeaQL/sea-query/stargazers/)
If you like what we do, consider starring, commenting, sharing and contributing!

[![Discord](https://img.shields.io/discord/873880840487206962?label=Discord)](https://discord.com/invite/uCPdDXzbdv)
Join our Discord server to chat with others in the SeaQL community!

## Install

```toml
# Cargo.toml
[dependencies]
sea-query = "0"
```

SeaQuery is very lightweight, all dependencies are optional (except `inherent`).

### Feature flags

Macro: `derive`

Async support: `thread-safe` (use `Arc` inplace of `Rc`)

SQL engine: `backend-mysql`, `backend-postgres`, `backend-sqlite`

Type support: `with-chrono`, `with-time`, `with-json`, `with-rust_decimal`, `with-bigdecimal`, `with-uuid`,
`with-ipnetwork`, `with-mac_address`, `postgres-array`, `postgres-interval`, `postgres-vector`

## Usage

Table of Content

1. Basics

    1. [Iden](#iden)
    1. [Expression](#expression)
    1. [Condition](#condition)
    1. [Statement Builders](#statement-builders)

1. Query Statement

    1. [Query Select](#query-select)
    1. [Query Insert](#query-insert)
    1. [Query Update](#query-update)
    1. [Query Delete](#query-delete)

1. Advanced
    1. [Aggregate Functions](#aggregate-functions)
    1. [Casting](#casting)
    1. [Custom Function](#custom-function)

1. Schema Statement

    1. [Table Create](#table-create)
    1. [Table Alter](#table-alter)
    1. [Table Drop](#table-drop)
    1. [Table Rename](#table-rename)
    1. [Table Truncate](#table-truncate)
    1. [Foreign Key Create](#foreign-key-create)
    1. [Foreign Key Drop](#foreign-key-drop)
    1. [Index Create](#index-create)
    1. [Index Drop](#index-drop)

### Motivation

Why would you want to use a dynamic query builder?

1. Parameter bindings

One of the headaches when using raw SQL is parameter binding. With SeaQuery you can:

```rust
assert_eq!(
    Query::select()
        .column(Glyph::Image)
        .from(Glyph::Table)
        .and_where(Expr::col(Glyph::Image).like("A"))
        .and_where(Expr::col(Glyph::Id).is_in([1, 2, 3]))
        .build(PostgresQueryBuilder),
    (
        r#"SELECT "image" FROM "glyph" WHERE "image" LIKE $1 AND "id" IN ($2, $3, $4)"#
            .to_owned(),
        Values(vec![
            Value::String(Some(Box::new("A".to_owned()))),
            Value::Int(Some(1)),
            Value::Int(Some(2)),
            Value::Int(Some(3))
        ])
    )
);
```

2. Dynamic query

You can construct the query at runtime based on user inputs:

```rust
Query::select()
    .column(Char::Character)
    .from(Char::Table)
    .conditions(
        // some runtime condition
        true,
        // if condition is true then add the following condition
        |q| {
            q.and_where(Expr::col(Char::Id).eq(1));
        },
        // otherwise leave it as is
        |q| {},
    );
```

### Iden

`Iden` is a trait for identifiers used in any query statement.

Commonly implemented by Enum where each Enum represents a table found in a database,
and its variants include table name and column name.

[`Iden::unquoted()`] must be implemented to provide a mapping between Enum variants and its
corresponding string value.

```rust
use sea_query::*;

// For example Character table with column id, character, font_size...
pub enum Character {
    Table,
    Id,
    FontId,
    FontSize,
}

// Mapping between Enum variant and its corresponding string value
impl Iden for Character {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Table => "character",
                Self::Id => "id",
                Self::FontId => "font_id",
                Self::FontSize => "font_size",
            }
        )
        .unwrap();
    }
}
```

If you're okay with running another procedural macro, you can activate
the `derive` feature on the crate to save you some boilerplate.
For more usage information, look at
[the derive examples](https://github.com/SeaQL/sea-query/tree/master/sea-query-derive/tests/pass).

```rust
#[cfg(feature = "derive")]
use sea_query::Iden;

// This will implement Iden exactly as shown above
#[derive(Iden)]
enum Character {
    Table,
}
assert_eq!(Character::Table.to_string(), "character");

// You can also derive a unit struct
#[derive(Iden)]
struct Glyph;
assert_eq!(Glyph.to_string(), "glyph");
```

```rust
#[cfg(feature = "derive")]
use sea_query::{enum_def, Iden};

#[enum_def]
struct Character {
    pub foo: u64,
}

// It generates the following along with Iden impl
enum CharacterIden {
    Table,
    Foo,
}

assert_eq!(CharacterIden::Table.to_string(), "character");
assert_eq!(CharacterIden::Foo.to_string(), "foo");
```


### Expression

Use [`Expr`] constructors and [`ExprTrait`] methods
to construct `SELECT`, `JOIN`, `WHERE` and `HAVING` expression in query.

```rust
assert_eq!(
    Query::select()
        .column(Char::Character)
        .from(Char::Table)
        .and_where(
            Expr::col(Char::SizeW)
                .add(1)
                .mul(2)
                .eq(Expr::col(Char::SizeH).div(2).sub(1))
        )
        .and_where(
            Expr::col(Char::SizeW).in_subquery(
                Query::select()
                    .expr(Expr::cust_with_values("ln($1 ^ $2)", [2.4, 1.2]))
                    .take()
            )
        )
        .and_where(
            Expr::col(Char::Character)
                .like("D")
                .and(Expr::col(Char::Character).like("E"))
        )
        .to_string(PostgresQueryBuilder),
    [
        r#"SELECT "character" FROM "character""#,
        r#"WHERE ("size_w" + 1) * 2 = ("size_h" / 2) - 1"#,
        r#"AND "size_w" IN (SELECT ln(2.4 ^ 1.2))"#,
        r#"AND ("character" LIKE 'D' AND "character" LIKE 'E')"#,
    ]
    .join(" ")
);
```

### Condition

If you have complex conditions to express, you can use the [`Condition`] builder,
usable for [`ConditionalStatement::cond_where`] and [`SelectStatement::cond_having`].

```rust
assert_eq!(
    Query::select()
        .column(Glyph::Id)
        .from(Glyph::Table)
        .cond_where(
            Cond::any()
                .add(
                    Cond::all()
                        .add(Expr::col(Glyph::Aspect).is_null())
                        .add(Expr::col(Glyph::Image).is_null())
                )
                .add(
                    Cond::all()
                        .add(Expr::col(Glyph::Aspect).is_in([3, 4]))
                        .add(Expr::col(Glyph::Image).like("A%"))
                )
        )
        .to_string(PostgresQueryBuilder),
    [
        r#"SELECT "id" FROM "glyph""#,
        r#"WHERE"#,
        r#"("aspect" IS NULL AND "image" IS NULL)"#,
        r#"OR"#,
        r#"("aspect" IN (3, 4) AND "image" LIKE 'A%')"#,
    ]
    .join(" ")
);
```

There is also the [`any!`] and [`all!`] macro at your convenience:

```rust
Query::select().cond_where(any![
    Expr::col(Glyph::Aspect).is_in([3, 4]),
    all![
        Expr::col(Glyph::Aspect).is_null(),
        Expr::col(Glyph::Image).like("A%")
    ]
]);
```

### Statement Builders

Statements are divided into 2 categories: Query and Schema, and to be serialized into SQL
with [`QueryStatementBuilder`] and [`SchemaStatementBuilder`] respectively.

Schema statement has the following interface:

```rust
fn build<T: SchemaBuilder>(&self, schema_builder: T) -> String;
```

Query statement has the following interfaces:

```rust
fn build<T: QueryBuilder>(&self, query_builder: T) -> (String, Values);

fn to_string<T: QueryBuilder>(&self, query_builder: T) -> String;
```

`build` builds a SQL statement as string and parameters to be passed to the database driver
through the binary protocol. This is the preferred way as it has less overhead and is more secure.

`to_string` builds a SQL statement as string with parameters injected. This is good for testing
and debugging.

### Query Select

```rust
let query = Query::select()
    .column(Char::Character)
    .column((Font::Table, Font::Name))
    .from(Char::Table)
    .left_join(Font::Table, Expr::col((Char::Table, Char::FontId)).equals((Font::Table, Font::Id)))
    .and_where(Expr::col(Char::SizeW).is_in([3, 4]))
    .and_where(Expr::col(Char::Character).like("A%"))
    .to_owned();

assert_eq!(
    query.to_string(MysqlQueryBuilder),
    r#"SELECT `character`, `font`.`name` FROM `character` LEFT JOIN `font` ON `character`.`font_id` = `font`.`id` WHERE `size_w` IN (3, 4) AND `character` LIKE 'A%'"#
);
assert_eq!(
    query.to_string(PostgresQueryBuilder),
    r#"SELECT "character", "font"."name" FROM "character" LEFT JOIN "font" ON "character"."font_id" = "font"."id" WHERE "size_w" IN (3, 4) AND "character" LIKE 'A%'"#
);
assert_eq!(
    query.to_string(SqliteQueryBuilder),
    r#"SELECT "character", "font"."name" FROM "character" LEFT JOIN "font" ON "character"."font_id" = "font"."id" WHERE "size_w" IN (3, 4) AND "character" LIKE 'A%'"#
);
```

### Query Insert

```rust
let query = Query::insert()
    .into_table(Glyph::Table)
    .columns([Glyph::Aspect, Glyph::Image])
    .values_panic([5.15.into(), "12A".into()])
    .values_panic([4.21.into(), "123".into()])
    .to_owned();

assert_eq!(
    query.to_string(MysqlQueryBuilder),
    r#"INSERT INTO `glyph` (`aspect`, `image`) VALUES (5.15, '12A'), (4.21, '123')"#
);
assert_eq!(
    query.to_string(PostgresQueryBuilder),
    r#"INSERT INTO "glyph" ("aspect", "image") VALUES (5.15, '12A'), (4.21, '123')"#
);
assert_eq!(
    query.to_string(SqliteQueryBuilder),
    r#"INSERT INTO "glyph" ("aspect", "image") VALUES (5.15, '12A'), (4.21, '123')"#
);
```

### Query Update

```rust
let query = Query::update()
    .table(Glyph::Table)
    .values([(Glyph::Aspect, 1.23.into()), (Glyph::Image, "123".into())])
    .and_where(Expr::col(Glyph::Id).eq(1))
    .to_owned();

assert_eq!(
    query.to_string(MysqlQueryBuilder),
    r#"UPDATE `glyph` SET `aspect` = 1.23, `image` = '123' WHERE `id` = 1"#
);
assert_eq!(
    query.to_string(PostgresQueryBuilder),
    r#"UPDATE "glyph" SET "aspect" = 1.23, "image" = '123' WHERE "id" = 1"#
);
assert_eq!(
    query.to_string(SqliteQueryBuilder),
    r#"UPDATE "glyph" SET "aspect" = 1.23, "image" = '123' WHERE "id" = 1"#
);
```

### Query Delete

```rust
let query = Query::delete()
    .from_table(Glyph::Table)
    .cond_where(
        Cond::any()
            .add(Expr::col(Glyph::Id).lt(1))
            .add(Expr::col(Glyph::Id).gt(10)),
    )
    .to_owned();

assert_eq!(
    query.to_string(MysqlQueryBuilder),
    r#"DELETE FROM `glyph` WHERE `id` < 1 OR `id` > 10"#
);
assert_eq!(
    query.to_string(PostgresQueryBuilder),
    r#"DELETE FROM "glyph" WHERE "id" < 1 OR "id" > 10"#
);
assert_eq!(
    query.to_string(SqliteQueryBuilder),
    r#"DELETE FROM "glyph" WHERE "id" < 1 OR "id" > 10"#
);
```

### Aggregate Functions

`max`, `min`, `sum`, `avg`, `count` etc

```rust
let query = Query::select()
    .expr(Func::sum(Expr::col((Char::Table, Char::SizeH))))
    .from(Char::Table)
    .to_owned();
assert_eq!(
    query.to_string(MysqlQueryBuilder),
    r#"SELECT SUM(`character`.`size_h`) FROM `character`"#
);
assert_eq!(
    query.to_string(PostgresQueryBuilder),
    r#"SELECT SUM("character"."size_h") FROM "character""#
);
assert_eq!(
    query.to_string(SqliteQueryBuilder),
    r#"SELECT SUM("character"."size_h") FROM "character""#
);
```

### Casting

```rust
let query = Query::select()
    .expr(Func::cast_as("hello", "MyType"))
    .to_owned();

assert_eq!(
    query.to_string(MysqlQueryBuilder),
    r#"SELECT CAST('hello' AS MyType)"#
);
assert_eq!(
    query.to_string(PostgresQueryBuilder),
    r#"SELECT CAST('hello' AS MyType)"#
);
assert_eq!(
    query.to_string(SqliteQueryBuilder),
    r#"SELECT CAST('hello' AS MyType)"#
);
```

### Custom Function

```rust
struct MyFunction;

impl Iden for MyFunction {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(s, "MY_FUNCTION").unwrap();
    }
}

let query = Query::select()
    .expr(Func::cust(MyFunction).arg(Expr::val("hello")))
    .to_owned();

assert_eq!(
    query.to_string(MysqlQueryBuilder),
    r#"SELECT MY_FUNCTION('hello')"#
);
assert_eq!(
    query.to_string(PostgresQueryBuilder),
    r#"SELECT MY_FUNCTION('hello')"#
);
assert_eq!(
    query.to_string(SqliteQueryBuilder),
    r#"SELECT MY_FUNCTION('hello')"#
);
```

### Table Create

```rust
let table = Table::create()
    .table(Char::Table)
    .if_not_exists()
    .col(ColumnDef::new(Char::Id).integer().not_null().auto_increment().primary_key())
    .col(ColumnDef::new(Char::FontSize).integer().not_null())
    .col(ColumnDef::new(Char::Character).string().not_null())
    .col(ColumnDef::new(Char::SizeW).integer().not_null())
    .col(ColumnDef::new(Char::SizeH).integer().not_null())
    .col(ColumnDef::new(Char::FontId).integer().default(Value::Int(None)))
    .foreign_key(
        ForeignKey::create()
            .name("FK_2e303c3a712662f1fc2a4d0aad6")
            .from(Char::Table, Char::FontId)
            .to(Font::Table, Font::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
    )
    .to_owned();

assert_eq!(
    table.to_string(MysqlQueryBuilder),
    [
        r#"CREATE TABLE IF NOT EXISTS `character` ("#,
            r#"`id` int NOT NULL AUTO_INCREMENT PRIMARY KEY,"#,
            r#"`font_size` int NOT NULL,"#,
            r#"`character` varchar(255) NOT NULL,"#,
            r#"`size_w` int NOT NULL,"#,
            r#"`size_h` int NOT NULL,"#,
            r#"`font_id` int DEFAULT NULL,"#,
            r#"CONSTRAINT `FK_2e303c3a712662f1fc2a4d0aad6`"#,
                r#"FOREIGN KEY (`font_id`) REFERENCES `font` (`id`)"#,
                r#"ON DELETE CASCADE ON UPDATE CASCADE"#,
        r#")"#,
    ].join(" ")
);
assert_eq!(
    table.to_string(PostgresQueryBuilder),
    [
        r#"CREATE TABLE IF NOT EXISTS "character" ("#,
            r#""id" serial NOT NULL PRIMARY KEY,"#,
            r#""font_size" integer NOT NULL,"#,
            r#""character" varchar NOT NULL,"#,
            r#""size_w" integer NOT NULL,"#,
            r#""size_h" integer NOT NULL,"#,
            r#""font_id" integer DEFAULT NULL,"#,
            r#"CONSTRAINT "FK_2e303c3a712662f1fc2a4d0aad6""#,
                r#"FOREIGN KEY ("font_id") REFERENCES "font" ("id")"#,
                r#"ON DELETE CASCADE ON UPDATE CASCADE"#,
        r#")"#,
    ].join(" ")
);
assert_eq!(
    table.to_string(SqliteQueryBuilder),
    [
       r#"CREATE TABLE IF NOT EXISTS "character" ("#,
           r#""id" integer NOT NULL PRIMARY KEY AUTOINCREMENT,"#,
           r#""font_size" integer NOT NULL,"#,
           r#""character" varchar NOT NULL,"#,
           r#""size_w" integer NOT NULL,"#,
           r#""size_h" integer NOT NULL,"#,
           r#""font_id" integer DEFAULT NULL,"#,
           r#"FOREIGN KEY ("font_id") REFERENCES "font" ("id") ON DELETE CASCADE ON UPDATE CASCADE"#,
       r#")"#,
    ].join(" ")
);
```

### Table Alter

```rust
let table = Table::alter()
    .table(Font::Table)
    .add_column(
        ColumnDef::new("new_col")
            .integer()
            .not_null()
            .default(100),
    )
    .to_owned();

assert_eq!(
    table.to_string(MysqlQueryBuilder),
    r#"ALTER TABLE `font` ADD COLUMN `new_col` int NOT NULL DEFAULT 100"#
);
assert_eq!(
    table.to_string(PostgresQueryBuilder),
    r#"ALTER TABLE "font" ADD COLUMN "new_col" integer NOT NULL DEFAULT 100"#
);
assert_eq!(
    table.to_string(SqliteQueryBuilder),
    r#"ALTER TABLE "font" ADD COLUMN "new_col" integer NOT NULL DEFAULT 100"#,
);
```

### Table Drop

```rust
let table = Table::drop()
    .table(Glyph::Table)
    .table(Char::Table)
    .to_owned();

assert_eq!(
    table.to_string(MysqlQueryBuilder),
    r#"DROP TABLE `glyph`, `character`"#
);
assert_eq!(
    table.to_string(PostgresQueryBuilder),
    r#"DROP TABLE "glyph", "character""#
);
assert_eq!(
    table.to_string(SqliteQueryBuilder),
    r#"DROP TABLE "glyph", "character""#
);
```

### Table Rename

```rust
let table = Table::rename()
    .table(Font::Table, "font_new")
    .to_owned();

assert_eq!(
    table.to_string(MysqlQueryBuilder),
    r#"RENAME TABLE `font` TO `font_new`"#
);
assert_eq!(
    table.to_string(PostgresQueryBuilder),
    r#"ALTER TABLE "font" RENAME TO "font_new""#
);
assert_eq!(
    table.to_string(SqliteQueryBuilder),
    r#"ALTER TABLE "font" RENAME TO "font_new""#
);
```

### Table Truncate

```rust
let table = Table::truncate().table(Font::Table).to_owned();

assert_eq!(
    table.to_string(MysqlQueryBuilder),
    r#"TRUNCATE TABLE `font`"#
);
assert_eq!(
    table.to_string(PostgresQueryBuilder),
    r#"TRUNCATE TABLE "font""#
);
// Sqlite does not support the TRUNCATE statement
```

### Foreign Key Create

```rust
let foreign_key = ForeignKey::create()
    .name("FK_character_font")
    .from(Char::Table, Char::FontId)
    .to(Font::Table, Font::Id)
    .on_delete(ForeignKeyAction::Cascade)
    .on_update(ForeignKeyAction::Cascade)
    .to_owned();

assert_eq!(
    foreign_key.to_string(MysqlQueryBuilder),
    [
        r#"ALTER TABLE `character`"#,
        r#"ADD CONSTRAINT `FK_character_font`"#,
        r#"FOREIGN KEY (`font_id`) REFERENCES `font` (`id`)"#,
        r#"ON DELETE CASCADE ON UPDATE CASCADE"#,
    ]
    .join(" ")
);
assert_eq!(
    foreign_key.to_string(PostgresQueryBuilder),
    [
        r#"ALTER TABLE "character" ADD CONSTRAINT "FK_character_font""#,
        r#"FOREIGN KEY ("font_id") REFERENCES "font" ("id")"#,
        r#"ON DELETE CASCADE ON UPDATE CASCADE"#,
    ]
    .join(" ")
);
// Sqlite does not support modification of foreign key constraints to existing tables
```

### Foreign Key Drop

```rust
let foreign_key = ForeignKey::drop()
    .name("FK_character_font")
    .table(Char::Table)
    .to_owned();

assert_eq!(
    foreign_key.to_string(MysqlQueryBuilder),
    r#"ALTER TABLE `character` DROP FOREIGN KEY `FK_character_font`"#
);
assert_eq!(
    foreign_key.to_string(PostgresQueryBuilder),
    r#"ALTER TABLE "character" DROP CONSTRAINT "FK_character_font""#
);
// Sqlite does not support modification of foreign key constraints to existing tables
```

### Index Create

```rust
let index = Index::create()
    .name("idx-glyph-aspect")
    .table(Glyph::Table)
    .col(Glyph::Aspect)
    .to_owned();

assert_eq!(
    index.to_string(MysqlQueryBuilder),
    r#"CREATE INDEX `idx-glyph-aspect` ON `glyph` (`aspect`)"#
);
assert_eq!(
    index.to_string(PostgresQueryBuilder),
    r#"CREATE INDEX "idx-glyph-aspect" ON "glyph" ("aspect")"#
);
assert_eq!(
    index.to_string(SqliteQueryBuilder),
    r#"CREATE INDEX "idx-glyph-aspect" ON "glyph" ("aspect")"#
);
```

### Index Drop

```rust
let index = Index::drop()
    .name("idx-glyph-aspect")
    .table(Glyph::Table)
    .to_owned();

assert_eq!(
    index.to_string(MysqlQueryBuilder),
    r#"DROP INDEX `idx-glyph-aspect` ON `glyph`"#
);
assert_eq!(
    index.to_string(PostgresQueryBuilder),
    r#"DROP INDEX "idx-glyph-aspect""#
);
assert_eq!(
    index.to_string(SqliteQueryBuilder),
    r#"DROP INDEX "idx-glyph-aspect""#
);
```

## License

Licensed under either of

-   Apache License, Version 2.0
    ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
-   MIT license
    ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

SeaQuery is a community driven project. We welcome you to participate, contribute and together build for Rust's future.

A big shout out to our contributors:

[![Contributors](https://opencollective.com/sea-query/contributors.svg?width=1000&button=false)](https://github.com/SeaQL/sea-query/graphs/contributors)

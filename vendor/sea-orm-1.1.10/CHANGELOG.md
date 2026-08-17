# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## 1.1.10 - 2025-04-14

### Upgrades

* Upgrade sqlx to 0.8.4

## 1.1.9 - 2025-04-14

### Enhancements

* [sea-orm-macros] Use fully-qualified syntax for ActiveEnum associated type https://github.com/SeaQL/sea-orm/pull/2552
* Accept `LikeExpr` in `like` and `not_like` https://github.com/SeaQL/sea-orm/pull/2549

### Bug fixes

* Check if url is well-formed before parsing https://github.com/SeaQL/sea-orm/pull/2558
* `QuerySelect::column_as` method cast ActiveEnum column https://github.com/SeaQL/sea-orm/pull/2551

### House keeping

* Remove redundant `Expr::expr` from internal code https://github.com/SeaQL/sea-orm/pull/2554

## 1.1.8 - 2025-03-30

### New Features

* Implement `DeriveValueType` for enum strings
```rust
#[derive(DeriveValueType)]
#[sea_orm(value_type = "String")]
pub enum Tag {
    Hard,
    Soft,
}

// `from_str` defaults to `std::str::FromStr::from_str`
impl std::str::FromStr for Tag {
    type Err = sea_orm::sea_query::ValueTypeErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> { .. }
}

// `to_str` defaults to `std::string::ToString::to_string`.
impl std::fmt::Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { .. }
}

// you can override from_str and to_str with custom functions
#[derive(DeriveValueType)]
#[sea_orm(value_type = "String", from_str = "Tag::from_str", to_str = "Tag::to_str")]
pub enum Tag {
    Color,
    Grey,
}

impl Tag {
    fn from_str(s: &str) -> Result<Self, ValueTypeErr> { .. }

    fn to_str(&self) -> &'static str { .. }
}
```
* Support Postgres Ipnetwork (under feature flag `with-ipnetwork`) https://github.com/SeaQL/sea-orm/pull/2395
```rust
// Model
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "host_network")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub ipaddress: IpNetwork,
    #[sea_orm(column_type = "Cidr")]
    pub network: IpNetwork,
}

// Schema
sea_query::Table::create()
    .table(host_network::Entity)
    .col(ColumnDef::new(host_network::Column::Id).integer().not_null().auto_increment().primary_key())
    .col(ColumnDef::new(host_network::Column::Ipaddress).inet().not_null())
    .col(ColumnDef::new(host_network::Column::Network).cidr().not_null())
    .to_owned();

// CRUD
host_network::ActiveModel {
    ipaddress: Set(IpNetwork::new(Ipv6Addr::new(..))),
    network: Set(IpNetwork::new(Ipv4Addr::new(..))),
    ..Default::default()
}
```

### Enhancements

* Added `try_getable_postgres_array!(Vec<u8>)` (to support `bytea[]`) https://github.com/SeaQL/sea-orm/pull/2503

### Bug fixes

* [sea-orm-codegen] Support postgres array in expanded format https://github.com/SeaQL/sea-orm/pull/2545

### House keeping

* Replace `once_cell` crate with `std` equivalent https://github.com/SeaQL/sea-orm/pull/2524
(available since rust 1.80)

## 1.1.7 - 2025-03-02

### New Features

* Support nested entities in `FromQueryResult` https://github.com/SeaQL/sea-orm/pull/2508
```rust
#[derive(FromQueryResult)]
struct Cake {
    id: i32,
    name: String,
    #[sea_orm(nested)]
    bakery: Option<CakeBakery>,
}

#[derive(FromQueryResult)]
struct CakeBakery {
    #[sea_orm(from_alias = "bakery_id")]
    id: i32,
    #[sea_orm(from_alias = "bakery_name")]
    title: String,
}

let cake: Cake = cake::Entity::find()
    .select_only()
    .column(cake::Column::Id)
    .column(cake::Column::Name)
    .column_as(bakery::Column::Id, "bakery_id")
    .column_as(bakery::Column::Name, "bakery_name")
    .left_join(bakery::Entity)
    .order_by_asc(cake::Column::Id)
    .into_model()
    .one(&ctx.db)
    .await?
    .unwrap();

assert_eq!(
    cake,
    Cake {
        id: 1,
        name: "Cake".to_string(),
        bakery: Some(CakeBakery {
            id: 20,
            title: "Bakery".to_string(),
        })
    }
);
```
* Support nested entities in `DerivePartialModel` https://github.com/SeaQL/sea-orm/pull/2508
```rust
#[derive(DerivePartialModel)] // FromQueryResult is no longer needed
#[sea_orm(entity = "cake::Entity", from_query_result)]
struct Cake {
    id: i32,
    name: String,
    #[sea_orm(nested)]
    bakery: Option<Bakery>,
}

#[derive(DerivePartialModel)]
#[sea_orm(entity = "bakery::Entity", from_query_result)]
struct Bakery {
    id: i32,
    #[sea_orm(from_col = "Name")]
    title: String,
}

// same as previous example, but without the custom selects
let cake: Cake = cake::Entity::find()
    .left_join(bakery::Entity)
    .order_by_asc(cake::Column::Id)
    .into_partial_model()
    .one(&ctx.db)
    .await?
    .unwrap();

assert_eq!(
    cake,
    Cake {
        id: 1,
        name: "Cake".to_string(),
        bakery: Some(CakeBakery {
            id: 20,
            title: "Bakery".to_string(),
        })
    }
);
```
* Derive also `IntoActiveModel` with `DerivePartialModel` https://github.com/SeaQL/sea-orm/pull/2517
```rust
#[derive(DerivePartialModel)]
#[sea_orm(entity = "cake::Entity", into_active_model)]
struct Cake {
    id: i32,
    name: String,
}

assert_eq!(
    Cake {
        id: 12,
        name: "Lemon Drizzle".to_owned(),
    }
    .into_active_model(),
    cake::ActiveModel {
        id: Set(12),
        name: Set("Lemon Drizzle".to_owned()),
        ..Default::default()
    }
);
```
* Added `SelectThree` https://github.com/SeaQL/sea-orm/pull/2518
```rust
// Order -> (many) Lineitem -> Cake
let items: Vec<(order::Model, Option<lineitem::Model>, Option<cake::Model>)> =
    order::Entity::find()
        .find_also_related(lineitem::Entity)
        .and_also_related(cake::Entity)
        .order_by_asc(order::Column::Id)
        .order_by_asc(lineitem::Column::Id)
        .all(&ctx.db)
        .await?;
```

### Enhancements

* Support complex type path in `DeriveIntoActiveModel` https://github.com/SeaQL/sea-orm/pull/2517
```rust 
#[derive(DeriveIntoActiveModel)]
#[sea_orm(active_model = "<fruit::Entity as EntityTrait>::ActiveModel")]
struct Fruit {
    cake_id: Option<Option<i32>>,
}
```
* Added `DatabaseConnection::close_by_ref` https://github.com/SeaQL/sea-orm/pull/2511
```rust
pub async fn close(self) -> Result<(), DbErr> { .. } // existing
pub async fn close_by_ref(&self) -> Result<(), DbErr> { .. } // new
```

### House Keeping

* Cleanup legacy `ActiveValue::Set` https://github.com/SeaQL/sea-orm/pull/2515

## 1.1.6 - 2025-02-24

### New Features

* Support PgVector (under feature flag `postgres-vector`) https://github.com/SeaQL/sea-orm/pull/2500
```rust
// Model
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "image_model")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    pub embedding: PgVector,
}
 
// Schema
sea_query::Table::create()
    .table(image_model::Entity.table_ref())
    .col(ColumnDef::new(Column::Id).integer().not_null().primary_key())
    .col(ColumnDef::new(Column::Embedding).vector(None).not_null())
    ..

// Insert
ActiveModel {
    id: NotSet,
    embedding: Set(PgVector::from(vec![1., 2., 3.])),
}
.insert(db)
.await?
```
* Added `Insert::exec_with_returning_keys` & `Insert::exec_with_returning_many` (Postgres only)
```rust
assert_eq!(
    Entity::insert_many([
        ActiveModel { id: NotSet, name: Set("two".into()) },
        ActiveModel { id: NotSet, name: Set("three".into()) },
    ])
    .exec_with_returning_many(db)
    .await
    .unwrap(),
    [
        Model { id: 2, name: "two".into() },
        Model { id: 3, name: "three".into() },
    ]
);

assert_eq!(
    cakes_bakers::Entity::insert_many([
        cakes_bakers::ActiveModel {
            cake_id: Set(1),
            baker_id: Set(2),
        },
        cakes_bakers::ActiveModel {
            cake_id: Set(2),
            baker_id: Set(1),
        },
    ])
    .exec_with_returning_keys(db)
    .await
    .unwrap(),
    [(1, 2), (2, 1)]
);
```
* Added `DeleteOne::exec_with_returning` & `DeleteMany::exec_with_returning` https://github.com/SeaQL/sea-orm/pull/2432

### Enhancements

* Expose underlying row types (e.g. `sqlx::postgres::PgRow`) https://github.com/SeaQL/sea-orm/pull/2265
* [sea-orm-cli] Added `acquire-timeout` option https://github.com/SeaQL/sea-orm/pull/2461
* [sea-orm-cli] Added `with-prelude` option https://github.com/SeaQL/sea-orm/pull/2322
* [sea-orm-cli] Added `impl-active-model-behavior` option https://github.com/SeaQL/sea-orm/pull/2487

### Bug Fixes

* Fixed `seaography::register_active_enums` macro https://github.com/SeaQL/sea-orm/pull/2475

### House keeping

* Remove `futures` crate, replace with `futures-util` https://github.com/SeaQL/sea-orm/pull/2466

## 1.1.5 - 2025-02-14

### New Features

* Added `Schema::json_schema_from_entity` to construct a schema description in json for the given Entity

## 1.1.4 - 2025-01-10

### Enhancements

* Allow modifying the connection in migrations https://github.com/SeaQL/sea-orm/pull/2397
* `DeriveRelatedEntity` proc_macro use `async-graphql` re-exported by `seaography` https://github.com/SeaQL/sea-orm/pull/2469

## 1.1.3 - 2024-12-24

### New Features

* [sea-orm-codegen] register seaography entity modules & active enums https://github.com/SeaQL/sea-orm/pull/2403
```rust
pub mod prelude;

pub mod sea_orm_active_enums;

pub mod baker;
pub mod bakery;
pub mod cake;
pub mod cakes_bakers;
pub mod customer;
pub mod lineitem;
pub mod order;

seaography::register_entity_modules!([
    baker,
    bakery,
    cake,
    cakes_bakers,
    customer,
    lineitem,
    order,
]);

seaography::register_active_enums!([
    sea_orm_active_enums::Tea,
    sea_orm_active_enums::Color,
]);
```

### Enhancements

* Insert many allow active models to have different column set https://github.com/SeaQL/sea-orm/pull/2433
```rust
// this previously panics
let apple = cake_filling::ActiveModel {
    cake_id: ActiveValue::set(2),
    filling_id: ActiveValue::NotSet,
};
let orange = cake_filling::ActiveModel {
    cake_id: ActiveValue::NotSet,
    filling_id: ActiveValue::set(3),
};
assert_eq!(
    Insert::<cake_filling::ActiveModel>::new()
        .add_many([apple, orange])
        .build(DbBackend::Postgres)
        .to_string(),
    r#"INSERT INTO "cake_filling" ("cake_id", "filling_id") VALUES (2, NULL), (NULL, 3)"#,
);
```
* [sea-orm-cli] Added `MIGRATION_DIR` environment variable https://github.com/SeaQL/sea-orm/pull/2419
* Added `ColumnDef::is_unique` https://github.com/SeaQL/sea-orm/pull/2401
* Postgres: quote schema in `search_path` https://github.com/SeaQL/sea-orm/pull/2436

### Bug Fixes

* MySQL: fix transaction isolation level not respected when used with access mode https://github.com/SeaQL/sea-orm/pull/2450

## 1.1.2 - 2024-12-02

### Enhancements

* Added `ColumnTrait::enum_type_name()` to signify enum types https://github.com/SeaQL/sea-orm/pull/2415
* Added `DbBackend::boolean_value()` for database dependent boolean value https://github.com/SeaQL/sea-orm/pull/2415

## 1.1.1 - 2024-11-04

### Enhancements

* [sea-orm-macros] `impl From<Model> for ActiveModel` instead of `impl From<<Entity as sea_orm::EntityTrait>::Model> for ActiveModel` https://github.com/SeaQL/sea-orm/pull/2349.
Now the following can compile:
```rust
use sea_orm::{tests_cfg::cake, Set};

struct Cake {
    id: i32,
    name: String,
}

impl From<Cake> for cake::ActiveModel {
    fn from(value: Cake) -> Self {
        Self {
            id: Set(value.id),
            name: Set(value.name),
        }
    }
}
```

## 1.1.0 - 2024-10-15

### Versions

+ `1.1.0-rc.1`: 2024-08-09
+ `1.1.0-rc.2`: 2024-10-04
+ `1.1.0-rc.3`: 2024-10-08

### Enhancements

* [sea-orm-macros] Call `EnumIter::get` using fully qualified syntax https://github.com/SeaQL/sea-orm/pull/2321
* Construct `DatabaseConnection` directly from `sqlx::PgPool`, `sqlx::SqlitePool` and `sqlx::MySqlPool` https://github.com/SeaQL/sea-orm/pull/2348
* [sea-orm-migration] Add `pk_uuid` schema helper https://github.com/SeaQL/sea-orm/pull/2329
* [sea-orm-migration] Allow `custom` and `custom_null` schema helper to take column name and alias of different `IntoIden` types https://github.com/SeaQL/sea-orm/pull/2326
* Add `ColumnDef::get_column_default` getter https://github.com/SeaQL/sea-orm/pull/2387

### Upgrades

* Upgrade `sqlx` to `0.8.2` https://github.com/SeaQL/sea-orm/pull/2305, https://github.com/SeaQL/sea-orm/pull/2371
* Upgrade `bigdecimal` to `0.4` https://github.com/SeaQL/sea-orm/pull/2305
* Upgrade `sea-query` to `0.32.0-rc` https://github.com/SeaQL/sea-orm/pull/2305
* Upgrade `sea-query-binder` to `0.7.0-rc` https://github.com/SeaQL/sea-orm/pull/2305
* Upgrade `sea-schema` to `0.16.0-rc` https://github.com/SeaQL/sea-orm/pull/2305
* Upgrade `ouroboros` to `0.18` https://github.com/SeaQL/sea-orm/pull/2353

### House keeping

* Fix typos https://github.com/SeaQL/sea-orm/pull/2360
* Update documentations https://github.com/SeaQL/sea-orm/pull/2345

## 1.0.1 - 2024-08-26

### New Features

* Added `ConnectOptions::connect_lazy` for creating DB connection pools without establishing connections up front https://github.com/SeaQL/sea-orm/pull/2268

### Breaking Changes

* Changed `ProxyDatabaseTrait` methods to async. It's a breaking change, but it should have been part of the 1.0 release.
    The feature is behind the feature guard `proxy`, and we believe it shouldn't impact majority of users.
    https://github.com/SeaQL/sea-orm/pull/2278

### Bug Fixes

* [sea-orm-codegen] Fix `ColumnType` to Rust type resolution https://github.com/SeaQL/sea-orm/pull/2313

## 1.0.0 - 2024-08-02

### Versions

+ `1.0.0-rc.1`: 2024-02-06
+ `1.0.0-rc.2`: 2024-03-15
+ `1.0.0-rc.3`: 2024-03-26
+ `1.0.0-rc.4`: 2024-05-13
+ `1.0.0-rc.5`: 2024-05-29
+ `1.0.0-rc.6`: 2024-06-19
+ `1.0.0-rc.7`: 2024-06-25

### New Features

* Introduce `PrimaryKeyArity` with `ARITY` constant https://github.com/SeaQL/sea-orm/pull/2185
```rust
fn get_arity_of<E: EntityTrait>() -> usize {
    E::PrimaryKey::iter().count() // before; runtime
    <<E::PrimaryKey as PrimaryKeyTrait>::ValueType as PrimaryKeyArity>::ARITY // now; compile-time
}
```
* Associate `ActiveModel` to `EntityTrait` https://github.com/SeaQL/sea-orm/pull/2186
* [sea-orm-macros] Added `rename_all` attribute to `DeriveEntityModel` & `DeriveActiveEnum` https://github.com/SeaQL/sea-orm/pull/2170
```rust
#[derive(DeriveEntityModel)]
#[sea_orm(table_name = "user", rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key)]
    id: i32,
    first_name: String, // firstName
    #[sea_orm(column_name = "lAsTnAmE")]
    last_name: String, // lAsTnAmE
}

#[derive(EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)", rename_all = "camelCase")]
pub enum TestEnum {
    DefaultVariant, // defaultVariant
    #[sea_orm(rename = "kebab-case")]
    VariantKebabCase, // variant-kebab-case
    #[sea_orm(rename = "snake_case")]
    VariantSnakeCase, // variant_snake_case
    #[sea_orm(string_value = "CuStOmStRiNgVaLuE")]
    CustomStringValue, // CuStOmStRiNgVaLuE
}
```
* [sea-orm-migration] schema helper https://github.com/SeaQL/sea-orm/pull/2099
```rust
// Remember to import `sea_orm_migration::schema::*`
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(pk_auto(Users::Id)) // Primary key with auto-increment
                    .col(uuid(Users::Pid)) // UUID column
                    .col(string_uniq(Users::Email)) // String column with unique constraint
                    .col(string(Users::Password)) // String column
                    .col(string(Users::ApiKey).unique_key())
                    .col(string(Users::Name))
                    .col(string_null(Users::ResetToken)) // Nullable string column
                    .col(timestamp_null(Users::ResetSentAt)) // Nullable timestamp column
                    .col(string_null(Users::EmailVerificationToken))
                    .col(timestamp_null(Users::EmailVerificationSentAt))
                    .col(timestamp_null(Users::EmailVerifiedAt))
                    .to_owned(),
            )
            .await
    }

    // ...
}
```

### Enhancements

* Added non-TLS runtime https://github.com/SeaQL/sea-orm/pull/2256
* Added `QuerySelect::tbl_col_as`
* Added `Insert::on_conflict_do_nothing` https://github.com/SeaQL/sea-orm/pull/2244
* Migration schema nullable column set NULL explicitly https://github.com/SeaQL/sea-orm/pull/2255
* Added `ActiveValue::set_if_not_equals()` https://github.com/SeaQL/sea-orm/pull/2194
* Added `ActiveValue::try_as_ref()` https://github.com/SeaQL/sea-orm/pull/2197
* Added `QuerySelect::order_by_with_nulls` https://github.com/SeaQL/sea-orm/pull/2228
* Expose `get_xxx_connection_pool` by default https://github.com/SeaQL/sea-orm/pull/2233
* Added `QueryResult::column_names` https://github.com/SeaQL/sea-orm/pull/2148
* [sea-orm-macro] Add `@generated` in generated code https://github.com/SeaQL/sea-orm/pull/2199
* [sea-orm-macro] Qualify traits in `DeriveActiveModel` macro https://github.com/SeaQL/sea-orm/pull/1665
* [sea-orm-cli] Fix `migrate generate` on empty `mod.rs` files https://github.com/SeaQL/sea-orm/pull/2064
* `DerivePartialModel` macro attribute `entity` now supports `syn::Type` https://github.com/SeaQL/sea-orm/pull/2137
```rust
#[derive(DerivePartialModel)]
#[sea_orm(entity = "<entity::Model as ModelTrait>::Entity")]
struct EntityNameNotAIdent {
    #[sea_orm(from_col = "foo2")]
    _foo: i32,
    #[sea_orm(from_col = "bar2")]
    _bar: String,
}
```
* Added `RelationDef::from_alias()` https://github.com/SeaQL/sea-orm/pull/2146
```rust
let cf = Alias::new("cf");

assert_eq!(
    cake::Entity::find()
        .join_as(
            JoinType::LeftJoin,
            cake_filling::Relation::Cake.def().rev(),
            cf.clone()
        )
        .join(
            JoinType::LeftJoin,
            cake_filling::Relation::Filling.def().from_alias(cf)
        )
        .build(DbBackend::MySql)
        .to_string(),
    [
        "SELECT `cake`.`id`, `cake`.`name` FROM `cake`",
        "LEFT JOIN `cake_filling` AS `cf` ON `cake`.`id` = `cf`.`cake_id`",
        "LEFT JOIN `filling` ON `cf`.`filling_id` = `filling`.`id`",
    ]
    .join(" ")
);
```

### Bug Fixes

* Set schema search path in Postgres without enclosing single quote https://github.com/SeaQL/sea-orm/pull/2241
* [sea-orm-cli] Generate `has_one` relation for foreign key of unique index / constraint https://github.com/SeaQL/sea-orm/pull/2254

### Breaking changes

* Renamed `ConnectOptions::pool_options()` to `ConnectOptions::sqlx_pool_options()` https://github.com/SeaQL/sea-orm/pull/2145
* Made `sqlx_common` private, hiding `sqlx_error_to_xxx_err` https://github.com/SeaQL/sea-orm/pull/2145
* Rework SQLite type mappings https://github.com/SeaQL/sea-orm/pull/2077, https://github.com/SeaQL/sea-orm/pull/2078

### Upgrades

* Upgrade `time` to `0.3.36` https://github.com/SeaQL/sea-orm/pull/2267
* Upgrade `strum` to `0.26` https://github.com/SeaQL/sea-orm/pull/2088
* Upgrade `sea-schema` to `0.15.0`
* Upgrade `sea-query-binder` to `0.6.0`
* Upgrade `sea-query` to `0.31.0`

### House keeping

* Reduce warnings in integration tests https://github.com/SeaQL/sea-orm/pull/2177
* Improved Actix example to return 404 not found on unexpected inputs https://github.com/SeaQL/sea-orm/pull/2140
* Re-enable `rocket_okapi` example https://github.com/SeaQL/sea-orm/pull/2136

## 1.0.0-rc.7 - 2024-06-25

### Upgrades

* Upgrade `sea-query-binder` to `0.6.0-rc.4` https://github.com/SeaQL/sea-orm/pull/2267
* Upgrade `time` to `0.3.36` https://github.com/SeaQL/sea-orm/pull/2267

## 1.0.0-rc.6 - 2024-06-19

### Enhancements

* Added non-TLS runtime https://github.com/SeaQL/sea-orm/pull/2256
* Added `QuerySelect::tbl_col_as`
* Added `Insert::on_conflict_do_nothing` https://github.com/SeaQL/sea-orm/pull/2244
* Migration schema nullable column set NULL explicitly https://github.com/SeaQL/sea-orm/pull/2255

### Bug Fixes

* Set schema search path in Postgres without enclosing single quote https://github.com/SeaQL/sea-orm/pull/2241
* [sea-orm-cli] Generate `has_one` relation for foreign key of unique index / constraint https://github.com/SeaQL/sea-orm/pull/2254

## 1.0.0-rc.5 - 2024-05-29

### New Features

* Introduce `PrimaryKeyArity` with `ARITY` constant https://github.com/SeaQL/sea-orm/pull/2185
```rust
fn get_arity_of<E: EntityTrait>() -> usize {
    E::PrimaryKey::iter().count() // before; runtime
    <<E::PrimaryKey as PrimaryKeyTrait>::ValueType as PrimaryKeyArity>::ARITY // now; compile-time
}
```
* Associate `ActiveModel` to `EntityTrait` https://github.com/SeaQL/sea-orm/pull/2186
* [sea-orm-macros] Added `rename_all` attribute to `DeriveEntityModel` & `DeriveActiveEnum` https://github.com/SeaQL/sea-orm/pull/2170
```rust
#[derive(DeriveEntityModel)]
#[sea_orm(table_name = "user", rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key)]
    id: i32,
    first_name: String, // firstName
    #[sea_orm(column_name = "lAsTnAmE")]
    last_name: String, // lAsTnAmE
}

#[derive(EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)", rename_all = "camelCase")]
pub enum TestEnum {
    DefaultVariant, // defaultVariant
    #[sea_orm(rename = "kebab-case")]
    VariantKebabCase, // variant-kebab-case
    #[sea_orm(rename = "snake_case")]
    VariantSnakeCase, // variant_snake_case
    #[sea_orm(string_value = "CuStOmStRiNgVaLuE")]
    CustomStringValue, // CuStOmStRiNgVaLuE
}
```

### Enhancements

* Added `ActiveValue::set_if_not_equals()` https://github.com/SeaQL/sea-orm/pull/2194
* Added `ActiveValue::try_as_ref()` https://github.com/SeaQL/sea-orm/pull/2197
* Added `QuerySelect::order_by_with_nulls` https://github.com/SeaQL/sea-orm/pull/2228
* Expose `get_xxx_connection_pool` by default https://github.com/SeaQL/sea-orm/pull/2233

## 1.0.0-rc.4 - 2024-05-13

### Enhancements

* Added `QueryResult::column_names` https://github.com/SeaQL/sea-orm/pull/2148
* [sea-orm-macro] Add `@generated` in generated code https://github.com/SeaQL/sea-orm/pull/2199

### Upgrades

* Upgrade `sea-query` to `0.31.0-rc.6`
* Upgrade `sea-schema` to `0.15.0-rc.6`

### House Keeping

* Reduce warnings in integration tests https://github.com/SeaQL/sea-orm/pull/2177

## 1.0.0-rc.3 - 2024-03-26

### Enhancements

* [sea-orm-macro] Qualify traits in `DeriveActiveModel` macro https://github.com/SeaQL/sea-orm/pull/1665

## 1.0.0-rc.2 - 2024-03-15

### Breaking Changes

* Renamed `ConnectOptions::pool_options()` to `ConnectOptions::sqlx_pool_options()` https://github.com/SeaQL/sea-orm/pull/2145
* Made `sqlx_common` private, hiding `sqlx_error_to_xxx_err` https://github.com/SeaQL/sea-orm/pull/2145

### Enhancements

* [sea-orm-cli] Fix `migrate generate` on empty `mod.rs` files https://github.com/SeaQL/sea-orm/pull/2064
* `DerivePartialModel` macro attribute `entity` now supports `syn::Type` https://github.com/SeaQL/sea-orm/pull/2137
```rust
#[derive(DerivePartialModel)]
#[sea_orm(entity = "<entity::Model as ModelTrait>::Entity")]
struct EntityNameNotAIdent {
    #[sea_orm(from_col = "foo2")]
    _foo: i32,
    #[sea_orm(from_col = "bar2")]
    _bar: String,
}
```
* Added `RelationDef::from_alias()` https://github.com/SeaQL/sea-orm/pull/2146
```rust
let cf = Alias::new("cf");

assert_eq!(
    cake::Entity::find()
        .join_as(
            JoinType::LeftJoin,
            cake_filling::Relation::Cake.def().rev(),
            cf.clone()
        )
        .join(
            JoinType::LeftJoin,
            cake_filling::Relation::Filling.def().from_alias(cf)
        )
        .build(DbBackend::MySql)
        .to_string(),
    [
        "SELECT `cake`.`id`, `cake`.`name` FROM `cake`",
        "LEFT JOIN `cake_filling` AS `cf` ON `cake`.`id` = `cf`.`cake_id`",
        "LEFT JOIN `filling` ON `cf`.`filling_id` = `filling`.`id`",
    ]
    .join(" ")
);
```

### Upgrades

* Upgrade `sea-schema` to `0.15.0-rc.3`
* Upgrade `strum` to `0.26` https://github.com/SeaQL/sea-orm/pull/2088

### House keeping

* Improved Actix example to return 404 not found on unexpected inputs https://github.com/SeaQL/sea-orm/pull/2140
* Re-enable `rocket_okapi` example https://github.com/SeaQL/sea-orm/pull/2136

## 1.0.0-rc.1 - 2024-02-06

### New Features

* [sea-orm-migration] schema helper https://github.com/SeaQL/sea-orm/pull/2099
```rust
// Remember to import `sea_orm_migration::schema::*`
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(pk_auto(Users::Id)) // Primary key with auto-increment
                    .col(uuid(Users::Pid)) // UUID column
                    .col(string_uniq(Users::Email)) // String column with unique constraint
                    .col(string(Users::Password)) // String column
                    .col(string(Users::ApiKey).unique_key())
                    .col(string(Users::Name))
                    .col(string_null(Users::ResetToken)) // Nullable string column
                    .col(timestamp_null(Users::ResetSentAt)) // Nullable timestamp column
                    .col(string_null(Users::EmailVerificationToken))
                    .col(timestamp_null(Users::EmailVerificationSentAt))
                    .col(timestamp_null(Users::EmailVerifiedAt))
                    .to_owned(),
            )
            .await
    }

    // ...
}
```

### Breaking Changes

* Rework SQLite type mappings https://github.com/SeaQL/sea-orm/pull/2077, https://github.com/SeaQL/sea-orm/pull/2078
* Updated `sea-query` to `0.31`

## 0.12.14 - 2024-02-05

* Added feature flag `sqlite-use-returning-for-3_35` to use SQLite's returning https://github.com/SeaQL/sea-orm/pull/2070
* Added Loco example https://github.com/SeaQL/sea-orm/pull/2092

## 0.12.12 - 2024-01-22

### Bug Fixes

* [sea-orm-cli] Fix entity generation for non-alphanumeric enum variants https://github.com/SeaQL/sea-orm/pull/1821
* [sea-orm-cli] Fix entity generation for relations with composite keys https://github.com/SeaQL/sea-orm/pull/2071

### Enhancements

* Added `ConnectOptions::test_before_acquire`

## 0.12.11 - 2024-01-14

### New Features

* Added `desc` to `Cursor` paginator https://github.com/SeaQL/sea-orm/pull/2037

### Enhancements

* Improve query performance of `Paginator`'s `COUNT` query https://github.com/SeaQL/sea-orm/pull/2030
* Added SQLx slow statements logging to `ConnectOptions` https://github.com/SeaQL/sea-orm/pull/2055
* Added `QuerySelect::lock_with_behavior` https://github.com/SeaQL/sea-orm/pull/1867

### Bug Fixes

* [sea-orm-macro] Qualify types in `DeriveValueType` macro https://github.com/SeaQL/sea-orm/pull/2054

### House keeping

* Fix clippy warnings on 1.75 https://github.com/SeaQL/sea-orm/pull/2057

## 0.12.10 - 2023-12-14

### New Features

* [sea-orm-macro] Comment attribute for Entity (`#[sea_orm(comment = "action")]`); `create_table_from_entity` supports comment https://github.com/SeaQL/sea-orm/pull/2009
* Added "proxy" (feature flag `proxy`) to database backend https://github.com/SeaQL/sea-orm/pull/1881, https://github.com/SeaQL/sea-orm/pull/2000

### Enhancements

* Cast enums in `is_in` and `is_not_in` https://github.com/SeaQL/sea-orm/pull/2002

### Upgrades

* Updated `sea-query` to `0.30.5` https://github.com/SeaQL/sea-query/releases/tag/0.30.5

## 0.12.9 - 2023-12-08

### Enhancements

* Add source annotations to errors https://github.com/SeaQL/sea-orm/pull/1999

### Upgrades

* Updated `sea-query` to `0.30.4` https://github.com/SeaQL/sea-query/releases/tag/0.30.4

## 0.12.8 - 2023-12-04

### Enhancements

* Implement `StatementBuilder` for `sea_query::WithQuery` https://github.com/SeaQL/sea-orm/issues/1960

### Upgrades

* Upgrade `axum` example to `0.7` https://github.com/SeaQL/sea-orm/pull/1984

## 0.12.7 - 2023-11-22

### Enhancements

* Added method `expr_as_` that accepts `self` https://github.com/SeaQL/sea-orm/pull/1979

### Upgrades

* Updated `sea-query` to `0.30.3` https://github.com/SeaQL/sea-query/releases/tag/0.30.3

## 0.12.6 - 2023-11-13

### New Features

* Added `#[sea_orm(skip)]` for `FromQueryResult` derive macro https://github.com/SeaQL/sea-orm/pull/1954

## 0.12.5 - 2023-11-12

### Bug Fixes

* [sea-orm-cli] Fix duplicated active enum use statements on generated entities https://github.com/SeaQL/sea-orm/pull/1953
* [sea-orm-cli] Added `--enum-extra-derives` https://github.com/SeaQL/sea-orm/pull/1934
* [sea-orm-cli] Added `--enum-extra-attributes` https://github.com/SeaQL/sea-orm/pull/1952

## 0.12.4 - 2023-10-19

### New Features

* Add support for root JSON arrays https://github.com/SeaQL/sea-orm/pull/1898
    Now the following works (requires the `json-array` / `postgres-array` feature)!
```rust
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "json_struct_vec")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Json")]
    pub struct_vec: Vec<JsonColumn>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct JsonColumn {
    pub value: String,
}
```

### Enhancements

* Loader: use `ValueTuple` as hash key https://github.com/SeaQL/sea-orm/pull/1868

### Upgrades

* Updated `sea-query` to `0.30.2` https://github.com/SeaQL/sea-query/releases/tag/0.30.2

## 0.12.3 - 2023-09-22

### New Features

* [sea-orm-migration] Check if an index exists https://github.com/SeaQL/sea-orm/pull/1828
* Added `cursor_by` to `SelectTwo` https://github.com/SeaQL/sea-orm/pull/1826

### Enhancements

* [sea-orm-cli] Support generation of related entity with composite foreign key https://github.com/SeaQL/sea-orm/pull/1693

### Bug Fixes

* [sea-orm-macro] Fixed `DeriveValueType` by qualifying `QueryResult` https://github.com/SeaQL/sea-orm/pull/1855
* Fixed `Loader` panic on empty inputs

### Upgrades

* Upgraded `salvo` to `0.50`
* Upgraded `chrono` to `0.4.30` https://github.com/SeaQL/sea-orm/pull/1858
* Updated `sea-query` to `0.30.1`
* Updated `sea-schema` to `0.14.1`

### House keeping

* Added test cases for `find_xxx_related/linked` https://github.com/SeaQL/sea-orm/pull/1811

## 0.12.2 - 2023-08-04

### Enhancements

* Added support for Postgres arrays in `FromQueryResult` impl of `JsonValue` https://github.com/SeaQL/sea-orm/pull/1598

### Bug fixes

* Fixed `find_with_related` consolidation logic https://github.com/SeaQL/sea-orm/issues/1800

## 0.12.1 - 2023-07-27

+ `0.12.0-rc.1`: Yanked    
+ `0.12.0-rc.2`: 2023-05-19
+ `0.12.0-rc.3`: 2023-06-22
+ `0.12.0-rc.4`: 2023-07-08
+ `0.12.0-rc.5`: 2023-07-22

### New Features

* Added `MigratorTrait::migration_table_name()` method to configure the name of migration table https://github.com/SeaQL/sea-orm/pull/1511
```rust
#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    // Override the name of migration table
    fn migration_table_name() -> sea_orm::DynIden {
        Alias::new("override_migration_table_name").into_iden()
    }
    ...
}
```
* Added option to construct chained AND / OR join on condition https://github.com/SeaQL/sea-orm/pull/1433
```rust
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "cake")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // By default, it's
    // `JOIN `fruit` ON `cake`.`id` = `fruit`.`cake_id` AND `fruit`.`name` LIKE '%tropical%'`
    #[sea_orm(
        has_many = "super::fruit::Entity",
        on_condition = r#"super::fruit::Column::Name.like("%tropical%")"#
    )]
    TropicalFruit,
    // Or specify `condition_type = "any"` to override it,
    // `JOIN `fruit` ON `cake`.`id` = `fruit`.`cake_id` OR `fruit`.`name` LIKE '%tropical%'`
    #[sea_orm(
        has_many = "super::fruit::Entity",
        on_condition = r#"super::fruit::Column::Name.like("%tropical%")"#
        condition_type = "any",
    )]
    OrTropicalFruit,
}
```
* Supports entity with composite primary key of arity 12 https://github.com/SeaQL/sea-orm/pull/1508
    * `Identity` supports tuple of `DynIden` with arity up to 12
```rust
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "primary_key_of_12")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id_1: String,
    ...
    #[sea_orm(primary_key, auto_increment = false)]
    pub id_12: bool,
}
```
* Added macro `DerivePartialModel` https://github.com/SeaQL/sea-orm/pull/1597
```rust
#[derive(DerivePartialModel, FromQueryResult)]
#[sea_orm(entity = "Cake")]
struct PartialCake {
    name: String,
    #[sea_orm(
        from_expr = r#"SimpleExpr::FunctionCall(Func::upper(Expr::col((Cake, cake::Column::Name))))"#
    )]
    name_upper: String,
}

assert_eq!(
    cake::Entity::find()
        .into_partial_model::<PartialCake>()
        .into_statement(DbBackend::Sqlite)
        .to_string(),
    r#"SELECT "cake"."name", UPPER("cake"."name") AS "name_upper" FROM "cake""#
);
```
* Added `DbErr::sql_err()` method to convert error into common database errors `SqlErr`, such as unique constraint or foreign key violation errors. https://github.com/SeaQL/sea-orm/pull/1707
```rust
assert!(matches!(
    cake.into_active_model().insert(db).await
        .expect_err("Insert a row with duplicated primary key")
        .sql_err(),
    Some(SqlErr::UniqueConstraintViolation(_))
));

assert!(matches!(
    fk_cake.insert(db).await
        .expect_err("Insert a row with invalid foreign key")
        .sql_err(),
    Some(SqlErr::ForeignKeyConstraintViolation(_))
));
```
* Added `Select::find_with_linked`, similar to `find_with_related`: https://github.com/SeaQL/sea-orm/pull/1728, https://github.com/SeaQL/sea-orm/pull/1743
```rust
fn find_with_related<R>(self, r: R) -> SelectTwoMany<E, R>
    where R: EntityTrait, E: Related<R>;
fn find_with_linked<L, T>(self, l: L) -> SelectTwoMany<E, T>
    where L: Linked<FromEntity = E, ToEntity = T>, T: EntityTrait;

// boths yields `Vec<(E::Model, Vec<F::Model>)>`
```
* Added `DeriveValueType` derive macro for custom wrapper types, implementations of the required traits will be provided, you can customize the `column_type` and `array_type` if needed https://github.com/SeaQL/sea-orm/pull/1720
```rust
#[derive(DeriveValueType)]
#[sea_orm(array_type = "Int")]
pub struct Integer(i32);

#[derive(DeriveValueType)]
#[sea_orm(column_type = "Boolean", array_type = "Bool")]
pub struct Boolbean(pub String);

#[derive(DeriveValueType)]
pub struct StringVec(pub Vec<String>);
```
* Added `DeriveDisplay` derive macro to implements `std::fmt::Display` for enum https://github.com/SeaQL/sea-orm/pull/1726
```rust
#[derive(DeriveDisplay)]
enum DisplayTea {
    EverydayTea,
    #[sea_orm(display_value = "Breakfast Tea")]
    BreakfastTea,
}
assert_eq!(format!("{}", DisplayTea::EverydayTea), "EverydayTea");
assert_eq!(format!("{}", DisplayTea::BreakfastTea), "Breakfast Tea");
```
* Added `UpdateMany::exec_with_returning()` https://github.com/SeaQL/sea-orm/pull/1677
```rust
let models: Vec<Model> = Entity::update_many()
    .col_expr(Column::Values, Expr::expr(..))
    .exec_with_returning(db)
    .await?;
```
* Supporting `default_expr` in `DeriveEntityModel` https://github.com/SeaQL/sea-orm/pull/1474
```rust
#[derive(DeriveEntityModel)]
#[sea_orm(table_name = "hello")]
pub struct Model {
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub timestamp: DateTimeUtc,
}

assert_eq!(
    Column::Timestamp.def(),
    ColumnType::TimestampWithTimeZone.def()
        .default(Expr::current_timestamp())
);
```
* Introduced new `ConnAcquireErr` https://github.com/SeaQL/sea-orm/pull/1737
```rust
enum DbErr {
    ConnectionAcquire(ConnAcquireErr),
    ..
}

enum ConnAcquireErr {
    Timeout,
    ConnectionClosed,
}
```

#### Seaography

Added Seaography integration https://github.com/SeaQL/sea-orm/pull/1599

* Added `DeriveEntityRelated` macro which will implement `seaography::RelationBuilder` for `RelatedEntity` enumeration when the `seaography` feature is enabled
* Added generation of `seaography` related information to `sea-orm-codegen`.

    The `RelatedEntity` enum is added in entities files by `sea-orm-cli` when flag `seaography` is set:
```rust
/// SeaORM Entity
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
    #[sea_orm(entity = "super::bakery::Entity")]
    Bakery,
    #[sea_orm(entity = "super::cake_baker::Entity")]
    CakeBaker,
    #[sea_orm(entity = "super::cake::Entity")]
    Cake,
}
```
* Added [`seaography_example`](https://github.com/SeaQL/sea-orm/tree/master/examples/seaography_example)

### Enhancements

* Supports for partial select of `Option<T>` model field. A `None` value will be filled when the select result does not contain the `Option<T>` field without throwing an error. https://github.com/SeaQL/sea-orm/pull/1513
* [sea-orm-cli] the `migrate init` command will create a `.gitignore` file when the migration folder reside in a Git repository https://github.com/SeaQL/sea-orm/pull/1334
* [sea-orm-cli] Added support for generating migration of space separated name, for example executing `sea-orm-cli migrate generate "create accounts table"` command will create `m20230503_000000_create_accounts_table.rs` for you https://github.com/SeaQL/sea-orm/pull/1570
* Added `Migration::name()` and `Migration::status()` getters for the name and status of `sea_orm_migration::Migration` https://github.com/SeaQL/sea-orm/pull/1519
```rust
let migrations = Migrator::get_pending_migrations(db).await?;
assert_eq!(migrations.len(), 5);

let migration = migrations.get(0).unwrap();
assert_eq!(migration.name(), "m20220118_000002_create_fruit_table");
assert_eq!(migration.status(), MigrationStatus::Pending);
```
* The `postgres-array` feature will be enabled when `sqlx-postgres` backend is selected https://github.com/SeaQL/sea-orm/pull/1565
* Replace `String` parameters in API with `Into<String>` https://github.com/SeaQL/sea-orm/pull/1439
    * Implements `IntoMockRow` for any `BTreeMap` that is indexed by string `impl IntoMockRow for BTreeMap<T, Value> where T: Into<String>`
    * Converts any string value into `ConnectOptions` - `impl From<T> for ConnectOptions where T: Into<String>`
    * Changed the parameter of method `ConnectOptions::new(T) where T: Into<String>` to takes any string SQL
    * Changed the parameter of method `Statement::from_string(DbBackend, T) where T: Into<String>` to takes any string SQL
    * Changed the parameter of method `Statement::from_sql_and_values(DbBackend, T, I) where I: IntoIterator<Item = Value>, T: Into<String>` to takes any string SQL
    * Changed the parameter of method `Transaction::from_sql_and_values(DbBackend, T, I) where I: IntoIterator<Item = Value>, T: Into<String>` to takes any string SQL
    * Changed the parameter of method `ConnectOptions::set_schema_search_path(T) where T: Into<String>` to takes any string
    * Changed the parameter of method `ColumnTrait::like()`, `ColumnTrait::not_like()`, `ColumnTrait::starts_with()`, `ColumnTrait::ends_with()` and `ColumnTrait::contains()` to takes any string
* Added `sea_query::{DynIden, RcOrArc, SeaRc}` to entity prelude https://github.com/SeaQL/sea-orm/pull/1661
* Added `expr`, `exprs` and `expr_as` methods to `QuerySelect` trait https://github.com/SeaQL/sea-orm/pull/1702
* Added `DatabaseConnection::ping` https://github.com/SeaQL/sea-orm/pull/1627
```rust
|db: DatabaseConnection| {
    assert!(db.ping().await.is_ok());
    db.clone().close().await;
    assert!(matches!(db.ping().await, Err(DbErr::ConnectionAcquire)));
}
```
* Added `TryInsert` that does not panic on empty inserts https://github.com/SeaQL/sea-orm/pull/1708
```rust
// now, you can do:
let res = Bakery::insert_many(std::iter::empty())
    .on_empty_do_nothing()
    .exec(db)
    .await;

assert!(matches!(res, Ok(TryInsertResult::Empty)));
```
* Insert on conflict do nothing to return Ok https://github.com/SeaQL/sea-orm/pull/1712
```rust
let on = OnConflict::column(Column::Id).do_nothing().to_owned();

// Existing behaviour
let res = Entity::insert_many([..]).on_conflict(on).exec(db).await;
assert!(matches!(res, Err(DbErr::RecordNotInserted)));

// New API; now you can:
let res =
Entity::insert_many([..]).on_conflict(on).do_nothing().exec(db).await;
assert!(matches!(res, Ok(TryInsertResult::Conflicted)));
```

### Bug Fixes

* Fixed `DeriveActiveEnum` throwing errors because `string_value` consists non-UAX#31 compliant characters https://github.com/SeaQL/sea-orm/pull/1374
```rust
#[derive(EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(None)")]
pub enum StringValue {
    #[sea_orm(string_value = "")]
    Member1,
    #[sea_orm(string_value = "$$")]
    Member2,
}
// will now produce the following enum:
pub enum StringValueVariant {
    __Empty,
    _0x240x24,
}
```
* [sea-orm-cli] Fix Postgres enum arrays https://github.com/SeaQL/sea-orm/pull/1678
* [sea-orm-cli] The implementation of `Related<R>` with `via` and `to` methods will not be generated if there exists multiple paths via an intermediate table https://github.com/SeaQL/sea-orm/pull/1435
* [sea-orm-cli] fixed entity generation includes partitioned tables https://github.com/SeaQL/sea-orm/issues/1582, https://github.com/SeaQL/sea-schema/pull/105
* Fixed `ActiveEnum::db_type()` return type does not implement `ColumnTypeTrait` https://github.com/SeaQL/sea-orm/pull/1576
* Resolved `insert_many` failing if the models iterator is empty https://github.com/SeaQL/sea-orm/issues/873

### Breaking changes

* Supports for partial select of `Option<T>` model field. A `None` value will be filled when the select result does not contain the `Option<T>` field instead of throwing an error. https://github.com/SeaQL/sea-orm/pull/1513
* Replaced `sea-strum` dependency with upstream `strum` in `sea-orm` https://github.com/SeaQL/sea-orm/pull/1535
    * Added `derive` and `strum` features to `sea-orm-macros`
    * The derive macro `EnumIter` is now shipped by `sea-orm-macros`
* Added a new variant `Many` to `Identity` https://github.com/SeaQL/sea-orm/pull/1508
* Enabled `hashable-value` feature in SeaQuery, thus `Value::Float(NaN) == Value::Float(NaN)` would be true https://github.com/SeaQL/sea-orm/pull/1728, https://github.com/SeaQL/sea-orm/pull/1743
* The `DeriveActiveEnum` derive macro no longer implement `std::fmt::Display`. You can use the new `DeriveDisplay` macro https://github.com/SeaQL/sea-orm/pull/1726
* `sea-query/derive` is no longer enabled by `sea-orm`, as such, `Iden` no longer works as a derive macro (it's still a trait). Instead, we are shipping a new macro `DeriveIden` https://github.com/SeaQL/sea-orm/pull/1740 https://github.com/SeaQL/sea-orm/pull/1755
```rust
// then:

#[derive(Iden)]
#[iden = "category"]
pub struct CategoryEnum;

#[derive(Iden)]
pub enum Tea {
    Table,
    #[iden = "EverydayTea"]
    EverydayTea,
}

// now:

#[derive(DeriveIden)]
#[sea_orm(iden = "category")]
pub struct CategoryEnum;

#[derive(DeriveIden)]
pub enum Tea {
    Table,
    #[sea_orm(iden = "EverydayTea")]
    EverydayTea,
}
```
* Definition of `DbErr::ConnectionAcquire` changed to `ConnectionAcquire(ConnAcquireErr)` https://github.com/SeaQL/sea-orm/pull/1737
* `FromJsonQueryResult` removed from entity prelude

### Upgrades

* Upgraded `sqlx` to `0.7` https://github.com/SeaQL/sea-orm/pull/1742
* Upgraded `sea-query` to `0.30` https://github.com/SeaQL/sea-orm/pull/1742
* Upgraded `sea-schema` to `0.14` https://github.com/SeaQL/sea-orm/pull/1742
* Upgraded `syn` to `2` https://github.com/SeaQL/sea-orm/pull/1713
* Upgraded `heck` to `0.4` https://github.com/SeaQL/sea-orm/pull/1520, https://github.com/SeaQL/sea-orm/pull/1544
* Upgraded `strum` to `0.25` https://github.com/SeaQL/sea-orm/pull/1752
* Upgraded `clap` to `4.3` https://github.com/SeaQL/sea-orm/pull/1468
* Upgraded `ouroboros` to `0.17` https://github.com/SeaQL/sea-orm/pull/1724

### House keeping

* Replaced `bae` with `sea-bae` https://github.com/SeaQL/sea-orm/pull/1739

**Full Changelog**: https://github.com/SeaQL/sea-orm/compare/0.11.1...0.12.1

## 0.11.3 - 2023-04-24

### Enhancements

* Re-export `sea_orm::ConnectionTrait` in `sea_orm_migration::prelude` https://github.com/SeaQL/sea-orm/pull/1577
* Support generic structs in `FromQueryResult` derive macro https://github.com/SeaQL/sea-orm/pull/1464, https://github.com/SeaQL/sea-orm/pull/1603
```rust
#[derive(FromQueryResult)]
struct GenericTest<T: TryGetable> {
    foo: i32,
    bar: T,
}
```
```rust
trait MyTrait {
    type Item: TryGetable;
}

#[derive(FromQueryResult)]
struct TraitAssociateTypeTest<T>
where
    T: MyTrait,
{
    foo: T::Item,
}
```

### Bug Fixes

* Fixed https://github.com/SeaQL/sea-orm/issues/1608 by pinning the version of `tracing-subscriber` dependency to 0.3.17 https://github.com/SeaQL/sea-orm/pull/1609

## 0.11.2 - 2023-03-25

### Enhancements

* Enable required `syn` features https://github.com/SeaQL/sea-orm/pull/1556
* Re-export `sea_query::BlobSize` in `sea_orm::entity::prelude` https://github.com/SeaQL/sea-orm/pull/1548

## 0.11.1 - 2023-03-10

### Bug Fixes

* Fixes `DeriveActiveEnum` (by qualifying `ColumnTypeTrait::def`) https://github.com/SeaQL/sea-orm/issues/1478
* The CLI command `sea-orm-cli generate entity -u '<DB-URL>'` will now generate the following code for each `Binary` or `VarBinary` columns in compact format https://github.com/SeaQL/sea-orm/pull/1529
```rust
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "binary")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(None))")]
    pub binary: Vec<u8>,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(Some(10)))")]
    pub binary_10: Vec<u8>,
    #[sea_orm(column_type = "Binary(BlobSize::Tiny)")]
    pub binary_tiny: Vec<u8>,
    #[sea_orm(column_type = "Binary(BlobSize::Medium)")]
    pub binary_medium: Vec<u8>,
    #[sea_orm(column_type = "Binary(BlobSize::Long)")]
    pub binary_long: Vec<u8>,
    #[sea_orm(column_type = "VarBinary(10)")]
    pub var_binary: Vec<u8>,
}
```
* The CLI command `sea-orm-cli generate entity -u '<DB-URL>' --expanded-format` will now generate the following code for each `Binary` or `VarBinary` columns in expanded format https://github.com/SeaQL/sea-orm/pull/1529
```rust
impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::Integer.def(),
            Self::Binary => ColumnType::Binary(sea_orm::sea_query::BlobSize::Blob(None)).def(),
            Self::Binary10 => {
                ColumnType::Binary(sea_orm::sea_query::BlobSize::Blob(Some(10u32))).def()
            }
            Self::BinaryTiny => ColumnType::Binary(sea_orm::sea_query::BlobSize::Tiny).def(),
            Self::BinaryMedium => ColumnType::Binary(sea_orm::sea_query::BlobSize::Medium).def(),
            Self::BinaryLong => ColumnType::Binary(sea_orm::sea_query::BlobSize::Long).def(),
            Self::VarBinary => ColumnType::VarBinary(10u32).def(),
        }
    }
}
```
* Fix missing documentation on type generated by derive macros https://github.com/SeaQL/sea-orm/pull/1522, https://github.com/SeaQL/sea-orm/pull/1531

## 0.11.0 - 2023-02-07

+ 2023-02-02: `0.11.0-rc.1`
+ 2023-02-04: `0.11.0-rc.2`

### New Features

#### SeaORM Core

* Simple data loader https://github.com/SeaQL/sea-orm/pull/1238, https://github.com/SeaQL/sea-orm/pull/1443
* Transactions Isolation level and Access mode https://github.com/SeaQL/sea-orm/pull/1230
* Support various UUID formats that are available in `uuid::fmt` module https://github.com/SeaQL/sea-orm/pull/1325
* Support Vector of enum for Postgres https://github.com/SeaQL/sea-orm/pull/1210
* Support `ActiveEnum` field as primary key https://github.com/SeaQL/sea-orm/pull/1414
* Casting columns as a different data type on select, insert and update https://github.com/SeaQL/sea-orm/pull/1304
* Methods of `ActiveModelBehavior` receive db connection as a parameter https://github.com/SeaQL/sea-orm/pull/1145, https://github.com/SeaQL/sea-orm/pull/1328
* Added `execute_unprepared` method to `DatabaseConnection` and `DatabaseTransaction` https://github.com/SeaQL/sea-orm/pull/1327
* Added `Select::into_tuple` to select rows as tuples (instead of defining a custom Model) https://github.com/SeaQL/sea-orm/pull/1311

#### SeaORM CLI

* Generate `#[serde(skip_deserializing)]` for primary key columns https://github.com/SeaQL/sea-orm/pull/846, https://github.com/SeaQL/sea-orm/pull/1186, https://github.com/SeaQL/sea-orm/pull/1318
* Generate `#[serde(skip)]` for hidden columns https://github.com/SeaQL/sea-orm/pull/1171, https://github.com/SeaQL/sea-orm/pull/1320
* Generate entity with extra derives and attributes for model struct https://github.com/SeaQL/sea-orm/pull/1124, https://github.com/SeaQL/sea-orm/pull/1321

#### SeaORM Migration

* Migrations are now performed inside a transaction for Postgres https://github.com/SeaQL/sea-orm/pull/1379

### Enhancements

* Refactor schema module to expose functions for database alteration https://github.com/SeaQL/sea-orm/pull/1256
* Generate compact entity with `#[sea_orm(column_type = "JsonBinary")]` macro attribute https://github.com/SeaQL/sea-orm/pull/1346
* `MockDatabase::append_exec_results()`, `MockDatabase::append_query_results()`, `MockDatabase::append_exec_errors()` and `MockDatabase::append_query_errors()` take any types implemented `IntoIterator` trait https://github.com/SeaQL/sea-orm/pull/1367
* `find_by_id` and `delete_by_id` take any `Into` primary key value https://github.com/SeaQL/sea-orm/pull/1362
* `QuerySelect::offset` and `QuerySelect::limit` takes in `Into<Option<u64>>` where `None` would reset them https://github.com/SeaQL/sea-orm/pull/1410
* Added `DatabaseConnection::close` https://github.com/SeaQL/sea-orm/pull/1236
* Added `is_null` getter for `ColumnDef` https://github.com/SeaQL/sea-orm/pull/1381
* Added `ActiveValue::reset` to convert `Unchanged` into `Set` https://github.com/SeaQL/sea-orm/pull/1177
* Added `QueryTrait::apply_if` to optionally apply a filter https://github.com/SeaQL/sea-orm/pull/1415
* Added the `sea-orm-internal` feature flag to expose some SQLx types
    * Added `DatabaseConnection::get_*_connection_pool()` for accessing the inner SQLx connection pool https://github.com/SeaQL/sea-orm/pull/1297
    * Re-exporting SQLx errors https://github.com/SeaQL/sea-orm/pull/1434

### Upgrades

* Upgrade `axum` to `0.6.1` https://github.com/SeaQL/sea-orm/pull/1285
* Upgrade `sea-query` to `0.28` https://github.com/SeaQL/sea-orm/pull/1366
* Upgrade `sea-query-binder` to `0.3` https://github.com/SeaQL/sea-orm/pull/1366
* Upgrade `sea-schema` to `0.11` https://github.com/SeaQL/sea-orm/pull/1366

### House Keeping

* Fixed all clippy warnings as of `1.67.0` https://github.com/SeaQL/sea-orm/pull/1426
* Removed dependency where not needed https://github.com/SeaQL/sea-orm/pull/1213
* Disabled default features and enabled only the needed ones https://github.com/SeaQL/sea-orm/pull/1300
* Cleanup panic and unwrap https://github.com/SeaQL/sea-orm/pull/1231
* Cleanup the use of `vec!` macro https://github.com/SeaQL/sea-orm/pull/1367

### Bug Fixes

* [sea-orm-cli] Propagate error on the spawned child processes https://github.com/SeaQL/sea-orm/pull/1402
    * Fixes sea-orm-cli errors exit with error code 0 https://github.com/SeaQL/sea-orm/issues/1342
* Fixes `DeriveColumn` (by qualifying `IdenStatic::as_str`) https://github.com/SeaQL/sea-orm/pull/1280
* Prevent returning connections to pool with a positive transaction depth https://github.com/SeaQL/sea-orm/pull/1283
* Postgres insert many will throw `RecordNotInserted` error if non of them are being inserted https://github.com/SeaQL/sea-orm/pull/1021
    * Fixes inserting active models by `insert_many` with `on_conflict` and `do_nothing` panics if no rows are inserted on Postgres https://github.com/SeaQL/sea-orm/issues/899
* Don't call `last_insert_id` if not needed https://github.com/SeaQL/sea-orm/pull/1403
    * Fixes hitting 'negative last_insert_rowid' panic with Sqlite https://github.com/SeaQL/sea-orm/issues/1357
* Noop when update without providing any values https://github.com/SeaQL/sea-orm/pull/1384
    * Fixes Syntax Error when saving active model that sets nothing https://github.com/SeaQL/sea-orm/pull/1376

### Breaking Changes

* [sea-orm-cli] Enable --universal-time by default https://github.com/SeaQL/sea-orm/pull/1420
* Added `RecordNotInserted` and `RecordNotUpdated` to `DbErr`
* Added `ConnectionTrait::execute_unprepared` method https://github.com/SeaQL/sea-orm/pull/1327
* As part of https://github.com/SeaQL/sea-orm/pull/1311, the required method of `TryGetable` changed:
```rust
// then
fn try_get(res: &QueryResult, pre: &str, col: &str) -> Result<Self, TryGetError>;
// now; ColIdx can be `&str` or `usize`
fn try_get_by<I: ColIdx>(res: &QueryResult, index: I) -> Result<Self, TryGetError>;
```
So if you implemented it yourself:
```patch
impl TryGetable for XXX {
-   fn try_get(res: &QueryResult, pre: &str, col: &str) -> Result<Self, TryGetError> {
+   fn try_get_by<I: sea_orm::ColIdx>(res: &QueryResult, idx: I) -> Result<Self, TryGetError> {
-       let value: YYY = res.try_get(pre, col).map_err(TryGetError::DbErr)?;
+       let value: YYY = res.try_get_by(idx).map_err(TryGetError::DbErr)?;
        ..
    }
}
```
* The `ActiveModelBehavior` trait becomes async trait https://github.com/SeaQL/sea-orm/pull/1328.
If you overridden the default `ActiveModelBehavior` implementation:
```rust
#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        // ...
    }

    // ...
}
```
* `DbErr::RecordNotFound("None of the database rows are affected")` is moved to a dedicated error variant `DbErr::RecordNotUpdated` https://github.com/SeaQL/sea-orm/pull/1425
```rust
let res = Update::one(cake::ActiveModel {
        name: Set("Cheese Cake".to_owned()),
        ..model.into_active_model()
    })
    .exec(&db)
    .await;

// then
assert_eq!(
    res,
    Err(DbErr::RecordNotFound(
        "None of the database rows are affected".to_owned()
    ))
);

// now
assert_eq!(res, Err(DbErr::RecordNotUpdated));
```
* `sea_orm::ColumnType` was replaced by `sea_query::ColumnType` https://github.com/SeaQL/sea-orm/pull/1395
    * Method `ColumnType::def` was moved to `ColumnTypeTrait`
    * `ColumnType::Binary` becomes a tuple variant which takes in additional option `sea_query::BlobSize`
    * `ColumnType::Custom` takes a `sea_query::DynIden` instead of `String` and thus a new method `custom` is added (note the lowercase)
```diff
// Compact Entity
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "fruit")]
pub struct Model {
-   #[sea_orm(column_type = r#"Custom("citext".to_owned())"#)]
+   #[sea_orm(column_type = r#"custom("citext")"#)]
    pub column: String,
}
```
```diff
// Expanded Entity
impl ColumnTrait for Column {
    type EntityName = Entity;

    fn def(&self) -> ColumnDef {
        match self {
-           Self::Column => ColumnType::Custom("citext".to_owned()).def(),
+           Self::Column => ColumnType::custom("citext").def(),
        }
    }
}
```

### Miscellaneous

* Fixed a small typo https://github.com/SeaQL/sea-orm/pull/1391
* `axum` example should use tokio runtime https://github.com/SeaQL/sea-orm/pull/1428

**Full Changelog**: https://github.com/SeaQL/sea-orm/compare/0.10.0...0.11.0

## 0.10.7 - 2023-01-19

### Bug Fixes

* Inserting active models by `insert_many` with `on_conflict` and `do_nothing` panics if no rows are inserted on Postgres https://github.com/SeaQL/sea-orm/issues/899
* Hitting 'negative last_insert_rowid' panic with Sqlite https://github.com/SeaQL/sea-orm/issues/1357

## 0.10.6 - 2022-12-23

### Enhancements

* Cast enum values when constructing update many query https://github.com/SeaQL/sea-orm/pull/1178

### Bug Fixes

* Fixes `DeriveColumn` (by qualifying `IdenStatic::as_str`) https://github.com/SeaQL/sea-orm/pull/1280
* Prevent returning connections to pool with a positive transaction depth https://github.com/SeaQL/sea-orm/pull/1283
* [sea-orm-codegen] Skip implementing Related if the same related entity is being referenced by a conjunct relation https://github.com/SeaQL/sea-orm/pull/1298
* [sea-orm-cli] CLI depends on codegen of the same version https://github.com/SeaQL/sea-orm/pull/1299/

## 0.10.5 - 2022-12-02

### New Features

* Add `QuerySelect::columns` method - select multiple columns https://github.com/SeaQL/sea-orm/pull/1264
* Transactions Isolation level and Access mode https://github.com/SeaQL/sea-orm/pull/1230

### Bug Fixes

* `DeriveEntityModel` derive macro: when parsing field type, always treat field with `Option<T>` as nullable column https://github.com/SeaQL/sea-orm/pull/1257

### Enhancements

* [sea-orm-cli] Generate `Related` implementation for many-to-many relation with extra columns https://github.com/SeaQL/sea-orm/pull/1260
* Optimize the default implementation of `TryGetableFromJson::try_get_from_json()` - deserializing into `Self` directly without the need of a intermediate `serde_json::Value` https://github.com/SeaQL/sea-orm/pull/1249

## 0.10.4 - 2022-11-24

### Bug Fixes

* Fix DeriveActiveEnum expand enum variant starts with number https://github.com/SeaQL/sea-orm/pull/1219
* [sea-orm-cli] Generate entity file for specified tables only https://github.com/SeaQL/sea-orm/pull/1245
* Support appending `DbErr` to `MockDatabase` https://github.com/SeaQL/sea-orm/pull/1241

### Enhancements

* Filter rows with `IS IN` enum values expression https://github.com/SeaQL/sea-orm/pull/1183
* [sea-orm-cli] Generate entity with relation variant order by name of reference table https://github.com/SeaQL/sea-orm/pull/1229

## 0.10.3 - 2022-11-14

### Bug Fixes

* [sea-orm-cli] Set search path when initializing Postgres connection for CLI generate entity https://github.com/SeaQL/sea-orm/pull/1212
* [sea-orm-cli] Generate `_` prefix to enum variant starts with number https://github.com/SeaQL/sea-orm/pull/1211
* Fix composite key cursor pagination https://github.com/SeaQL/sea-orm/pull/1216
    + The logic for single-column primary key was correct, but for composite keys the logic was incorrect

### Enhancements

* Added `Insert::exec_without_returning` https://github.com/SeaQL/sea-orm/pull/1208

### House Keeping

* Remove dependency when not needed https://github.com/SeaQL/sea-orm/pull/1207

## 0.10.2 - 2022-11-06

### Enhancements

* [sea-orm-rocket] added `sqlx_logging` to `Config` https://github.com/SeaQL/sea-orm/pull/1192
* Collecting metrics for `query_one/all` https://github.com/SeaQL/sea-orm/pull/1165
* Use GAT to elide `StreamTrait` lifetime https://github.com/SeaQL/sea-orm/pull/1161

### Bug Fixes

* corrected the error name `UpdateGetPrimaryKey` https://github.com/SeaQL/sea-orm/pull/1180

### Upgrades

* Update MSRV to 1.65

## 0.10.1 - 2022-10-27

### Enhancements

* [sea-orm-cli] Escape module name defined with Rust keywords https://github.com/SeaQL/sea-orm/pull/1052
* [sea-orm-cli] Check to make sure migration name doesn't contain hyphen `-` in it https://github.com/SeaQL/sea-orm/pull/879, https://github.com/SeaQL/sea-orm/pull/1155
* Support `time` crate for SQLite https://github.com/SeaQL/sea-orm/pull/995

### Bug Fixes

* [sea-orm-cli] Generate `Related` for m-to-n relation https://github.com/SeaQL/sea-orm/pull/1075
* [sea-orm-cli] Generate model entity with Postgres Enum field https://github.com/SeaQL/sea-orm/pull/1153
* [sea-orm-cli] Migrate up command apply all pending migrations https://github.com/SeaQL/sea-orm/pull/1010
* [sea-orm-cli] Conflicting short flag `-u` when executing `migrate generate` command https://github.com/SeaQL/sea-orm/pull/1157
* Prefix the usage of types with `sea_orm::` inside `DeriveActiveEnum` derive macros https://github.com/SeaQL/sea-orm/pull/1146, https://github.com/SeaQL/sea-orm/pull/1154
* [sea-orm-cli] Generate model with `Vec<f32>` or `Vec<f64>` should not derive `Eq` on the model struct https://github.com/SeaQL/sea-orm/pull/1158

### House Keeping

* [sea-orm-cli] [sea-orm-migration] Add `cli` feature to optionally include dependencies that are required by the CLI https://github.com/SeaQL/sea-orm/pull/978

### Upgrades

* Upgrade `sea-schema` to 0.10.2 https://github.com/SeaQL/sea-orm/pull/1153

## 0.10.0 - 2022-10-23

### New Features

* Better error types (carrying SQLx Error) https://github.com/SeaQL/sea-orm/pull/1002
* Support array datatype in PostgreSQL https://github.com/SeaQL/sea-orm/pull/1132
* [sea-orm-cli] Generate entity files as a library or module https://github.com/SeaQL/sea-orm/pull/953
* [sea-orm-cli] Generate a new migration template with name prefix of unix timestamp https://github.com/SeaQL/sea-orm/pull/947
* [sea-orm-cli] Generate migration in modules https://github.com/SeaQL/sea-orm/pull/933
* [sea-orm-cli] Generate `DeriveRelation` on empty `Relation` enum https://github.com/SeaQL/sea-orm/pull/1019
* [sea-orm-cli] Generate entity derive `Eq` if possible https://github.com/SeaQL/sea-orm/pull/988
* [sea-orm-cli] Run migration on any PostgreSQL schema https://github.com/SeaQL/sea-orm/pull/1056

### Enhancements

* Support `distinct` & `distinct_on` expression https://github.com/SeaQL/sea-orm/pull/902
* `fn column()` also handle enum type https://github.com/SeaQL/sea-orm/pull/973
* Added `acquire_timeout` on `ConnectOptions` https://github.com/SeaQL/sea-orm/pull/897
* [sea-orm-cli] `migrate fresh` command will drop all PostgreSQL types https://github.com/SeaQL/sea-orm/pull/864, https://github.com/SeaQL/sea-orm/pull/991
* Better compile error for entity without primary key https://github.com/SeaQL/sea-orm/pull/1020
* Added blanket implementations of `IntoActiveValue` for `Option` values https://github.com/SeaQL/sea-orm/pull/833
* Added `into_model` & `into_json` to `Cursor` https://github.com/SeaQL/sea-orm/pull/1112
* Added `set_schema_search_path` method to `ConnectOptions` for setting schema search path of PostgreSQL connection https://github.com/SeaQL/sea-orm/pull/1056
* Serialize `time` types as `serde_json::Value` https://github.com/SeaQL/sea-orm/pull/1042
* Implements `fmt::Display` for `ActiveEnum` https://github.com/SeaQL/sea-orm/pull/986
* Implements `TryFrom<ActiveModel>` for `Model` https://github.com/SeaQL/sea-orm/pull/990

### Bug Fixes

* Trim spaces when paginating raw SQL https://github.com/SeaQL/sea-orm/pull/1094

### Breaking Changes

* Replaced `usize` with `u64` in `PaginatorTrait` https://github.com/SeaQL/sea-orm/pull/789
* Type signature of `DbErr` changed as a result of https://github.com/SeaQL/sea-orm/pull/1002
* `ColumnType::Enum` structure changed:
```rust
enum ColumnType {
    // then
    Enum(String, Vec<String>)

    // now
    Enum {
        /// Name of enum
        name: DynIden,
        /// Variants of enum
        variants: Vec<DynIden>,
    }
    ...
}

// example

#[derive(Iden)]
enum TeaEnum {
    #[iden = "tea"]
    Enum,
    #[iden = "EverydayTea"]
    EverydayTea,
    #[iden = "BreakfastTea"]
    BreakfastTea,
}

// then
ColumnDef::new(active_enum_child::Column::Tea)
    .enumeration("tea", vec!["EverydayTea", "BreakfastTea"])

// now
ColumnDef::new(active_enum_child::Column::Tea)
    .enumeration(TeaEnum::Enum, [TeaEnum::EverydayTea, TeaEnum::BreakfastTea])
```

* A new method `array_type` was added to `ValueType`:
```rust
impl sea_orm::sea_query::ValueType for MyType {
    fn array_type() -> sea_orm::sea_query::ArrayType {
        sea_orm::sea_query::ArrayType::TypeName
    }
    ...
}
```

* `ActiveEnum::name()` changed return type to `DynIden`:
```rust
#[derive(Debug, Iden)]
#[iden = "category"]
pub struct CategoryEnum;

impl ActiveEnum for Category {
    // then
    fn name() -> String {
        "category".to_owned()
    }

    // now
    fn name() -> DynIden {
        SeaRc::new(CategoryEnum)
    }
    ...
}
```

### House Keeping

* Documentation grammar fixes https://github.com/SeaQL/sea-orm/pull/1050
* Replace `dotenv` with `dotenvy` in examples https://github.com/SeaQL/sea-orm/pull/1085
* Exclude test_cfg module from SeaORM https://github.com/SeaQL/sea-orm/pull/1077

### Integration

* Support `rocket_okapi` https://github.com/SeaQL/sea-orm/pull/1071

### Upgrades

* Upgrade `sea-query` to 0.26 https://github.com/SeaQL/sea-orm/pull/985

**Full Changelog**: https://github.com/SeaQL/sea-orm/compare/0.9.0...0.10.0

## 0.9.3 - 2022-09-30

### Enhancements

* `fn column()` also handle enum type https://github.com/SeaQL/sea-orm/pull/973
* Generate migration in modules https://github.com/SeaQL/sea-orm/pull/933
* Generate `DeriveRelation` on empty `Relation` enum https://github.com/SeaQL/sea-orm/pull/1019
* Documentation grammar fixes https://github.com/SeaQL/sea-orm/pull/1050

### Bug Fixes

* Implement `IntoActiveValue` for `time` types https://github.com/SeaQL/sea-orm/pull/1041
* Fixed module import for `FromJsonQueryResult` derive macro https://github.com/SeaQL/sea-orm/pull/1081

## 0.9.2 - 2022-08-20

### Enhancements

* [sea-orm-cli] Migrator CLI handles init and generate commands https://github.com/SeaQL/sea-orm/pull/931
* [sea-orm-cli] added `with-copy-enums` flag to conditional derive `Copy` on `ActiveEnum` https://github.com/SeaQL/sea-orm/pull/936

### House Keeping

* Exclude `chrono` default features https://github.com/SeaQL/sea-orm/pull/950
* Set minimal rustc version to `1.60` https://github.com/SeaQL/sea-orm/pull/938
* Update `sea-query` to `0.26.3`

### Notes

In this minor release, we removed `time` v0.1 from the dependency graph

## 0.9.1 - 2022-07-22

### Enhancements

* [sea-orm-cli] Codegen support for `VarBinary` column type https://github.com/SeaQL/sea-orm/pull/746
* [sea-orm-cli] Generate entity for SYSTEM VERSIONED tables on MariaDB https://github.com/SeaQL/sea-orm/pull/876

### Bug Fixes

* `RelationDef` & `RelationBuilder` should be `Send` & `Sync` https://github.com/SeaQL/sea-orm/pull/898

### House Keeping

* Remove unnecessary `async_trait` https://github.com/SeaQL/sea-orm/pull/737

## 0.9.0 - 2022-07-17

### New Features

* Cursor pagination https://github.com/SeaQL/sea-orm/pull/822
* Custom join on conditions https://github.com/SeaQL/sea-orm/pull/793
* `DeriveMigrationName` and `sea_orm_migration::util::get_file_stem` https://github.com/SeaQL/sea-orm/pull/736
* `FromJsonQueryResult` for deserializing `Json` from query result https://github.com/SeaQL/sea-orm/pull/794

### Enhancements

* Added `sqlx_logging_level` to `ConnectOptions` https://github.com/SeaQL/sea-orm/pull/800
* Added `num_items_and_pages` to `Paginator` https://github.com/SeaQL/sea-orm/pull/768
* Added `TryFromU64` for `time` https://github.com/SeaQL/sea-orm/pull/849
* Added `Insert::on_conflict` https://github.com/SeaQL/sea-orm/pull/791
* Added `QuerySelect::join_as` and `QuerySelect::join_as_rev` https://github.com/SeaQL/sea-orm/pull/852
* Include column name in `TryGetError::Null` https://github.com/SeaQL/sea-orm/pull/853
* [sea-orm-cli] Improve logging https://github.com/SeaQL/sea-orm/pull/735
* [sea-orm-cli] Generate enum with numeric like variants https://github.com/SeaQL/sea-orm/pull/588
* [sea-orm-cli] Allow old pending migration to be applied https://github.com/SeaQL/sea-orm/pull/755
* [sea-orm-cli] Skip generating entity for ignored tables https://github.com/SeaQL/sea-orm/pull/837
* [sea-orm-cli] Generate code for `time` crate https://github.com/SeaQL/sea-orm/pull/724
* [sea-orm-cli] Add various blob column types https://github.com/SeaQL/sea-orm/pull/850
* [sea-orm-cli] Generate entity files with Postgres's schema name https://github.com/SeaQL/sea-orm/pull/422

### Upgrades

* Upgrade `clap` to 3.2 https://github.com/SeaQL/sea-orm/pull/706
* Upgrade `time` to 0.3 https://github.com/SeaQL/sea-orm/pull/834
* Upgrade `sqlx` to 0.6 https://github.com/SeaQL/sea-orm/pull/834
* Upgrade `uuid` to 1.0 https://github.com/SeaQL/sea-orm/pull/834
* Upgrade `sea-query` to 0.26 https://github.com/SeaQL/sea-orm/pull/834
* Upgrade `sea-schema` to 0.9 https://github.com/SeaQL/sea-orm/pull/834

### House Keeping

* Refactor stream metrics https://github.com/SeaQL/sea-orm/pull/778

### Bug Fixes

* [sea-orm-cli] skip checking connection string for credentials https://github.com/SeaQL/sea-orm/pull/851

### Breaking Changes

* `SelectTwoMany::one()` has been dropped https://github.com/SeaQL/sea-orm/pull/813, you can get `(Entity, Vec<RelatedEntity>)` by first querying a single model from Entity, then use [`ModelTrait::find_related`] on the model.
* #### Feature flag revamp
    We now adopt the [weak dependency](https://blog.rust-lang.org/2022/04/07/Rust-1.60.0.html#new-syntax-for-cargo-features) syntax in Cargo. That means the flags `["sqlx-json", "sqlx-chrono", "sqlx-decimal", "sqlx-uuid", "sqlx-time"]` are not needed and now removed. Instead, `with-time` will enable `sqlx?/time` only if `sqlx` is already enabled. As a consequence, now the features `with-json`, `with-chrono`, `with-rust_decimal`, `with-uuid`, `with-time` will not be enabled as a side-effect of enabling `sqlx`.

**Full Changelog**: https://github.com/SeaQL/sea-orm/compare/0.8.0...0.9.0

## sea-orm-migration 0.8.3

* Removed `async-std` from dependency https://github.com/SeaQL/sea-orm/pull/758

## 0.8.0 - 2022-05-10

### New Features
* [sea-orm-cli] `sea migrate generate` to generate a new, empty migration file https://github.com/SeaQL/sea-orm/pull/656

### Enhancements
* Add `max_connections` option to CLI https://github.com/SeaQL/sea-orm/pull/670
* Derive `Eq`, `Clone` for `DbErr` https://github.com/SeaQL/sea-orm/pull/677
* Add `is_changed` to `ActiveModelTrait` https://github.com/SeaQL/sea-orm/pull/683

### Bug Fixes
* Fix `DerivePrimaryKey` with custom primary key column name https://github.com/SeaQL/sea-orm/pull/694
* Fix `DeriveEntityModel` macros override column name https://github.com/SeaQL/sea-orm/pull/695
* Fix Insert with no value supplied using `DEFAULT` https://github.com/SeaQL/sea-orm/pull/589

### Breaking Changes
* Migration utilities are moved from sea-schema to sea-orm repo, under a new sub-crate `sea-orm-migration`. `sea_schema::migration::prelude` should be replaced by `sea_orm_migration::prelude` in all migration files

### Upgrades
* Upgrade `sea-query` to 0.24.x, `sea-schema` to 0.8.x
* Upgrade example to Actix Web 4, Actix Web 3 remains https://github.com/SeaQL/sea-orm/pull/638
* Added Tonic gRPC example https://github.com/SeaQL/sea-orm/pull/659
* Upgrade GraphQL example to use axum 0.5.x
* Upgrade axum example to 0.5.x

### Fixed Issues
* Failed to insert row with only default values https://github.com/SeaQL/sea-orm/issues/420
* Reduce database connections to 1 during codegen https://github.com/SeaQL/sea-orm/issues/511
* Column names with single letters separated by underscores are concatenated https://github.com/SeaQL/sea-orm/issues/630
* Update Actix Web examples https://github.com/SeaQL/sea-orm/issues/639
* Lower function missing https://github.com/SeaQL/sea-orm/issues/672
* is_changed on active_model https://github.com/SeaQL/sea-orm/issues/674
* Failing find_with_related with column_name attribute https://github.com/SeaQL/sea-orm/issues/693

**Full Changelog**: https://github.com/SeaQL/sea-orm/compare/0.7.1...0.8.0

## 0.7.1 - 2022-03-26

* Fix sea-orm-cli error
* Fix sea-orm cannot build without `with-json`

## 0.7.0 - 2022-03-26

### New Features
* Update ActiveModel by JSON by @billy1624 in https://github.com/SeaQL/sea-orm/pull/492
* Supports `time` crate by @billy1624 https://github.com/SeaQL/sea-orm/pull/602
* Allow for creation of indexes for PostgreSQL and SQLite @nickb937 https://github.com/SeaQL/sea-orm/pull/593
* Added `delete_by_id` @ShouvikGhosh2048 https://github.com/SeaQL/sea-orm/pull/590
* Implement `PaginatorTrait` for `SelectorRaw` @shinbunbun https://github.com/SeaQL/sea-orm/pull/617

### Enhancements
* Added axum graphql example by @aaronleopold in https://github.com/SeaQL/sea-orm/pull/587
* Add example for integrate with jsonrpsee by @hunjixin https://github.com/SeaQL/sea-orm/pull/632
* Codegen add serde derives to enums, if specified by @BenJeau https://github.com/SeaQL/sea-orm/pull/463
* Codegen Unsigned Integer by @billy1624 https://github.com/SeaQL/sea-orm/pull/397
* Add `Send` bound to `QueryStream` and `TransactionStream` by @sebpuetz https://github.com/SeaQL/sea-orm/pull/471
* Add `Send` to `StreamTrait` by @nappa85 https://github.com/SeaQL/sea-orm/pull/622
* `sea` as an alternative bin name to `sea-orm-cli` by @ZhangHanDong https://github.com/SeaQL/sea-orm/pull/558

### Bug Fixes
* Fix codegen with Enum in expanded format by @billy1624 https://github.com/SeaQL/sea-orm/pull/624
* Fixing and testing into_json of various field types by @billy1624 https://github.com/SeaQL/sea-orm/pull/539

### Breaking Changes
* Exclude `mock` from default features by @billy1624 https://github.com/SeaQL/sea-orm/pull/562
* `create_table_from_entity` will no longer create index for MySQL, please use the new method `create_index_from_entity`

### Documentations
* Describe default value of ActiveValue on document by @Ken-Miura in https://github.com/SeaQL/sea-orm/pull/556
* community: add axum-book-management by @lz1998 in https://github.com/SeaQL/sea-orm/pull/564
* Add Backpack to project showcase by @JSH32 in https://github.com/SeaQL/sea-orm/pull/567
* Add mediarepo to showcase by @Trivernis in https://github.com/SeaQL/sea-orm/pull/569
* COMMUNITY: add a link to Svix to showcase by @tasn in https://github.com/SeaQL/sea-orm/pull/537
* Update COMMUNITY.md by @naryand in https://github.com/SeaQL/sea-orm/pull/570
* Update COMMUNITY.md by @BobAnkh in https://github.com/SeaQL/sea-orm/pull/568
* Update COMMUNITY.md by @KaniyaSimeji in https://github.com/SeaQL/sea-orm/pull/566
* Update COMMUNITY.md by @aaronleopold in https://github.com/SeaQL/sea-orm/pull/565
* Update COMMUNITY.md by @gudaoxuri in https://github.com/SeaQL/sea-orm/pull/572
* Update Wikijump's entry in COMMUNITY.md by @ammongit in https://github.com/SeaQL/sea-orm/pull/573
* Update COMMUNITY.md by @koopa1338 in https://github.com/SeaQL/sea-orm/pull/574
* Update COMMUNITY.md by @gengteng in https://github.com/SeaQL/sea-orm/pull/580
* Update COMMUNITY.md by @Yama-Tomo in https://github.com/SeaQL/sea-orm/pull/582
* add oura-postgres-sink to COMMUNITY.md by @rvcas in https://github.com/SeaQL/sea-orm/pull/594
* Add rust-example-caster-api to COMMUNITY.md by @bkonkle in https://github.com/SeaQL/sea-orm/pull/623

### Fixed Issues
* orm-cli generated incorrect type for #[sea_orm(primary_key)]. Should be u64. Was i64. https://github.com/SeaQL/sea-orm/issues/295
* how to update dynamically from json value https://github.com/SeaQL/sea-orm/issues/346
* Make `DatabaseConnection` `Clone` with the default features enabled https://github.com/SeaQL/sea-orm/issues/438
* Updating multiple fields in a Model by passing a reference https://github.com/SeaQL/sea-orm/issues/460
* SeaORM CLI not adding serde derives to Enums https://github.com/SeaQL/sea-orm/issues/461
* sea-orm-cli generates wrong data type for nullable blob https://github.com/SeaQL/sea-orm/issues/490
* Support the time crate in addition (instead of?) chrono https://github.com/SeaQL/sea-orm/issues/499
* PaginatorTrait for SelectorRaw https://github.com/SeaQL/sea-orm/issues/500
* sea_orm::DatabaseConnection should implement `Clone` by default https://github.com/SeaQL/sea-orm/issues/517
* How do you seed data in migrations using ActiveModels? https://github.com/SeaQL/sea-orm/issues/522
* Datetime fields are not serialized by `.into_json()` on queries https://github.com/SeaQL/sea-orm/issues/530
* Update / Delete by id https://github.com/SeaQL/sea-orm/issues/552
* `#[sea_orm(indexed)]` only works for MySQL https://github.com/SeaQL/sea-orm/issues/554
* `sea-orm-cli generate --with-serde` does not work on Postgresql custom type https://github.com/SeaQL/sea-orm/issues/581
* `sea-orm-cli generate --expanded-format` panic when postgres table contains enum type https://github.com/SeaQL/sea-orm/issues/614
* UUID fields are not serialized by `.into_json()` on queries https://github.com/SeaQL/sea-orm/issues/619

**Full Changelog**: https://github.com/SeaQL/sea-orm/compare/0.6.0...0.7.0

## 0.6.0 - 2022-02-07

### New Features
* Migration Support by @billy1624 in https://github.com/SeaQL/sea-orm/pull/335
* Support `DateTime<Utc>` & `DateTime<Local>` by @billy1624 in https://github.com/SeaQL/sea-orm/pull/489
* Add `max_lifetime` connection option by @billy1624 in https://github.com/SeaQL/sea-orm/pull/493

### Enhancements
* Model with Generics by @billy1624 in https://github.com/SeaQL/sea-orm/pull/400
* Add Poem example by @sunli829 in https://github.com/SeaQL/sea-orm/pull/446
* Codegen `column_name` proc_macro attribute by @billy1624 in https://github.com/SeaQL/sea-orm/pull/433
* Easy joins with MockDatabase #447 by @cemoktra in https://github.com/SeaQL/sea-orm/pull/455

### Bug Fixes
* CLI allow generate entity with url without password by @billy1624 in https://github.com/SeaQL/sea-orm/pull/436
* Support up to 6-ary composite primary key by @billy1624 in https://github.com/SeaQL/sea-orm/pull/423
* Fix FromQueryResult when Result is redefined by @tasn in https://github.com/SeaQL/sea-orm/pull/495
* Remove `r#` prefix when deriving `FromQueryResult` by @smrtrfszm in https://github.com/SeaQL/sea-orm/pull/494

### Breaking Changes
* Name conflict of foreign key constraints when two entities have more than one foreign keys by @billy1624 in https://github.com/SeaQL/sea-orm/pull/417

### Fixed Issues
* Is it possible to have 4 values Composite Key? https://github.com/SeaQL/sea-orm/issues/352
* Support `DateTime<Utc>` & `DateTime<Local>` https://github.com/SeaQL/sea-orm/issues/381
* Codegen `column_name` proc_macro attribute if column name isn't in snake case https://github.com/SeaQL/sea-orm/issues/395
* Model with Generics https://github.com/SeaQL/sea-orm/issues/402
* Foreign key constraint collision when multiple keys exist between the same two tables https://github.com/SeaQL/sea-orm/issues/405
* sea-orm-cli passwordless database user causes "No password was found in the database url" error https://github.com/SeaQL/sea-orm/issues/435
* Testing joins with MockDatabase https://github.com/SeaQL/sea-orm/issues/447
* Surface max_lifetime connection option https://github.com/SeaQL/sea-orm/issues/475

**Full Changelog**: https://github.com/SeaQL/sea-orm/compare/0.5.0...0.6.0

## 0.5.0 - 2022-01-01

### Fixed Issues
* Why insert, update, etc return an ActiveModel instead of Model? https://github.com/SeaQL/sea-orm/issues/289
* Rework `ActiveValue` https://github.com/SeaQL/sea-orm/issues/321
* Some missing ActiveEnum utilities https://github.com/SeaQL/sea-orm/issues/338

### Merged PRs
* First metric and tracing implementation by @nappa85 in https://github.com/SeaQL/sea-orm/pull/373
* Update sea-orm to depends on SeaQL/sea-query#202 by @billy1624 in https://github.com/SeaQL/sea-orm/pull/370
* Codegen ActiveEnum & Create Enum From ActiveEnum by @billy1624 in https://github.com/SeaQL/sea-orm/pull/348
* Axum example: update to Axum v0.4.2 by @ttys3 in https://github.com/SeaQL/sea-orm/pull/383
* Fix rocket version by @Gabriel-Paulucci in https://github.com/SeaQL/sea-orm/pull/384
* Insert & Update Return `Model` by @billy1624 in https://github.com/SeaQL/sea-orm/pull/339
* Rework `ActiveValue` by @billy1624 in https://github.com/SeaQL/sea-orm/pull/340
* Add wrapper method `ModelTrait::delete` by @billy1624 in https://github.com/SeaQL/sea-orm/pull/396
* Add docker create script for contributors to setup databases locally by @billy1624 in https://github.com/SeaQL/sea-orm/pull/378
* Log with tracing-subscriber by @billy1624 in https://github.com/SeaQL/sea-orm/pull/399
* Codegen SQLite by @billy1624 in https://github.com/SeaQL/sea-orm/pull/386
* PR without clippy warnings in file changed tab by @billy1624 in https://github.com/SeaQL/sea-orm/pull/401
* Rename `sea-strum` lib back to `strum` by @billy1624 in https://github.com/SeaQL/sea-orm/pull/361

### Breaking Changes
* `ActiveModel::insert` and `ActiveModel::update` return `Model` instead of `ActiveModel`
* Method `ActiveModelBehavior::after_save` takes `Model` as input instead of `ActiveModel`
* Rename method `sea_orm::unchanged_active_value_not_intended_for_public_use` to `sea_orm::Unchanged`
* Rename method `ActiveValue::unset` to `ActiveValue::not_set`
* Rename method `ActiveValue::is_unset` to `ActiveValue::is_not_set`
* `PartialEq` of `ActiveValue` will also check the equality of state instead of just checking the equality of value

**Full Changelog**: https://github.com/SeaQL/sea-orm/compare/0.4.2...0.5.0

## 0.4.2 - 2021-12-12

### Fixed Issues
* Delete::many() doesn't work when schema_name is defined https://github.com/SeaQL/sea-orm/issues/362
* find_with_related panic https://github.com/SeaQL/sea-orm/issues/374
* How to define the rust type of TIMESTAMP? https://github.com/SeaQL/sea-orm/issues/344
* Add Table on the generated Column enum https://github.com/SeaQL/sea-orm/issues/356

### Merged PRs
* `Delete::many()` with `TableRef` by @billy1624 in https://github.com/SeaQL/sea-orm/pull/363
* Fix related & linked with enum columns by @billy1624 in https://github.com/SeaQL/sea-orm/pull/376
* Temporary Fix: Handling MySQL & SQLite timestamp columns by @billy1624 in https://github.com/SeaQL/sea-orm/pull/379
* Add feature to generate table Iden by @Sytten in https://github.com/SeaQL/sea-orm/pull/360

## 0.4.1 - 2021-12-05

### Fixed Issues
* Is it possible to have 4 values Composite Key? https://github.com/SeaQL/sea-orm/issues/352
* [sea-orm-cli] Better handling of relation generations https://github.com/SeaQL/sea-orm/issues/239

### Merged PRs
* Add TryFromU64 trait for `DateTime<FixedOffset>`. by @kev0960 in https://github.com/SeaQL/sea-orm/pull/331
* add offset and limit by @lz1998 in https://github.com/SeaQL/sea-orm/pull/351
* For some reason the `axum_example` fail to compile by @billy1624 in https://github.com/SeaQL/sea-orm/pull/355
* Support Up to 6 Values Composite Primary Key by @billy1624 in https://github.com/SeaQL/sea-orm/pull/353
* Codegen Handle Self Referencing & Multiple Relations to the Same Related Entity by @billy1624 in https://github.com/SeaQL/sea-orm/pull/347

## 0.4.0 - 2021-11-19

### Fixed Issues
* Disable SQLx query logging https://github.com/SeaQL/sea-orm/issues/290
* Code generated by `sea-orm-cli` cannot pass clippy https://github.com/SeaQL/sea-orm/issues/296
* Should return detailed error message for connection failure https://github.com/SeaQL/sea-orm/issues/310
* `DateTimeWithTimeZone` does not implement `Serialize` and `Deserialize` https://github.com/SeaQL/sea-orm/issues/319
* Support returning clause to avoid database hits https://github.com/SeaQL/sea-orm/issues/183

### Merged PRs
* chore: update to Rust 2021 Edition by @sno2 in https://github.com/SeaQL/sea-orm/pull/273
* Enumeration - 3 by @billy1624 in https://github.com/SeaQL/sea-orm/pull/274
* Enumeration - 2 by @billy1624 in https://github.com/SeaQL/sea-orm/pull/261
* Codegen fix clippy warnings by @billy1624 in https://github.com/SeaQL/sea-orm/pull/303
* Add axum example by @YoshieraHuang in https://github.com/SeaQL/sea-orm/pull/297
* Enumeration by @billy1624 in https://github.com/SeaQL/sea-orm/pull/258
* Add `PaginatorTrait` and `CountTrait` for more constraints by @YoshieraHuang in https://github.com/SeaQL/sea-orm/pull/306
* Continue `PaginatorTrait` by @billy1624 in https://github.com/SeaQL/sea-orm/pull/307
* Refactor `Schema` by @billy1624 in https://github.com/SeaQL/sea-orm/pull/309
* Detailed connection errors by @billy1624 in https://github.com/SeaQL/sea-orm/pull/312
* Suppress `ouroboros` missing docs warnings by @billy1624 in https://github.com/SeaQL/sea-orm/pull/288
* `with-json` feature requires `chrono/serde` by @billy1624 in https://github.com/SeaQL/sea-orm/pull/320
* Pass the argument `entity.table_ref()` instead of just `entity`. by @josh-codes in https://github.com/SeaQL/sea-orm/pull/318
* Unknown types could be a newtypes instead of `ActiveEnum` by @billy1624 in https://github.com/SeaQL/sea-orm/pull/324
* Returning by @billy1624 in https://github.com/SeaQL/sea-orm/pull/292

### Breaking Changes
* Refactor `paginate()` & `count()` utilities into `PaginatorTrait`. You can use the paginator as usual but you might need to import `PaginatorTrait` manually when upgrading from the previous version.
    ```rust
    use futures::TryStreamExt;
    use sea_orm::{entity::*, query::*, tests_cfg::cake};

    let mut cake_stream = cake::Entity::find()
        .order_by_asc(cake::Column::Id)
        .paginate(db, 50)
        .into_stream();

    while let Some(cakes) = cake_stream.try_next().await? {
        // Do something on cakes: Vec<cake::Model>
    }
    ```
* The helper struct `Schema` converting `EntityTrait` into different `sea-query` statements now has to be initialized with `DbBackend`.
    ```rust
    use sea_orm::{tests_cfg::*, DbBackend, Schema};
    use sea_orm::sea_query::TableCreateStatement;

    // 0.3.x
    let _: TableCreateStatement = Schema::create_table_from_entity(cake::Entity);

    // 0.4.x
    let schema: Schema = Schema::new(DbBackend::MySql);
    let _: TableCreateStatement = schema.create_table_from_entity(cake::Entity);
    ```
* When performing insert or update operation on `ActiveModel` against PostgreSQL, `RETURNING` clause will be used to perform select in a single SQL statement.
    ```rust
    // For PostgreSQL
    cake::ActiveModel {
        name: Set("Apple Pie".to_owned()),
        ..Default::default()
    }
    .insert(&postgres_db)
    .await?;

    assert_eq!(
        postgres_db.into_transaction_log(),
        vec![Transaction::from_sql_and_values(
            DbBackend::Postgres,
            r#"INSERT INTO "cake" ("name") VALUES ($1) RETURNING "id", "name""#,
            vec!["Apple Pie".into()]
        )]);
    ```
    ```rust
    // For MySQL & SQLite
    cake::ActiveModel {
        name: Set("Apple Pie".to_owned()),
        ..Default::default()
    }
    .insert(&other_db)
    .await?;

    assert_eq!(
        other_db.into_transaction_log(),
        vec![
            Transaction::from_sql_and_values(
                DbBackend::MySql,
                r#"INSERT INTO `cake` (`name`) VALUES (?)"#,
                vec!["Apple Pie".into()]
            ),
            Transaction::from_sql_and_values(
                DbBackend::MySql,
                r#"SELECT `cake`.`id`, `cake`.`name` FROM `cake` WHERE `cake`.`id` = ? LIMIT ?"#,
                vec![15.into(), 1u64.into()]
            )]);
    ```

**Full Changelog**: https://github.com/SeaQL/sea-orm/compare/0.3.2...0.4.0

## 0.3.2 - 2021-11-03

### Fixed Issues
* Support for BYTEA Postgres primary keys https://github.com/SeaQL/sea-orm/issues/286

### Merged PRs
* Documentation for sea-orm by @charleschege in https://github.com/SeaQL/sea-orm/pull/280
* Support `Vec<u8>` primary key by @billy1624 in https://github.com/SeaQL/sea-orm/pull/287

## 0.3.1 - 2021-10-23

(We are changing our Changelog format from now on)

### Fixed Issues
* Align case transforms across derive macros https://github.com/SeaQL/sea-orm/issues/262
* Added `is_null` and `is_not_null` to `ColumnTrait` https://github.com/SeaQL/sea-orm/issues/267

(The following is generated by GitHub)

### Merged PRs
* Changed manual url parsing to use Url crate by @AngelOnFira in https://github.com/SeaQL/sea-orm/pull/253
* Test self referencing relation by @billy1624 in https://github.com/SeaQL/sea-orm/pull/256
* Unify case-transform using the same crate by @billy1624 in https://github.com/SeaQL/sea-orm/pull/264
* CI cleaning by @AngelOnFira in https://github.com/SeaQL/sea-orm/pull/263
* CI install sea-orm-cli in debug mode by @billy1624 in https://github.com/SeaQL/sea-orm/pull/265

## 0.3.0 - 2021-10-15

https://www.sea-ql.org/SeaORM/blog/2021-10-15-whats-new-in-0.3.0

- Built-in Rocket support
- `ConnectOptions`

```rust
let mut opt = ConnectOptions::new("protocol://username:password@host/database".to_owned());
opt.max_connections(100)
    .min_connections(5)
    .connect_timeout(Duration::from_secs(8))
    .idle_timeout(Duration::from_secs(8));
let db = Database::connect(opt).await?;
```

- [[#211]] Throw error if none of the db rows are affected

```rust
assert_eq!(
    Update::one(cake::ActiveModel {
        name: Set("Cheese Cake".to_owned()),
        ..model.into_active_model()
    })
    .exec(&db)
    .await,
    Err(DbErr::RecordNotFound(
        "None of the database rows are affected".to_owned()
    ))
);

// update many remains the same
assert_eq!(
    Update::many(cake::Entity)
        .col_expr(cake::Column::Name, Expr::value("Cheese Cake".to_owned()))
        .filter(cake::Column::Id.eq(2))
        .exec(&db)
        .await,
    Ok(UpdateResult { rows_affected: 0 })
);
```

- [[#223]] `ActiveValue::take()` & `ActiveValue::into_value()` without `unwrap()`
- [[#205]] Drop `Default` trait bound of `PrimaryKeyTrait::ValueType`
- [[#222]] Transaction & streaming
- [[#210]] Update `ActiveModelBehavior` API
- [[#240]] Add derive `DeriveIntoActiveModel` and `IntoActiveValue` trait
- [[#237]] Introduce optional serde support for model code generation
- [[#246]] Add `#[automatically_derived]` to all derived implementations

[#211]: https://github.com/SeaQL/sea-orm/pull/211
[#223]: https://github.com/SeaQL/sea-orm/pull/223
[#205]: https://github.com/SeaQL/sea-orm/pull/205
[#222]: https://github.com/SeaQL/sea-orm/pull/222
[#210]: https://github.com/SeaQL/sea-orm/pull/210
[#240]: https://github.com/SeaQL/sea-orm/pull/240
[#237]: https://github.com/SeaQL/sea-orm/pull/237
[#246]: https://github.com/SeaQL/sea-orm/pull/246

**Full Changelog**: https://github.com/SeaQL/sea-orm/compare/0.2.6...0.3.0

## 0.2.6 - 2021-10-09

- [[#224]] [sea-orm-cli] Date & Time column type mapping
- Escape rust keywords with `r#` raw identifier

[#224]: https://github.com/SeaQL/sea-orm/pull/224

## 0.2.5 - 2021-10-06

- [[#227]] Resolve "Inserting actual none value of Option<Date> results in panic"
- [[#219]] [sea-orm-cli] Add `--tables` option
- [[#189]] Add `debug_query` and `debug_query_stmt` macro

[#227]: https://github.com/SeaQL/sea-orm/issues/227
[#219]: https://github.com/SeaQL/sea-orm/pull/219
[#189]: https://github.com/SeaQL/sea-orm/pull/189

## 0.2.4 - 2021-10-01

https://www.sea-ql.org/SeaORM/blog/2021-10-01-whats-new-in-0.2.4

- [[#186]] [sea-orm-cli] Foreign key handling
- [[#191]] [sea-orm-cli] Unique key handling
- [[#182]] `find_linked` join with alias
- [[#202]] Accept both `postgres://` and `postgresql://`
- [[#208]] Support fetching T, (T, U), (T, U, P) etc
- [[#209]] Rename column name & column enum variant
- [[#207]] Support `chrono::NaiveDate` & `chrono::NaiveTime`
- Support `Condition::not` (from sea-query)

[#186]: https://github.com/SeaQL/sea-orm/issues/186
[#191]: https://github.com/SeaQL/sea-orm/issues/191
[#182]: https://github.com/SeaQL/sea-orm/pull/182
[#202]: https://github.com/SeaQL/sea-orm/pull/202
[#208]: https://github.com/SeaQL/sea-orm/pull/208
[#209]: https://github.com/SeaQL/sea-orm/pull/209
[#207]: https://github.com/SeaQL/sea-orm/pull/207

## 0.2.3 - 2021-09-22

- [[#152]] DatabaseConnection impl `Clone`
- [[#175]] Impl `TryGetableMany` for different types of generics
- Codegen `TimestampWithTimeZone` fixup

[#152]: https://github.com/SeaQL/sea-orm/issues/152
[#175]: https://github.com/SeaQL/sea-orm/issues/175

## 0.2.2 - 2021-09-18

- [[#105]] Compact entity format
- [[#132]] Add ActiveModel `insert` & `update`
- [[#129]] Add `set` method to `UpdateMany`
- [[#118]] Initial lock support
- [[#167]] Add `FromQueryResult::find_by_statement`

[#105]: https://github.com/SeaQL/sea-orm/issues/105
[#132]: https://github.com/SeaQL/sea-orm/issues/132
[#129]: https://github.com/SeaQL/sea-orm/issues/129
[#118]: https://github.com/SeaQL/sea-orm/issues/118
[#167]: https://github.com/SeaQL/sea-orm/issues/167

## 0.2.1 - 2021-09-04

- Update dependencies

## 0.2.0 - 2021-09-03

- [[#37]] Rocket example
- [[#114]] `log` crate and `env-logger`
- [[#103]] `InsertResult` to return the primary key's type
- [[#89]] Represent several relations between same types by `Linked`
- [[#59]] Transforming an Entity into `TableCreateStatement`

[#37]: https://github.com/SeaQL/sea-orm/issues/37
[#114]: https://github.com/SeaQL/sea-orm/issues/114
[#103]: https://github.com/SeaQL/sea-orm/issues/103
[#89]: https://github.com/SeaQL/sea-orm/issues/89
[#59]: https://github.com/SeaQL/sea-orm/issues/59

**Full Changelog**: https://github.com/SeaQL/sea-orm/compare/0.1.3...0.2.0

## 0.1.3 - 2021-08-30

- [[#108]] Remove impl TryGetable for Option<T>

[#108]: https://github.com/SeaQL/sea-orm/issues/108

## 0.1.2 - 2021-08-23

- [[#68]] Added `DateTimeWithTimeZone` as supported attribute type
- [[#70]] Generate arbitrary named entity
- [[#80]] Custom column name
- [[#81]] Support join on multiple columns
- [[#99]] Implement FromStr for ColumnTrait

[#68]: https://github.com/SeaQL/sea-orm/issues/68
[#70]: https://github.com/SeaQL/sea-orm/issues/70
[#80]: https://github.com/SeaQL/sea-orm/issues/80
[#81]: https://github.com/SeaQL/sea-orm/issues/81
[#99]: https://github.com/SeaQL/sea-orm/issues/99

## 0.1.1 - 2021-08-08

- Early release of SeaORM

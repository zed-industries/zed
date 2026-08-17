<div align="center">

  <img src="https://www.sea-ql.org/SeaORM/img/SeaORM banner.png"/>

  <h1>SeaORM</h1>

  <h3>🐚 An async & dynamic ORM for Rust</h3>

  [![crate](https://img.shields.io/crates/v/sea-orm.svg)](https://crates.io/crates/sea-orm)
  [![docs](https://docs.rs/sea-orm/badge.svg)](https://docs.rs/sea-orm)
  [![build status](https://github.com/SeaQL/sea-orm/actions/workflows/rust.yml/badge.svg)](https://github.com/SeaQL/sea-orm/actions/workflows/rust.yml)

</div>

# SeaORM

#### SeaORM is a relational ORM to help you build web services in Rust with the familiarity of dynamic languages.

[![GitHub stars](https://img.shields.io/github/stars/SeaQL/sea-orm.svg?style=social&label=Star&maxAge=1)](https://github.com/SeaQL/sea-orm/stargazers/)
If you like what we do, consider starring, sharing and contributing!

Please help us with maintaining SeaORM by completing the [SeaQL Community Survey 2024](https://sea-ql.org/community-survey)!

[![Discord](https://img.shields.io/discord/873880840487206962?label=Discord)](https://discord.com/invite/uCPdDXzbdv)
Join our Discord server to chat with other members of the SeaQL community!

## Getting Started

+ [Documentation](https://www.sea-ql.org/SeaORM)
+ [Tutorial](https://www.sea-ql.org/sea-orm-tutorial)
+ [Cookbook](https://www.sea-ql.org/sea-orm-cookbook)

Integration examples:

+ [Actix v4 Example](https://github.com/SeaQL/sea-orm/tree/master/examples/actix_example)
+ [Axum Example](https://github.com/SeaQL/sea-orm/tree/master/examples/axum_example)
+ [GraphQL Example](https://github.com/SeaQL/sea-orm/tree/master/examples/graphql_example)
+ [jsonrpsee Example](https://github.com/SeaQL/sea-orm/tree/master/examples/jsonrpsee_example)
+ [Loco TODO Example](https://github.com/SeaQL/sea-orm/tree/master/examples/loco_example) / [Loco REST Starter](https://github.com/SeaQL/sea-orm/tree/master/examples/loco_starter)
+ [Poem Example](https://github.com/SeaQL/sea-orm/tree/master/examples/poem_example)
+ [Rocket Example](https://github.com/SeaQL/sea-orm/tree/master/examples/rocket_example) / [Rocket OpenAPI Example](https://github.com/SeaQL/sea-orm/tree/master/examples/rocket_okapi_example)
+ [Salvo Example](https://github.com/SeaQL/sea-orm/tree/master/examples/salvo_example)
+ [Tonic Example](https://github.com/SeaQL/sea-orm/tree/master/examples/tonic_example)
+ [Seaography Example](https://github.com/SeaQL/sea-orm/tree/master/examples/seaography_example)

## Features

1. Async

    Relying on [SQLx](https://github.com/launchbadge/sqlx), SeaORM is a new library with async support from day 1.

2. Dynamic

    Built upon [SeaQuery](https://github.com/SeaQL/sea-query), SeaORM allows you to build complex dynamic queries.

3. Service Oriented

    Quickly build services that join, filter, sort and paginate data in REST, GraphQL and gRPC APIs.

4. Production Ready

    SeaORM is feature-rich, well-tested and used in production by companies and startups.

## A quick taste of SeaORM

### Entity
```rust
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "cake")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::fruit::Entity")]
    Fruit,
}

impl Related<super::fruit::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Fruit.def()
    }
}
```

### Select
```rust
// find all models
let cakes: Vec<cake::Model> = Cake::find().all(db).await?;

// find and filter
let chocolate: Vec<cake::Model> = Cake::find()
    .filter(cake::Column::Name.contains("chocolate"))
    .all(db)
    .await?;

// find one model
let cheese: Option<cake::Model> = Cake::find_by_id(1).one(db).await?;
let cheese: cake::Model = cheese.unwrap();

// find related models (lazy)
let fruits: Vec<fruit::Model> = cheese.find_related(Fruit).all(db).await?;

// find related models (eager)
let cake_with_fruits: Vec<(cake::Model, Vec<fruit::Model>)> =
    Cake::find().find_with_related(Fruit).all(db).await?;

```
### Insert
```rust
let apple = fruit::ActiveModel {
    name: Set("Apple".to_owned()),
    ..Default::default() // no need to set primary key
};

let pear = fruit::ActiveModel {
    name: Set("Pear".to_owned()),
    ..Default::default()
};

// insert one
let pear = pear.insert(db).await?;

// insert many
Fruit::insert_many([apple, pear]).exec(db).await?;
```
### Update
```rust
use sea_orm::sea_query::{Expr, Value};

let pear: Option<fruit::Model> = Fruit::find_by_id(1).one(db).await?;
let mut pear: fruit::ActiveModel = pear.unwrap().into();

pear.name = Set("Sweet pear".to_owned());

// update one
let pear: fruit::Model = pear.update(db).await?;

// update many: UPDATE "fruit" SET "cake_id" = NULL WHERE "fruit"."name" LIKE '%Apple%'
Fruit::update_many()
    .col_expr(fruit::Column::CakeId, Expr::value(Value::Int(None)))
    .filter(fruit::Column::Name.contains("Apple"))
    .exec(db)
    .await?;

```
### Save
```rust
let banana = fruit::ActiveModel {
    id: NotSet,
    name: Set("Banana".to_owned()),
    ..Default::default()
};

// create, because primary key `id` is `NotSet`
let mut banana = banana.save(db).await?;

banana.name = Set("Banana Mongo".to_owned());

// update, because primary key `id` is `Set`
let banana = banana.save(db).await?;

```
### Delete
```rust
// delete one
let orange: Option<fruit::Model> = Fruit::find_by_id(1).one(db).await?;
let orange: fruit::Model = orange.unwrap();
fruit::Entity::delete(orange.into_active_model())
    .exec(db)
    .await?;

// or simply
let orange: Option<fruit::Model> = Fruit::find_by_id(1).one(db).await?;
let orange: fruit::Model = orange.unwrap();
orange.delete(db).await?;

// delete many: DELETE FROM "fruit" WHERE "fruit"."name" LIKE 'Orange'
fruit::Entity::delete_many()
    .filter(fruit::Column::Name.contains("Orange"))
    .exec(db)
    .await?;

```

## 🧭 Seaography: instant GraphQL API

[Seaography](https://github.com/SeaQL/seaography) is a GraphQL framework built on top of SeaORM. Seaography allows you to build GraphQL resolvers quickly. With just a few commands, you can launch a GraphQL server from SeaORM entities!

Look at the [Seaography Example](https://github.com/SeaQL/sea-orm/tree/master/examples/seaography_example) to learn more.

<img src="https://raw.githubusercontent.com/SeaQL/sea-orm/master/examples/seaography_example/Seaography%20example.png"/>

## 🖥️ SeaORM Pro: Effortless Admin Panel

[SeaORM Pro](https://www.sea-ql.org/sea-orm-pro/) is an admin panel solution allowing you to quickly and easily launch an admin panel for your application - frontend development skills not required, but certainly nice to have!

Features:

+ Full CRUD
+ Built on React + GraphQL
+ Built-in GraphQL resolver
+ Customize the UI with simple TOML

Learn More

+ [Example Repo](https://github.com/SeaQL/sea-orm-pro)
+ [Getting Started with Loco](https://www.sea-ql.org/sea-orm-pro/docs/install-and-config/getting-started-loco/)
+ [Getting Started with Axum](https://www.sea-ql.org/sea-orm-pro/docs/install-and-config/getting-started-axum/)

![](https://raw.githubusercontent.com/SeaQL/sea-orm/refs/heads/master/docs/sea-orm-pro-dark.png#gh-dark-mode-only)
![](https://raw.githubusercontent.com/SeaQL/sea-orm/refs/heads/master/docs/sea-orm-pro-light.png#gh-light-mode-only)

## Releases

[SeaORM 1.0](https://www.sea-ql.org/blog/2024-08-04-sea-orm-1.0/) is a stable release. The 1.x version will be updated until at least October 2025, and we'll decide whether to release a 2.0 version or extend the 1.x life cycle.

It doesn't mean that SeaORM is 'done', we've designed an architecture to allow us to deliver new features without major breaking changes. In fact, more features are coming!

+ [Change Log](https://github.com/SeaQL/sea-orm/tree/master/CHANGELOG.md)

### Who's using SeaORM?

Here is a short list of awesome open source software built with SeaORM. [Full list here](https://github.com/SeaQL/sea-orm/blob/master/COMMUNITY.md#built-with-seaorm). Feel free to submit yours!

| Project | GitHub | Tagline |
|---------|--------|---------|
| [Zed](https://github.com/zed-industries/zed) | ![GitHub stars](https://img.shields.io/github/stars/zed-industries/zed.svg?style=social) | A high-performance, multiplayer code editor |
| [OpenObserve](https://github.com/openobserve/openobserve) | ![GitHub stars](https://img.shields.io/github/stars/openobserve/openobserve.svg?style=social) | Open-source observability platform |
| [RisingWave](https://github.com/risingwavelabs/risingwave) | ![GitHub stars](https://img.shields.io/github/stars/risingwavelabs/risingwave.svg?style=social) | Stream processing and management platform |
| [LLDAP](https://github.com/nitnelave/lldap) | ![GitHub stars](https://img.shields.io/github/stars/nitnelave/lldap.svg?style=social) | A light LDAP server for user management |
| [Warpgate](https://github.com/warp-tech/warpgate) | ![GitHub stars](https://img.shields.io/github/stars/warp-tech/warpgate.svg?style=social) | Smart SSH bastion that works with any SSH client |
| [Svix](https://github.com/svix/svix-webhooks) | ![GitHub stars](https://img.shields.io/github/stars/svix/svix-webhooks.svg?style=social) | The enterprise ready webhooks service |
| [Ryot](https://github.com/IgnisDa/ryot) | ![GitHub stars](https://img.shields.io/github/stars/ignisda/ryot.svg?style=social) | The only self hosted tracker you will ever need |
| [Lapdev](https://github.com/lapce/lapdev) | ![GitHub stars](https://img.shields.io/github/stars/lapce/lapdev.svg?style=social) | Self-hosted remote development enviroment |
| [System Initiative](https://github.com/systeminit/si) | ![GitHub stars](https://img.shields.io/github/stars/systeminit/si.svg?style=social) | DevOps Automation Platform |
| [OctoBase](https://github.com/toeverything/OctoBase) | ![GitHub stars](https://img.shields.io/github/stars/toeverything/OctoBase.svg?style=social) | A light-weight, scalable, offline collaborative data backend |

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

We invite you to participate, contribute and together help build Rust's future.

A big shout out to our contributors!

[![Contributors](https://opencollective.com/sea-orm/contributors.svg?width=1000&button=false)](https://github.com/SeaQL/sea-orm/graphs/contributors)

## Sponsorship

[SeaQL.org](https://www.sea-ql.org/) is an independent open-source organization run by passionate developers. If you enjoy using our libraries, please star and share our repositories. If you feel generous, a small donation via [GitHub Sponsor](https://github.com/sponsors/SeaQL) will be greatly appreciated, and goes a long way towards sustaining the organization.

### Silver Sponsors

<table><tr>
<td><a href="https://www.digitalocean.com/">
  <img src="https://www.sea-ql.org/static/sponsors/DigitalOcean.svg" width="125">
</a></td>

<td><a href="https://www.jetbrains.com/">
  <img src="https://www.sea-ql.org/static/sponsors/JetBrains.svg" width="125">
</a></td>
</tr></table>

We’re immensely grateful to our sponsors: Digital Ocean, for sponsoring our servers. And JetBrains, for sponsoring our IDE.

## Mascot

A friend of Ferris, Terres the hermit crab is the official mascot of SeaORM. His hobby is collecting shells.

<img alt="Terres" src="https://www.sea-ql.org/SeaORM/img/Terres.png" width="400"/>

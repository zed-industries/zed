//! 1. Async
//!
//!     Relying on [SQLx](https://github.com/launchbadge/sqlx), SeaORM is a new library with async support from day 1.
//!
//! ```
//! # use sea_orm::{error::*, tests_cfg::*, *};
//! #
//! # #[smol_potat::main]
//! # #[cfg(feature = "mock")]
//! # pub async fn main() -> Result<(), DbErr> {
//! #
//! # let db = MockDatabase::new(DbBackend::Postgres)
//! #     .append_query_results([
//! #         [cake::Model {
//! #             id: 1,
//! #             name: "New York Cheese".to_owned(),
//! #         }
//! #         .into_mock_row()],
//! #         [fruit::Model {
//! #             id: 1,
//! #             name: "Apple".to_owned(),
//! #             cake_id: Some(1),
//! #         }
//! #         .into_mock_row()],
//! #     ])
//! #     .into_connection();
//! #
//! // execute multiple queries in parallel
//! let cakes_and_fruits: (Vec<cake::Model>, Vec<fruit::Model>) =
//!     futures::try_join!(Cake::find().all(&db), Fruit::find().all(&db))?;
//! # assert_eq!(
//! #     cakes_and_fruits,
//! #     (
//! #         vec![cake::Model {
//! #             id: 1,
//! #             name: "New York Cheese".to_owned(),
//! #         }],
//! #         vec![fruit::Model {
//! #             id: 1,
//! #             name: "Apple".to_owned(),
//! #             cake_id: Some(1),
//! #         }]
//! #     )
//! # );
//! # assert_eq!(
//! #     db.into_transaction_log(),
//! #     [
//! #         Transaction::from_sql_and_values(
//! #             DbBackend::Postgres,
//! #             r#"SELECT "cake"."id", "cake"."name" FROM "cake""#,
//! #             []
//! #         ),
//! #         Transaction::from_sql_and_values(
//! #             DbBackend::Postgres,
//! #             r#"SELECT "fruit"."id", "fruit"."name", "fruit"."cake_id" FROM "fruit""#,
//! #             []
//! #         ),
//! #     ]
//! # );
//! # Ok(())
//! # }
//! ```
//!
//! 2. Dynamic
//!
//!     Built upon [SeaQuery](https://github.com/SeaQL/sea-query), SeaORM allows you to build complex queries without 'fighting the ORM'.
//!
//! ```
//! # use sea_query::Query;
//! # use sea_orm::{DbConn, error::*, entity::*, query::*, tests_cfg::*};
//! # async fn function(db: DbConn) -> Result<(), DbErr> {
//! // build subquery with ease
//! let cakes_with_filling: Vec<cake::Model> = cake::Entity::find()
//!     .filter(
//!         Condition::any().add(
//!             cake::Column::Id.in_subquery(
//!                 Query::select()
//!                     .column(cake_filling::Column::CakeId)
//!                     .from(cake_filling::Entity)
//!                     .to_owned(),
//!             ),
//!         ),
//!     )
//!     .all(&db)
//!     .await?;
//!
//! # Ok(())
//! # }
//! ```
//!
//! 3. Testable
//!
//!     Use mock connections to write unit tests for your logic.
//!
//! ```
//! # use sea_orm::{error::*, entity::*, query::*, tests_cfg::*, DbConn, MockDatabase, Transaction, DbBackend};
//! # async fn function(db: DbConn) -> Result<(), DbErr> {
//! // Setup mock connection
//! let db = MockDatabase::new(DbBackend::Postgres)
//!     .append_query_results([
//!         [
//!             cake::Model {
//!                 id: 1,
//!                 name: "New York Cheese".to_owned(),
//!             },
//!         ],
//!     ])
//!     .into_connection();
//!
//! // Perform your application logic
//! assert_eq!(
//!     cake::Entity::find().one(&db).await?,
//!     Some(cake::Model {
//!         id: 1,
//!         name: "New York Cheese".to_owned(),
//!     })
//! );
//!
//! // Compare it against the expected transaction log
//! assert_eq!(
//!     db.into_transaction_log(),
//!     [
//!         Transaction::from_sql_and_values(
//!             DbBackend::Postgres,
//!             r#"SELECT "cake"."id", "cake"."name" FROM "cake" LIMIT $1"#,
//!             [1u64.into()]
//!         ),
//!     ]
//! );
//! # Ok(())
//! # }
//! ```
//!
//! 4. Service Oriented
//!
//!     Quickly build services that join, filter, sort and paginate data in APIs.
//!
//! ```ignore
//! #[get("/?<page>&<posts_per_page>")]
//! async fn list(
//!     conn: Connection<Db>,
//!     page: Option<usize>,
//!     per_page: Option<usize>,
//! ) -> Template {
//!     // Set page number and items per page
//!     let page = page.unwrap_or(1);
//!     let per_page = per_page.unwrap_or(10);
//!
//!     // Setup paginator
//!     let paginator = Post::find()
//!         .order_by_asc(post::Column::Id)
//!         .paginate(&conn, per_page);
//!     let num_pages = paginator.num_pages().await.unwrap();
//!
//!     // Fetch paginated posts
//!     let posts = paginator
//!         .fetch_page(page - 1)
//!         .await
//!         .expect("could not retrieve posts");
//!
//!     Template::render(
//!         "index",
//!         context! {
//!             page: page,
//!             per_page: per_page,
//!             posts: posts,
//!             num_pages: num_pages,
//!         },
//!     )
//! }
//! ```

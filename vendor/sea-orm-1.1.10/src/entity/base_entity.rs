use crate::{
    ActiveModelBehavior, ActiveModelTrait, ColumnTrait, Delete, DeleteMany, DeleteOne,
    FromQueryResult, Insert, ModelTrait, PrimaryKeyToColumn, PrimaryKeyTrait, QueryFilter, Related,
    RelationBuilder, RelationTrait, RelationType, Select, Update, UpdateMany, UpdateOne,
};
use sea_query::{Alias, Iden, IntoIden, IntoTableRef, IntoValueTuple, TableRef};
use std::fmt::Debug;
pub use strum::IntoEnumIterator as Iterable;

/// Ensure the identifier for an Entity can be converted to a static str
pub trait IdenStatic: Iden + Copy + Debug + 'static {
    /// Method to call to get the static string identity
    fn as_str(&self) -> &str;
}

/// A Trait for mapping an Entity to a database table
pub trait EntityName: IdenStatic + Default {
    /// Method to get the name for the schema, defaults to [Option::None] if not set
    fn schema_name(&self) -> Option<&str> {
        None
    }

    /// Method to get the comment for the schema, defaults to [Option::None] if not set
    fn comment(&self) -> Option<&str> {
        None
    }

    /// Get the name of the table
    fn table_name(&self) -> &str;

    /// Get the name of the module from the invoking `self.table_name()`
    fn module_name(&self) -> &str {
        self.table_name()
    }

    /// Get the [TableRef] from invoking the `self.schema_name()`
    fn table_ref(&self) -> TableRef {
        match self.schema_name() {
            Some(schema) => (Alias::new(schema).into_iden(), self.into_iden()).into_table_ref(),
            None => self.into_table_ref(),
        }
    }
}

/// An abstract base class for defining Entities.
///
/// This trait provides an API for you to inspect it's properties
/// - Column (implemented [`ColumnTrait`])
/// - Relation (implemented [`RelationTrait`])
/// - Primary Key (implemented [`PrimaryKeyTrait`] and [`PrimaryKeyToColumn`])
///
/// This trait also provides an API for CRUD actions
/// - Select: `find`, `find_*`
/// - Insert: `insert`, `insert_*`
/// - Update: `update`, `update_*`
/// - Delete: `delete`, `delete_*`
pub trait EntityTrait: EntityName {
    #[allow(missing_docs)]
    type Model: ModelTrait<Entity = Self> + FromQueryResult;

    #[allow(missing_docs)]
    type ActiveModel: ActiveModelBehavior<Entity = Self>;

    #[allow(missing_docs)]
    type Column: ColumnTrait;

    #[allow(missing_docs)]
    type Relation: RelationTrait;

    #[allow(missing_docs)]
    type PrimaryKey: PrimaryKeyTrait + PrimaryKeyToColumn<Column = Self::Column>;

    /// Construct a belongs to relation
    fn belongs_to<R>(related: R) -> RelationBuilder<Self, R>
    where
        R: EntityTrait,
    {
        RelationBuilder::new(RelationType::HasOne, Self::default(), related, false)
    }

    /// Construct a has one relation
    fn has_one<R>(_: R) -> RelationBuilder<Self, R>
    where
        R: EntityTrait + Related<Self>,
    {
        RelationBuilder::from_rel(RelationType::HasOne, R::to().rev(), true)
    }

    /// Construct a has many relation
    fn has_many<R>(_: R) -> RelationBuilder<Self, R>
    where
        R: EntityTrait + Related<Self>,
    {
        RelationBuilder::from_rel(RelationType::HasMany, R::to().rev(), true)
    }

    /// Construct select statement to find one / all models
    ///
    /// - To select columns, join tables and group by expressions, see [`QuerySelect`](crate::query::QuerySelect)
    /// - To apply where conditions / filters, see [`QueryFilter`](crate::query::QueryFilter)
    /// - To apply order by expressions, see [`QueryOrder`](crate::query::QueryOrder)
    ///
    /// # Example
    ///
    /// ```
    /// # use sea_orm::{error::*, tests_cfg::*, *};
    /// #
    /// # #[smol_potat::main]
    /// # #[cfg(feature = "mock")]
    /// # pub async fn main() -> Result<(), DbErr> {
    /// #
    /// # let db = MockDatabase::new(DbBackend::Postgres)
    /// #     .append_query_results([
    /// #         vec![
    /// #             cake::Model {
    /// #                 id: 1,
    /// #                 name: "New York Cheese".to_owned(),
    /// #             },
    /// #         ],
    /// #         vec![
    /// #             cake::Model {
    /// #                 id: 1,
    /// #                 name: "New York Cheese".to_owned(),
    /// #             },
    /// #             cake::Model {
    /// #                 id: 2,
    /// #                 name: "Chocolate Forest".to_owned(),
    /// #             },
    /// #         ],
    /// #     ])
    /// #     .into_connection();
    /// #
    /// use sea_orm::{entity::*, query::*, tests_cfg::cake};
    ///
    /// assert_eq!(
    ///     cake::Entity::find().one(&db).await?,
    ///     Some(cake::Model {
    ///         id: 1,
    ///         name: "New York Cheese".to_owned(),
    ///     })
    /// );
    ///
    /// assert_eq!(
    ///     cake::Entity::find().all(&db).await?,
    ///     [
    ///         cake::Model {
    ///             id: 1,
    ///             name: "New York Cheese".to_owned(),
    ///         },
    ///         cake::Model {
    ///             id: 2,
    ///             name: "Chocolate Forest".to_owned(),
    ///         },
    ///     ]
    /// );
    ///
    /// assert_eq!(
    ///     db.into_transaction_log(),
    ///     [
    ///         Transaction::from_sql_and_values(
    ///             DbBackend::Postgres,
    ///             r#"SELECT "cake"."id", "cake"."name" FROM "cake" LIMIT $1"#,
    ///             [1u64.into()]
    ///         ),
    ///         Transaction::from_sql_and_values(
    ///             DbBackend::Postgres,
    ///             r#"SELECT "cake"."id", "cake"."name" FROM "cake""#,
    ///             []
    ///         ),
    ///     ]
    /// );
    /// #
    /// # Ok(())
    /// # }
    /// ```
    fn find() -> Select<Self> {
        Select::new()
    }

    /// Find a model by primary key
    ///
    /// # Example
    ///
    /// ```
    /// # use sea_orm::{error::*, tests_cfg::*, *};
    /// #
    /// # #[smol_potat::main]
    /// # #[cfg(feature = "mock")]
    /// # pub async fn main() -> Result<(), DbErr> {
    /// #
    /// # let db = MockDatabase::new(DbBackend::Postgres)
    /// #     .append_query_results([
    /// #         [
    /// #             cake::Model {
    /// #                 id: 11,
    /// #                 name: "Sponge Cake".to_owned(),
    /// #             },
    /// #         ],
    /// #     ])
    /// #     .into_connection();
    /// #
    /// use sea_orm::{entity::*, query::*, tests_cfg::cake};
    ///
    /// assert_eq!(
    ///     cake::Entity::find_by_id(11).all(&db).await?,
    ///     [cake::Model {
    ///         id: 11,
    ///         name: "Sponge Cake".to_owned(),
    ///     }]
    /// );
    ///
    /// assert_eq!(
    ///     db.into_transaction_log(),
    ///     [Transaction::from_sql_and_values(
    ///         DbBackend::Postgres,
    ///         r#"SELECT "cake"."id", "cake"."name" FROM "cake" WHERE "cake"."id" = $1"#,
    ///         [11i32.into()]
    ///     )]
    /// );
    /// #
    /// # Ok(())
    /// # }
    /// ```
    /// Find by composite key
    /// ```
    /// # use sea_orm::{error::*, tests_cfg::*, *};
    /// #
    /// # #[smol_potat::main]
    /// # #[cfg(feature = "mock")]
    /// # pub async fn main() -> Result<(), DbErr> {
    /// #
    /// # let db = MockDatabase::new(DbBackend::Postgres)
    /// #     .append_query_results([
    /// #         [
    /// #             cake_filling::Model {
    /// #                 cake_id: 2,
    /// #                 filling_id: 3,
    /// #             },
    /// #         ],
    /// #     ])
    /// #     .into_connection();
    /// #
    /// use sea_orm::{entity::*, query::*, tests_cfg::cake_filling};
    ///
    /// assert_eq!(
    ///     cake_filling::Entity::find_by_id((2, 3)).all(&db).await?,
    ///     [cake_filling::Model {
    ///         cake_id: 2,
    ///         filling_id: 3,
    ///     }]
    /// );
    ///
    /// assert_eq!(
    ///     db.into_transaction_log(),
    ///     [Transaction::from_sql_and_values(
    ///         DbBackend::Postgres,
    ///         [
    ///             r#"SELECT "cake_filling"."cake_id", "cake_filling"."filling_id" FROM "cake_filling""#,
    ///             r#"WHERE "cake_filling"."cake_id" = $1 AND "cake_filling"."filling_id" = $2"#,
    ///         ].join(" ").as_str(),
    ///         [2i32.into(), 3i32.into()]
    ///     )]);
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if arity of input values don't match arity of primary key
    fn find_by_id<T>(values: T) -> Select<Self>
    where
        T: Into<<Self::PrimaryKey as PrimaryKeyTrait>::ValueType>,
    {
        let mut select = Self::find();
        let mut keys = Self::PrimaryKey::iter();
        for v in values.into().into_value_tuple() {
            if let Some(key) = keys.next() {
                let col = key.into_column();
                select = select.filter(col.eq(v));
            } else {
                panic!("primary key arity mismatch");
            }
        }
        if keys.next().is_some() {
            panic!("primary key arity mismatch");
        }
        select
    }

    /// Insert a model into database
    ///
    /// # Example (Postgres)
    ///
    /// ```
    /// # use sea_orm::{error::*, tests_cfg::*, *};
    /// #
    /// # #[smol_potat::main]
    /// # #[cfg(feature = "mock")]
    /// # pub async fn main() -> Result<(), DbErr> {
    /// #
    /// # let db = MockDatabase::new(DbBackend::Postgres)
    /// #     .append_query_results([[maplit::btreemap! {
    /// #         "id" => Into::<Value>::into(15),
    /// #     }]])
    /// #     .into_connection();
    /// #
    /// use sea_orm::{entity::*, query::*, tests_cfg::cake};
    ///
    /// let apple = cake::ActiveModel {
    ///     name: Set("Apple Pie".to_owned()),
    ///     ..Default::default()
    /// };
    ///
    /// let insert_result = cake::Entity::insert(apple).exec(&db).await?;
    ///
    /// assert_eq!(dbg!(insert_result.last_insert_id), 15);
    ///
    /// assert_eq!(
    ///     db.into_transaction_log(),
    ///     [Transaction::from_sql_and_values(
    ///         DbBackend::Postgres,
    ///         r#"INSERT INTO "cake" ("name") VALUES ($1) RETURNING "id""#,
    ///         ["Apple Pie".into()]
    ///     )]
    /// );
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Example (MySQL)
    ///
    /// ```
    /// # use sea_orm::{error::*, tests_cfg::*, *};
    /// #
    /// # #[smol_potat::main]
    /// # #[cfg(feature = "mock")]
    /// # pub async fn main() -> Result<(), DbErr> {
    /// #
    /// # let db = MockDatabase::new(DbBackend::MySql)
    /// #     .append_exec_results([
    /// #         MockExecResult {
    /// #             last_insert_id: 15,
    /// #             rows_affected: 1,
    /// #         },
    /// #     ])
    /// #     .into_connection();
    /// #
    /// use sea_orm::{entity::*, query::*, tests_cfg::cake};
    ///
    /// let apple = cake::ActiveModel {
    ///     name: Set("Apple Pie".to_owned()),
    ///     ..Default::default()
    /// };
    ///
    /// let insert_result = cake::Entity::insert(apple).exec(&db).await?;
    ///
    /// assert_eq!(insert_result.last_insert_id, 15);
    ///
    /// assert_eq!(
    ///     db.into_transaction_log(),
    ///     [Transaction::from_sql_and_values(
    ///         DbBackend::MySql,
    ///         r#"INSERT INTO `cake` (`name`) VALUES (?)"#,
    ///         ["Apple Pie".into()]
    ///     )]
    /// );
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// To get back inserted Model
    ///
    /// ```
    /// # use sea_orm::{error::*, tests_cfg::*, *};
    /// #
    /// # #[smol_potat::main]
    /// # #[cfg(feature = "mock")]
    /// # pub async fn main() -> Result<(), DbErr> {
    /// #
    /// # let db = MockDatabase::new(DbBackend::Postgres)
    /// #     .append_query_results([
    /// #         [cake::Model {
    /// #             id: 1,
    /// #             name: "Apple Pie".to_owned(),
    /// #         }],
    /// #     ])
    /// #     .into_connection();
    /// #
    /// use sea_orm::{entity::*, query::*, tests_cfg::fruit};
    ///
    /// assert_eq!(
    ///     cake::Entity::insert(cake::ActiveModel {
    ///         id: NotSet,
    ///         name: Set("Apple Pie".to_owned()),
    ///     })
    ///     .exec_with_returning(&db)
    ///     .await?,
    ///     cake::Model {
    ///         id: 1,
    ///         name: "Apple Pie".to_owned(),
    ///     }
    /// );
    ///
    /// assert_eq!(
    ///     db.into_transaction_log()[0].statements()[0].sql,
    ///     r#"INSERT INTO "cake" ("name") VALUES ($1) RETURNING "id", "name""#
    /// );
    /// #
    /// # Ok(())
    /// # }
    /// ```
    fn insert<A>(model: A) -> Insert<A>
    where
        A: ActiveModelTrait<Entity = Self>,
    {
        Insert::one(model)
    }

    /// Insert many models into database
    ///
    /// # Example (Postgres)
    ///
    /// ```
    /// # use sea_orm::{error::*, tests_cfg::*, *};
    /// #
    /// # #[smol_potat::main]
    /// # #[cfg(feature = "mock")]
    /// # pub async fn main() -> Result<(), DbErr> {
    /// #
    /// # let db = MockDatabase::new(DbBackend::Postgres)
    /// #     .append_query_results([[maplit::btreemap! {
    /// #         "id" => Into::<Value>::into(28),
    /// #     }]])
    /// #     .into_connection();
    /// #
    /// use sea_orm::{entity::*, query::*, tests_cfg::cake};
    ///
    /// let apple = cake::ActiveModel {
    ///     name: Set("Apple Pie".to_owned()),
    ///     ..Default::default()
    /// };
    /// let orange = cake::ActiveModel {
    ///     name: Set("Orange Scone".to_owned()),
    ///     ..Default::default()
    /// };
    ///
    /// let insert_result = cake::Entity::insert_many([apple, orange]).exec(&db).await?;
    ///
    /// assert_eq!(insert_result.last_insert_id, 28);
    ///
    /// assert_eq!(
    ///     db.into_transaction_log(),
    ///     [Transaction::from_sql_and_values(
    ///         DbBackend::Postgres,
    ///         r#"INSERT INTO "cake" ("name") VALUES ($1), ($2) RETURNING "id""#,
    ///         ["Apple Pie".into(), "Orange Scone".into()]
    ///     )]
    /// );
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Example (MySQL)
    ///
    /// ```
    /// # use sea_orm::{error::*, tests_cfg::*, *};
    /// #
    /// # #[smol_potat::main]
    /// # #[cfg(feature = "mock")]
    /// # pub async fn main() -> Result<(), DbErr> {
    /// #
    /// # let db = MockDatabase::new(DbBackend::MySql)
    /// #     .append_exec_results([
    /// #         MockExecResult {
    /// #             last_insert_id: 28,
    /// #             rows_affected: 2,
    /// #         },
    /// #     ])
    /// #     .into_connection();
    /// #
    /// use sea_orm::{entity::*, query::*, tests_cfg::cake};
    ///
    /// let apple = cake::ActiveModel {
    ///     name: Set("Apple Pie".to_owned()),
    ///     ..Default::default()
    /// };
    /// let orange = cake::ActiveModel {
    ///     name: Set("Orange Scone".to_owned()),
    ///     ..Default::default()
    /// };
    ///
    /// let insert_result = cake::Entity::insert_many([apple, orange]).exec(&db).await?;
    ///
    /// assert_eq!(insert_result.last_insert_id, 28);
    ///
    /// assert_eq!(
    ///     db.into_transaction_log(),
    ///     [Transaction::from_sql_and_values(
    ///         DbBackend::MySql,
    ///         r#"INSERT INTO `cake` (`name`) VALUES (?), (?)"#,
    ///         ["Apple Pie".into(), "Orange Scone".into()]
    ///     )]
    /// );
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Before 1.1.3, if the active models have different column set, this method would panic.
    /// Now, it'd attempt to fill in the missing columns with null
    /// (which may or may not be correct, depending on whether the column is nullable):
    ///
    /// ```
    /// use sea_orm::{
    ///     entity::*,
    ///     query::*,
    ///     tests_cfg::{cake, cake_filling},
    ///     DbBackend,
    /// };
    ///
    /// assert_eq!(
    ///     cake::Entity::insert_many([
    ///         cake::ActiveModel {
    ///             id: NotSet,
    ///             name: Set("Apple Pie".to_owned()),
    ///         },
    ///         cake::ActiveModel {
    ///             id: NotSet,
    ///             name: Set("Orange Scone".to_owned()),
    ///         }
    ///     ])
    ///     .build(DbBackend::Postgres)
    ///     .to_string(),
    ///     r#"INSERT INTO "cake" ("name") VALUES ('Apple Pie'), ('Orange Scone')"#,
    /// );
    ///
    /// assert_eq!(
    ///     cake_filling::Entity::insert_many([
    ///         cake_filling::ActiveModel {
    ///             cake_id: ActiveValue::set(2),
    ///             filling_id: ActiveValue::NotSet,
    ///         },
    ///         cake_filling::ActiveModel {
    ///             cake_id: ActiveValue::NotSet,
    ///             filling_id: ActiveValue::set(3),
    ///         }
    ///     ])
    ///     .build(DbBackend::Postgres)
    ///     .to_string(),
    ///     r#"INSERT INTO "cake_filling" ("cake_id", "filling_id") VALUES (2, NULL), (NULL, 3)"#,
    /// );
    /// ```
    ///
    /// To get back inserted Models
    ///
    /// ```
    /// # use sea_orm::{error::*, tests_cfg::*, *};
    /// #
    /// # #[smol_potat::main]
    /// # #[cfg(feature = "mock")]
    /// # pub async fn main() -> Result<(), DbErr> {
    /// #
    /// # let db = MockDatabase::new(DbBackend::Postgres)
    /// #     .append_query_results([
    /// #         [cake::Model {
    /// #             id: 1,
    /// #             name: "Apple Pie".to_owned(),
    /// #         }, cake::Model {
    /// #             id: 2,
    /// #             name: "Choco Pie".to_owned(),
    /// #         }],
    /// #     ])
    /// #     .into_connection();
    /// #
    /// use sea_orm::{entity::*, query::*, tests_cfg::fruit};
    ///
    /// assert_eq!(
    ///     cake::Entity::insert_many([
    ///         cake::ActiveModel {
    ///             id: NotSet,
    ///             name: Set("Apple Pie".to_owned()),
    ///         },
    ///         cake::ActiveModel {
    ///             id: NotSet,
    ///             name: Set("Choco Pie".to_owned()),
    ///         },
    ///     ])
    ///     .exec_with_returning_many(&db)
    ///     .await?,
    ///     [
    ///         cake::Model {
    ///             id: 1,
    ///             name: "Apple Pie".to_owned(),
    ///         },
    ///         cake::Model {
    ///             id: 2,
    ///             name: "Choco Pie".to_owned(),
    ///         }
    ///     ]
    /// );
    ///
    /// assert_eq!(
    ///     db.into_transaction_log()[0].statements()[0].sql,
    ///     r#"INSERT INTO "cake" ("name") VALUES ($1), ($2) RETURNING "id", "name""#
    /// );
    /// #
    /// # Ok(())
    /// # }
    /// ```
    fn insert_many<A, I>(models: I) -> Insert<A>
    where
        A: ActiveModelTrait<Entity = Self>,
        I: IntoIterator<Item = A>,
    {
        Insert::many(models)
    }

    /// Update a model in database
    ///
    /// - To apply where conditions / filters, see [`QueryFilter`](crate::query::QueryFilter)
    ///
    /// # Example (Postgres)
    ///
    /// ```
    /// # use sea_orm::{error::*, tests_cfg::*, *};
    /// #
    /// # #[smol_potat::main]
    /// # #[cfg(feature = "mock")]
    /// # pub async fn main() -> Result<(), DbErr> {
    /// #
    /// # let db = MockDatabase::new(DbBackend::Postgres)
    /// #     .append_query_results([
    /// #         [fruit::Model {
    /// #             id: 1,
    /// #             name: "Orange".to_owned(),
    /// #             cake_id: None,
    /// #         }],
    /// #     ])
    /// #     .into_connection();
    /// #
    /// use sea_orm::{entity::*, query::*, tests_cfg::fruit};
    ///
    /// let orange = fruit::ActiveModel {
    ///     id: Set(1),
    ///     name: Set("Orange".to_owned()),
    ///     ..Default::default()
    /// };
    ///
    /// assert_eq!(
    ///     fruit::Entity::update(orange.clone())
    ///         .filter(fruit::Column::Name.contains("orange"))
    ///         .exec(&db)
    ///         .await?,
    ///     fruit::Model {
    ///         id: 1,
    ///         name: "Orange".to_owned(),
    ///         cake_id: None,
    ///     }
    /// );
    ///
    /// assert_eq!(
    ///     db.into_transaction_log(),
    ///     [Transaction::from_sql_and_values(
    ///         DbBackend::Postgres,
    ///         r#"UPDATE "fruit" SET "name" = $1 WHERE "fruit"."id" = $2 AND "fruit"."name" LIKE $3 RETURNING "id", "name", "cake_id""#,
    ///         ["Orange".into(), 1i32.into(), "%orange%".into()]
    ///     )]);
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Example (MySQL)
    ///
    /// ```
    /// # use sea_orm::{error::*, tests_cfg::*, *};
    /// #
    /// # #[smol_potat::main]
    /// # #[cfg(feature = "mock")]
    /// # pub async fn main() -> Result<(), DbErr> {
    /// #
    /// # let db = MockDatabase::new(DbBackend::MySql)
    /// #     .append_exec_results([
    /// #         MockExecResult {
    /// #             last_insert_id: 0,
    /// #             rows_affected: 1,
    /// #         },
    /// #     ])
    /// #     .append_query_results([
    /// #         [fruit::Model {
    /// #             id: 1,
    /// #             name: "Orange".to_owned(),
    /// #             cake_id: None,
    /// #         }],
    /// #     ])
    /// #     .into_connection();
    /// #
    /// use sea_orm::{entity::*, query::*, tests_cfg::fruit};
    ///
    /// let orange = fruit::ActiveModel {
    ///     id: Set(1),
    ///     name: Set("Orange".to_owned()),
    ///     ..Default::default()
    /// };
    ///
    /// assert_eq!(
    ///     fruit::Entity::update(orange.clone())
    ///         .filter(fruit::Column::Name.contains("orange"))
    ///         .exec(&db)
    ///         .await?,
    ///     fruit::Model {
    ///         id: 1,
    ///         name: "Orange".to_owned(),
    ///         cake_id: None,
    ///     }
    /// );
    ///
    /// assert_eq!(
    ///     db.into_transaction_log(),
    ///     [
    ///         Transaction::from_sql_and_values(
    ///             DbBackend::MySql,
    ///             r#"UPDATE `fruit` SET `name` = ? WHERE `fruit`.`id` = ? AND `fruit`.`name` LIKE ?"#,
    ///             ["Orange".into(), 1i32.into(), "%orange%".into()]
    ///         ),
    ///         Transaction::from_sql_and_values(
    ///             DbBackend::MySql,
    ///             r#"SELECT `fruit`.`id`, `fruit`.`name`, `fruit`.`cake_id` FROM `fruit` WHERE `fruit`.`id` = ? LIMIT ?"#,
    ///             [1i32.into(), 1u64.into()]
    ///         )]);
    /// #
    /// # Ok(())
    /// # }
    /// ```
    fn update<A>(model: A) -> UpdateOne<A>
    where
        A: ActiveModelTrait<Entity = Self>,
    {
        Update::one(model)
    }

    /// Update many models in database
    ///
    /// - To apply where conditions / filters, see [`QueryFilter`](crate::query::QueryFilter)
    ///
    /// # Example
    ///
    /// ```
    /// # use sea_orm::{error::*, tests_cfg::*, *};
    /// #
    /// # #[smol_potat::main]
    /// # #[cfg(feature = "mock")]
    /// # pub async fn main() -> Result<(), DbErr> {
    /// #
    /// # let db = MockDatabase::new(DbBackend::Postgres)
    /// #     .append_exec_results([
    /// #         MockExecResult {
    /// #             last_insert_id: 0,
    /// #             rows_affected: 5,
    /// #         },
    /// #     ])
    /// #     .into_connection();
    /// #
    /// use sea_orm::{
    ///     entity::*,
    ///     query::*,
    ///     sea_query::{Expr, Value},
    ///     tests_cfg::fruit,
    /// };
    ///
    /// let update_result = fruit::Entity::update_many()
    ///     .col_expr(fruit::Column::CakeId, Expr::value(Value::Int(None)))
    ///     .filter(fruit::Column::Name.contains("Apple"))
    ///     .exec(&db)
    ///     .await?;
    ///
    /// assert_eq!(update_result.rows_affected, 5);
    ///
    /// assert_eq!(
    ///     db.into_transaction_log(),
    ///     [Transaction::from_sql_and_values(
    ///         DbBackend::Postgres,
    ///         r#"UPDATE "fruit" SET "cake_id" = $1 WHERE "fruit"."name" LIKE $2"#,
    ///         [Value::Int(None), "%Apple%".into()]
    ///     )]
    /// );
    /// #
    /// # Ok(())
    /// # }
    /// ```
    fn update_many() -> UpdateMany<Self> {
        Update::many(Self::default())
    }

    /// Delete a model from database
    ///
    /// - To apply where conditions / filters, see [`QueryFilter`](crate::query::QueryFilter)
    ///
    /// # Example
    ///
    /// ```
    /// # use sea_orm::{error::*, tests_cfg::*, *};
    /// #
    /// # #[smol_potat::main]
    /// # #[cfg(feature = "mock")]
    /// # pub async fn main() -> Result<(), DbErr> {
    /// #
    /// # let db = MockDatabase::new(DbBackend::Postgres)
    /// #     .append_exec_results([
    /// #         MockExecResult {
    /// #             last_insert_id: 0,
    /// #             rows_affected: 1,
    /// #         },
    /// #     ])
    /// #     .into_connection();
    /// #
    /// use sea_orm::{entity::*, query::*, tests_cfg::fruit};
    ///
    /// let orange = fruit::ActiveModel {
    ///     id: Set(3),
    ///     ..Default::default()
    /// };
    ///
    /// let delete_result = fruit::Entity::delete(orange).exec(&db).await?;
    ///
    /// assert_eq!(delete_result.rows_affected, 1);
    ///
    /// assert_eq!(
    ///     db.into_transaction_log(),
    ///     [Transaction::from_sql_and_values(
    ///         DbBackend::Postgres,
    ///         r#"DELETE FROM "fruit" WHERE "fruit"."id" = $1"#,
    ///         [3i32.into()]
    ///     )]
    /// );
    /// #
    /// # Ok(())
    /// # }
    /// ```
    fn delete<A>(model: A) -> DeleteOne<A>
    where
        A: ActiveModelTrait<Entity = Self>,
    {
        Delete::one(model)
    }

    /// Delete many models from database
    ///
    /// - To apply where conditions / filters, see [`QueryFilter`](crate::query::QueryFilter)
    ///
    /// # Example
    ///
    /// ```
    /// # use sea_orm::{error::*, tests_cfg::*, *};
    /// #
    /// # #[smol_potat::main]
    /// # #[cfg(feature = "mock")]
    /// # pub async fn main() -> Result<(), DbErr> {
    /// #
    /// # let db = MockDatabase::new(DbBackend::Postgres)
    /// #     .append_exec_results([
    /// #         MockExecResult {
    /// #             last_insert_id: 0,
    /// #             rows_affected: 5,
    /// #         },
    /// #     ])
    /// #     .append_query_results([
    /// #         [cake::Model {
    /// #             id: 15,
    /// #             name: "Apple Pie".to_owned(),
    /// #         }],
    /// #     ])
    /// #     .into_connection();
    /// #
    /// use sea_orm::{entity::*, query::*, tests_cfg::fruit};
    ///
    /// let delete_result = fruit::Entity::delete_many()
    ///     .filter(fruit::Column::Name.contains("Apple"))
    ///     .exec(&db)
    ///     .await?;
    ///
    /// assert_eq!(delete_result.rows_affected, 5);
    ///
    /// assert_eq!(
    ///     db.into_transaction_log(),
    ///     [Transaction::from_sql_and_values(
    ///         DbBackend::Postgres,
    ///         r#"DELETE FROM "fruit" WHERE "fruit"."name" LIKE $1"#,
    ///         ["%Apple%".into()]
    ///     )]
    /// );
    /// #
    /// # Ok(())
    /// # }
    /// ```
    fn delete_many() -> DeleteMany<Self> {
        Delete::many(Self::default())
    }

    /// Delete a model based on primary key
    ///
    /// # Example
    ///
    /// ```
    /// # use sea_orm::{error::*, tests_cfg::*, *};
    /// #
    /// # #[smol_potat::main]
    /// # #[cfg(feature = "mock")]
    /// # pub async fn main() -> Result<(), DbErr> {
    /// #
    /// # let db = MockDatabase::new(DbBackend::Postgres)
    /// #     .append_exec_results([
    /// #         MockExecResult {
    /// #             last_insert_id: 0,
    /// #             rows_affected: 1,
    /// #         },
    /// #     ])
    /// #     .into_connection();
    /// #
    /// use sea_orm::{entity::*, query::*, tests_cfg::fruit};
    ///
    /// let delete_result = fruit::Entity::delete_by_id(1).exec(&db).await?;
    ///
    /// assert_eq!(delete_result.rows_affected, 1);
    ///
    /// assert_eq!(
    ///     db.into_transaction_log(),
    ///     [Transaction::from_sql_and_values(
    ///         DbBackend::Postgres,
    ///         r#"DELETE FROM "fruit" WHERE "fruit"."id" = $1"#,
    ///         [1i32.into()]
    ///     )]
    /// );
    /// #
    /// # Ok(())
    /// # }
    /// ```
    /// Delete by composite key
    /// ```
    /// # use sea_orm::{error::*, tests_cfg::*, *};
    /// #
    /// # #[smol_potat::main]
    /// # #[cfg(feature = "mock")]
    /// # pub async fn main() -> Result<(), DbErr> {
    ///
    /// # let db = MockDatabase::new(DbBackend::Postgres)
    /// #     .append_exec_results([
    /// #         MockExecResult {
    /// #             last_insert_id: 0,
    /// #             rows_affected: 1,
    /// #         },
    /// #     ])
    /// #     .into_connection();
    /// #
    /// use sea_orm::{entity::*, query::*, tests_cfg::cake_filling};
    ///
    /// let delete_result = cake_filling::Entity::delete_by_id((2, 3)).exec(&db).await?;
    ///
    /// assert_eq!(delete_result.rows_affected, 1);
    ///
    /// assert_eq!(
    ///     db.into_transaction_log(),
    ///     [Transaction::from_sql_and_values(
    ///         DbBackend::Postgres,
    ///         r#"DELETE FROM "cake_filling" WHERE "cake_filling"."cake_id" = $1 AND "cake_filling"."filling_id" = $2"#,
    ///         [2i32.into(), 3i32.into()]
    ///     )]
    /// );
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if arity of input values don't match arity of primary key
    fn delete_by_id<T>(values: T) -> DeleteMany<Self>
    where
        T: Into<<Self::PrimaryKey as PrimaryKeyTrait>::ValueType>,
    {
        let mut delete = Self::delete_many();
        let mut keys = Self::PrimaryKey::iter();
        for v in values.into().into_value_tuple() {
            if let Some(key) = keys.next() {
                let col = key.into_column();
                delete = delete.filter(col.eq(v));
            } else {
                panic!("primary key arity mismatch");
            }
        }
        if keys.next().is_some() {
            panic!("primary key arity mismatch");
        }
        delete
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_delete_by_id_1() {
        use crate::tests_cfg::cake;
        use crate::{entity::*, query::*, DbBackend};
        assert_eq!(
            cake::Entity::delete_by_id(1)
                .build(DbBackend::Sqlite)
                .to_string(),
            r#"DELETE FROM "cake" WHERE "cake"."id" = 1"#,
        );
    }

    #[test]
    fn test_delete_by_id_2() {
        use crate::tests_cfg::cake_filling_price;
        use crate::{entity::*, query::*, DbBackend};
        assert_eq!(
            cake_filling_price::Entity::delete_by_id((1, 2))
                .build(DbBackend::Sqlite)
                .to_string(),
            r#"DELETE FROM "public"."cake_filling_price" WHERE "cake_filling_price"."cake_id" = 1 AND "cake_filling_price"."filling_id" = 2"#,
        );
    }

    #[test]
    #[cfg(feature = "macros")]
    fn entity_model_1() {
        use crate::entity::*;

        mod hello {
            use crate as sea_orm;
            use crate::entity::prelude::*;

            #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
            #[sea_orm(table_name = "hello")]
            pub struct Model {
                #[sea_orm(primary_key)]
                pub id: i32,
            }

            #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
            pub enum Relation {}

            impl ActiveModelBehavior for ActiveModel {}
        }

        assert_eq!(hello::Entity.table_name(), "hello");
        assert_eq!(hello::Entity.schema_name(), None);
    }

    #[test]
    #[cfg(feature = "macros")]
    fn entity_model_2() {
        use crate::entity::*;

        mod hello {
            use crate as sea_orm;
            use crate::entity::prelude::*;

            #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
            #[sea_orm(table_name = "hello", schema_name = "world")]
            pub struct Model {
                #[sea_orm(primary_key)]
                pub id: i32,
            }

            #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
            pub enum Relation {}

            impl ActiveModelBehavior for ActiveModel {}
        }

        assert_eq!(hello::Entity.table_name(), "hello");
        assert_eq!(hello::Entity.schema_name(), Some("world"));
    }

    #[test]
    #[cfg(feature = "macros")]
    fn entity_model_3() {
        use crate::{entity::*, query::*, DbBackend};
        use std::borrow::Cow;

        mod hello {
            use crate as sea_orm;
            use crate::entity::prelude::*;

            #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
            #[sea_orm(table_name = "hello", schema_name = "world")]
            pub struct Model {
                #[sea_orm(primary_key, auto_increment = false)]
                pub id: String,
            }

            #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
            pub enum Relation {}

            impl ActiveModelBehavior for ActiveModel {}
        }

        fn delete_by_id<T>(value: T)
        where
            T: Into<<<hello::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType>,
        {
            assert_eq!(
                hello::Entity::delete_by_id(value)
                    .build(DbBackend::Sqlite)
                    .to_string(),
                r#"DELETE FROM "world"."hello" WHERE "hello"."id" = 'UUID'"#
            );
        }

        delete_by_id("UUID".to_string());
        delete_by_id("UUID");
        delete_by_id(Cow::from("UUID"));
    }
}

use crate::{
    ActiveModelBehavior, ActiveModelTrait, ConnectionTrait, DbErr, DeleteResult, EntityTrait,
    IntoActiveModel, Linked, QueryFilter, QueryResult, Related, Select, SelectModel, SelectorRaw,
    Statement, TryGetError,
};
use async_trait::async_trait;
pub use sea_query::Value;
use std::fmt::Debug;

/// The interface for Model, implemented by data structs
#[async_trait]
pub trait ModelTrait: Clone + Send + Debug {
    #[allow(missing_docs)]
    type Entity: EntityTrait;

    /// Get the [Value] of a column from an Entity
    fn get(&self, c: <Self::Entity as EntityTrait>::Column) -> Value;

    /// Set the [Value] of a column in an Entity
    fn set(&mut self, c: <Self::Entity as EntityTrait>::Column, v: Value);

    /// Find related Models
    fn find_related<R>(&self, _: R) -> Select<R>
    where
        R: EntityTrait,
        Self::Entity: Related<R>,
    {
        <Self::Entity as Related<R>>::find_related().belongs_to(self)
    }

    /// Find linked Models
    fn find_linked<L>(&self, l: L) -> Select<L::ToEntity>
    where
        L: Linked<FromEntity = Self::Entity>,
    {
        let tbl_alias = &format!("r{}", l.link().len() - 1);
        l.find_linked().belongs_to_tbl_alias(self, tbl_alias)
    }

    /// Delete a model
    async fn delete<'a, A, C>(self, db: &'a C) -> Result<DeleteResult, DbErr>
    where
        Self: IntoActiveModel<A>,
        C: ConnectionTrait,
        A: ActiveModelTrait<Entity = Self::Entity> + ActiveModelBehavior + Send + 'a,
    {
        self.into_active_model().delete(db).await
    }
}

/// A Trait for implementing a [QueryResult]
pub trait FromQueryResult: Sized {
    /// Instantiate a Model from a [QueryResult]
    ///
    /// NOTE: Please also override `from_query_result_nullable` when manually implementing.
    ///       The future default implementation will be along the lines of:
    ///
    /// ```rust,ignore
    /// fn from_query_result(res: &QueryResult, pre: &str) -> Result<Self, DbErr> {
    ///     (Self::from_query_result_nullable(res, pre)?)
    /// }
    /// ```
    fn from_query_result(res: &QueryResult, pre: &str) -> Result<Self, DbErr>;

    /// Transform the error from instantiating a Model from a [QueryResult]
    /// and converting it to an [Option]
    fn from_query_result_optional(res: &QueryResult, pre: &str) -> Result<Option<Self>, DbErr> {
        Ok(Self::from_query_result(res, pre).ok())

        // would really like to do the following, but can't without version bump:
        // match Self::from_query_result_nullable(res, pre) {
        //     Ok(v) => Ok(Some(v)),
        //     Err(TryGetError::Null(_)) => Ok(None),
        //     Err(TryGetError::DbErr(err)) => Err(err),
        // }
    }

    /// Transform the error from instantiating a Model from a [QueryResult]
    /// and converting it to an [Option]
    ///
    /// NOTE: This will most likely stop being a provided method in the next major version!
    fn from_query_result_nullable(res: &QueryResult, pre: &str) -> Result<Self, TryGetError> {
        Self::from_query_result(res, pre).map_err(TryGetError::DbErr)
    }

    /// ```
    /// # use sea_orm::{error::*, tests_cfg::*, *};
    /// #
    /// # #[smol_potat::main]
    /// # #[cfg(feature = "mock")]
    /// # pub async fn main() -> Result<(), DbErr> {
    /// #
    /// # let db = MockDatabase::new(DbBackend::Postgres)
    /// #     .append_query_results([[
    /// #         maplit::btreemap! {
    /// #             "name" => Into::<Value>::into("Chocolate Forest"),
    /// #             "num_of_cakes" => Into::<Value>::into(2),
    /// #         },
    /// #     ]])
    /// #     .into_connection();
    /// #
    /// use sea_orm::{query::*, FromQueryResult};
    ///
    /// #[derive(Debug, PartialEq, FromQueryResult)]
    /// struct SelectResult {
    ///     name: String,
    ///     num_of_cakes: i32,
    /// }
    ///
    /// let res: Vec<SelectResult> = SelectResult::find_by_statement(Statement::from_sql_and_values(
    ///     DbBackend::Postgres,
    ///     r#"SELECT "name", COUNT(*) AS "num_of_cakes" FROM "cake" GROUP BY("name")"#,
    ///     [],
    /// ))
    /// .all(&db)
    /// .await?;
    ///
    /// assert_eq!(
    ///     res,
    ///     [SelectResult {
    ///         name: "Chocolate Forest".to_owned(),
    ///         num_of_cakes: 2,
    ///     },]
    /// );
    /// #
    /// # assert_eq!(
    /// #     db.into_transaction_log(),
    /// #     [Transaction::from_sql_and_values(
    /// #         DbBackend::Postgres,
    /// #         r#"SELECT "name", COUNT(*) AS "num_of_cakes" FROM "cake" GROUP BY("name")"#,
    /// #         []
    /// #     ),]
    /// # );
    /// #
    /// # Ok(())
    /// # }
    /// ```
    fn find_by_statement(stmt: Statement) -> SelectorRaw<SelectModel<Self>> {
        SelectorRaw::<SelectModel<Self>>::from_statement(stmt)
    }
}

impl<T: FromQueryResult> FromQueryResult for Option<T> {
    fn from_query_result(res: &QueryResult, pre: &str) -> Result<Self, DbErr> {
        Ok(Self::from_query_result_nullable(res, pre)?)
    }

    fn from_query_result_optional(res: &QueryResult, pre: &str) -> Result<Option<Self>, DbErr> {
        match Self::from_query_result_nullable(res, pre) {
            Ok(v) => Ok(Some(v)),
            Err(TryGetError::Null(_)) => Ok(None),
            Err(TryGetError::DbErr(err)) => Err(err),
        }
    }

    fn from_query_result_nullable(res: &QueryResult, pre: &str) -> Result<Self, TryGetError> {
        match T::from_query_result_nullable(res, pre) {
            Ok(v) => Ok(Some(v)),
            Err(TryGetError::Null(_)) => Ok(None),
            Err(err @ TryGetError::DbErr(_)) => Err(err),
        }
    }
}

/// A Trait for any type that can be converted into an Model
pub trait TryIntoModel<M>
where
    M: ModelTrait,
{
    /// Method to call to perform the conversion
    fn try_into_model(self) -> Result<M, DbErr>;
}

impl<M> TryIntoModel<M> for M
where
    M: ModelTrait,
{
    fn try_into_model(self) -> Result<M, DbErr> {
        Ok(self)
    }
}

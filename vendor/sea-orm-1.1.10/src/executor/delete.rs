use crate::{
    error::*, ActiveModelTrait, ColumnTrait, ConnectionTrait, DeleteMany, DeleteOne, EntityTrait,
    Iterable,
};
use sea_query::{DeleteStatement, Query};
use std::future::Future;

use super::{SelectModel, SelectorRaw};

/// Handles DELETE operations in a ActiveModel using [DeleteStatement]
#[derive(Clone, Debug)]
pub struct Deleter {
    query: DeleteStatement,
}

/// The result of a DELETE operation
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeleteResult {
    /// The number of rows affected by the DELETE operation
    pub rows_affected: u64,
}

impl<'a, A: 'a> DeleteOne<A>
where
    A: ActiveModelTrait,
{
    /// Execute a DELETE operation on one ActiveModel
    pub fn exec<C>(self, db: &'a C) -> impl Future<Output = Result<DeleteResult, DbErr>> + 'a
    where
        C: ConnectionTrait,
    {
        // so that self is dropped before entering await
        exec_delete_only(self.query, db)
    }

    /// Execute an delete operation and return the deleted model
    ///
    /// # Panics
    ///
    /// Panics if the database backend does not support `DELETE RETURNING`
    pub fn exec_with_returning<C>(
        self,
        db: &'a C,
    ) -> impl Future<Output = Result<Option<<A::Entity as EntityTrait>::Model>, DbErr>> + 'a
    where
        C: ConnectionTrait,
    {
        exec_delete_with_returning_one::<A::Entity, _>(self.query, db)
    }
}

impl<'a, E> DeleteMany<E>
where
    E: EntityTrait,
{
    /// Execute a DELETE operation on many ActiveModels
    pub fn exec<C>(self, db: &'a C) -> impl Future<Output = Result<DeleteResult, DbErr>> + 'a
    where
        C: ConnectionTrait,
    {
        // so that self is dropped before entering await
        exec_delete_only(self.query, db)
    }

    /// Execute an delete operation and return the deleted model
    ///
    /// # Panics
    ///
    /// Panics if the database backend does not support `DELETE RETURNING`
    pub fn exec_with_returning<C>(
        self,
        db: &C,
    ) -> impl Future<Output = Result<Vec<E::Model>, DbErr>> + '_
    where
        E: EntityTrait,
        C: ConnectionTrait,
    {
        exec_delete_with_returning_many::<E, _>(self.query, db)
    }
}

impl Deleter {
    /// Instantiate a new [Deleter] by passing it a [DeleteStatement]
    pub fn new(query: DeleteStatement) -> Self {
        Self { query }
    }

    /// Execute a DELETE operation
    pub fn exec<C>(self, db: &C) -> impl Future<Output = Result<DeleteResult, DbErr>> + '_
    where
        C: ConnectionTrait,
    {
        exec_delete(self.query, db)
    }

    /// Execute an delete operation and return the deleted model
    ///
    /// # Panics
    ///
    /// Panics if the database backend does not support `DELETE RETURNING`
    pub fn exec_with_returning<E, C>(
        self,
        db: &C,
    ) -> impl Future<Output = Result<Vec<E::Model>, DbErr>> + '_
    where
        E: EntityTrait,
        C: ConnectionTrait,
    {
        exec_delete_with_returning_many::<E, _>(self.query, db)
    }
}

async fn exec_delete_only<C>(query: DeleteStatement, db: &C) -> Result<DeleteResult, DbErr>
where
    C: ConnectionTrait,
{
    Deleter::new(query).exec(db).await
}

async fn exec_delete<C>(query: DeleteStatement, db: &C) -> Result<DeleteResult, DbErr>
where
    C: ConnectionTrait,
{
    let builder = db.get_database_backend();
    let statement = builder.build(&query);

    let result = db.execute(statement).await?;
    Ok(DeleteResult {
        rows_affected: result.rows_affected(),
    })
}

async fn exec_delete_with_returning_one<E, C>(
    mut query: DeleteStatement,
    db: &C,
) -> Result<Option<E::Model>, DbErr>
where
    E: EntityTrait,
    C: ConnectionTrait,
{
    let models = match db.support_returning() {
        true => {
            let db_backend = db.get_database_backend();
            let delete_statement = db_backend.build(&query.returning_all().to_owned());
            SelectorRaw::<SelectModel<<E>::Model>>::from_statement(delete_statement)
                .one(db)
                .await?
        }
        false => unimplemented!("Database backend doesn't support RETURNING"),
    };
    Ok(models)
}

async fn exec_delete_with_returning_many<E, C>(
    mut query: DeleteStatement,
    db: &C,
) -> Result<Vec<E::Model>, DbErr>
where
    E: EntityTrait,
    C: ConnectionTrait,
{
    let models = match db.support_returning() {
        true => {
            let db_backend = db.get_database_backend();
            let returning = Query::returning().exprs(
                E::Column::iter().map(|c| c.select_enum_as(c.into_returning_expr(db_backend))),
            );
            let query = query.returning(returning);
            let delete_statement = db_backend.build(&query.to_owned());
            SelectorRaw::<SelectModel<<E>::Model>>::from_statement(delete_statement)
                .all(db)
                .await?
        }
        false => unimplemented!("Database backend doesn't support RETURNING"),
    };
    Ok(models)
}

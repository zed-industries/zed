use crate::{
    error::*, ActiveModelTrait, ColumnTrait, ConnectionTrait, DbBackend, EntityTrait, Insert,
    IntoActiveModel, Iterable, PrimaryKeyToColumn, PrimaryKeyTrait, SelectModel, SelectorRaw,
    TryFromU64, TryInsert,
};
use sea_query::{FromValueTuple, Iden, InsertStatement, Query, ValueTuple};
use std::{future::Future, marker::PhantomData};

type PrimaryKey<A> = <<A as ActiveModelTrait>::Entity as EntityTrait>::PrimaryKey;

/// Defines a structure to perform INSERT operations in an ActiveModel
#[derive(Debug)]
pub struct Inserter<A>
where
    A: ActiveModelTrait,
{
    primary_key: Option<ValueTuple>,
    query: InsertStatement,
    model: PhantomData<A>,
}

/// The result of an INSERT operation on an ActiveModel
#[derive(Debug)]
pub struct InsertResult<A>
where
    A: ActiveModelTrait,
{
    /// The id performed when AUTOINCREMENT was performed on the PrimaryKey
    pub last_insert_id: <PrimaryKey<A> as PrimaryKeyTrait>::ValueType,
}

/// The types of results for an INSERT operation
#[derive(Debug)]
pub enum TryInsertResult<T> {
    /// The INSERT statement did not have any value to insert
    Empty,
    /// The INSERT operation did not insert any valid value
    Conflicted,
    /// Successfully inserted
    Inserted(T),
}

impl<A> TryInsert<A>
where
    A: ActiveModelTrait,
{
    /// Execute an insert operation
    #[allow(unused_mut)]
    pub async fn exec<'a, C>(self, db: &'a C) -> Result<TryInsertResult<InsertResult<A>>, DbErr>
    where
        C: ConnectionTrait,
        A: 'a,
    {
        if self.insert_struct.columns.is_empty() {
            return Ok(TryInsertResult::Empty);
        }
        let res = self.insert_struct.exec(db).await;
        match res {
            Ok(res) => Ok(TryInsertResult::Inserted(res)),
            Err(DbErr::RecordNotInserted) => Ok(TryInsertResult::Conflicted),
            Err(err) => Err(err),
        }
    }

    /// Execute an insert operation without returning (don't use `RETURNING` syntax)
    /// Number of rows affected is returned
    pub async fn exec_without_returning<'a, C>(
        self,
        db: &'a C,
    ) -> Result<TryInsertResult<u64>, DbErr>
    where
        <A::Entity as EntityTrait>::Model: IntoActiveModel<A>,
        C: ConnectionTrait,
        A: 'a,
    {
        if self.insert_struct.columns.is_empty() {
            return Ok(TryInsertResult::Empty);
        }
        let res = self.insert_struct.exec_without_returning(db).await;
        match res {
            Ok(res) => Ok(TryInsertResult::Inserted(res)),
            Err(DbErr::RecordNotInserted) => Ok(TryInsertResult::Conflicted),
            Err(err) => Err(err),
        }
    }

    /// Execute an insert operation and return the inserted model (use `RETURNING` syntax if supported)
    pub async fn exec_with_returning<'a, C>(
        self,
        db: &'a C,
    ) -> Result<TryInsertResult<<A::Entity as EntityTrait>::Model>, DbErr>
    where
        <A::Entity as EntityTrait>::Model: IntoActiveModel<A>,
        C: ConnectionTrait,
        A: 'a,
    {
        if self.insert_struct.columns.is_empty() {
            return Ok(TryInsertResult::Empty);
        }
        let res = self.insert_struct.exec_with_returning(db).await;
        match res {
            Ok(res) => Ok(TryInsertResult::Inserted(res)),
            Err(DbErr::RecordNotInserted) => Ok(TryInsertResult::Conflicted),
            Err(err) => Err(err),
        }
    }

    /// Execute an insert operation and return primary keys of inserted models
    ///
    /// # Panics
    ///
    /// Panics if the database backend does not support `INSERT RETURNING`.
    pub async fn exec_with_returning_keys<'a, C>(
        self,
        db: &'a C,
    ) -> Result<TryInsertResult<Vec<<PrimaryKey<A> as PrimaryKeyTrait>::ValueType>>, DbErr>
    where
        <A::Entity as EntityTrait>::Model: IntoActiveModel<A>,
        C: ConnectionTrait,
        A: 'a,
    {
        if self.insert_struct.columns.is_empty() {
            return Ok(TryInsertResult::Empty);
        }

        let res = self.insert_struct.exec_with_returning_keys(db).await;
        match res {
            Ok(res) => Ok(TryInsertResult::Inserted(res)),
            Err(DbErr::RecordNotInserted) => Ok(TryInsertResult::Conflicted),
            Err(err) => Err(err),
        }
    }

    /// Execute an insert operation and return all inserted models
    ///
    /// # Panics
    ///
    /// Panics if the database backend does not support `INSERT RETURNING`.
    pub async fn exec_with_returning_many<'a, C>(
        self,
        db: &'a C,
    ) -> Result<TryInsertResult<Vec<<A::Entity as EntityTrait>::Model>>, DbErr>
    where
        <A::Entity as EntityTrait>::Model: IntoActiveModel<A>,
        C: ConnectionTrait,
        A: 'a,
    {
        if self.insert_struct.columns.is_empty() {
            return Ok(TryInsertResult::Empty);
        }

        let res = self.insert_struct.exec_with_returning_many(db).await;
        match res {
            Ok(res) => Ok(TryInsertResult::Inserted(res)),
            Err(DbErr::RecordNotInserted) => Ok(TryInsertResult::Conflicted),
            Err(err) => Err(err),
        }
    }
}

impl<A> Insert<A>
where
    A: ActiveModelTrait,
{
    /// Execute an insert operation
    #[allow(unused_mut)]
    pub fn exec<'a, C>(self, db: &'a C) -> impl Future<Output = Result<InsertResult<A>, DbErr>> + 'a
    where
        C: ConnectionTrait,
        A: 'a,
    {
        // so that self is dropped before entering await
        let mut query = self.query;
        if db.support_returning() {
            let db_backend = db.get_database_backend();
            let returning =
                Query::returning().exprs(<A::Entity as EntityTrait>::PrimaryKey::iter().map(|c| {
                    c.into_column()
                        .select_as(c.into_column().into_returning_expr(db_backend))
                }));
            query.returning(returning);
        }
        Inserter::<A>::new(self.primary_key, query).exec(db)
    }

    /// Execute an insert operation without returning (don't use `RETURNING` syntax)
    /// Number of rows affected is returned
    pub fn exec_without_returning<'a, C>(
        self,
        db: &'a C,
    ) -> impl Future<Output = Result<u64, DbErr>> + 'a
    where
        <A::Entity as EntityTrait>::Model: IntoActiveModel<A>,
        C: ConnectionTrait,
        A: 'a,
    {
        Inserter::<A>::new(self.primary_key, self.query).exec_without_returning(db)
    }

    /// Execute an insert operation and return the inserted model (use `RETURNING` syntax if supported)
    ///
    /// + To get back all inserted models, use [`exec_with_returning_many`].
    /// + To get back all inserted primary keys, use [`exec_with_returning_keys`].
    pub fn exec_with_returning<'a, C>(
        self,
        db: &'a C,
    ) -> impl Future<Output = Result<<A::Entity as EntityTrait>::Model, DbErr>> + 'a
    where
        <A::Entity as EntityTrait>::Model: IntoActiveModel<A>,
        C: ConnectionTrait,
        A: 'a,
    {
        Inserter::<A>::new(self.primary_key, self.query).exec_with_returning(db)
    }

    /// Execute an insert operation and return primary keys of inserted models
    ///
    /// # Panics
    ///
    /// Panics if the database backend does not support `INSERT RETURNING`.
    pub fn exec_with_returning_keys<'a, C>(
        self,
        db: &'a C,
    ) -> impl Future<Output = Result<Vec<<PrimaryKey<A> as PrimaryKeyTrait>::ValueType>, DbErr>> + 'a
    where
        <A::Entity as EntityTrait>::Model: IntoActiveModel<A>,
        C: ConnectionTrait,
        A: 'a,
    {
        Inserter::<A>::new(self.primary_key, self.query).exec_with_returning_keys(db)
    }

    /// Execute an insert operation and return all inserted models
    ///
    /// # Panics
    ///
    /// Panics if the database backend does not support `INSERT RETURNING`.
    pub fn exec_with_returning_many<'a, C>(
        self,
        db: &'a C,
    ) -> impl Future<Output = Result<Vec<<A::Entity as EntityTrait>::Model>, DbErr>> + 'a
    where
        <A::Entity as EntityTrait>::Model: IntoActiveModel<A>,
        C: ConnectionTrait,
        A: 'a,
    {
        Inserter::<A>::new(self.primary_key, self.query).exec_with_returning_many(db)
    }
}

impl<A> Inserter<A>
where
    A: ActiveModelTrait,
{
    /// Instantiate a new insert operation
    pub fn new(primary_key: Option<ValueTuple>, query: InsertStatement) -> Self {
        Self {
            primary_key,
            query,
            model: PhantomData,
        }
    }

    /// Execute an insert operation, returning the last inserted id
    pub fn exec<'a, C>(self, db: &'a C) -> impl Future<Output = Result<InsertResult<A>, DbErr>> + 'a
    where
        C: ConnectionTrait,
        A: 'a,
    {
        exec_insert(self.primary_key, self.query, db)
    }

    /// Execute an insert operation
    pub fn exec_without_returning<'a, C>(
        self,
        db: &'a C,
    ) -> impl Future<Output = Result<u64, DbErr>> + 'a
    where
        C: ConnectionTrait,
        A: 'a,
    {
        exec_insert_without_returning(self.query, db)
    }

    /// Execute an insert operation and return the inserted model (use `RETURNING` syntax if supported)
    pub fn exec_with_returning<'a, C>(
        self,
        db: &'a C,
    ) -> impl Future<Output = Result<<A::Entity as EntityTrait>::Model, DbErr>> + 'a
    where
        <A::Entity as EntityTrait>::Model: IntoActiveModel<A>,
        C: ConnectionTrait,
        A: 'a,
    {
        exec_insert_with_returning::<A, _>(self.primary_key, self.query, db)
    }

    /// Execute an insert operation and return primary keys of inserted models
    ///
    /// # Panics
    ///
    /// Panics if the database backend does not support `INSERT RETURNING`.
    pub fn exec_with_returning_keys<'a, C>(
        self,
        db: &'a C,
    ) -> impl Future<Output = Result<Vec<<PrimaryKey<A> as PrimaryKeyTrait>::ValueType>, DbErr>> + 'a
    where
        <A::Entity as EntityTrait>::Model: IntoActiveModel<A>,
        C: ConnectionTrait,
        A: 'a,
    {
        exec_insert_with_returning_keys::<A, _>(self.query, db)
    }

    /// Execute an insert operation and return all inserted models
    ///
    /// # Panics
    ///
    /// Panics if the database backend does not support `INSERT RETURNING`.
    pub fn exec_with_returning_many<'a, C>(
        self,
        db: &'a C,
    ) -> impl Future<Output = Result<Vec<<A::Entity as EntityTrait>::Model>, DbErr>> + 'a
    where
        <A::Entity as EntityTrait>::Model: IntoActiveModel<A>,
        C: ConnectionTrait,
        A: 'a,
    {
        exec_insert_with_returning_many::<A, _>(self.query, db)
    }
}

async fn exec_insert<A, C>(
    primary_key: Option<ValueTuple>,
    statement: InsertStatement,
    db: &C,
) -> Result<InsertResult<A>, DbErr>
where
    C: ConnectionTrait,
    A: ActiveModelTrait,
{
    type ValueTypeOf<A> = <PrimaryKey<A> as PrimaryKeyTrait>::ValueType;

    let db_backend = db.get_database_backend();
    let statement = db_backend.build(&statement);

    let last_insert_id = match (primary_key, db.support_returning()) {
        (Some(value_tuple), _) => {
            let res = db.execute(statement).await?;
            if res.rows_affected() == 0 {
                return Err(DbErr::RecordNotInserted);
            }
            FromValueTuple::from_value_tuple(value_tuple)
        }
        (None, true) => {
            let mut rows = db.query_all(statement).await?;
            let row = match rows.pop() {
                Some(row) => row,
                None => return Err(DbErr::RecordNotInserted),
            };
            let cols = PrimaryKey::<A>::iter()
                .map(|col| col.to_string())
                .collect::<Vec<_>>();
            row.try_get_many("", cols.as_ref())
                .map_err(|_| DbErr::UnpackInsertId)?
        }
        (None, false) => {
            let res = db.execute(statement).await?;
            if res.rows_affected() == 0 {
                return Err(DbErr::RecordNotInserted);
            }
            let last_insert_id = res.last_insert_id();
            // For MySQL, the affected-rows number:
            //   - The affected-rows value per row is `1` if the row is inserted as a new row,
            //   - `2` if an existing row is updated,
            //   - and `0` if an existing row is set to its current values.
            // Reference: https://dev.mysql.com/doc/refman/8.4/en/insert-on-duplicate.html
            if db_backend == DbBackend::MySql && last_insert_id == 0 {
                return Err(DbErr::RecordNotInserted);
            }
            ValueTypeOf::<A>::try_from_u64(last_insert_id).map_err(|_| DbErr::UnpackInsertId)?
        }
    };

    Ok(InsertResult { last_insert_id })
}

async fn exec_insert_without_returning<C>(
    insert_statement: InsertStatement,
    db: &C,
) -> Result<u64, DbErr>
where
    C: ConnectionTrait,
{
    let db_backend = db.get_database_backend();
    let insert_statement = db_backend.build(&insert_statement);
    let exec_result = db.execute(insert_statement).await?;
    Ok(exec_result.rows_affected())
}

async fn exec_insert_with_returning<A, C>(
    primary_key: Option<ValueTuple>,
    mut insert_statement: InsertStatement,
    db: &C,
) -> Result<<A::Entity as EntityTrait>::Model, DbErr>
where
    <A::Entity as EntityTrait>::Model: IntoActiveModel<A>,
    C: ConnectionTrait,
    A: ActiveModelTrait,
{
    let db_backend = db.get_database_backend();
    let found = match db.support_returning() {
        true => {
            let returning = Query::returning().exprs(
                <A::Entity as EntityTrait>::Column::iter()
                    .map(|c| c.select_as(c.into_returning_expr(db_backend))),
            );
            insert_statement.returning(returning);
            let insert_statement = db_backend.build(&insert_statement);
            SelectorRaw::<SelectModel<<A::Entity as EntityTrait>::Model>>::from_statement(
                insert_statement,
            )
            .one(db)
            .await?
        }
        false => {
            let insert_res = exec_insert::<A, _>(primary_key, insert_statement, db).await?;
            <A::Entity as EntityTrait>::find_by_id(insert_res.last_insert_id)
                .one(db)
                .await?
        }
    };
    match found {
        Some(model) => Ok(model),
        None => Err(DbErr::RecordNotFound(
            "Failed to find inserted item".to_owned(),
        )),
    }
}

async fn exec_insert_with_returning_keys<A, C>(
    mut insert_statement: InsertStatement,
    db: &C,
) -> Result<Vec<<PrimaryKey<A> as PrimaryKeyTrait>::ValueType>, DbErr>
where
    <A::Entity as EntityTrait>::Model: IntoActiveModel<A>,
    C: ConnectionTrait,
    A: ActiveModelTrait,
{
    let db_backend = db.get_database_backend();
    match db.support_returning() {
        true => {
            let returning =
                Query::returning().exprs(<A::Entity as EntityTrait>::PrimaryKey::iter().map(|c| {
                    c.into_column()
                        .select_as(c.into_column().into_returning_expr(db_backend))
                }));
            insert_statement.returning(returning);
            let statement = db_backend.build(&insert_statement);
            let rows = db.query_all(statement).await?;
            let cols = PrimaryKey::<A>::iter()
                .map(|col| col.to_string())
                .collect::<Vec<_>>();
            let mut keys = Vec::new();
            for row in rows {
                keys.push(
                    row.try_get_many("", cols.as_ref())
                        .map_err(|_| DbErr::UnpackInsertId)?,
                );
            }
            Ok(keys)
        }
        false => unimplemented!("Database backend doesn't support RETURNING"),
    }
}

async fn exec_insert_with_returning_many<A, C>(
    mut insert_statement: InsertStatement,
    db: &C,
) -> Result<Vec<<A::Entity as EntityTrait>::Model>, DbErr>
where
    <A::Entity as EntityTrait>::Model: IntoActiveModel<A>,
    C: ConnectionTrait,
    A: ActiveModelTrait,
{
    let db_backend = db.get_database_backend();
    match db.support_returning() {
        true => {
            let returning = Query::returning().exprs(
                <A::Entity as EntityTrait>::Column::iter()
                    .map(|c| c.select_as(c.into_returning_expr(db_backend))),
            );
            insert_statement.returning(returning);
            let insert_statement = db_backend.build(&insert_statement);
            SelectorRaw::<SelectModel<<A::Entity as EntityTrait>::Model>>::from_statement(
                insert_statement,
            )
            .all(db)
            .await
        }
        false => unimplemented!("Database backend doesn't support RETURNING"),
    }
}

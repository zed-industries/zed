use anyhow::{Context, Result};

use crate::{
    bindable::{Bind, Column},
    connection::Connection,
    statement::Statement,
};

impl Connection {
    /// Prepare a statement which has no bindings and returns nothing.
    ///
    /// Note: If there are multiple statements that depend upon each other
    /// (such as those which make schema changes), preparation will fail.
    /// Use a true migration instead.
    pub fn exec<'a>(&'a self, query: &str) -> Result<impl 'a + FnMut() -> Result<()>> {
        let mut statement = Statement::prepare(self, query)?;
        Ok(move || statement.exec())
    }

    /// Prepare a statement which takes a binding, but returns nothing.
    /// The bindings for a given invocation should be passed to the returned
    /// closure
    ///
    /// Note: If there are multiple statements that depend upon each other
    /// (such as those which make schema changes), preparation will fail.
    /// Use a true migration instead.
    pub fn exec_bound<'a, B: Bind>(
        &'a self,
        query: &str,
    ) -> Result<impl 'a + FnMut(B) -> Result<()>> {
        let mut statement = Statement::prepare(self, query)?;
        Ok(move |bindings| statement.with_bindings(&bindings)?.exec())
    }

    /// Prepare a statement which has no bindings and returns a `Vec<C>`.
    ///
    /// Note: If there are multiple statements that depend upon each other
    /// (such as those which make schema changes), preparation will fail.
    /// Use a true migration instead.
    pub fn select<'a, C: Column>(
        &'a self,
        query: &str,
    ) -> Result<impl 'a + FnMut() -> Result<Vec<C>>> {
        let mut statement = Statement::prepare(self, query)?;
        Ok(move || statement.rows::<C>())
    }

    /// Prepare a statement which takes a binding and returns a `Vec<C>`.
    ///
    /// Note: If there are multiple statements that depend upon each other
    /// (such as those which make schema changes), preparation will fail.
    /// Use a true migration instead.
    pub fn select_bound<'a, B: Bind, C: Column>(
        &'a self,
        query: &str,
    ) -> Result<impl 'a + FnMut(B) -> Result<Vec<C>>> {
        let mut statement = Statement::prepare(self, query)?;
        Ok(move |bindings| statement.with_bindings(&bindings)?.rows::<C>())
    }

    /// Prepare a statement that selects a single row from the database.
    /// Will return none if no rows are returned and will error if more than
    /// 1 row
    ///
    /// Note: If there are multiple statements that depend upon each other
    /// (such as those which make schema changes), preparation will fail.
    /// Use a true migration instead.
    pub fn select_row<'a, C: Column>(
        &'a self,
        query: &str,
    ) -> Result<impl 'a + FnMut() -> Result<Option<C>>> {
        let mut statement = Statement::prepare(self, query)?;
        Ok(move || statement.maybe_row::<C>())
    }

    /// Prepare a statement which takes a binding and selects a single row
    /// from the database. Will return none if no rows are returned and will
    /// error if more than 1 row is returned.
    ///
    /// Note: If there are multiple statements that depend upon each other
    /// (such as those which make schema changes), preparation will fail.
    /// Use a true migration instead.
    pub fn select_row_bound<'a, B: Bind, C: Column>(
        &'a self,
        query: &str,
    ) -> Result<impl 'a + FnMut(B) -> Result<Option<C>>> {
        let mut statement = Statement::prepare(self, query)?;
        Ok(move |bindings| {
            statement
                .with_bindings(&bindings)
                .context("Bindings failed")?
                .maybe_row::<C>()
                .context("Maybe row failed")
        })
    }
}

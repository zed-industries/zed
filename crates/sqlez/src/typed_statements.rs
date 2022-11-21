use anyhow::Result;

use crate::{
    bindable::{Bind, Column},
    connection::Connection,
    statement::Statement,
};

impl Connection {
    pub fn exec<'a>(&'a self, query: &str) -> Result<impl 'a + FnMut() -> Result<()>> {
        let mut statement = Statement::prepare(&self, query)?;
        Ok(move || statement.exec())
    }

    pub fn exec_bound<'a, B: Bind>(
        &'a self,
        query: &str,
    ) -> Result<impl 'a + FnMut(B) -> Result<()>> {
        let mut statement = Statement::prepare(&self, query)?;
        Ok(move |bindings| statement.with_bindings(bindings)?.exec())
    }

    pub fn select<'a, C: Column>(
        &'a self,
        query: &str,
    ) -> Result<impl 'a + FnMut() -> Result<Vec<C>>> {
        let mut statement = Statement::prepare(&self, query)?;
        Ok(move || statement.rows::<C>())
    }

    pub fn select_bound<'a, B: Bind, C: Column>(
        &'a self,
        query: &str,
    ) -> Result<impl 'a + FnMut(B) -> Result<Vec<C>>> {
        let mut statement = Statement::prepare(&self, query)?;
        Ok(move |bindings| statement.with_bindings(bindings)?.rows::<C>())
    }

    pub fn select_row<'a, C: Column>(
        &'a self,
        query: &str,
    ) -> Result<impl 'a + FnMut() -> Result<Option<C>>> {
        let mut statement = Statement::prepare(&self, query)?;
        Ok(move || statement.maybe_row::<C>())
    }

    pub fn select_row_bound<'a, B: Bind, C: Column>(
        &'a self,
        query: &str,
    ) -> Result<impl 'a + FnMut(B) -> Result<Option<C>>> {
        let mut statement = Statement::prepare(&self, query)?;
        Ok(move |bindings| statement.with_bindings(bindings)?.maybe_row::<C>())
    }
}

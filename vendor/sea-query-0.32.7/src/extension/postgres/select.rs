use crate::SelectStatement;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TableSample {
    pub method: SampleMethod,
    pub percentage: f64,
    pub repeatable: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SampleMethod {
    BERNOULLI,
    SYSTEM,
}

pub trait PostgresSelectStatementExt {
    fn table_sample(
        &mut self,
        method: SampleMethod,
        percentage: f64,
        repeatable: Option<f64>,
    ) -> &mut Self;
}

impl PostgresSelectStatementExt for SelectStatement {
    /// TABLESAMPLE
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{extension::postgres::*, tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .columns([Glyph::Image])
    ///     .from(Glyph::Table)
    ///     .table_sample(SampleMethod::SYSTEM, 50.0, None)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "image" FROM "glyph" TABLESAMPLE SYSTEM (50)"#
    /// );
    ///
    /// let query = Query::select()
    ///     .columns([Glyph::Image])
    ///     .from(Glyph::Table)
    ///     .table_sample(SampleMethod::SYSTEM, 50.0, Some(3.14))
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "image" FROM "glyph" TABLESAMPLE SYSTEM (50) REPEATABLE (3.14)"#
    /// );
    /// ```
    fn table_sample(
        &mut self,
        method: SampleMethod,
        percentage: f64,
        repeatable: Option<f64>,
    ) -> &mut Self {
        self.table_sample = Some(TableSample {
            method,
            percentage,
            repeatable,
        });
        self
    }
}

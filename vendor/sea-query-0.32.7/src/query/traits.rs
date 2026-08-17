use std::fmt::Debug;

use crate::{backend::QueryBuilder, value::Values, SqlWriter, SqlWriterValues, SubQueryStatement};

pub trait QueryStatementBuilder: Debug {
    /// Build corresponding SQL statement for certain database backend and collect query parameters into a vector
    fn build_any(&self, query_builder: &dyn QueryBuilder) -> (String, Values) {
        let (placeholder, numbered) = query_builder.placeholder();
        let mut sql = SqlWriterValues::new(placeholder, numbered);
        self.build_collect_any_into(query_builder, &mut sql);
        sql.into_parts()
    }

    /// Build corresponding SQL statement for certain database backend and collect query parameters
    fn build_collect_any(
        &self,
        query_builder: &dyn QueryBuilder,
        sql: &mut dyn SqlWriter,
    ) -> String {
        self.build_collect_any_into(query_builder, sql);
        sql.to_string()
    }

    /// Build corresponding SQL statement into the SqlWriter for certain database backend and collect query parameters
    fn build_collect_any_into(&self, query_builder: &dyn QueryBuilder, sql: &mut dyn SqlWriter);

    fn into_sub_query_statement(self) -> SubQueryStatement;
}

pub trait QueryStatementWriter: QueryStatementBuilder {
    /// Build corresponding SQL statement for certain database backend and return SQL string
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .column(Glyph::Aspect)
    ///     .from(Glyph::Table)
    ///     .and_where(Expr::col(Glyph::Aspect).if_null(0).gt(2))
    ///     .order_by(Glyph::Image, Order::Desc)
    ///     .order_by((Glyph::Table, Glyph::Aspect), Order::Asc)
    ///     .to_string(MysqlQueryBuilder);
    ///
    /// assert_eq!(
    ///     query,
    ///     r#"SELECT `aspect` FROM `glyph` WHERE IFNULL(`aspect`, 0) > 2 ORDER BY `image` DESC, `glyph`.`aspect` ASC"#
    /// );
    /// ```
    fn to_string<T: QueryBuilder>(&self, query_builder: T) -> String {
        let mut sql = String::with_capacity(256);
        self.build_collect_any_into(&query_builder, &mut sql);
        sql
    }

    /// Build corresponding SQL statement for certain database backend and collect query parameters into a vector
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let (query, params) = Query::select()
    ///     .column(Glyph::Aspect)
    ///     .from(Glyph::Table)
    ///     .and_where(Expr::col(Glyph::Aspect).if_null(0).gt(2))
    ///     .order_by(Glyph::Image, Order::Desc)
    ///     .order_by((Glyph::Table, Glyph::Aspect), Order::Asc)
    ///     .build(MysqlQueryBuilder);
    ///
    /// assert_eq!(
    ///     query,
    ///     r#"SELECT `aspect` FROM `glyph` WHERE IFNULL(`aspect`, ?) > ? ORDER BY `image` DESC, `glyph`.`aspect` ASC"#
    /// );
    /// assert_eq!(
    ///     params,
    ///     Values(vec![Value::Int(Some(0)), Value::Int(Some(2))])
    /// );
    /// ```
    fn build<T: QueryBuilder>(&self, query_builder: T) -> (String, Values) {
        let (placeholder, numbered) = query_builder.placeholder();
        let mut sql = SqlWriterValues::new(placeholder, numbered);
        self.build_collect_into(query_builder, &mut sql);
        sql.into_parts()
    }

    /// Build corresponding SQL statement for certain database backend and collect query parameters
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .column(Glyph::Aspect)
    ///     .from(Glyph::Table)
    ///     .and_where(Expr::col(Glyph::Aspect).if_null(0).gt(2))
    ///     .order_by(Glyph::Image, Order::Desc)
    ///     .order_by((Glyph::Table, Glyph::Aspect), Order::Asc)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `aspect` FROM `glyph` WHERE IFNULL(`aspect`, 0) > 2 ORDER BY `image` DESC, `glyph`.`aspect` ASC"#
    /// );
    ///
    /// let (placeholder, numbered) = MysqlQueryBuilder.placeholder();
    /// let mut sql = SqlWriterValues::new(placeholder, numbered);
    ///
    /// assert_eq!(
    ///     query.build_collect(MysqlQueryBuilder, &mut sql),
    ///     r#"SELECT `aspect` FROM `glyph` WHERE IFNULL(`aspect`, ?) > ? ORDER BY `image` DESC, `glyph`.`aspect` ASC"#
    /// );
    ///
    /// let (sql, values) = sql.into_parts();
    /// assert_eq!(
    ///     values,
    ///     Values(vec![Value::Int(Some(0)), Value::Int(Some(2))])
    /// );
    /// ```
    fn build_collect<T: QueryBuilder>(&self, query_builder: T, sql: &mut dyn SqlWriter) -> String {
        self.build_collect_into(query_builder, sql);
        sql.to_string()
    }

    fn build_collect_into<T: QueryBuilder>(&self, query_builder: T, sql: &mut dyn SqlWriter);
}

use crate::{Condition, IntoCondition, SimpleExpr};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CaseStatementCondition {
    pub(crate) condition: Condition,
    pub(crate) result: SimpleExpr,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CaseStatement {
    pub(crate) when: Vec<CaseStatementCondition>,
    pub(crate) r#else: Option<SimpleExpr>,
}

impl CaseStatement {
    /// Creates a new case statement expression
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .expr_as(
    ///         CaseStatement::new()
    ///             .case(Expr::col((Glyph::Table, Glyph::Aspect)).is_in([2, 4]), true)
    ///             .finally(false),
    ///          "is_even"
    ///     )
    ///     .from(Glyph::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT (CASE WHEN ("glyph"."aspect" IN (2, 4)) THEN TRUE ELSE FALSE END) AS "is_even" FROM "glyph""#
    /// );
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds new `CASE WHEN` to existing case statement.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .expr_as(
    ///             Expr::case(
    ///                 Expr::col((Glyph::Table, Glyph::Aspect)).gt(0),
    ///                 "positive"
    ///              )
    ///             .case(
    ///                 Expr::col((Glyph::Table, Glyph::Aspect)).lt(0),
    ///                 "negative"
    ///              )
    ///             .finally("zero"),
    ///          "polarity"
    ///     )
    ///     .from(Glyph::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT (CASE WHEN ("glyph"."aspect" > 0) THEN 'positive' WHEN ("glyph"."aspect" < 0) THEN 'negative' ELSE 'zero' END) AS "polarity" FROM "glyph""#
    /// );
    /// ```
    pub fn case<C, T>(mut self, cond: C, then: T) -> Self
    where
        C: IntoCondition,
        T: Into<SimpleExpr>,
    {
        self.when.push(CaseStatementCondition {
            condition: cond.into_condition(),
            result: then.into(),
        });
        self
    }

    /// Ends the case statement with the final `ELSE` result.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_query::{*, tests_cfg::*};
    ///
    /// let query = Query::select()
    ///     .expr_as(
    ///         Expr::case(
    ///             Cond::any()
    ///                 .add(Expr::col((Character::Table, Character::FontSize)).gt(48))
    ///                 .add(Expr::col((Character::Table, Character::SizeW)).gt(500)),
    ///             "large"
    ///         )
    ///         .case(
    ///             Cond::any()
    ///                 .add(Expr::col((Character::Table, Character::FontSize)).between(24,48))
    ///                 .add(Expr::col((Character::Table, Character::SizeW)).between(300,500)),
    ///             "medium"
    ///         )
    ///         .finally("small"),
    ///         "char_size")
    ///     .from(Character::Table)
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     [
    ///         r#"SELECT"#,
    ///         r#"(CASE WHEN ("character"."font_size" > 48 OR "character"."size_w" > 500) THEN 'large'"#,
    ///         r#"WHEN (("character"."font_size" BETWEEN 24 AND 48) OR ("character"."size_w" BETWEEN 300 AND 500)) THEN 'medium'"#,
    ///         r#"ELSE 'small' END) AS "char_size""#,
    ///         r#"FROM "character""#
    ///     ]
    ///     .join(" ")
    /// );
    /// ```
    pub fn finally<E>(mut self, r#else: E) -> Self
    where
        E: Into<SimpleExpr>,
    {
        self.r#else = Some(r#else.into());
        self
    }
}

#[allow(clippy::from_over_into)]
impl Into<SimpleExpr> for CaseStatement {
    fn into(self) -> SimpleExpr {
        SimpleExpr::Case(Box::new(self))
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    #[cfg(feature = "backend-postgres")]
    fn test_where_case_eq() {
        let case_statement: SimpleExpr = Expr::case(Expr::col("col").lt(5), Expr::col("othercol"))
            .finally(Expr::col("finalcol"))
            .into();

        let result = Query::select()
            .column(Asterisk)
            .from("tbl")
            .and_where(case_statement.eq(10))
            .to_string(PostgresQueryBuilder);
        assert_eq!(
            result,
            r#"SELECT * FROM "tbl" WHERE (CASE WHEN ("col" < 5) THEN "othercol" ELSE "finalcol" END) = 10"#
        );
    }
}

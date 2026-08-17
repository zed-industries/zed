use crate::{expr::*, query::*, types::*};
use inherent::inherent;

pub trait OverStatement {
    #[doc(hidden)]
    // Implementation for the trait.
    fn add_partition_by(&mut self, partition: SimpleExpr) -> &mut Self;

    /// Partition by column.
    fn partition_by<T>(&mut self, col: T) -> &mut Self
    where
        T: IntoColumnRef,
    {
        self.add_partition_by(SimpleExpr::Column(col.into_column_ref()))
    }

    /// Partition by custom string.
    fn partition_by_customs<I, T>(&mut self, cols: I) -> &mut Self
    where
        T: ToString,
        I: IntoIterator<Item = T>,
    {
        cols.into_iter().for_each(|c| {
            self.add_partition_by(SimpleExpr::Custom(c.to_string()));
        });
        self
    }

    /// Partition by vector of columns.
    fn partition_by_columns<I, T>(&mut self, cols: I) -> &mut Self
    where
        T: IntoColumnRef,
        I: IntoIterator<Item = T>,
    {
        cols.into_iter().for_each(|c| {
            self.add_partition_by(SimpleExpr::Column(c.into_column_ref()));
        });
        self
    }
}

/// frame_start or frame_end clause
#[derive(Debug, Clone, PartialEq)]
pub enum Frame {
    UnboundedPreceding,
    Preceding(u32),
    CurrentRow,
    Following(u32),
    UnboundedFollowing,
}

/// Frame type
#[derive(Debug, Clone, PartialEq)]
pub enum FrameType {
    Range,
    Rows,
}

/// Frame clause
#[derive(Debug, Clone, PartialEq)]
pub struct FrameClause {
    pub(crate) r#type: FrameType,
    pub(crate) start: Frame,
    pub(crate) end: Option<Frame>,
}

/// Window expression
///
/// # References:
///
/// 1. <https://dev.mysql.com/doc/refman/8.0/en/window-function-descriptions.html>
/// 2. <https://www.sqlite.org/windowfunctions.html>
/// 3. <https://www.postgresql.org/docs/current/tutorial-window.html>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct WindowStatement {
    pub(crate) partition_by: Vec<SimpleExpr>,
    pub(crate) order_by: Vec<OrderExpr>,
    pub(crate) frame: Option<FrameClause>,
}

impl WindowStatement {
    /// Construct a new [`WindowStatement`]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn take(&mut self) -> Self {
        Self {
            partition_by: std::mem::take(&mut self.partition_by),
            order_by: std::mem::take(&mut self.order_by),
            frame: self.frame.take(),
        }
    }

    /// Construct a new [`WindowStatement`] with PARTITION BY column
    pub fn partition_by<T>(col: T) -> Self
    where
        T: IntoColumnRef,
    {
        let mut window = Self::new();
        window.add_partition_by(SimpleExpr::Column(col.into_column_ref()));
        window
    }

    /// Construct a new [`WindowStatement`] with PARTITION BY custom
    pub fn partition_by_custom<T>(col: T) -> Self
    where
        T: ToString,
    {
        let mut window = Self::new();
        window.add_partition_by(SimpleExpr::Custom(col.to_string()));
        window
    }

    /// frame clause for frame_start
    /// # Examples:
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .from(Char::Table)
    ///     .expr_window_as(
    ///         Expr::col(Char::Character),
    ///         WindowStatement::partition_by(Char::FontSize)
    ///             .frame_start(FrameType::Rows, Frame::UnboundedPreceding)
    ///             .take(),
    ///         "C")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character` OVER ( PARTITION BY `font_size` ROWS UNBOUNDED PRECEDING ) AS `C` FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character" OVER ( PARTITION BY "font_size" ROWS UNBOUNDED PRECEDING ) AS "C" FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character" OVER ( PARTITION BY "font_size" ROWS UNBOUNDED PRECEDING ) AS "C" FROM "character""#
    /// );
    /// ```
    pub fn frame_start(&mut self, r#type: FrameType, start: Frame) -> &mut Self {
        self.frame(r#type, start, None)
    }

    /// frame clause for BETWEEN frame_start AND frame_end
    ///
    /// # Examples:
    ///
    /// ```
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .from(Char::Table)
    ///     .expr_window_as(
    ///         Expr::col(Char::Character),
    ///         WindowStatement::partition_by(Char::FontSize)
    ///             .frame_between(FrameType::Rows, Frame::UnboundedPreceding, Frame::UnboundedFollowing)
    ///             .take(),
    ///         "C")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(MysqlQueryBuilder),
    ///     r#"SELECT `character` OVER ( PARTITION BY `font_size` ROWS BETWEEN UNBOUNDED PRECEDING AND UNBOUNDED FOLLOWING ) AS `C` FROM `character`"#
    /// );
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "character" OVER ( PARTITION BY "font_size" ROWS BETWEEN UNBOUNDED PRECEDING AND UNBOUNDED FOLLOWING ) AS "C" FROM "character""#
    /// );
    /// assert_eq!(
    ///     query.to_string(SqliteQueryBuilder),
    ///     r#"SELECT "character" OVER ( PARTITION BY "font_size" ROWS BETWEEN UNBOUNDED PRECEDING AND UNBOUNDED FOLLOWING ) AS "C" FROM "character""#
    /// );
    /// ```
    pub fn frame_between(&mut self, r#type: FrameType, start: Frame, end: Frame) -> &mut Self {
        self.frame(r#type, start, Some(end))
    }

    /// frame clause
    pub fn frame(&mut self, r#type: FrameType, start: Frame, end: Option<Frame>) -> &mut Self {
        let frame_clause = FrameClause { r#type, start, end };
        self.frame = Some(frame_clause);
        self
    }
}

impl OverStatement for WindowStatement {
    fn add_partition_by(&mut self, partition: SimpleExpr) -> &mut Self {
        self.partition_by.push(partition);
        self
    }
}

#[inherent]
impl OrderedStatement for WindowStatement {
    pub fn add_order_by(&mut self, order: OrderExpr) -> &mut Self {
        self.order_by.push(order);
        self
    }

    pub fn clear_order_by(&mut self) -> &mut Self {
        self.order_by = Vec::new();
        self
    }

    pub fn order_by<T>(&mut self, col: T, order: Order) -> &mut Self
    where
        T: IntoColumnRef;

    pub fn order_by_expr(&mut self, expr: SimpleExpr, order: Order) -> &mut Self;
    pub fn order_by_customs<I, T>(&mut self, cols: I) -> &mut Self
    where
        T: ToString,
        I: IntoIterator<Item = (T, Order)>;
    pub fn order_by_columns<I, T>(&mut self, cols: I) -> &mut Self
    where
        T: IntoColumnRef,
        I: IntoIterator<Item = (T, Order)>;
    pub fn order_by_with_nulls<T>(
        &mut self,
        col: T,
        order: Order,
        nulls: NullOrdering,
    ) -> &mut Self
    where
        T: IntoColumnRef;
    pub fn order_by_expr_with_nulls(
        &mut self,
        expr: SimpleExpr,
        order: Order,
        nulls: NullOrdering,
    ) -> &mut Self;
    pub fn order_by_customs_with_nulls<I, T>(&mut self, cols: I) -> &mut Self
    where
        T: ToString,
        I: IntoIterator<Item = (T, Order, NullOrdering)>;
    pub fn order_by_columns_with_nulls<I, T>(&mut self, cols: I) -> &mut Self
    where
        T: IntoColumnRef,
        I: IntoIterator<Item = (T, Order, NullOrdering)>;
}

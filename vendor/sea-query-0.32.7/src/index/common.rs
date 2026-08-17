use crate::expr::SimpleExpr;
use crate::{types::*, FunctionCall};

/// Specification of a table index
#[derive(Default, Debug, Clone)]
pub struct TableIndex {
    pub(crate) name: Option<String>,
    pub(crate) columns: Vec<IndexColumn>,
}

#[derive(Debug, Clone)]
pub enum IndexColumn {
    TableColumn(IndexColumnTableColumn),
    Expr(IndexColumnExpr),
}

#[derive(Debug, Clone)]
pub struct IndexColumnTableColumn {
    pub(crate) name: DynIden,
    pub(crate) prefix: Option<u32>,
    pub(crate) order: Option<IndexOrder>,
}

#[derive(Debug, Clone)]
pub struct IndexColumnExpr {
    pub(crate) expr: SimpleExpr,
    pub(crate) order: Option<IndexOrder>,
}

impl IndexColumn {
    pub(crate) fn name(&self) -> Option<&DynIden> {
        match self {
            IndexColumn::TableColumn(IndexColumnTableColumn { name, .. }) => Some(name),
            IndexColumn::Expr(_) => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum IndexOrder {
    Asc,
    Desc,
}

pub trait IntoIndexColumn {
    fn into_index_column(self) -> IndexColumn;
}

impl IntoIndexColumn for IndexColumn {
    fn into_index_column(self) -> IndexColumn {
        self
    }
}

impl<I> IntoIndexColumn for I
where
    I: IntoIden,
{
    fn into_index_column(self) -> IndexColumn {
        IndexColumn::TableColumn(IndexColumnTableColumn {
            name: self.into_iden(),
            prefix: None,
            order: None,
        })
    }
}

impl<I> IntoIndexColumn for (I, u32)
where
    I: IntoIden,
{
    fn into_index_column(self) -> IndexColumn {
        IndexColumn::TableColumn(IndexColumnTableColumn {
            name: self.0.into_iden(),
            prefix: Some(self.1),
            order: None,
        })
    }
}

impl<I> IntoIndexColumn for (I, IndexOrder)
where
    I: IntoIden,
{
    fn into_index_column(self) -> IndexColumn {
        IndexColumn::TableColumn(IndexColumnTableColumn {
            name: self.0.into_iden(),
            prefix: None,
            order: Some(self.1),
        })
    }
}

impl<I> IntoIndexColumn for (I, u32, IndexOrder)
where
    I: IntoIden,
{
    fn into_index_column(self) -> IndexColumn {
        IndexColumn::TableColumn(IndexColumnTableColumn {
            name: self.0.into_iden(),
            prefix: Some(self.1),
            order: Some(self.2),
        })
    }
}

impl IntoIndexColumn for FunctionCall {
    fn into_index_column(self) -> IndexColumn {
        IndexColumn::Expr(IndexColumnExpr {
            expr: self.into(),
            order: None,
        })
    }
}

impl IntoIndexColumn for (FunctionCall, IndexOrder) {
    fn into_index_column(self) -> IndexColumn {
        IndexColumn::Expr(IndexColumnExpr {
            expr: self.0.into(),
            order: Some(self.1),
        })
    }
}

impl IntoIndexColumn for SimpleExpr {
    fn into_index_column(self) -> IndexColumn {
        IndexColumn::Expr(IndexColumnExpr {
            expr: self,
            order: None,
        })
    }
}

impl IntoIndexColumn for (SimpleExpr, IndexOrder) {
    fn into_index_column(self) -> IndexColumn {
        IndexColumn::Expr(IndexColumnExpr {
            expr: self.0,
            order: Some(self.1),
        })
    }
}

impl TableIndex {
    /// Construct a new table index
    pub fn new() -> Self {
        Self::default()
    }

    /// Set index name
    pub fn name<T>(&mut self, name: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.name = Some(name.into());
        self
    }

    /// Set index column
    pub fn col(&mut self, col: IndexColumn) -> &mut Self {
        self.columns.push(col);
        self
    }

    pub fn get_column_names(&self) -> Vec<String> {
        self.columns
            .iter()
            .filter_map(|col| col.name().map(|name| name.to_string()))
            .collect()
    }

    pub fn take(&mut self) -> Self {
        Self {
            name: self.name.take(),
            columns: std::mem::take(&mut self.columns),
        }
    }
}

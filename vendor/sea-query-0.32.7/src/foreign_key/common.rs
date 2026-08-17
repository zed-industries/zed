use crate::types::*;

/// Specification of a foreign key
#[derive(Default, Debug, Clone)]
pub struct TableForeignKey {
    pub(crate) name: Option<String>,
    pub(crate) table: Option<TableRef>,
    pub(crate) ref_table: Option<TableRef>,
    pub(crate) columns: Vec<DynIden>,
    pub(crate) ref_columns: Vec<DynIden>,
    pub(crate) on_delete: Option<ForeignKeyAction>,
    pub(crate) on_update: Option<ForeignKeyAction>,
}

/// Foreign key on update & on delete actions
#[derive(Debug, Clone, Copy)]
pub enum ForeignKeyAction {
    Restrict,
    Cascade,
    SetNull,
    NoAction,
    SetDefault,
}

impl TableForeignKey {
    /// Construct a new foreign key
    pub fn new() -> Self {
        Self::default()
    }

    /// Set foreign key name
    pub fn name<T>(&mut self, name: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.name = Some(name.into());
        self
    }

    /// Set key table
    pub fn from_tbl<T>(&mut self, table: T) -> &mut Self
    where
        T: IntoTableRef,
    {
        self.table = Some(table.into_table_ref());
        self
    }

    /// Set referencing table
    pub fn to_tbl<R>(&mut self, ref_table: R) -> &mut Self
    where
        R: IntoTableRef,
    {
        self.ref_table = Some(ref_table.into_table_ref());
        self
    }

    /// Add key column
    pub fn from_col<T>(&mut self, column: T) -> &mut Self
    where
        T: IntoIden,
    {
        self.columns.push(column.into_iden());
        self
    }

    /// Add referencing column
    pub fn to_col<R>(&mut self, ref_column: R) -> &mut Self
    where
        R: IntoIden,
    {
        self.ref_columns.push(ref_column.into_iden());
        self
    }

    /// Set on delete action
    pub fn on_delete(&mut self, action: ForeignKeyAction) -> &mut Self {
        self.on_delete = Some(action);
        self
    }

    /// Set on update action
    pub fn on_update(&mut self, action: ForeignKeyAction) -> &mut Self {
        self.on_update = Some(action);
        self
    }

    pub fn get_ref_table(&self) -> Option<&TableRef> {
        self.ref_table.as_ref()
    }

    pub fn get_columns(&self) -> Vec<String> {
        self.columns.iter().map(|col| col.to_string()).collect()
    }

    pub fn get_ref_columns(&self) -> Vec<String> {
        self.ref_columns
            .iter()
            .map(|ref_col| ref_col.to_string())
            .collect()
    }

    pub fn get_on_delete(&self) -> Option<ForeignKeyAction> {
        self.on_delete
    }

    pub fn get_on_update(&self) -> Option<ForeignKeyAction> {
        self.on_update
    }

    pub fn take(&mut self) -> Self {
        Self {
            name: self.name.take(),
            table: self.table.take(),
            ref_table: self.ref_table.take(),
            columns: std::mem::take(&mut self.columns),
            ref_columns: std::mem::take(&mut self.ref_columns),
            on_delete: self.on_delete.take(),
            on_update: self.on_update.take(),
        }
    }
}

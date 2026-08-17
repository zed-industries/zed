use crate::{
    ActiveEnum, ColumnTrait, ColumnType, DbBackend, EntityTrait, Iterable, PrimaryKeyArity,
    PrimaryKeyToColumn, PrimaryKeyTrait, RelationTrait, Schema,
};
use sea_query::{
    extension::postgres::{Type, TypeCreateStatement},
    ColumnDef, Iden, Index, IndexCreateStatement, SeaRc, TableCreateStatement,
};

impl Schema {
    /// Creates Postgres enums from an ActiveEnum. See [TypeCreateStatement] for more details
    pub fn create_enum_from_active_enum<A>(&self) -> TypeCreateStatement
    where
        A: ActiveEnum,
    {
        create_enum_from_active_enum::<A>(self.backend)
    }

    /// Creates Postgres enums from an Entity. See [TypeCreateStatement] for more details
    pub fn create_enum_from_entity<E>(&self, entity: E) -> Vec<TypeCreateStatement>
    where
        E: EntityTrait,
    {
        create_enum_from_entity(entity, self.backend)
    }

    /// Creates a table from an Entity. See [TableCreateStatement] for more details.
    pub fn create_table_from_entity<E>(&self, entity: E) -> TableCreateStatement
    where
        E: EntityTrait,
    {
        create_table_from_entity(entity, self.backend)
    }

    /// Creates the indexes from an Entity, returning an empty Vec if there are none
    /// to create. See [IndexCreateStatement] for more details
    pub fn create_index_from_entity<E>(&self, entity: E) -> Vec<IndexCreateStatement>
    where
        E: EntityTrait,
    {
        create_index_from_entity(entity, self.backend)
    }

    /// Creates a column definition for example to update a table.
    ///
    /// ```
    /// use crate::sea_orm::IdenStatic;
    /// use sea_orm::{
    ///     ActiveModelBehavior, ColumnDef, ColumnTrait, ColumnType, DbBackend, EntityName,
    ///     EntityTrait, EnumIter, PrimaryKeyTrait, RelationDef, RelationTrait, Schema,
    /// };
    /// use sea_orm_macros::{DeriveEntityModel, DerivePrimaryKey};
    /// use sea_query::{MysqlQueryBuilder, TableAlterStatement};
    ///
    /// #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    /// #[sea_orm(table_name = "posts")]
    /// pub struct Model {
    ///     #[sea_orm(primary_key)]
    ///     pub id: u32,
    ///     pub title: String,
    /// }
    ///
    /// #[derive(Copy, Clone, Debug, EnumIter)]
    /// pub enum Relation {}
    ///
    /// impl RelationTrait for Relation {
    ///     fn def(&self) -> RelationDef {
    ///         panic!("No RelationDef")
    ///     }
    /// }
    /// impl ActiveModelBehavior for ActiveModel {}
    ///
    /// let schema = Schema::new(DbBackend::MySql);
    ///
    /// let mut alter_table = TableAlterStatement::new()
    ///     .table(Entity)
    ///     .add_column(&mut schema.get_column_def::<Entity>(Column::Title))
    ///     .take();
    /// assert_eq!(
    ///     alter_table.to_string(MysqlQueryBuilder::default()),
    ///     "ALTER TABLE `posts` ADD COLUMN `title` varchar(255) NOT NULL"
    /// );
    /// ```
    pub fn get_column_def<E>(&self, column: E::Column) -> ColumnDef
    where
        E: EntityTrait,
    {
        column_def_from_entity_column::<E>(column, self.backend)
    }
}

pub(crate) fn create_enum_from_active_enum<A>(backend: DbBackend) -> TypeCreateStatement
where
    A: ActiveEnum,
{
    if matches!(backend, DbBackend::MySql | DbBackend::Sqlite) {
        panic!("TypeCreateStatement is not supported in MySQL & SQLite");
    }
    let col_def = A::db_type();
    let col_type = col_def.get_column_type();
    create_enum_from_column_type(col_type)
}

pub(crate) fn create_enum_from_column_type(col_type: &ColumnType) -> TypeCreateStatement {
    let (name, values) = match col_type {
        ColumnType::Enum { name, variants } => (name.clone(), variants.clone()),
        _ => panic!("Should be ColumnType::Enum"),
    };
    Type::create().as_enum(name).values(values).to_owned()
}

#[allow(clippy::needless_borrow)]
pub(crate) fn create_enum_from_entity<E>(_: E, backend: DbBackend) -> Vec<TypeCreateStatement>
where
    E: EntityTrait,
{
    if matches!(backend, DbBackend::MySql | DbBackend::Sqlite) {
        return Vec::new();
    }
    let mut vec = Vec::new();
    for col in E::Column::iter() {
        let col_def = col.def();
        let col_type = col_def.get_column_type();
        if !matches!(col_type, ColumnType::Enum { .. }) {
            continue;
        }
        let stmt = create_enum_from_column_type(&col_type);
        vec.push(stmt);
    }
    vec
}

pub(crate) fn create_index_from_entity<E>(
    entity: E,
    _backend: DbBackend,
) -> Vec<IndexCreateStatement>
where
    E: EntityTrait,
{
    let mut vec = Vec::new();
    for column in E::Column::iter() {
        let column_def = column.def();
        if !column_def.indexed {
            continue;
        }
        let stmt = Index::create()
            .name(format!("idx-{}-{}", entity.to_string(), column.to_string()))
            .table(entity)
            .col(column)
            .to_owned();
        vec.push(stmt)
    }
    vec
}

pub(crate) fn create_table_from_entity<E>(entity: E, backend: DbBackend) -> TableCreateStatement
where
    E: EntityTrait,
{
    let mut stmt = TableCreateStatement::new();

    if let Some(comment) = entity.comment() {
        stmt.comment(comment);
    }

    for column in E::Column::iter() {
        let mut column_def = column_def_from_entity_column::<E>(column, backend);
        stmt.col(&mut column_def);
    }

    if <<E::PrimaryKey as PrimaryKeyTrait>::ValueType as PrimaryKeyArity>::ARITY > 1 {
        let mut idx_pk = Index::create();
        for primary_key in E::PrimaryKey::iter() {
            idx_pk.col(primary_key);
        }
        stmt.primary_key(idx_pk.name(format!("pk-{}", entity.to_string())).primary());
    }

    for relation in E::Relation::iter() {
        let relation = relation.def();
        if relation.is_owner {
            continue;
        }
        stmt.foreign_key(&mut relation.into());
    }

    stmt.table(entity.table_ref()).take()
}

fn column_def_from_entity_column<E>(column: E::Column, backend: DbBackend) -> ColumnDef
where
    E: EntityTrait,
{
    let orm_column_def = column.def();
    let types = match &orm_column_def.col_type {
        ColumnType::Enum { name, variants } => match backend {
            DbBackend::MySql => {
                let variants: Vec<String> = variants.iter().map(|v| v.to_string()).collect();
                ColumnType::custom(format!("ENUM('{}')", variants.join("', '")).as_str())
            }
            DbBackend::Postgres => ColumnType::Custom(SeaRc::clone(name)),
            DbBackend::Sqlite => orm_column_def.col_type,
        },
        _ => orm_column_def.col_type,
    };
    let mut column_def = ColumnDef::new_with_type(column, types);
    if !orm_column_def.null {
        column_def.not_null();
    }
    if orm_column_def.unique {
        column_def.unique_key();
    }
    if let Some(default) = orm_column_def.default {
        column_def.default(default);
    }
    if let Some(comment) = orm_column_def.comment {
        column_def.comment(comment);
    }
    for primary_key in E::PrimaryKey::iter() {
        if column.to_string() == primary_key.into_column().to_string() {
            if E::PrimaryKey::auto_increment() {
                column_def.auto_increment();
            }
            if <<E::PrimaryKey as PrimaryKeyTrait>::ValueType as PrimaryKeyArity>::ARITY == 1 {
                column_def.primary_key();
            }
        }
    }
    column_def
}

#[cfg(test)]
mod tests {
    use crate::{sea_query::*, tests_cfg::*, DbBackend, EntityName, Schema};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_create_table_from_entity_table_ref() {
        for builder in [DbBackend::MySql, DbBackend::Postgres, DbBackend::Sqlite] {
            let schema = Schema::new(builder);
            assert_eq!(
                builder.build(&schema.create_table_from_entity(CakeFillingPrice)),
                builder.build(
                    &get_cake_filling_price_stmt()
                        .table(CakeFillingPrice.table_ref())
                        .to_owned()
                )
            );
        }
    }

    fn get_cake_filling_price_stmt() -> TableCreateStatement {
        Table::create()
            .col(
                ColumnDef::new(cake_filling_price::Column::CakeId)
                    .integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(cake_filling_price::Column::FillingId)
                    .integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(cake_filling_price::Column::Price)
                    .decimal()
                    .not_null(),
            )
            .primary_key(
                Index::create()
                    .name("pk-cake_filling_price")
                    .col(cake_filling_price::Column::CakeId)
                    .col(cake_filling_price::Column::FillingId)
                    .primary(),
            )
            .foreign_key(
                ForeignKeyCreateStatement::new()
                    .name("fk-cake_filling_price-cake_id-filling_id")
                    .from_tbl(CakeFillingPrice)
                    .from_col(cake_filling_price::Column::CakeId)
                    .from_col(cake_filling_price::Column::FillingId)
                    .to_tbl(CakeFilling)
                    .to_col(cake_filling::Column::CakeId)
                    .to_col(cake_filling::Column::FillingId),
            )
            .to_owned()
    }

    #[test]
    fn test_create_index_from_entity_table_ref() {
        for builder in [DbBackend::MySql, DbBackend::Postgres, DbBackend::Sqlite] {
            let schema = Schema::new(builder);

            assert_eq!(
                builder.build(&schema.create_table_from_entity(indexes::Entity)),
                builder.build(
                    &get_indexes_stmt()
                        .table(indexes::Entity.table_ref())
                        .to_owned()
                )
            );

            let stmts = schema.create_index_from_entity(indexes::Entity);
            assert_eq!(stmts.len(), 2);

            let idx: IndexCreateStatement = Index::create()
                .name("idx-indexes-index1_attr")
                .table(indexes::Entity)
                .col(indexes::Column::Index1Attr)
                .to_owned();
            assert_eq!(builder.build(&stmts[0]), builder.build(&idx));

            let idx: IndexCreateStatement = Index::create()
                .name("idx-indexes-index2_attr")
                .table(indexes::Entity)
                .col(indexes::Column::Index2Attr)
                .to_owned();
            assert_eq!(builder.build(&stmts[1]), builder.build(&idx));
        }
    }

    fn get_indexes_stmt() -> TableCreateStatement {
        Table::create()
            .col(
                ColumnDef::new(indexes::Column::IndexesId)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(indexes::Column::UniqueAttr)
                    .integer()
                    .not_null()
                    .unique_key(),
            )
            .col(
                ColumnDef::new(indexes::Column::Index1Attr)
                    .integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(indexes::Column::Index2Attr)
                    .integer()
                    .not_null()
                    .unique_key(),
            )
            .to_owned()
    }
}

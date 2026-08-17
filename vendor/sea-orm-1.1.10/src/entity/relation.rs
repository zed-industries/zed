use crate::{unpack_table_ref, EntityTrait, Identity, IdentityOf, Iterable, QuerySelect, Select};
use core::marker::PhantomData;
use sea_query::{
    Alias, Condition, ConditionType, DynIden, ForeignKeyCreateStatement, IntoIden, JoinType, SeaRc,
    TableForeignKey, TableRef,
};
use std::fmt::Debug;

/// Defines the type of relationship
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RelationType {
    /// An Entity has one relationship
    HasOne,
    /// An Entity has many relationships
    HasMany,
}

/// Action to perform on a foreign key whenever there are changes
/// to an ActiveModel
pub type ForeignKeyAction = sea_query::ForeignKeyAction;

/// Defines the relations of an Entity
pub trait RelationTrait: Iterable + Debug + 'static {
    /// The method to call
    fn def(&self) -> RelationDef;
}

/// Checks if Entities are related
pub trait Related<R>
where
    R: EntityTrait,
{
    /// Check if an entity is related to another entity
    fn to() -> RelationDef;

    /// Check if an entity is related through another entity
    fn via() -> Option<RelationDef> {
        None
    }

    /// Find related Entities
    fn find_related() -> Select<R> {
        Select::<R>::new().join_join_rev(JoinType::InnerJoin, Self::to(), Self::via())
    }
}

/// Defines a relationship
pub struct RelationDef {
    /// The type of relationship defined in [RelationType]
    pub rel_type: RelationType,
    /// Reference from another Entity
    pub from_tbl: TableRef,
    /// Reference to another ENtity
    pub to_tbl: TableRef,
    /// Reference to from a Column
    pub from_col: Identity,
    /// Reference to another column
    pub to_col: Identity,
    /// Defines the owner of the Relation
    pub is_owner: bool,
    /// Defines an operation to be performed on a Foreign Key when a
    /// `DELETE` Operation is performed
    pub on_delete: Option<ForeignKeyAction>,
    /// Defines an operation to be performed on a Foreign Key when a
    /// `UPDATE` Operation is performed
    pub on_update: Option<ForeignKeyAction>,
    /// Custom join ON condition
    pub on_condition: Option<Box<dyn Fn(DynIden, DynIden) -> Condition + Send + Sync>>,
    /// The name of foreign key constraint
    pub fk_name: Option<String>,
    /// Condition type of join on expression
    pub condition_type: ConditionType,
}

impl std::fmt::Debug for RelationDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("RelationDef");
        d.field("rel_type", &self.rel_type)
            .field("from_tbl", &self.from_tbl)
            .field("to_tbl", &self.to_tbl)
            .field("from_col", &self.from_col)
            .field("to_col", &self.to_col)
            .field("is_owner", &self.is_owner)
            .field("on_delete", &self.on_delete)
            .field("on_update", &self.on_update);
        debug_on_condition(&mut d, &self.on_condition);
        d.field("fk_name", &self.fk_name).finish()
    }
}

fn debug_on_condition(
    d: &mut core::fmt::DebugStruct<'_, '_>,
    on_condition: &Option<Box<dyn Fn(DynIden, DynIden) -> Condition + Send + Sync>>,
) {
    match on_condition {
        Some(func) => {
            d.field(
                "on_condition",
                &func(
                    SeaRc::new(Alias::new("left")),
                    SeaRc::new(Alias::new("right")),
                ),
            );
        }
        None => {
            d.field("on_condition", &Option::<Condition>::None);
        }
    }
}

/// Defines a helper to build a relation
pub struct RelationBuilder<E, R>
where
    E: EntityTrait,
    R: EntityTrait,
{
    entities: PhantomData<(E, R)>,
    rel_type: RelationType,
    from_tbl: TableRef,
    to_tbl: TableRef,
    from_col: Option<Identity>,
    to_col: Option<Identity>,
    is_owner: bool,
    on_delete: Option<ForeignKeyAction>,
    on_update: Option<ForeignKeyAction>,
    on_condition: Option<Box<dyn Fn(DynIden, DynIden) -> Condition + Send + Sync>>,
    fk_name: Option<String>,
    condition_type: ConditionType,
}

impl<E, R> std::fmt::Debug for RelationBuilder<E, R>
where
    E: EntityTrait,
    R: EntityTrait,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("RelationBuilder");
        d.field("entities", &self.entities)
            .field("rel_type", &self.rel_type)
            .field("from_tbl", &self.from_tbl)
            .field("to_tbl", &self.to_tbl)
            .field("from_col", &self.from_col)
            .field("to_col", &self.to_col)
            .field("is_owner", &self.is_owner)
            .field("on_delete", &self.on_delete)
            .field("on_update", &self.on_update);
        debug_on_condition(&mut d, &self.on_condition);
        d.field("fk_name", &self.fk_name).finish()
    }
}

impl RelationDef {
    /// Reverse this relation (swap from and to)
    pub fn rev(self) -> Self {
        Self {
            rel_type: self.rel_type,
            from_tbl: self.to_tbl,
            to_tbl: self.from_tbl,
            from_col: self.to_col,
            to_col: self.from_col,
            is_owner: !self.is_owner,
            on_delete: self.on_delete,
            on_update: self.on_update,
            on_condition: self.on_condition,
            fk_name: None,
            condition_type: self.condition_type,
        }
    }

    /// Express the relation from a table alias.
    ///
    /// This is a shorter and more discoverable equivalent to modifying `from_tbl` field by hand.
    ///
    /// # Examples
    ///
    /// Here's a short synthetic example.
    /// In real life you'd use aliases when the table name comes up twice and you need to disambiguate,
    /// e.g. https://github.com/SeaQL/sea-orm/discussions/2133
    ///
    /// ```
    /// use sea_orm::{
    ///     entity::*,
    ///     query::*,
    ///     tests_cfg::{cake, cake_filling},
    ///     DbBackend,
    /// };
    /// use sea_query::Alias;
    ///
    /// let cf = Alias::new("cf");
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .join_as(
    ///             JoinType::LeftJoin,
    ///             cake_filling::Relation::Cake.def().rev(),
    ///             cf.clone()
    ///         )
    ///         .join(
    ///             JoinType::LeftJoin,
    ///             cake_filling::Relation::Filling.def().from_alias(cf)
    ///         )
    ///         .build(DbBackend::MySql)
    ///         .to_string(),
    ///     [
    ///         "SELECT `cake`.`id`, `cake`.`name` FROM `cake`",
    ///         "LEFT JOIN `cake_filling` AS `cf` ON `cake`.`id` = `cf`.`cake_id`",
    ///         "LEFT JOIN `filling` ON `cf`.`filling_id` = `filling`.`id`",
    ///     ]
    ///     .join(" ")
    /// );
    /// ```
    pub fn from_alias<A>(mut self, alias: A) -> Self
    where
        A: IntoIden,
    {
        self.from_tbl = self.from_tbl.alias(alias);
        self
    }

    /// Set custom join ON condition.
    ///
    /// This method takes a closure with two parameters
    /// denoting the left-hand side and right-hand side table in the join expression.
    ///
    /// This replaces the current condition if it is already set.
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_orm::{entity::*, query::*, DbBackend, tests_cfg::{cake, cake_filling}};
    /// use sea_query::{Expr, IntoCondition};
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .join(
    ///             JoinType::LeftJoin,
    ///             cake_filling::Relation::Cake
    ///                 .def()
    ///                 .rev()
    ///                 .on_condition(|_left, right| {
    ///                     Expr::col((right, cake_filling::Column::CakeId))
    ///                         .gt(10i32)
    ///                         .into_condition()
    ///                 })
    ///         )
    ///         .build(DbBackend::MySql)
    ///         .to_string(),
    ///     [
    ///         "SELECT `cake`.`id`, `cake`.`name` FROM `cake`",
    ///         "LEFT JOIN `cake_filling` ON `cake`.`id` = `cake_filling`.`cake_id` AND `cake_filling`.`cake_id` > 10",
    ///     ]
    ///     .join(" ")
    /// );
    /// ```
    pub fn on_condition<F>(mut self, f: F) -> Self
    where
        F: Fn(DynIden, DynIden) -> Condition + 'static + Send + Sync,
    {
        self.on_condition = Some(Box::new(f));
        self
    }

    /// Set the condition type of join on expression
    ///
    /// # Examples
    ///
    /// ```
    /// use sea_orm::{entity::*, query::*, DbBackend, tests_cfg::{cake, cake_filling}};
    /// use sea_query::{Expr, IntoCondition, ConditionType};
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .join(
    ///             JoinType::LeftJoin,
    ///             cake_filling::Relation::Cake
    ///                 .def()
    ///                 .rev()
    ///                 .condition_type(ConditionType::Any)
    ///                 .on_condition(|_left, right| {
    ///                     Expr::col((right, cake_filling::Column::CakeId))
    ///                         .gt(10i32)
    ///                         .into_condition()
    ///                 })
    ///         )
    ///         .build(DbBackend::MySql)
    ///         .to_string(),
    ///     [
    ///         "SELECT `cake`.`id`, `cake`.`name` FROM `cake`",
    ///         "LEFT JOIN `cake_filling` ON `cake`.`id` = `cake_filling`.`cake_id` OR `cake_filling`.`cake_id` > 10",
    ///     ]
    ///     .join(" ")
    /// );
    /// ```
    pub fn condition_type(mut self, condition_type: ConditionType) -> Self {
        self.condition_type = condition_type;
        self
    }
}

impl<E, R> RelationBuilder<E, R>
where
    E: EntityTrait,
    R: EntityTrait,
{
    pub(crate) fn new(rel_type: RelationType, from: E, to: R, is_owner: bool) -> Self {
        Self {
            entities: PhantomData,
            rel_type,
            from_tbl: from.table_ref(),
            to_tbl: to.table_ref(),
            from_col: None,
            to_col: None,
            is_owner,
            on_delete: None,
            on_update: None,
            on_condition: None,
            fk_name: None,
            condition_type: ConditionType::All,
        }
    }

    pub(crate) fn from_rel(rel_type: RelationType, rel: RelationDef, is_owner: bool) -> Self {
        Self {
            entities: PhantomData,
            rel_type,
            from_tbl: rel.from_tbl,
            to_tbl: rel.to_tbl,
            from_col: Some(rel.from_col),
            to_col: Some(rel.to_col),
            is_owner,
            on_delete: None,
            on_update: None,
            on_condition: None,
            fk_name: None,
            condition_type: ConditionType::All,
        }
    }

    /// Build a relationship from an Entity
    pub fn from<T>(mut self, identifier: T) -> Self
    where
        T: IdentityOf<E>,
    {
        self.from_col = Some(identifier.identity_of());
        self
    }

    /// Build a relationship to an Entity
    pub fn to<T>(mut self, identifier: T) -> Self
    where
        T: IdentityOf<R>,
    {
        self.to_col = Some(identifier.identity_of());
        self
    }

    /// An operation to perform on a foreign key when a delete operation occurs
    pub fn on_delete(mut self, action: ForeignKeyAction) -> Self {
        self.on_delete = Some(action);
        self
    }

    /// An operation to perform on a foreign key when an update operation occurs
    pub fn on_update(mut self, action: ForeignKeyAction) -> Self {
        self.on_update = Some(action);
        self
    }

    /// Set custom join ON condition.
    ///
    /// This method takes a closure with parameters
    /// denoting the left-hand side and right-hand side table in the join expression.
    pub fn on_condition<F>(mut self, f: F) -> Self
    where
        F: Fn(DynIden, DynIden) -> Condition + 'static + Send + Sync,
    {
        self.on_condition = Some(Box::new(f));
        self
    }

    /// Set the name of foreign key constraint
    pub fn fk_name(mut self, fk_name: &str) -> Self {
        self.fk_name = Some(fk_name.to_owned());
        self
    }

    /// Set the condition type of join on expression
    pub fn condition_type(mut self, condition_type: ConditionType) -> Self {
        self.condition_type = condition_type;
        self
    }
}

impl<E, R> From<RelationBuilder<E, R>> for RelationDef
where
    E: EntityTrait,
    R: EntityTrait,
{
    fn from(b: RelationBuilder<E, R>) -> Self {
        RelationDef {
            rel_type: b.rel_type,
            from_tbl: b.from_tbl,
            to_tbl: b.to_tbl,
            from_col: b.from_col.expect("Reference column is not set"),
            to_col: b.to_col.expect("Owner column is not set"),
            is_owner: b.is_owner,
            on_delete: b.on_delete,
            on_update: b.on_update,
            on_condition: b.on_condition,
            fk_name: b.fk_name,
            condition_type: b.condition_type,
        }
    }
}

macro_rules! set_foreign_key_stmt {
    ( $relation: ident, $foreign_key: ident ) => {
        let from_cols: Vec<String> = $relation
            .from_col
            .into_iter()
            .map(|col| {
                let col_name = col.to_string();
                $foreign_key.from_col(col);
                col_name
            })
            .collect();
        for col in $relation.to_col.into_iter() {
            $foreign_key.to_col(col);
        }
        if let Some(action) = $relation.on_delete {
            $foreign_key.on_delete(action);
        }
        if let Some(action) = $relation.on_update {
            $foreign_key.on_update(action);
        }
        let name = if let Some(name) = $relation.fk_name {
            name
        } else {
            let from_tbl = unpack_table_ref(&$relation.from_tbl);
            format!("fk-{}-{}", from_tbl.to_string(), from_cols.join("-"))
        };
        $foreign_key.name(&name);
    };
}

impl From<RelationDef> for ForeignKeyCreateStatement {
    fn from(relation: RelationDef) -> Self {
        let mut foreign_key_stmt = Self::new();
        set_foreign_key_stmt!(relation, foreign_key_stmt);
        foreign_key_stmt
            .from_tbl(unpack_table_ref(&relation.from_tbl))
            .to_tbl(unpack_table_ref(&relation.to_tbl))
            .take()
    }
}

/// Creates a column definition for example to update a table.
/// ```
/// use sea_query::{Alias, IntoIden, MysqlQueryBuilder, TableAlterStatement, TableRef, ConditionType};
/// use sea_orm::{EnumIter, Iden, Identity, PrimaryKeyTrait, RelationDef, RelationTrait, RelationType};
///
/// let relation = RelationDef {
///     rel_type: RelationType::HasOne,
///     from_tbl: TableRef::Table(Alias::new("foo").into_iden()),
///     to_tbl: TableRef::Table(Alias::new("bar").into_iden()),
///     from_col: Identity::Unary(Alias::new("bar_id").into_iden()),
///     to_col: Identity::Unary(Alias::new("bar_id").into_iden()),
///     is_owner: false,
///     on_delete: None,
///     on_update: None,
///     on_condition: None,
///     fk_name: Some("foo-bar".to_string()),
///     condition_type: ConditionType::All,
/// };
///
/// let mut alter_table = TableAlterStatement::new()
///     .table(TableRef::Table(Alias::new("foo").into_iden()))
///     .add_foreign_key(&mut relation.into()).take();
/// assert_eq!(
///     alter_table.to_string(MysqlQueryBuilder::default()),
///     "ALTER TABLE `foo` ADD CONSTRAINT `foo-bar` FOREIGN KEY (`bar_id`) REFERENCES `bar` (`bar_id`)"
/// );
/// ```
impl From<RelationDef> for TableForeignKey {
    fn from(relation: RelationDef) -> Self {
        let mut foreign_key = Self::new();
        set_foreign_key_stmt!(relation, foreign_key);
        foreign_key
            .from_tbl(unpack_table_ref(&relation.from_tbl))
            .to_tbl(unpack_table_ref(&relation.to_tbl))
            .take()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        tests_cfg::{cake, fruit},
        RelationBuilder, RelationDef,
    };

    #[test]
    fn assert_relation_traits() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<RelationDef>();
        assert_send_sync::<RelationBuilder<cake::Entity, fruit::Entity>>();
    }
}

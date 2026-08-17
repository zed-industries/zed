/// This modules contains types and traits for an  Entity, ActiveMode, Model, PrimaryKey, ForeignKey and Relations.
///
/// // An Entity
/// A unit struct implements [EntityTrait](crate::EntityTrait) representing a table in the database.
///
/// This trait contains the properties of an entity including
///
/// - The Table Name which is implemented by [EntityName](crate::EntityName)
/// - The Column which is implemented by [ColumnTrait](crate::ColumnTrait)
/// - A Relation which is implemented by [RelationTrait](crate::RelationTrait)
/// - The Primary Key which is implemented by [PrimaryKeyTrait](crate::PrimaryKeyTrait)
///   and [PrimaryKeyToColumn](crate::PrimaryKeyToColumn)
///
/// This trait also provides an API for CRUD actions
///
/// #### Example for creating an Entity, Model and ActiveModel
/// ```
/// #[cfg(feature = "macros")]
/// # use sea_orm::entity::prelude::*;
/// use sea_orm::ActiveModelBehavior;
/// use sea_orm::ColumnDef;
/// use sea_orm::ColumnTrait;
/// use sea_orm::ColumnType;
/// use sea_orm::EntityName;
/// use sea_orm::PrimaryKeyTrait;
/// use sea_orm::RelationDef;
/// use sea_orm::RelationTrait;
///
/// // Use [DeriveEntity] to derive the EntityTrait automatically
/// #[derive(Copy, Clone, Default, Debug, DeriveEntity)]
/// pub struct Entity;
///
/// /// The [EntityName] describes the name of a table
/// impl EntityName for Entity {
///     fn table_name(&self) -> &str {
///         "filling"
///     }
/// }
///
/// // Create a Model for the Entity through [DeriveModel].
/// // The `Model` handles `READ` operations on a table in a database.
/// // The [DeriveActiveModel] creates a way to perform `CREATE` , `READ` and `UPDATE` operations
/// // in a database
/// #[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel)]
/// pub struct Model {
///     pub id: i32,
///     pub name: String,
/// }
///
/// // Use the [DeriveColumn] to create a Column for an the table called Entity
/// // The [EnumIter] which creates a new type that iterates of the variants of a Column.
/// #[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
/// pub enum Column {
///     Id,
///     Name,
/// }
///
/// // Create a PrimaryKey for the Entity using the [PrimaryKeyTrait]
/// // The [EnumIter] which creates a new type that iterates of the variants of a PrimaryKey.
/// #[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
/// pub enum PrimaryKey {
///     Id,
/// }
///
/// // Or implement the [PrimaryKeyTrait] manually instead of using the macro [DerivePrimaryKey]
/// impl PrimaryKeyTrait for PrimaryKey {
///     type ValueType = i32;
///
///     fn auto_increment() -> bool {
///         true
///     }
/// }
///
/// #[derive(Copy, Clone, Debug, EnumIter)]
/// pub enum Relation {}
///
/// impl ColumnTrait for Column {
///     type EntityName = Entity;
///
///     fn def(&self) -> ColumnDef {
///         match self {
///             Self::Id => ColumnType::Integer.def(),
///             Self::Name => ColumnType::String(StringLen::None).def(),
///         }
///     }
/// }
///
/// // Create a Relation for the Entity
/// impl RelationTrait for Relation {
///     fn def(&self) -> RelationDef {
///         unimplemented!()
///     }
/// }
///
/// // Implement user defined operations for CREATE, UPDATE and DELETE operations
/// // to create an ActiveModel using the [ActiveModelBehavior]
/// impl ActiveModelBehavior for ActiveModel {}
/// ```
mod active_enum;
mod active_model;
mod base_entity;
mod column;
mod identity;
mod link;
mod model;
mod partial_model;
/// Re-export common types from the entity
pub mod prelude;
mod primary_key;
mod relation;

pub use active_enum::*;
pub use active_model::*;
pub use base_entity::*;
pub use column::*;
pub use identity::*;
pub use link::*;
pub use model::*;
pub use partial_model::*;
// pub use prelude::*;
pub use primary_key::*;
pub use relation::*;

extern crate proc_macro;

use proc_macro::TokenStream;

use syn::{parse_macro_input, DeriveInput, Error};

#[cfg(feature = "derive")]
mod derives;

#[cfg(feature = "strum")]
mod strum;

/// Create an Entity
///
/// ### Usage
///
/// ```
/// use sea_orm::entity::prelude::*;
///
/// #[derive(Copy, Clone, Default, Debug, DeriveEntity)]
/// pub struct Entity;
///
/// # impl EntityName for Entity {
/// #     fn table_name(&self) -> &str {
/// #         "cake"
/// #     }
/// # }
/// #
/// # #[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel)]
/// # pub struct Model {
/// #     pub id: i32,
/// #     pub name: String,
/// # }
/// #
/// # #[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
/// # pub enum Column {
/// #     Id,
/// #     Name,
/// # }
/// #
/// # #[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
/// # pub enum PrimaryKey {
/// #     Id,
/// # }
/// #
/// # impl PrimaryKeyTrait for PrimaryKey {
/// #     type ValueType = i32;
/// #
/// #     fn auto_increment() -> bool {
/// #         true
/// #     }
/// # }
/// #
/// # #[derive(Copy, Clone, Debug, EnumIter)]
/// # pub enum Relation {}
/// #
/// # impl ColumnTrait for Column {
/// #     type EntityName = Entity;
/// #
/// #     fn def(&self) -> ColumnDef {
/// #         match self {
/// #             Self::Id => ColumnType::Integer.def(),
/// #             Self::Name => ColumnType::String(StringLen::None).def(),
/// #         }
/// #     }
/// # }
/// #
/// # impl RelationTrait for Relation {
/// #     fn def(&self) -> RelationDef {
/// #         panic!("No Relation");
/// #     }
/// # }
/// #
/// # impl ActiveModelBehavior for ActiveModel {}
/// ```
#[cfg(feature = "derive")]
#[proc_macro_derive(DeriveEntity, attributes(sea_orm))]
pub fn derive_entity(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derives::expand_derive_entity(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

/// This derive macro is the 'almighty' macro which automatically generates
/// Entity, Column, and PrimaryKey from a given Model.
///
/// ### Usage
///
/// ```
/// use sea_orm::entity::prelude::*;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
/// #[sea_orm(table_name = "posts")]
/// pub struct Model {
///     #[sea_orm(primary_key)]
///     pub id: i32,
///     pub title: String,
///     #[sea_orm(column_type = "Text")]
///     pub text: String,
/// }
///
/// # #[derive(Copy, Clone, Debug, EnumIter)]
/// # pub enum Relation {}
/// #
/// # impl RelationTrait for Relation {
/// #     fn def(&self) -> RelationDef {
/// #         panic!("No Relation");
/// #     }
/// # }
/// #
/// # impl ActiveModelBehavior for ActiveModel {}
/// ```
///
/// Entity should always have a primary key.
/// Or, it will result in a compile error.
/// See <https://github.com/SeaQL/sea-orm/issues/485> for details.
///
/// ```compile_fail
/// use sea_orm::entity::prelude::*;
///
/// #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
/// #[sea_orm(table_name = "posts")]
/// pub struct Model {
///     pub title: String,
///     #[sea_orm(column_type = "Text")]
///     pub text: String,
/// }
///
/// # #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
/// # pub enum Relation {}
/// #
/// # impl ActiveModelBehavior for ActiveModel {}
/// ```
#[cfg(feature = "derive")]
#[proc_macro_derive(DeriveEntityModel, attributes(sea_orm))]
pub fn derive_entity_model(input: TokenStream) -> TokenStream {
    let input_ts = input.clone();
    let DeriveInput {
        ident, data, attrs, ..
    } = parse_macro_input!(input as DeriveInput);

    if ident != "Model" {
        panic!("Struct name must be Model");
    }

    let mut ts: TokenStream = derives::expand_derive_entity_model(data, attrs)
        .unwrap_or_else(Error::into_compile_error)
        .into();
    ts.extend([
        derive_model(input_ts.clone()),
        derive_active_model(input_ts),
    ]);
    ts
}

/// The DerivePrimaryKey derive macro will implement [PrimaryKeyToColumn]
/// for PrimaryKey which defines tedious mappings between primary keys and columns.
/// The [EnumIter] is also derived, allowing iteration over all enum variants.
///
/// ### Usage
///
/// ```
/// use sea_orm::entity::prelude::*;
///
/// #[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
/// pub enum PrimaryKey {
///     CakeId,
///     FillingId,
/// }
///
/// # #[derive(Copy, Clone, Default, Debug, DeriveEntity)]
/// # pub struct Entity;
/// #
/// # impl EntityName for Entity {
/// #     fn table_name(&self) -> &str {
/// #         "cake"
/// #     }
/// # }
/// #
/// # #[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel)]
/// # pub struct Model {
/// #     pub cake_id: i32,
/// #     pub filling_id: i32,
/// # }
/// #
/// # #[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
/// # pub enum Column {
/// #     CakeId,
/// #     FillingId,
/// # }
/// #
/// # #[derive(Copy, Clone, Debug, EnumIter)]
/// # pub enum Relation {}
/// #
/// # impl ColumnTrait for Column {
/// #     type EntityName = Entity;
/// #
/// #     fn def(&self) -> ColumnDef {
/// #         match self {
/// #             Self::CakeId => ColumnType::Integer.def(),
/// #             Self::FillingId => ColumnType::Integer.def(),
/// #         }
/// #     }
/// # }
/// #
/// # impl PrimaryKeyTrait for PrimaryKey {
/// #     type ValueType = (i32, i32);
/// #
/// #     fn auto_increment() -> bool {
/// #         false
/// #     }
/// # }
/// #
/// # impl RelationTrait for Relation {
/// #     fn def(&self) -> RelationDef {
/// #         panic!("No Relation");
/// #     }
/// # }
/// #
/// # impl ActiveModelBehavior for ActiveModel {}
/// ```
#[cfg(feature = "derive")]
#[proc_macro_derive(DerivePrimaryKey, attributes(sea_orm))]
pub fn derive_primary_key(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    match derives::expand_derive_primary_key(ident, data) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

/// The DeriveColumn derive macro will implement [ColumnTrait] for Columns.
/// It defines the identifier of each column by implementing Iden and IdenStatic.
/// The EnumIter is also derived, allowing iteration over all enum variants.
///
/// ### Usage
///
/// ```
/// use sea_orm::entity::prelude::*;
///
/// #[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
/// pub enum Column {
///     CakeId,
///     FillingId,
/// }
/// ```
#[cfg(feature = "derive")]
#[proc_macro_derive(DeriveColumn, attributes(sea_orm))]
pub fn derive_column(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    match derives::expand_derive_column(&ident, &data) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

/// Derive a column if column names are not in snake-case
///
/// ### Usage
///
/// ```
/// use sea_orm::entity::prelude::*;
///
/// #[derive(Copy, Clone, Debug, EnumIter, DeriveCustomColumn)]
/// pub enum Column {
///     Id,
///     Name,
///     VendorId,
/// }
///
/// impl IdenStatic for Column {
///     fn as_str(&self) -> &str {
///         match self {
///             Self::Id => "id",
///             _ => self.default_as_str(),
///         }
///     }
/// }
/// ```
#[cfg(feature = "derive")]
#[proc_macro_derive(DeriveCustomColumn)]
pub fn derive_custom_column(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    match derives::expand_derive_custom_column(&ident, &data) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

/// The DeriveModel derive macro will implement ModelTrait for Model,
/// which provides setters and getters for all attributes in the mod
/// It also implements FromQueryResult to convert a query result into the corresponding Model.
///
/// ### Usage
///
/// ```
/// use sea_orm::entity::prelude::*;
///
/// #[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel)]
/// pub struct Model {
///     pub id: i32,
///     pub name: String,
/// }
///
/// # #[derive(Copy, Clone, Default, Debug, DeriveEntity)]
/// # pub struct Entity;
/// #
/// # impl EntityName for Entity {
/// #     fn table_name(&self) -> &str {
/// #         "cake"
/// #     }
/// # }
/// #
/// # #[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
/// # pub enum Column {
/// #     Id,
/// #     Name,
/// # }
/// #
/// # #[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
/// # pub enum PrimaryKey {
/// #     Id,
/// # }
/// #
/// # impl PrimaryKeyTrait for PrimaryKey {
/// #     type ValueType = i32;
/// #
/// #     fn auto_increment() -> bool {
/// #         true
/// #     }
/// # }
/// #
/// # #[derive(Copy, Clone, Debug, EnumIter)]
/// # pub enum Relation {}
/// #
/// # impl ColumnTrait for Column {
/// #     type EntityName = Entity;
/// #
/// #     fn def(&self) -> ColumnDef {
/// #         match self {
/// #             Self::Id => ColumnType::Integer.def(),
/// #             Self::Name => ColumnType::String(StringLen::None).def(),
/// #         }
/// #     }
/// # }
/// #
/// # impl RelationTrait for Relation {
/// #     fn def(&self) -> RelationDef {
/// #         panic!("No Relation");
/// #     }
/// # }
/// #
/// # impl ActiveModelBehavior for ActiveModel {}
/// ```
#[cfg(feature = "derive")]
#[proc_macro_derive(DeriveModel, attributes(sea_orm))]
pub fn derive_model(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derives::expand_derive_model(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

/// The DeriveActiveModel derive macro will implement ActiveModelTrait for ActiveModel
/// which provides setters and getters for all active values in the active model.
///
/// ### Usage
///
/// ```
/// use sea_orm::entity::prelude::*;
///
/// #[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel)]
/// pub struct Model {
///     pub id: i32,
///     pub name: String,
/// }
///
/// # #[derive(Copy, Clone, Default, Debug, DeriveEntity)]
/// # pub struct Entity;
/// #
/// # impl EntityName for Entity {
/// #     fn table_name(&self) -> &str {
/// #         "cake"
/// #     }
/// # }
/// #
/// # #[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
/// # pub enum Column {
/// #     Id,
/// #     Name,
/// # }
/// #
/// # #[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
/// # pub enum PrimaryKey {
/// #     Id,
/// # }
/// #
/// # impl PrimaryKeyTrait for PrimaryKey {
/// #     type ValueType = i32;
/// #
/// #     fn auto_increment() -> bool {
/// #         true
/// #     }
/// # }
/// #
/// # #[derive(Copy, Clone, Debug, EnumIter)]
/// # pub enum Relation {}
/// #
/// # impl ColumnTrait for Column {
/// #     type EntityName = Entity;
/// #
/// #     fn def(&self) -> ColumnDef {
/// #         match self {
/// #             Self::Id => ColumnType::Integer.def(),
/// #             Self::Name => ColumnType::String(StringLen::None).def(),
/// #         }
/// #     }
/// # }
/// #
/// # impl RelationTrait for Relation {
/// #     fn def(&self) -> RelationDef {
/// #         panic!("No Relation");
/// #     }
/// # }
/// #
/// # impl ActiveModelBehavior for ActiveModel {}
/// ```
#[cfg(feature = "derive")]
#[proc_macro_derive(DeriveActiveModel, attributes(sea_orm))]
pub fn derive_active_model(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    match derives::expand_derive_active_model(ident, data) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

/// Derive into an active model
#[cfg(feature = "derive")]
#[proc_macro_derive(DeriveIntoActiveModel, attributes(sea_orm))]
pub fn derive_into_active_model(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derives::expand_into_active_model(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

/// Models that a user can override
///
/// ### Usage
///
/// ```
/// use sea_orm::entity::prelude::*;
///
/// #[derive(
///     Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, DeriveActiveModelBehavior,
/// )]
/// pub struct Model {
///     pub id: i32,
///     pub name: String,
/// }
///
/// # #[derive(Copy, Clone, Default, Debug, DeriveEntity)]
/// # pub struct Entity;
/// #
/// # impl EntityName for Entity {
/// #     fn table_name(&self) -> &str {
/// #         "cake"
/// #     }
/// # }
/// #
/// # #[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
/// # pub enum Column {
/// #     Id,
/// #     Name,
/// # }
/// #
/// # #[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
/// # pub enum PrimaryKey {
/// #     Id,
/// # }
/// #
/// # impl PrimaryKeyTrait for PrimaryKey {
/// #     type ValueType = i32;
/// #
/// #     fn auto_increment() -> bool {
/// #         true
/// #     }
/// # }
/// #
/// # #[derive(Copy, Clone, Debug, EnumIter)]
/// # pub enum Relation {}
/// #
/// # impl ColumnTrait for Column {
/// #     type EntityName = Entity;
/// #
/// #     fn def(&self) -> ColumnDef {
/// #         match self {
/// #             Self::Id => ColumnType::Integer.def(),
/// #             Self::Name => ColumnType::String(StringLen::None).def(),
/// #         }
/// #     }
/// # }
/// #
/// # impl RelationTrait for Relation {
/// #     fn def(&self) -> RelationDef {
/// #         panic!("No Relation");
/// #     }
/// # }
/// ```
#[cfg(feature = "derive")]
#[proc_macro_derive(DeriveActiveModelBehavior)]
pub fn derive_active_model_behavior(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    match derives::expand_derive_active_model_behavior(ident, data) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

/// A derive macro to implement `sea_orm::ActiveEnum` trait for enums.
///
/// # Limitations
///
/// This derive macros can only be used on enums.
///
/// # Macro Attributes
///
/// All macro attributes listed below have to be annotated in the form of `#[sea_orm(attr = value)]`.
///
/// - For enum
///     - `rs_type`: Define `ActiveEnum::Value`
///         - Possible values: `String`, `i8`, `i16`, `i32`, `i64`, `u8`, `u16`, `u32`, `u64`
///         - Note that value has to be passed as string, i.e. `rs_type = "i8"`
///     - `db_type`: Define `ColumnType` returned by `ActiveEnum::db_type()`
///         - Possible values: all available enum variants of `ColumnType`, e.g. `String(StringLen::None)`, `String(StringLen::N(1))`, `Integer`
///         - Note that value has to be passed as string, i.e. `db_type = "Integer"`
///     - `enum_name`: Define `String` returned by `ActiveEnum::name()`
///         - This attribute is optional with default value being the name of enum in camel-case
///         - Note that value has to be passed as string, i.e. `enum_name = "MyEnum"`
///
/// - For enum variant
///     - `string_value` or `num_value`:
///         - For `string_value`, value should be passed as string, i.e. `string_value = "A"`
///             - Due to the way internal Enums are automatically derived, the following restrictions apply:
///                 - members cannot share identical `string_value`, case-insensitive.
///                 - in principle, any future Titlecased Rust keywords are not valid `string_value`.
///         - For `num_value`, value should be passed as integer, i.e. `num_value = 1` or `num_value = 1i32`
///         - Note that only one of it can be specified, and all variants of an enum have to annotate with the same `*_value` macro attribute
///
/// # Usage
///
/// ```
/// use sea_orm::{entity::prelude::*, DeriveActiveEnum};
///
/// #[derive(EnumIter, DeriveActiveEnum)]
/// #[sea_orm(rs_type = "i32", db_type = "Integer")]
/// pub enum Color {
///     Black = 0,
///     White = 1,
/// }
/// ```
#[cfg(feature = "derive")]
#[proc_macro_derive(DeriveActiveEnum, attributes(sea_orm))]
pub fn derive_active_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match derives::expand_derive_active_enum(input) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

/// Convert a query result into the corresponding Model.
///
/// ### Attributes
///
/// - `skip`: will not try to pull this field from the query result. And set it to the default value of the type.
/// - `nested`: allows nesting models. can be any type that implements `FromQueryResult`
/// - `from_alias`: get the value from this column alias
///
/// ### Usage
///
/// For more complete examples, please refer to https://github.com/SeaQL/sea-orm/blob/master/tests/from_query_result_tests.rs
///
/// ```
/// use sea_orm::{entity::prelude::*, FromQueryResult};
///
/// #[derive(FromQueryResult)]
/// struct Cake {
///     id: i32,
///     name: String,
///     #[sea_orm(nested)]
///     bakery: Option<CakeBakery>,
///     #[sea_orm(skip)]
///     skip_me: i32,
/// }
///
/// #[derive(FromQueryResult)]
/// struct CakeBakery {
///     #[sea_orm(from_alias = "bakery_id")]
///     id: i32,
///     #[sea_orm(from_alias = "bakery_name")]
///     title: String,
/// }
/// ```
///
/// You can compose this with regular Models, if there's no column collision:
///
/// ```ignore
/// #[derive(FromQueryResult)]
/// struct CakePlain {
///     id: i32,
///     name: String,
///     price: Decimal,
///     #[sea_orm(nested)]
///     baker: Option<cakes_bakers::Model>,
/// }
/// ```
#[cfg(feature = "derive")]
#[proc_macro_derive(FromQueryResult, attributes(sea_orm))]
pub fn derive_from_query_result(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input);

    match derives::expand_derive_from_query_result(derive_input) {
        Ok(token_stream) => token_stream.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

/// The DeriveRelation derive macro will implement RelationTrait for Relation.
///
/// ### Usage
///
/// ```
/// # use sea_orm::tests_cfg::fruit::Entity;
/// use sea_orm::entity::prelude::*;
///
/// #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
/// pub enum Relation {
///     #[sea_orm(
///         belongs_to = "sea_orm::tests_cfg::cake::Entity",
///         from = "sea_orm::tests_cfg::fruit::Column::CakeId",
///         to = "sea_orm::tests_cfg::cake::Column::Id"
///     )]
///     Cake,
///     #[sea_orm(
///         belongs_to = "sea_orm::tests_cfg::cake_expanded::Entity",
///         from = "sea_orm::tests_cfg::fruit::Column::CakeId",
///         to = "sea_orm::tests_cfg::cake_expanded::Column::Id"
///     )]
///     CakeExpanded,
/// }
/// ```
#[cfg(feature = "derive")]
#[proc_macro_derive(DeriveRelation, attributes(sea_orm))]
pub fn derive_relation(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derives::expand_derive_relation(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

/// The DeriveRelatedEntity derive macro will implement seaography::RelationBuilder for RelatedEntity enumeration.
///
/// ### Usage
///
/// ```ignore
/// use sea_orm::entity::prelude::*;
///
/// // ...
/// // Model, Relation enum, etc.
/// // ...
///
/// #[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
/// pub enum RelatedEntity {
///     #[sea_orm(entity = "super::address::Entity")]
///     Address,
///     #[sea_orm(entity = "super::payment::Entity")]
///     Payment,
///     #[sea_orm(entity = "super::rental::Entity")]
///     Rental,
///     #[sea_orm(entity = "Entity", def = "Relation::SelfRef.def()")]
///     SelfRef,
///     #[sea_orm(entity = "super::store::Entity")]
///     Store,
///     #[sea_orm(entity = "Entity", def = "Relation::SelfRef.def().rev()")]
///     SelfRefRev,
/// }
/// ```
#[cfg(feature = "derive")]
#[proc_macro_derive(DeriveRelatedEntity, attributes(sea_orm))]
pub fn derive_related_entity(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derives::expand_derive_related_entity(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

/// The DeriveMigrationName derive macro will implement `sea_orm_migration::MigrationName` for a migration.
///
/// ### Usage
///
/// ```ignore
/// #[derive(DeriveMigrationName)]
/// pub struct Migration;
/// ```
///
/// The derive macro above will provide following implementation,
/// given the file name is `m20220120_000001_create_post_table.rs`.
///
/// ```ignore
/// impl MigrationName for Migration {
///     fn name(&self) -> &str {
///         "m20220120_000001_create_post_table"
///     }
/// }
/// ```
#[cfg(feature = "derive")]
#[proc_macro_derive(DeriveMigrationName)]
pub fn derive_migration_name(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derives::expand_derive_migration_name(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

#[cfg(feature = "derive")]
#[proc_macro_derive(FromJsonQueryResult)]
pub fn derive_from_json_query_result(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);

    match derives::expand_derive_from_json_query_result(ident) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

/// The DerivePartialModel derive macro will implement [`sea_orm::PartialModelTrait`] for simplify partial model queries.
///
/// ## Usage
///
/// For more complete examples, please refer to https://github.com/SeaQL/sea-orm/blob/master/tests/partial_model_tests.rs
///
/// ```rust
/// use sea_orm::{entity::prelude::*, DerivePartialModel, FromQueryResult};
///
/// #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
/// #[sea_orm(table_name = "posts")]
/// pub struct Model {
///     #[sea_orm(primary_key)]
///     pub id: i32,
///     pub title: String,
///     #[sea_orm(column_type = "Text")]
///     pub text: String,
/// }
/// # #[derive(Copy, Clone, Debug, DeriveRelation, EnumIter)]
/// # pub enum Relation {}
/// # impl ActiveModelBehavior for ActiveModel {}
///
/// #[derive(Debug, FromQueryResult, DerivePartialModel)]
/// #[sea_orm(entity = "Entity")]
/// struct SelectResult {
///     title: String,
///     #[sea_orm(from_col = "text")]
///     content: String,
///     #[sea_orm(from_expr = "Expr::val(1).add(1)")]
///     sum: i32,
/// }
/// ```
///
/// If all fields in the partial model is `from_expr`, the specifying the `entity` can be skipped.
/// ```
/// use sea_orm::{entity::prelude::*, sea_query::Expr, DerivePartialModel, FromQueryResult};
///
/// #[derive(Debug, FromQueryResult, DerivePartialModel)]
/// struct SelectResult {
///     #[sea_orm(from_expr = "Expr::val(1).add(1)")]
///     sum: i32,
/// }
/// ```
///
/// Since SeaORM 1.1.7, `DerivePartialModel` can also assumes the function of `FromQueryResult`.
/// This is necessary to support nested partial models.
///
/// ```
/// use sea_orm::{DerivePartialModel, FromQueryResult};
/// #
/// # mod cake {
/// # use sea_orm::entity::prelude::*;
/// # #[derive(Clone, Debug, DeriveEntityModel)]
/// # #[sea_orm(table_name = "cake")]
/// # pub struct Model {
/// #     #[sea_orm(primary_key)]
/// #     pub id: i32,
/// #     pub name: String,
/// # }
/// # #[derive(Copy, Clone, Debug, DeriveRelation, EnumIter)]
/// # pub enum Relation {}
/// # impl ActiveModelBehavior for ActiveModel {}
/// # }
/// #
/// # mod bakery {
/// # use sea_orm::entity::prelude::*;
/// # #[derive(Clone, Debug, DeriveEntityModel)]
/// # #[sea_orm(table_name = "bakery")]
/// # pub struct Model {
/// #     #[sea_orm(primary_key)]
/// #     pub id: i32,
/// #     pub name: String,
/// # }
/// # #[derive(Copy, Clone, Debug, DeriveRelation, EnumIter)]
/// # pub enum Relation {}
/// # impl ActiveModelBehavior for ActiveModel {}
/// # }
///
/// #[derive(DerivePartialModel)]
/// #[sea_orm(entity = "cake::Entity", from_query_result)]
/// struct Cake {
///     id: i32,
///     name: String,
///     #[sea_orm(nested)]
///     bakery: Option<Bakery>,
///     #[sea_orm(skip)]
///     ignore: String,
/// }
///
/// #[derive(FromQueryResult, DerivePartialModel)]
/// #[sea_orm(entity = "bakery::Entity")]
/// struct Bakery {
///     id: i32,
///     #[sea_orm(from_col = "Name")]
///     title: String,
/// }
///
/// // In addition, there's an `alias` attribute to select the columns from an alias:
///
/// #[derive(DerivePartialModel)]
/// #[sea_orm(entity = "bakery::Entity", alias = "factory", from_query_result)]
/// struct Factory {
///     id: i32,
///     #[sea_orm(from_col = "name")]
///     plant: String,
/// }
///
/// #[derive(DerivePartialModel)]
/// #[sea_orm(entity = "cake::Entity", from_query_result)]
/// struct CakeFactory {
///     id: i32,
///     name: String,
///     #[sea_orm(nested)]
///     bakery: Option<Factory>,
/// }
/// ```
///
/// ```ignore
/// let cake: CakeFactory = cake::Entity::find()
///     .join_as(
///         JoinType::LeftJoin,
///         cake::Relation::Bakery.def(),
///         Alias::new("factory"),
///     )
///     .order_by_asc(cake::Column::Id)
///     .into_partial_model()
///     .one(&db)
///     .await
///     .unwrap()
///     .unwrap()
///
/// SELECT
///     "cake"."id" AS "id", "cake"."name" AS "name",
///     "factory"."id" AS "bakery_id", "factory"."name" AS "bakery_plant"
/// FROM "cake"
/// LEFT JOIN "bakery" AS "factory" ON "cake"."bakery_id" = "factory"."id"
/// LIMIT 1
/// ```
///
/// A field cannot have attributes `from_col`, `from_expr` or `nested` at the same time.
/// Or, it will result in a compile error.
///
/// ```compile_fail
/// use sea_orm::{entity::prelude::*, FromQueryResult, DerivePartialModel, sea_query::Expr};
///
/// #[derive(Debug, FromQueryResult, DerivePartialModel)]
/// #[sea_orm(entity = "Entity")]
/// struct SelectResult {
///     #[sea_orm(from_expr = "Expr::val(1).add(1)", from_col = "foo")]
///     sum: i32,
/// }
/// ```
#[cfg(feature = "derive")]
#[proc_macro_derive(DerivePartialModel, attributes(sea_orm))]
pub fn derive_partial_model(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input);

    match derives::expand_derive_partial_model(derive_input) {
        Ok(token_stream) => token_stream.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[doc(hidden)]
#[cfg(feature = "derive")]
#[proc_macro_attribute]
pub fn test(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::ItemFn);

    let ret = &input.sig.output;
    let name = &input.sig.ident;
    let body = &input.block;
    let attrs = &input.attrs;

    quote::quote! (
        #[test]
        #[cfg(any(
            feature = "sqlx-mysql",
            feature = "sqlx-sqlite",
            feature = "sqlx-postgres",
        ))]
        #(#attrs)*
        fn #name() #ret {
            let _ = ::tracing_subscriber::fmt()
                .with_max_level(::tracing::Level::DEBUG)
                .with_test_writer()
                .try_init();
            crate::block_on!(async { #body })
        }
    )
    .into()
}

/// Creates a new type that iterates of the variants of an enum.
///
/// Iterate over the variants of an Enum. Any additional data on your variants will be set to `Default::default()`.
/// The macro implements `strum::IntoEnumIterator` on your enum and creates a new type called `YourEnumIter` that is the iterator object.
/// You cannot derive `EnumIter` on any type with a lifetime bound (`<'a>`) because the iterator would surely
/// create [unbounded lifetimes](https://doc.rust-lang.org/nightly/nomicon/unbounded-lifetimes.html).
#[cfg(feature = "strum")]
#[proc_macro_derive(EnumIter, attributes(strum))]
pub fn enum_iter(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    strum::enum_iter::enum_iter_inner(&ast)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

/// Implements traits for types that wrap a database value type.
///
/// This procedure macro implements `From<T> for Value`, `sea_orm::TryGetTable`, and
/// `sea_query::ValueType` for the wrapper type `T`.
///
/// The wrapped type must be `sea_orm::Value` compatible.
///
/// ## Usage
///
/// ```rust
/// use sea_orm::DeriveValueType;
///
/// #[derive(DeriveValueType)]
/// struct MyString(String);
///
/// #[derive(DeriveValueType)]
/// struct MyNumber(i32);
/// ```
///
/// It's also possible to derive value type for enum-strings.
/// Basically the underlying type is String, and the custom must implement methods `to_str` and `from_str`.
///
/// ## Example
///
/// ```rust
/// use sea_orm::{sea_query::ValueTypeErr, DeriveValueType};
///
/// #[derive(DeriveValueType)]
/// #[sea_orm(value_type = "String")]
/// pub enum Tag {
///     Hard,
///     Soft,
/// }
///
/// impl std::fmt::Display for Tag {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         write!(
///             f,
///             "{}",
///             match self {
///                 Self::Hard => "hard",
///                 Self::Soft => "soft",
///             }
///         )
///     }
/// }
///
/// impl std::str::FromStr for Tag {
///     type Err = ValueTypeErr;
///
///     fn from_str(s: &str) -> Result<Self, Self::Err> {
///         Ok(match s {
///             "hard" => Self::Hard,
///             "soft" => Self::Soft,
///             _ => return Err(ValueTypeErr),
///         })
///     }
/// }
/// ```
///
/// `from_str` defaults to `std::str::FromStr::from_str`. `to_str` defaults to `std::string::ToString::to_string`.
/// They can be overridden with custom functions.
///
/// ```rust
/// use sea_orm::{sea_query::ValueTypeErr, DeriveValueType};
///
/// #[derive(DeriveValueType)]
/// #[sea_orm(
///     value_type = "String",
///     from_str = "Tag::from_str",
///     to_str = "Tag::to_str"
/// )]
/// pub enum Tag {
///     Color,
///     Grey,
/// }
///
/// impl Tag {
///     fn to_str(&self) -> &'static str {
///         match self {
///             Self::Color => "color",
///             Self::Grey => "grey",
///         }
///     }
///
///     fn from_str(s: &str) -> Result<Self, ValueTypeErr> {
///         Ok(match s {
///             "color" => Self::Color,
///             "grey" => Self::Grey,
///             _ => return Err(ValueTypeErr),
///         })
///     }
/// }
/// ```
#[cfg(feature = "derive")]
#[proc_macro_derive(DeriveValueType, attributes(sea_orm))]
pub fn derive_value_type(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    match derives::expand_derive_value_type(derive_input) {
        Ok(token_stream) => token_stream.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[cfg(feature = "derive")]
#[proc_macro_derive(DeriveDisplay, attributes(sea_orm))]
pub fn derive_active_enum_display(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match derives::expand_derive_active_enum_display(input) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

/// The DeriveIden derive macro will implement `sea_orm::sea_query::Iden` for simplify Iden implementation.
///
/// ## Usage
///
/// ```rust
/// use sea_orm::{DeriveIden, Iden};
///
/// #[derive(DeriveIden)]
/// pub enum MyClass {
///     Table, // this is a special case, which maps to the enum's name
///     Id,
///     #[sea_orm(iden = "turtle")]
///     Title,
///     Text,
/// }
///
/// #[derive(DeriveIden)]
/// struct MyOther;
///
/// assert_eq!(MyClass::Table.to_string(), "my_class");
/// assert_eq!(MyClass::Id.to_string(), "id");
/// assert_eq!(MyClass::Title.to_string(), "turtle"); // renamed!
/// assert_eq!(MyClass::Text.to_string(), "text");
/// assert_eq!(MyOther.to_string(), "my_other");
/// ```
#[cfg(feature = "derive")]
#[proc_macro_derive(DeriveIden, attributes(sea_orm))]
pub fn derive_iden(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    match derives::expand_derive_iden(derive_input) {
        Ok(token_stream) => token_stream.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

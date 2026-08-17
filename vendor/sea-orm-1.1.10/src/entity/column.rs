use crate::{DbBackend, EntityName, Iden, IdenStatic, IntoSimpleExpr, Iterable};
use sea_query::{
    Alias, BinOper, DynIden, Expr, IntoIden, IntoLikeExpr, SeaRc, SelectStatement, SimpleExpr,
    Value,
};
use std::str::FromStr;

// The original `sea_orm::ColumnType` enum was dropped since 0.11.0
// It was replaced by `sea_query::ColumnType`, we reexport it here to keep the `ColumnType` symbol
pub use sea_query::ColumnType;

/// Defines a Column for an Entity
#[derive(Debug, Clone, PartialEq)]
pub struct ColumnDef {
    pub(crate) col_type: ColumnType,
    pub(crate) null: bool,
    pub(crate) unique: bool,
    pub(crate) indexed: bool,
    pub(crate) default: Option<SimpleExpr>,
    pub(crate) comment: Option<String>,
}

macro_rules! bind_oper {
    ( $op: ident, $bin_op: ident ) => {
        #[allow(missing_docs)]
        fn $op<V>(&self, v: V) -> SimpleExpr
        where
            V: Into<Value>,
        {
            let expr = self.save_as(Expr::val(v));
            Expr::col((self.entity_name(), *self)).binary(BinOper::$bin_op, expr)
        }
    };
}

macro_rules! bind_func_no_params {
    ( $func: ident ) => {
        /// See also SeaQuery's method with same name.
        fn $func(&self) -> SimpleExpr {
            Expr::col((self.entity_name(), *self)).$func()
        }
    };
}

macro_rules! bind_vec_func {
    ( $func: ident ) => {
        #[allow(missing_docs)]
        #[allow(clippy::wrong_self_convention)]
        fn $func<V, I>(&self, v: I) -> SimpleExpr
        where
            V: Into<Value>,
            I: IntoIterator<Item = V>,
        {
            let v_with_enum_cast = v.into_iter().map(|v| self.save_as(Expr::val(v)));
            Expr::col((self.entity_name(), *self)).$func(v_with_enum_cast)
        }
    };
}

macro_rules! bind_subquery_func {
    ( $func: ident ) => {
        #[allow(clippy::wrong_self_convention)]
        #[allow(missing_docs)]
        fn $func(&self, s: SelectStatement) -> SimpleExpr {
            Expr::col((self.entity_name(), *self)).$func(s)
        }
    };
}

// LINT: when the operand value does not match column type
/// API for working with a `Column`. Mostly a wrapper of the identically named methods in [`sea_query::Expr`]
pub trait ColumnTrait: IdenStatic + Iterable + FromStr {
    #[allow(missing_docs)]
    type EntityName: EntityName;

    /// Define a column for an Entity
    fn def(&self) -> ColumnDef;

    /// Get the enum name of the column type
    fn enum_type_name(&self) -> Option<&'static str> {
        None
    }

    /// Get the name of the entity the column belongs to
    fn entity_name(&self) -> DynIden {
        SeaRc::new(Self::EntityName::default()) as DynIden
    }

    /// get the name of the entity the column belongs to
    fn as_column_ref(&self) -> (DynIden, DynIden) {
        (self.entity_name(), SeaRc::new(*self) as DynIden)
    }

    bind_oper!(eq, Equal);
    bind_oper!(ne, NotEqual);
    bind_oper!(gt, GreaterThan);
    bind_oper!(gte, GreaterThanOrEqual);
    bind_oper!(lt, SmallerThan);
    bind_oper!(lte, SmallerThanOrEqual);

    /// ```
    /// use sea_orm::{entity::*, query::*, tests_cfg::cake, DbBackend};
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .filter(cake::Column::Id.between(2, 3))
    ///         .build(DbBackend::MySql)
    ///         .to_string(),
    ///     "SELECT `cake`.`id`, `cake`.`name` FROM `cake` WHERE `cake`.`id` BETWEEN 2 AND 3"
    /// );
    /// ```
    fn between<V>(&self, a: V, b: V) -> SimpleExpr
    where
        V: Into<Value>,
    {
        Expr::col((self.entity_name(), *self)).between(a, b)
    }

    /// ```
    /// use sea_orm::{entity::*, query::*, tests_cfg::cake, DbBackend};
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .filter(cake::Column::Id.not_between(2, 3))
    ///         .build(DbBackend::MySql)
    ///         .to_string(),
    ///     "SELECT `cake`.`id`, `cake`.`name` FROM `cake` WHERE `cake`.`id` NOT BETWEEN 2 AND 3"
    /// );
    /// ```
    fn not_between<V>(&self, a: V, b: V) -> SimpleExpr
    where
        V: Into<Value>,
    {
        Expr::col((self.entity_name(), *self)).not_between(a, b)
    }

    /// ```
    /// use sea_orm::{entity::*, query::*, tests_cfg::cake, DbBackend};
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .filter(cake::Column::Name.like("cheese"))
    ///         .build(DbBackend::MySql)
    ///         .to_string(),
    ///     "SELECT `cake`.`id`, `cake`.`name` FROM `cake` WHERE `cake`.`name` LIKE 'cheese'"
    /// );
    /// ```
    fn like<T>(&self, s: T) -> SimpleExpr
    where
        T: IntoLikeExpr,
    {
        Expr::col((self.entity_name(), *self)).like(s)
    }

    /// ```
    /// use sea_orm::{entity::*, query::*, tests_cfg::cake, DbBackend};
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .filter(cake::Column::Name.not_like("cheese"))
    ///         .build(DbBackend::MySql)
    ///         .to_string(),
    ///     "SELECT `cake`.`id`, `cake`.`name` FROM `cake` WHERE `cake`.`name` NOT LIKE 'cheese'"
    /// );
    /// ```
    fn not_like<T>(&self, s: T) -> SimpleExpr
    where
        T: IntoLikeExpr,
    {
        Expr::col((self.entity_name(), *self)).not_like(s)
    }

    /// This is a simplified shorthand for a more general `like` method.
    /// Use `like` if you need something more complex, like specifying an escape character.
    ///
    /// ## Examples
    ///
    /// ```
    /// use sea_orm::{entity::*, query::*, tests_cfg::cake, DbBackend};
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .filter(cake::Column::Name.starts_with("cheese"))
    ///         .build(DbBackend::MySql)
    ///         .to_string(),
    ///     "SELECT `cake`.`id`, `cake`.`name` FROM `cake` WHERE `cake`.`name` LIKE 'cheese%'"
    /// );
    /// ```
    fn starts_with<T>(&self, s: T) -> SimpleExpr
    where
        T: Into<String>,
    {
        let pattern = format!("{}%", s.into());
        Expr::col((self.entity_name(), *self)).like(pattern)
    }

    /// This is a simplified shorthand for a more general `like` method.
    /// Use `like` if you need something more complex, like specifying an escape character.
    ///
    /// ## Examples
    ///
    /// ```
    /// use sea_orm::{entity::*, query::*, tests_cfg::cake, DbBackend};
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .filter(cake::Column::Name.ends_with("cheese"))
    ///         .build(DbBackend::MySql)
    ///         .to_string(),
    ///     "SELECT `cake`.`id`, `cake`.`name` FROM `cake` WHERE `cake`.`name` LIKE '%cheese'"
    /// );
    /// ```
    fn ends_with<T>(&self, s: T) -> SimpleExpr
    where
        T: Into<String>,
    {
        let pattern = format!("%{}", s.into());
        Expr::col((self.entity_name(), *self)).like(pattern)
    }

    /// This is a simplified shorthand for a more general `like` method.
    /// Use `like` if you need something more complex, like specifying an escape character.
    ///
    /// ## Examples
    ///
    /// ```
    /// use sea_orm::{entity::*, query::*, tests_cfg::cake, DbBackend};
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .filter(cake::Column::Name.contains("cheese"))
    ///         .build(DbBackend::MySql)
    ///         .to_string(),
    ///     "SELECT `cake`.`id`, `cake`.`name` FROM `cake` WHERE `cake`.`name` LIKE '%cheese%'"
    /// );
    /// ```
    fn contains<T>(&self, s: T) -> SimpleExpr
    where
        T: Into<String>,
    {
        let pattern = format!("%{}%", s.into());
        Expr::col((self.entity_name(), *self)).like(pattern)
    }

    bind_func_no_params!(max);
    bind_func_no_params!(min);
    bind_func_no_params!(sum);
    bind_func_no_params!(count);
    bind_func_no_params!(is_null);
    bind_func_no_params!(is_not_null);

    /// Perform an operation if the column is null
    fn if_null<V>(&self, v: V) -> SimpleExpr
    where
        V: Into<Value>,
    {
        Expr::col((self.entity_name(), *self)).if_null(v)
    }

    bind_vec_func!(is_in);
    bind_vec_func!(is_not_in);

    bind_subquery_func!(in_subquery);
    bind_subquery_func!(not_in_subquery);

    /// Construct a [`SimpleExpr::Column`] wrapped in [`Expr`].
    fn into_expr(self) -> Expr {
        Expr::expr(self.into_simple_expr())
    }

    /// Construct a returning [`Expr`].
    #[allow(clippy::match_single_binding)]
    fn into_returning_expr(self, db_backend: DbBackend) -> Expr {
        match db_backend {
            _ => Expr::col(self),
        }
    }

    /// Cast column expression used in select statement.
    /// It only cast database enum as text if it's an enum column.
    fn select_as(&self, expr: Expr) -> SimpleExpr {
        self.select_enum_as(expr)
    }

    /// Cast enum column as text; do nothing if `self` is not an enum.
    fn select_enum_as(&self, expr: Expr) -> SimpleExpr {
        cast_enum_as(expr, self, |col, _, col_type| {
            let type_name = match col_type {
                ColumnType::Array(_) => TextArray.into_iden(),
                _ => Text.into_iden(),
            };
            col.as_enum(type_name)
        })
    }

    /// Cast value of a column into the correct type for database storage.
    /// It only cast text as enum type if it's an enum column.
    fn save_as(&self, val: Expr) -> SimpleExpr {
        self.save_enum_as(val)
    }

    /// Cast value of an enum column as enum type; do nothing if `self` is not an enum.
    /// Will also transform `Array(Vec<Json>)` into `Json(Vec<Json>)` if the column type is `Json`.
    fn save_enum_as(&self, val: Expr) -> SimpleExpr {
        cast_enum_as(val, self, |col, enum_name, col_type| {
            let type_name = match col_type {
                ColumnType::Array(_) => {
                    Alias::new(format!("{}[]", enum_name.to_string())).into_iden()
                }
                _ => enum_name,
            };
            col.as_enum(type_name)
        })
    }
}

/// SeaORM's utility methods that act on [ColumnType]
pub trait ColumnTypeTrait {
    /// Instantiate a new [ColumnDef]
    fn def(self) -> ColumnDef;

    /// Get the name of the enum if this is a enum column
    fn get_enum_name(&self) -> Option<&DynIden>;
}

impl ColumnTypeTrait for ColumnType {
    fn def(self) -> ColumnDef {
        ColumnDef {
            col_type: self,
            null: false,
            unique: false,
            indexed: false,
            default: None,
            comment: None,
        }
    }

    fn get_enum_name(&self) -> Option<&DynIden> {
        enum_name(self)
    }
}

impl ColumnTypeTrait for ColumnDef {
    fn def(self) -> ColumnDef {
        self
    }

    fn get_enum_name(&self) -> Option<&DynIden> {
        enum_name(&self.col_type)
    }
}

fn enum_name(col_type: &ColumnType) -> Option<&DynIden> {
    match col_type {
        ColumnType::Enum { name, .. } => Some(name),
        ColumnType::Array(col_type) => enum_name(col_type),
        _ => None,
    }
}

impl ColumnDef {
    /// Marks the column as `UNIQUE`
    pub fn unique(mut self) -> Self {
        self.unique = true;
        self
    }
    /// Set column comment
    pub fn comment(mut self, v: &str) -> Self {
        self.comment = Some(v.into());
        self
    }

    /// Mark the column as nullable
    pub fn null(self) -> Self {
        self.nullable()
    }

    /// Mark the column as nullable
    pub fn nullable(mut self) -> Self {
        self.null = true;
        self
    }

    /// Set the `indexed` field  to `true`
    pub fn indexed(mut self) -> Self {
        self.indexed = true;
        self
    }

    /// Set the default value
    pub fn default_value<T>(mut self, value: T) -> Self
    where
        T: Into<Value>,
    {
        self.default = Some(value.into().into());
        self
    }

    /// Set the default value or expression of a column
    pub fn default<T>(mut self, default: T) -> Self
    where
        T: Into<SimpleExpr>,
    {
        self.default = Some(default.into());
        self
    }

    /// Get [ColumnType] as reference
    pub fn get_column_type(&self) -> &ColumnType {
        &self.col_type
    }

    /// Get [Option<SimpleExpr>] as reference
    pub fn get_column_default(&self) -> Option<&SimpleExpr> {
        self.default.as_ref()
    }

    /// Returns true if the column is nullable
    pub fn is_null(&self) -> bool {
        self.null
    }

    /// Returns true if the column is unique
    pub fn is_unique(&self) -> bool {
        self.unique
    }
}

struct Text;
struct TextArray;

impl Iden for Text {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(s, "text").unwrap();
    }
}

impl Iden for TextArray {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(s, "text[]").unwrap();
    }
}

fn cast_enum_as<C, F>(expr: Expr, col: &C, f: F) -> SimpleExpr
where
    C: ColumnTrait,
    F: Fn(Expr, DynIden, &ColumnType) -> SimpleExpr,
{
    let col_def = col.def();
    let col_type = col_def.get_column_type();

    match col_type {
        #[cfg(all(feature = "with-json", feature = "postgres-array"))]
        ColumnType::Json | ColumnType::JsonBinary => {
            use sea_query::ArrayType;
            use serde_json::Value as Json;

            #[allow(clippy::boxed_local)]
            fn unbox<T>(boxed: Box<T>) -> T {
                *boxed
            }

            let expr = expr.into();
            match expr {
                SimpleExpr::Value(Value::Array(ArrayType::Json, Some(json_vec))) => {
                    // flatten Array(Vec<Json>) into Json
                    let json_vec: Vec<Json> = json_vec
                        .into_iter()
                        .filter_map(|val| match val {
                            Value::Json(Some(json)) => Some(unbox(json)),
                            _ => None,
                        })
                        .collect();
                    SimpleExpr::Value(Value::Json(Some(Box::new(json_vec.into()))))
                }
                SimpleExpr::Value(Value::Array(ArrayType::Json, None)) => {
                    SimpleExpr::Value(Value::Json(None))
                }
                _ => expr,
            }
        }
        _ => match col_type.get_enum_name() {
            Some(enum_name) => f(expr, SeaRc::clone(enum_name), col_type),
            None => expr.into(),
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        tests_cfg::*, ColumnTrait, Condition, DbBackend, EntityTrait, QueryFilter, QueryTrait,
    };
    use sea_query::Query;

    #[test]
    fn test_in_subquery_1() {
        assert_eq!(
            cake::Entity::find()
                .filter(
                    Condition::any().add(
                        cake::Column::Id.in_subquery(
                            Query::select()
                                .expr(cake::Column::Id.max())
                                .from(cake::Entity)
                                .to_owned()
                        )
                    )
                )
                .build(DbBackend::MySql)
                .to_string(),
            [
                "SELECT `cake`.`id`, `cake`.`name` FROM `cake`",
                "WHERE `cake`.`id` IN (SELECT MAX(`cake`.`id`) FROM `cake`)",
            ]
            .join(" ")
        );
    }

    #[test]
    fn test_in_subquery_2() {
        assert_eq!(
            cake::Entity::find()
                .filter(
                    Condition::any().add(
                        cake::Column::Id.in_subquery(
                            Query::select()
                                .column(cake_filling::Column::CakeId)
                                .from(cake_filling::Entity)
                                .to_owned()
                        )
                    )
                )
                .build(DbBackend::MySql)
                .to_string(),
            [
                "SELECT `cake`.`id`, `cake`.`name` FROM `cake`",
                "WHERE `cake`.`id` IN (SELECT `cake_id` FROM `cake_filling`)",
            ]
            .join(" ")
        );
    }

    #[test]
    fn test_col_from_str() {
        use std::str::FromStr;

        assert!(matches!(
            fruit::Column::from_str("id"),
            Ok(fruit::Column::Id)
        ));
        assert!(matches!(
            fruit::Column::from_str("name"),
            Ok(fruit::Column::Name)
        ));
        assert!(matches!(
            fruit::Column::from_str("cake_id"),
            Ok(fruit::Column::CakeId)
        ));
        assert!(matches!(
            fruit::Column::from_str("cakeId"),
            Ok(fruit::Column::CakeId)
        ));
        assert!(matches!(
            fruit::Column::from_str("does_not_exist"),
            Err(crate::ColumnFromStrErr(_))
        ));
    }

    #[test]
    #[cfg(feature = "macros")]
    fn entity_model_column_1() {
        use crate::prelude::*;

        mod hello {
            use crate as sea_orm;
            use crate::entity::prelude::*;

            #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
            #[sea_orm(table_name = "hello")]
            pub struct Model {
                #[sea_orm(primary_key)]
                pub id: i32,
                pub one: i32,
                #[sea_orm(unique)]
                pub two: i8,
                #[sea_orm(indexed)]
                pub three: i16,
                #[sea_orm(nullable)]
                pub four: i32,
                #[sea_orm(unique, indexed, nullable)]
                pub five: i64,
                #[sea_orm(unique)]
                pub six: u8,
                #[sea_orm(indexed)]
                pub seven: u16,
                #[sea_orm(nullable)]
                pub eight: u32,
                #[sea_orm(unique, indexed, nullable)]
                pub nine: u64,
                #[sea_orm(default_expr = "Expr::current_timestamp()")]
                pub ten: DateTimeUtc,
                #[sea_orm(default_value = 7)]
                pub eleven: u8,
                #[sea_orm(default_value = "twelve_value")]
                pub twelve: String,
                #[sea_orm(default_expr = "\"twelve_value\"")]
                pub twelve_two: String,
            }

            #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
            pub enum Relation {}

            impl ActiveModelBehavior for ActiveModel {}
        }

        assert_eq!(hello::Column::One.def(), ColumnType::Integer.def());
        assert_eq!(
            hello::Column::Two.def(),
            ColumnType::TinyInteger.def().unique()
        );
        assert_eq!(
            hello::Column::Three.def(),
            ColumnType::SmallInteger.def().indexed()
        );
        assert_eq!(
            hello::Column::Four.def(),
            ColumnType::Integer.def().nullable()
        );
        assert_eq!(
            hello::Column::Five.def(),
            ColumnType::BigInteger.def().unique().indexed().nullable()
        );
        assert_eq!(
            hello::Column::Six.def(),
            ColumnType::TinyUnsigned.def().unique()
        );
        assert_eq!(
            hello::Column::Seven.def(),
            ColumnType::SmallUnsigned.def().indexed()
        );
        assert_eq!(
            hello::Column::Eight.def(),
            ColumnType::Unsigned.def().nullable()
        );
        assert_eq!(
            hello::Column::Nine.def(),
            ColumnType::BigUnsigned.def().unique().indexed().nullable()
        );
        assert_eq!(
            hello::Column::Ten.def(),
            ColumnType::TimestampWithTimeZone
                .def()
                .default(Expr::current_timestamp())
        );
        assert_eq!(
            hello::Column::Eleven.def(),
            ColumnType::TinyUnsigned.def().default(7)
        );
        assert_eq!(
            hello::Column::Twelve.def(),
            ColumnType::String(StringLen::None)
                .def()
                .default("twelve_value")
        );
        assert_eq!(
            hello::Column::TwelveTwo.def(),
            ColumnType::String(StringLen::None)
                .def()
                .default("twelve_value")
        );
    }

    #[test]
    #[cfg(feature = "macros")]
    fn column_name_1() {
        use sea_query::Iden;

        mod hello {
            use crate as sea_orm;
            use crate::entity::prelude::*;

            #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
            #[sea_orm(table_name = "hello")]
            pub struct Model {
                #[sea_orm(primary_key)]
                pub id: i32,
                #[sea_orm(column_name = "ONE")]
                pub one: i32,
                pub two: i32,
                #[sea_orm(column_name = "3")]
                pub three: i32,
            }

            #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
            pub enum Relation {}

            impl ActiveModelBehavior for ActiveModel {}
        }

        assert_eq!(hello::Column::One.to_string().as_str(), "ONE");
        assert_eq!(hello::Column::Two.to_string().as_str(), "two");
        assert_eq!(hello::Column::Three.to_string().as_str(), "3");
    }

    #[test]
    #[cfg(feature = "macros")]
    fn column_name_2() {
        use sea_query::Iden;

        mod hello {
            use crate as sea_orm;
            use crate::entity::prelude::*;

            #[derive(Copy, Clone, Default, Debug, DeriveEntity)]
            pub struct Entity;

            impl EntityName for Entity {
                fn table_name(&self) -> &str {
                    "hello"
                }
            }

            #[derive(Clone, Debug, PartialEq, Eq, DeriveModel, DeriveActiveModel)]
            pub struct Model {
                pub id: i32,
                pub one: i32,
                pub two: i32,
                pub three: i32,
            }

            #[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
            pub enum Column {
                Id,
                #[sea_orm(column_name = "ONE")]
                One,
                Two,
                #[sea_orm(column_name = "3")]
                Three,
            }

            impl ColumnTrait for Column {
                type EntityName = Entity;

                fn def(&self) -> ColumnDef {
                    match self {
                        Column::Id => ColumnType::Integer.def(),
                        Column::One => ColumnType::Integer.def(),
                        Column::Two => ColumnType::Integer.def(),
                        Column::Three => ColumnType::Integer.def(),
                    }
                }
            }

            #[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
            pub enum PrimaryKey {
                Id,
            }

            impl PrimaryKeyTrait for PrimaryKey {
                type ValueType = i32;

                fn auto_increment() -> bool {
                    true
                }
            }

            #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
            pub enum Relation {}

            impl ActiveModelBehavior for ActiveModel {}
        }

        assert_eq!(hello::Column::One.to_string().as_str(), "ONE");
        assert_eq!(hello::Column::Two.to_string().as_str(), "two");
        assert_eq!(hello::Column::Three.to_string().as_str(), "3");
    }

    #[test]
    #[cfg(feature = "macros")]
    fn enum_name_1() {
        use sea_query::Iden;

        mod hello {
            use crate as sea_orm;
            use crate::entity::prelude::*;

            #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
            #[sea_orm(table_name = "hello")]
            pub struct Model {
                #[sea_orm(primary_key)]
                pub id: i32,
                #[sea_orm(enum_name = "One1")]
                pub one: i32,
                pub two: i32,
                #[sea_orm(enum_name = "Three3")]
                pub three: i32,
            }

            #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
            pub enum Relation {}

            impl ActiveModelBehavior for ActiveModel {}
        }

        assert_eq!(hello::Column::One1.to_string().as_str(), "one1");
        assert_eq!(hello::Column::Two.to_string().as_str(), "two");
        assert_eq!(hello::Column::Three3.to_string().as_str(), "three3");
    }

    #[test]
    #[cfg(feature = "macros")]
    fn enum_name_2() {
        use sea_query::Iden;

        mod hello {
            use crate as sea_orm;
            use crate::entity::prelude::*;

            #[derive(Copy, Clone, Default, Debug, DeriveEntity)]
            pub struct Entity;

            impl EntityName for Entity {
                fn table_name(&self) -> &str {
                    "hello"
                }
            }

            #[derive(Clone, Debug, PartialEq, Eq, DeriveModel, DeriveActiveModel)]
            pub struct Model {
                pub id: i32,
                #[sea_orm(enum_name = "One1")]
                pub one: i32,
                pub two: i32,
                #[sea_orm(enum_name = "Three3")]
                pub three: i32,
            }

            #[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
            pub enum Column {
                Id,
                One1,
                Two,
                Three3,
            }

            impl ColumnTrait for Column {
                type EntityName = Entity;

                fn def(&self) -> ColumnDef {
                    match self {
                        Column::Id => ColumnType::Integer.def(),
                        Column::One1 => ColumnType::Integer.def(),
                        Column::Two => ColumnType::Integer.def(),
                        Column::Three3 => ColumnType::Integer.def(),
                    }
                }
            }

            #[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
            pub enum PrimaryKey {
                Id,
            }

            impl PrimaryKeyTrait for PrimaryKey {
                type ValueType = i32;

                fn auto_increment() -> bool {
                    true
                }
            }

            #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
            pub enum Relation {}

            impl ActiveModelBehavior for ActiveModel {}
        }

        assert_eq!(hello::Column::One1.to_string().as_str(), "one1");
        assert_eq!(hello::Column::Two.to_string().as_str(), "two");
        assert_eq!(hello::Column::Three3.to_string().as_str(), "three3");
    }

    #[test]
    #[cfg(feature = "macros")]
    fn column_name_enum_name_1() {
        use sea_query::Iden;

        #[allow(clippy::enum_variant_names)]
        mod hello {
            use crate as sea_orm;
            use crate::entity::prelude::*;

            #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
            #[sea_orm(table_name = "hello")]
            pub struct Model {
                #[sea_orm(primary_key, column_name = "ID", enum_name = "IdentityColumn")]
                pub id: i32,
                #[sea_orm(column_name = "ONE", enum_name = "One1")]
                pub one: i32,
                pub two: i32,
                #[sea_orm(column_name = "THREE", enum_name = "Three3")]
                pub three: i32,
            }

            #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
            pub enum Relation {}

            impl ActiveModelBehavior for ActiveModel {}
        }

        assert_eq!(hello::Column::IdentityColumn.to_string().as_str(), "ID");
        assert_eq!(hello::Column::One1.to_string().as_str(), "ONE");
        assert_eq!(hello::Column::Two.to_string().as_str(), "two");
        assert_eq!(hello::Column::Three3.to_string().as_str(), "THREE");
    }

    #[test]
    #[cfg(feature = "macros")]
    fn column_name_enum_name_2() {
        use sea_query::Iden;

        mod hello {
            use crate as sea_orm;
            use crate::entity::prelude::*;

            #[derive(Copy, Clone, Default, Debug, DeriveEntity)]
            pub struct Entity;

            impl EntityName for Entity {
                fn table_name(&self) -> &str {
                    "hello"
                }
            }

            #[derive(Clone, Debug, PartialEq, Eq, DeriveModel, DeriveActiveModel)]
            pub struct Model {
                #[sea_orm(enum_name = "IdentityCol")]
                pub id: i32,
                #[sea_orm(enum_name = "One1")]
                pub one: i32,
                pub two: i32,
                #[sea_orm(enum_name = "Three3")]
                pub three: i32,
            }

            #[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
            pub enum Column {
                #[sea_orm(column_name = "ID")]
                IdentityCol,
                #[sea_orm(column_name = "ONE")]
                One1,
                Two,
                #[sea_orm(column_name = "THREE")]
                Three3,
            }

            impl ColumnTrait for Column {
                type EntityName = Entity;

                fn def(&self) -> ColumnDef {
                    match self {
                        Column::IdentityCol => ColumnType::Integer.def(),
                        Column::One1 => ColumnType::Integer.def(),
                        Column::Two => ColumnType::Integer.def(),
                        Column::Three3 => ColumnType::Integer.def(),
                    }
                }
            }

            #[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
            pub enum PrimaryKey {
                IdentityCol,
            }

            impl PrimaryKeyTrait for PrimaryKey {
                type ValueType = i32;

                fn auto_increment() -> bool {
                    true
                }
            }

            #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
            pub enum Relation {}

            impl ActiveModelBehavior for ActiveModel {}
        }

        assert_eq!(hello::Column::IdentityCol.to_string().as_str(), "ID");
        assert_eq!(hello::Column::One1.to_string().as_str(), "ONE");
        assert_eq!(hello::Column::Two.to_string().as_str(), "two");
        assert_eq!(hello::Column::Three3.to_string().as_str(), "THREE");
    }

    #[test]
    #[cfg(feature = "macros")]
    fn column_name_enum_name_3() {
        use sea_query::Iden;

        mod my_entity {
            use crate as sea_orm;
            use crate::entity::prelude::*;

            #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
            #[sea_orm(table_name = "my_entity")]
            pub struct Model {
                #[sea_orm(primary_key, enum_name = "IdentityColumn", column_name = "id")]
                pub id: i32,
                #[sea_orm(column_name = "type")]
                pub type_: String,
            }

            #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
            pub enum Relation {}

            impl ActiveModelBehavior for ActiveModel {}
        }

        assert_eq!(my_entity::Column::IdentityColumn.to_string().as_str(), "id");
        assert_eq!(my_entity::Column::Type.to_string().as_str(), "type");
    }

    #[test]
    #[cfg(feature = "macros")]
    fn select_as_1() {
        use crate::{ActiveModelTrait, ActiveValue, Update};

        mod hello_expanded {
            use crate as sea_orm;
            use crate::entity::prelude::*;
            use crate::sea_query::{Alias, Expr, SimpleExpr};

            #[derive(Copy, Clone, Default, Debug, DeriveEntity)]
            pub struct Entity;

            impl EntityName for Entity {
                fn table_name(&self) -> &str {
                    "hello"
                }
            }

            #[derive(Clone, Debug, PartialEq, Eq, DeriveModel, DeriveActiveModel)]
            pub struct Model {
                pub id: i32,
                #[sea_orm(enum_name = "One1")]
                pub one: i32,
                pub two: i32,
                #[sea_orm(enum_name = "Three3")]
                pub three: i32,
            }

            #[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
            pub enum Column {
                Id,
                One1,
                Two,
                Three3,
            }

            impl ColumnTrait for Column {
                type EntityName = Entity;

                fn def(&self) -> ColumnDef {
                    match self {
                        Column::Id => ColumnType::Integer.def(),
                        Column::One1 => ColumnType::Integer.def(),
                        Column::Two => ColumnType::Integer.def(),
                        Column::Three3 => ColumnType::Integer.def(),
                    }
                }

                fn select_as(&self, expr: Expr) -> SimpleExpr {
                    match self {
                        Self::Two => expr.cast_as(Alias::new("integer")),
                        _ => self.select_enum_as(expr),
                    }
                }
            }

            #[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
            pub enum PrimaryKey {
                Id,
            }

            impl PrimaryKeyTrait for PrimaryKey {
                type ValueType = i32;

                fn auto_increment() -> bool {
                    true
                }
            }

            #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
            pub enum Relation {}

            impl ActiveModelBehavior for ActiveModel {}
        }

        #[allow(clippy::enum_variant_names)]
        mod hello_compact {
            use crate as sea_orm;
            use crate::entity::prelude::*;

            #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
            #[sea_orm(table_name = "hello")]
            pub struct Model {
                #[sea_orm(primary_key)]
                pub id: i32,
                #[sea_orm(enum_name = "One1")]
                pub one: i32,
                #[sea_orm(select_as = "integer")]
                pub two: i32,
                #[sea_orm(enum_name = "Three3")]
                pub three: i32,
            }

            #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
            pub enum Relation {}

            impl ActiveModelBehavior for ActiveModel {}
        }

        fn assert_it<E, A>(active_model: A)
        where
            E: EntityTrait,
            A: ActiveModelTrait<Entity = E>,
        {
            assert_eq!(
                E::find().build(DbBackend::Postgres).to_string(),
                r#"SELECT "hello"."id", "hello"."one1", CAST("hello"."two" AS integer), "hello"."three3" FROM "hello""#,
            );
            assert_eq!(
                Update::one(active_model)
                    .build(DbBackend::Postgres)
                    .to_string(),
                r#"UPDATE "hello" SET "one1" = 1, "two" = 2, "three3" = 3 WHERE "hello"."id" = 1"#,
            );
        }

        assert_it(hello_expanded::ActiveModel {
            id: ActiveValue::set(1),
            one: ActiveValue::set(1),
            two: ActiveValue::set(2),
            three: ActiveValue::set(3),
        });
        assert_it(hello_compact::ActiveModel {
            id: ActiveValue::set(1),
            one: ActiveValue::set(1),
            two: ActiveValue::set(2),
            three: ActiveValue::set(3),
        });
    }

    #[test]
    #[cfg(feature = "macros")]
    fn save_as_1() {
        use crate::{ActiveModelTrait, ActiveValue, Update};

        mod hello_expanded {
            use crate as sea_orm;
            use crate::entity::prelude::*;
            use crate::sea_query::{Alias, Expr, SimpleExpr};

            #[derive(Copy, Clone, Default, Debug, DeriveEntity)]
            pub struct Entity;

            impl EntityName for Entity {
                fn table_name(&self) -> &str {
                    "hello"
                }
            }

            #[derive(Clone, Debug, PartialEq, Eq, DeriveModel, DeriveActiveModel)]
            pub struct Model {
                pub id: i32,
                #[sea_orm(enum_name = "One1")]
                pub one: i32,
                pub two: i32,
                #[sea_orm(enum_name = "Three3")]
                pub three: i32,
            }

            #[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
            pub enum Column {
                Id,
                One1,
                Two,
                Three3,
            }

            impl ColumnTrait for Column {
                type EntityName = Entity;

                fn def(&self) -> ColumnDef {
                    match self {
                        Column::Id => ColumnType::Integer.def(),
                        Column::One1 => ColumnType::Integer.def(),
                        Column::Two => ColumnType::Integer.def(),
                        Column::Three3 => ColumnType::Integer.def(),
                    }
                }

                fn save_as(&self, val: Expr) -> SimpleExpr {
                    match self {
                        Self::Two => val.cast_as(Alias::new("text")),
                        _ => self.save_enum_as(val),
                    }
                }
            }

            #[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
            pub enum PrimaryKey {
                Id,
            }

            impl PrimaryKeyTrait for PrimaryKey {
                type ValueType = i32;

                fn auto_increment() -> bool {
                    true
                }
            }

            #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
            pub enum Relation {}

            impl ActiveModelBehavior for ActiveModel {}
        }

        #[allow(clippy::enum_variant_names)]
        mod hello_compact {
            use crate as sea_orm;
            use crate::entity::prelude::*;

            #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
            #[sea_orm(table_name = "hello")]
            pub struct Model {
                #[sea_orm(primary_key)]
                pub id: i32,
                #[sea_orm(enum_name = "One1")]
                pub one: i32,
                #[sea_orm(save_as = "text")]
                pub two: i32,
                #[sea_orm(enum_name = "Three3")]
                pub three: i32,
            }

            #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
            pub enum Relation {}

            impl ActiveModelBehavior for ActiveModel {}
        }

        fn assert_it<E, A>(active_model: A)
        where
            E: EntityTrait,
            A: ActiveModelTrait<Entity = E>,
        {
            assert_eq!(
                E::find().build(DbBackend::Postgres).to_string(),
                r#"SELECT "hello"."id", "hello"."one1", "hello"."two", "hello"."three3" FROM "hello""#,
            );
            assert_eq!(
                Update::one(active_model)
                    .build(DbBackend::Postgres)
                    .to_string(),
                r#"UPDATE "hello" SET "one1" = 1, "two" = CAST(2 AS text), "three3" = 3 WHERE "hello"."id" = 1"#,
            );
        }

        assert_it(hello_expanded::ActiveModel {
            id: ActiveValue::set(1),
            one: ActiveValue::set(1),
            two: ActiveValue::set(2),
            three: ActiveValue::set(3),
        });
        assert_it(hello_compact::ActiveModel {
            id: ActiveValue::set(1),
            one: ActiveValue::set(1),
            two: ActiveValue::set(2),
            three: ActiveValue::set(3),
        });
    }

    #[test]
    #[cfg(feature = "macros")]
    fn select_as_and_value_1() {
        use crate::{ActiveModelTrait, ActiveValue, Update};

        mod hello_expanded {
            use crate as sea_orm;
            use crate::entity::prelude::*;
            use crate::sea_query::{Alias, Expr, SimpleExpr};

            #[derive(Copy, Clone, Default, Debug, DeriveEntity)]
            pub struct Entity;

            impl EntityName for Entity {
                fn table_name(&self) -> &str {
                    "hello"
                }
            }

            #[derive(Clone, Debug, PartialEq, Eq, DeriveModel, DeriveActiveModel)]
            pub struct Model {
                pub id: i32,
                #[sea_orm(enum_name = "One1")]
                pub one: i32,
                pub two: i32,
                #[sea_orm(enum_name = "Three3")]
                pub three: i32,
            }

            #[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
            pub enum Column {
                Id,
                One1,
                Two,
                Three3,
            }

            impl ColumnTrait for Column {
                type EntityName = Entity;

                fn def(&self) -> ColumnDef {
                    match self {
                        Column::Id => ColumnType::Integer.def(),
                        Column::One1 => ColumnType::Integer.def(),
                        Column::Two => ColumnType::Integer.def(),
                        Column::Three3 => ColumnType::Integer.def(),
                    }
                }

                fn select_as(&self, expr: Expr) -> SimpleExpr {
                    match self {
                        Self::Two => expr.cast_as(Alias::new("integer")),
                        _ => self.select_enum_as(expr),
                    }
                }

                fn save_as(&self, val: Expr) -> SimpleExpr {
                    match self {
                        Self::Two => val.cast_as(Alias::new("text")),
                        _ => self.save_enum_as(val),
                    }
                }
            }

            #[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
            pub enum PrimaryKey {
                Id,
            }

            impl PrimaryKeyTrait for PrimaryKey {
                type ValueType = i32;

                fn auto_increment() -> bool {
                    true
                }
            }

            #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
            pub enum Relation {}

            impl ActiveModelBehavior for ActiveModel {}
        }

        #[allow(clippy::enum_variant_names)]
        mod hello_compact {
            use crate as sea_orm;
            use crate::entity::prelude::*;

            #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
            #[sea_orm(table_name = "hello")]
            pub struct Model {
                #[sea_orm(primary_key)]
                pub id: i32,
                #[sea_orm(enum_name = "One1")]
                pub one: i32,
                #[sea_orm(select_as = "integer", save_as = "text")]
                pub two: i32,
                #[sea_orm(enum_name = "Three3")]
                pub three: i32,
            }

            #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
            pub enum Relation {}

            impl ActiveModelBehavior for ActiveModel {}
        }

        fn assert_it<E, A>(active_model: A)
        where
            E: EntityTrait,
            A: ActiveModelTrait<Entity = E>,
        {
            assert_eq!(
                E::find().build(DbBackend::Postgres).to_string(),
                r#"SELECT "hello"."id", "hello"."one1", CAST("hello"."two" AS integer), "hello"."three3" FROM "hello""#,
            );
            assert_eq!(
                Update::one(active_model)
                    .build(DbBackend::Postgres)
                    .to_string(),
                r#"UPDATE "hello" SET "one1" = 1, "two" = CAST(2 AS text), "three3" = 3 WHERE "hello"."id" = 1"#,
            );
        }

        assert_it(hello_expanded::ActiveModel {
            id: ActiveValue::set(1),
            one: ActiveValue::set(1),
            two: ActiveValue::set(2),
            three: ActiveValue::set(3),
        });
        assert_it(hello_compact::ActiveModel {
            id: ActiveValue::set(1),
            one: ActiveValue::set(1),
            two: ActiveValue::set(2),
            three: ActiveValue::set(3),
        });
    }
}

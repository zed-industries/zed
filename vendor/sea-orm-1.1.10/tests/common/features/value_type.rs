pub mod value_type_general {
    use super::*;
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "value_type")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i32,
        pub number: Integer,
        pub tag_1: Tag1,
        pub tag_2: Tag2,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

pub mod value_type_pg {
    use super::*;
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "value_type_postgres")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i32,
        pub number: Integer,
        pub str_vec: StringVec,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveValueType)]
pub struct Integer(i32);

impl<T> From<T> for Integer
where
    T: Into<i32>,
{
    fn from(v: T) -> Integer {
        Integer(v.into())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveValueType)]
pub struct StringVec(pub Vec<String>);

#[derive(Copy, Clone, Debug, PartialEq, Eq, DeriveValueType)]
#[sea_orm(value_type = "String")]
pub enum Tag1 {
    Hard,
    Soft,
}

impl std::fmt::Display for Tag1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Hard => "hard",
                Self::Soft => "soft",
            }
        )
    }
}

impl std::str::FromStr for Tag1 {
    type Err = sea_query::ValueTypeErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "hard" => Self::Hard,
            "soft" => Self::Soft,
            _ => return Err(sea_query::ValueTypeErr),
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, DeriveValueType)]
#[sea_orm(
    value_type = "String",
    from_str = "Tag2::from_str",
    to_str = "Tag2::to_str"
)]
pub enum Tag2 {
    Color,
    Grey,
}

impl Tag2 {
    fn to_str(&self) -> &'static str {
        match self {
            Self::Color => "color",
            Self::Grey => "grey",
        }
    }

    fn from_str(s: &str) -> Result<Self, sea_query::ValueTypeErr> {
        Ok(match s {
            "color" => Self::Color,
            "grey" => Self::Grey,
            _ => return Err(sea_query::ValueTypeErr),
        })
    }
}

use sea_orm::entity::prelude::*;
use sea_orm::TryGetableFromJson;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "json_vec")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub str_vec: Option<StringVec>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StringVec(pub Vec<String>);

impl TryGetableFromJson for StringVec {}

impl From<StringVec> for Value {
    fn from(source: StringVec) -> Self {
        sea_orm::Value::Json(serde_json::to_value(source).ok().map(std::boxed::Box::new))
    }
}

impl sea_query::ValueType for StringVec {
    fn try_from(v: Value) -> Result<Self, sea_query::ValueTypeErr> {
        match v {
            sea_orm::Value::Json(Some(json)) => {
                Ok(serde_json::from_value(*json).map_err(|_| sea_orm::sea_query::ValueTypeErr)?)
            }
            _ => Err(sea_orm::sea_query::ValueTypeErr),
        }
    }

    fn type_name() -> String {
        stringify!(StringVec).to_owned()
    }

    fn array_type() -> sea_orm::sea_query::ArrayType {
        sea_orm::sea_query::ArrayType::Json
    }

    fn column_type() -> sea_query::ColumnType {
        sea_query::ColumnType::Json
    }
}

impl sea_orm::sea_query::Nullable for StringVec {
    fn null() -> sea_orm::Value {
        sea_orm::Value::Json(None)
    }
}

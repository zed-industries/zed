use sea_orm::entity::prelude::*;
use sea_orm::{
    sea_query::{ArrayType, ColumnType, ValueType},
    TryGetError, TryGetable,
};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "event_trigger")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub events: Events,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Event(pub String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Events(pub Vec<Event>);

impl From<Events> for Value {
    fn from(events: Events) -> Self {
        let Events(events) = events;
        Value::Array(
            ArrayType::String,
            Some(Box::new(
                events
                    .into_iter()
                    .map(|Event(s)| Value::String(Some(Box::new(s))))
                    .collect(),
            )),
        )
    }
}

impl TryGetable for Events {
    fn try_get_by<I: sea_orm::ColIdx>(res: &QueryResult, idx: I) -> Result<Self, TryGetError> {
        let vec: Vec<String> = res.try_get_by(idx).map_err(TryGetError::DbErr)?;
        Ok(Events(vec.into_iter().map(Event).collect()))
    }
}

impl ValueType for Events {
    fn try_from(v: Value) -> Result<Self, sea_query::ValueTypeErr> {
        let value: Option<Vec<String>> =
            v.expect("This Value::Array should consist of Value::String");
        let vec = match value {
            Some(v) => v.into_iter().map(Event).collect(),
            None => vec![],
        };
        Ok(Events(vec))
    }

    fn type_name() -> String {
        stringify!(Events).to_owned()
    }

    fn array_type() -> ArrayType {
        ArrayType::String
    }

    fn column_type() -> ColumnType {
        ColumnType::Array(RcOrArc::new(ColumnType::String(StringLen::None)))
    }
}

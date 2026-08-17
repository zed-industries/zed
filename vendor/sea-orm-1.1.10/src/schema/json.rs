use crate::{ColumnTrait, ColumnType, EntityTrait, Iden, Iterable, Schema};
use serde_json::{Map, Value};

impl Schema {
    /// Construct a schema description in json for the given Entity.
    pub fn json_schema_from_entity<E>(&self, entity: E) -> Value
    where
        E: EntityTrait,
    {
        json_schema_from_entity(entity)
    }
}

pub(crate) fn json_schema_from_entity<E>(entity: E) -> Value
where
    E: EntityTrait,
{
    let mut obj = Map::new();
    let mut cols = Vec::new();

    if let Some(comment) = entity.comment() {
        obj.insert("comment".to_owned(), Value::String(comment.to_owned()));
    }

    for column in E::Column::iter() {
        let col = json_schema_from_entity_column::<E>(column);
        cols.push(col);
    }
    obj.insert("columns".to_owned(), Value::Array(cols));

    let mut pk = Vec::new();
    for col in E::PrimaryKey::iter() {
        pk.push(Value::String(col.to_string()));
    }
    obj.insert("primary_key".to_owned(), Value::Array(pk));

    Value::Object(obj)
}

fn json_schema_from_entity_column<E>(column: E::Column) -> Value
where
    E: EntityTrait,
{
    let mut obj = Map::new();

    let column_def = column.def();
    obj.insert("name".to_owned(), Value::String(column.to_string()));
    obj.insert(
        "type".to_owned(),
        type_def_from_column_def(&column_def.col_type),
    );
    obj.insert("nullable".to_owned(), Value::Bool(column_def.null));
    if column_def.unique {
        obj.insert("unique".to_owned(), Value::Bool(true));
    }
    if let Some(comment) = column_def.comment {
        obj.insert("comment".to_owned(), Value::String(comment));
    }

    Value::Object(obj)
}

fn type_def_from_column_def(column_type: &ColumnType) -> Value {
    match column_type {
        ColumnType::Char(_) | ColumnType::String(_) | ColumnType::Text => {
            Value::String("string".to_owned())
        }
        ColumnType::TinyInteger
        | ColumnType::SmallInteger
        | ColumnType::Integer
        | ColumnType::BigInteger
        | ColumnType::TinyUnsigned
        | ColumnType::SmallUnsigned
        | ColumnType::Unsigned
        | ColumnType::BigUnsigned => Value::String("integer".to_owned()),
        ColumnType::Float | ColumnType::Double => Value::String("real".to_owned()),
        ColumnType::Decimal(_) | ColumnType::Money(_) => Value::String("decimal".to_owned()),
        ColumnType::DateTime | ColumnType::Timestamp | ColumnType::TimestampWithTimeZone => {
            Value::String("datetime".to_owned())
        }
        ColumnType::Time => Value::String("time".to_owned()),
        ColumnType::Date => Value::String("date".to_owned()),
        ColumnType::Year => Value::String("year".to_owned()),
        ColumnType::Binary(_)
        | ColumnType::VarBinary(_)
        | ColumnType::Bit(_)
        | ColumnType::VarBit(_) => Value::String("binary".to_owned()),
        ColumnType::Boolean => Value::String("bool".to_owned()),
        ColumnType::Json | ColumnType::JsonBinary => Value::String("json".to_owned()),
        ColumnType::Uuid => Value::String("uuid".to_owned()),
        ColumnType::Custom(typename) => Value::String(typename.to_string()),
        ColumnType::Enum { name, variants } => {
            let mut enum_def = Map::new();
            enum_def.insert("name".to_owned(), Value::String(name.to_string()));
            let variants: Vec<Value> = variants
                .iter()
                .map(|v| Value::String(v.to_string()))
                .collect();
            enum_def.insert("variants".to_owned(), Value::Array(variants));
            Value::Object(enum_def)
        }
        ColumnType::Array(inner) => {
            let mut obj = Map::new();
            obj.insert("array".to_owned(), type_def_from_column_def(inner));
            Value::Object(obj)
        }
        _ => Value::String("other".to_owned()),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        tests_cfg::{cake, lunch_set},
        DbBackend,
    };

    #[test]
    fn test_json_schema_from_entity() {
        let json = Schema::new(DbBackend::MySql).json_schema_from_entity(cake::Entity);
        println!("{}", serde_json::to_string_pretty(&json).unwrap());
        assert_eq!(
            json,
            serde_json::from_str::<Value>(
                r#"{
                "columns": [
                    {
                        "name": "id",
                        "nullable": false,
                        "type": "integer"
                    },
                    {
                        "name": "name",
                        "nullable": false,
                        "type": "string"
                    }
                ],
                "primary_key": [
                    "id"
                ]
            }"#
            )
            .unwrap()
        );

        let json = Schema::new(DbBackend::MySql).json_schema_from_entity(lunch_set::Entity);
        println!("{}", serde_json::to_string_pretty(&json).unwrap());
        assert_eq!(
            json,
            serde_json::from_str::<Value>(
                r#"{
                "columns": [
                    {
                        "name": "id",
                        "nullable": false,
                        "type": "integer"
                    },
                    {
                        "name": "name",
                        "nullable": false,
                        "type": "string"
                    },
                    {
                        "name": "tea",
                        "nullable": false,
                        "type": {
                            "name": "tea",
                            "variants": [
                                "EverydayTea",
                                "BreakfastTea"
                            ]
                        }
                    }
                ],
                "primary_key": [
                    "id"
                ]
            }"#
            )
            .unwrap()
        );
    }
}

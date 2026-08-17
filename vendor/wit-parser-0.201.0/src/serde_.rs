use crate::{Params, Type};
use id_arena::{Arena, Id};
use indexmap::IndexMap;
use serde::ser::{SerializeMap, SerializeSeq, Serializer};
use serde::Serialize;

pub fn serialize_none<S>(serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_none()
}

pub fn serialize_arena<T, S>(arena: &Arena<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    T: Serialize,
    S: Serializer,
{
    let mut seq = serializer.serialize_seq(Some(arena.len()))?;
    for (_, item) in arena.iter() {
        seq.serialize_element(&item)?;
    }
    seq.end()
}

pub fn serialize_id<T, S>(id: &Id<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_u64(id.index() as u64)
}

pub fn serialize_optional_id<T, S>(id: &Option<Id<T>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match id {
        Some(id) => serialize_id(&id, serializer),
        None => serializer.serialize_none(),
    }
}

pub fn serialize_id_map<K, T, S>(map: &IndexMap<K, Id<T>>, serializer: S) -> Result<S::Ok, S::Error>
where
    K: Serialize,
    S: Serializer,
{
    let mut s = serializer.serialize_map(Some(map.len()))?;
    for (key, id) in map.iter() {
        s.serialize_entry(key, &(id.index() as u64))?;
    }
    s.end()
}

impl Serialize for Type {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Type::Bool => serializer.serialize_str("bool"),
            Type::U8 => serializer.serialize_str("u8"),
            Type::U16 => serializer.serialize_str("u16"),
            Type::U32 => serializer.serialize_str("u32"),
            Type::U64 => serializer.serialize_str("u64"),
            Type::S8 => serializer.serialize_str("s8"),
            Type::S16 => serializer.serialize_str("s16"),
            Type::S32 => serializer.serialize_str("s32"),
            Type::S64 => serializer.serialize_str("s64"),
            Type::Float32 => serializer.serialize_str("float32"),
            Type::Float64 => serializer.serialize_str("float64"),
            Type::Char => serializer.serialize_str("char"),
            Type::String => serializer.serialize_str("string"),
            Type::Id(type_id) => serializer.serialize_u64(type_id.index() as u64),
        }
    }
}

pub fn serialize_anon_result<S>(typ: &Type, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let params: Params = vec![(String::default(), *typ)];
    serialize_params(&params, serializer)
}

pub fn serialize_params<S>(params: &Params, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut seq = serializer.serialize_seq(Some(params.len()))?;
    for (name, typ) in params.iter() {
        let param = Param {
            name: name.to_string(),
            typ: *typ,
        };
        seq.serialize_element(&param)?;
    }
    seq.end()
}

#[derive(Debug, Clone, PartialEq, serde_derive::Serialize)]
struct Param {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub name: String,
    #[serde(rename = "type")]
    pub typ: Type,
}

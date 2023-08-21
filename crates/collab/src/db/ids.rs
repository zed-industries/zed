use crate::Result;
use sea_orm::DbErr;
use sea_query::{Value, ValueTypeErr};
use serde::{Deserialize, Serialize};

macro_rules! id_type {
    ($name:ident) => {
        #[derive(
            Clone,
            Copy,
            Debug,
            Default,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
            Serialize,
            Deserialize,
        )]
        #[serde(transparent)]
        pub struct $name(pub i32);

        impl $name {
            #[allow(unused)]
            pub const MAX: Self = Self(i32::MAX);

            #[allow(unused)]
            pub fn from_proto(value: u64) -> Self {
                Self(value as i32)
            }

            #[allow(unused)]
            pub fn to_proto(self) -> u64 {
                self.0 as u64
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl From<$name> for sea_query::Value {
            fn from(value: $name) -> Self {
                sea_query::Value::Int(Some(value.0))
            }
        }

        impl sea_orm::TryGetable for $name {
            fn try_get(
                res: &sea_orm::QueryResult,
                pre: &str,
                col: &str,
            ) -> Result<Self, sea_orm::TryGetError> {
                Ok(Self(i32::try_get(res, pre, col)?))
            }
        }

        impl sea_query::ValueType for $name {
            fn try_from(v: Value) -> Result<Self, sea_query::ValueTypeErr> {
                Ok(Self(value_to_integer(v)?))
            }

            fn type_name() -> String {
                stringify!($name).into()
            }

            fn array_type() -> sea_query::ArrayType {
                sea_query::ArrayType::Int
            }

            fn column_type() -> sea_query::ColumnType {
                sea_query::ColumnType::Integer(None)
            }
        }

        impl sea_orm::TryFromU64 for $name {
            fn try_from_u64(n: u64) -> Result<Self, DbErr> {
                Ok(Self(n.try_into().map_err(|_| {
                    DbErr::ConvertFromU64(concat!(
                        "error converting ",
                        stringify!($name),
                        " to u64"
                    ))
                })?))
            }
        }

        impl sea_query::Nullable for $name {
            fn null() -> Value {
                Value::Int(None)
            }
        }
    };
}

fn value_to_integer(v: Value) -> Result<i32, ValueTypeErr> {
    match v {
        Value::TinyInt(Some(int)) => int.try_into().map_err(|_| ValueTypeErr),
        Value::SmallInt(Some(int)) => int.try_into().map_err(|_| ValueTypeErr),
        Value::Int(Some(int)) => int.try_into().map_err(|_| ValueTypeErr),
        Value::BigInt(Some(int)) => int.try_into().map_err(|_| ValueTypeErr),
        Value::TinyUnsigned(Some(int)) => int.try_into().map_err(|_| ValueTypeErr),
        Value::SmallUnsigned(Some(int)) => int.try_into().map_err(|_| ValueTypeErr),
        Value::Unsigned(Some(int)) => int.try_into().map_err(|_| ValueTypeErr),
        Value::BigUnsigned(Some(int)) => int.try_into().map_err(|_| ValueTypeErr),
        _ => Err(ValueTypeErr),
    }
}

id_type!(AccessTokenId);
id_type!(ChannelId);
id_type!(ChannelMemberId);
id_type!(ContactId);
id_type!(FollowerId);
id_type!(RoomId);
id_type!(RoomParticipantId);
id_type!(ProjectId);
id_type!(ProjectCollaboratorId);
id_type!(ReplicaId);
id_type!(ServerId);
id_type!(SignupId);
id_type!(UserId);

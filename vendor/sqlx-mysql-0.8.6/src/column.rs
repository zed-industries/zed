use crate::ext::ustr::UStr;
use crate::protocol::text::ColumnFlags;
use crate::{MySql, MySqlTypeInfo};
pub(crate) use sqlx_core::column::*;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "offline", derive(serde::Serialize, serde::Deserialize))]
pub struct MySqlColumn {
    pub(crate) ordinal: usize,
    pub(crate) name: UStr,
    pub(crate) type_info: MySqlTypeInfo,

    #[cfg_attr(feature = "offline", serde(skip))]
    pub(crate) flags: Option<ColumnFlags>,
}

impl Column for MySqlColumn {
    type Database = MySql;

    fn ordinal(&self) -> usize {
        self.ordinal
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn type_info(&self) -> &MySqlTypeInfo {
        &self.type_info
    }
}

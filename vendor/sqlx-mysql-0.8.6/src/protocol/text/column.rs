use std::str::from_utf8;

use bitflags::bitflags;
use bytes::{Buf, Bytes};

use crate::error::Error;
use crate::io::MySqlBufExt;
use crate::io::ProtocolDecode;
use crate::protocol::Capabilities;

// https://dev.mysql.com/doc/dev/mysql-server/8.0.12/group__group__cs__column__definition__flags.html

bitflags! {
    #[cfg_attr(feature = "offline", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub(crate) struct ColumnFlags: u16 {
        /// Field can't be `NULL`.
        const NOT_NULL = 1;

        /// Field is part of a primary key.
        const PRIMARY_KEY = 2;

        /// Field is part of a unique key.
        const UNIQUE_KEY = 4;

        /// Field is part of a multi-part unique or primary key.
        const MULTIPLE_KEY = 8;

        /// Field is a blob.
        const BLOB = 16;

        /// Field is unsigned.
        const UNSIGNED = 32;

        /// Field is zero filled.
        const ZEROFILL = 64;

        /// Field is binary.
        const BINARY = 128;

        /// Field is an enumeration.
        const ENUM = 256;

        /// Field is an auto-incement field.
        const AUTO_INCREMENT = 512;

        /// Field is a timestamp.
        const TIMESTAMP = 1024;

        /// Field is a set.
        const SET = 2048;

        /// Field does not have a default value.
        const NO_DEFAULT_VALUE = 4096;

        /// Field is set to NOW on UPDATE.
        const ON_UPDATE_NOW = 8192;

        /// Field is a number.
        const NUM = 32768;
    }
}

// https://dev.mysql.com/doc/internals/en/com-query-response.html#column-type

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "offline", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum ColumnType {
    Decimal = 0x00,
    Tiny = 0x01,
    Short = 0x02,
    Long = 0x03,
    Float = 0x04,
    Double = 0x05,
    Null = 0x06,
    Timestamp = 0x07,
    LongLong = 0x08,
    Int24 = 0x09,
    Date = 0x0a,
    Time = 0x0b,
    Datetime = 0x0c,
    Year = 0x0d,
    VarChar = 0x0f,
    Bit = 0x10,
    Json = 0xf5,
    NewDecimal = 0xf6,
    Enum = 0xf7,
    Set = 0xf8,
    TinyBlob = 0xf9,
    MediumBlob = 0xfa,
    LongBlob = 0xfb,
    Blob = 0xfc,
    VarString = 0xfd,
    String = 0xfe,
    Geometry = 0xff,
}

// https://dev.mysql.com/doc/dev/mysql-server/8.0.12/page_protocol_com_query_response_text_resultset_column_definition.html
// https://mariadb.com/kb/en/resultset/#column-definition-packet
// https://dev.mysql.com/doc/internals/en/com-query-response.html#packet-Protocol::ColumnDefinition41

#[derive(Debug)]
pub(crate) struct ColumnDefinition {
    #[allow(unused)]
    catalog: Bytes,
    #[allow(unused)]
    schema: Bytes,
    #[allow(unused)]
    table_alias: Bytes,
    #[allow(unused)]
    table: Bytes,
    alias: Bytes,
    name: Bytes,
    #[allow(unused)]
    pub(crate) collation: u16,
    pub(crate) max_size: u32,
    pub(crate) r#type: ColumnType,
    pub(crate) flags: ColumnFlags,
    #[allow(unused)]
    decimals: u8,
}

impl ColumnDefinition {
    // NOTE: strings in-protocol are transmitted according to the client character set
    //       as this is UTF-8, all these strings should be UTF-8

    pub(crate) fn name(&self) -> Result<&str, Error> {
        from_utf8(&self.name).map_err(Error::protocol)
    }

    pub(crate) fn alias(&self) -> Result<&str, Error> {
        from_utf8(&self.alias).map_err(Error::protocol)
    }
}

impl ProtocolDecode<'_, Capabilities> for ColumnDefinition {
    fn decode_with(mut buf: Bytes, _: Capabilities) -> Result<Self, Error> {
        let catalog = buf.get_bytes_lenenc()?;
        let schema = buf.get_bytes_lenenc()?;
        let table_alias = buf.get_bytes_lenenc()?;
        let table = buf.get_bytes_lenenc()?;
        let alias = buf.get_bytes_lenenc()?;
        let name = buf.get_bytes_lenenc()?;
        let _next_len = buf.get_uint_lenenc(); // always 0x0c
        let collation = buf.get_u16_le();
        let max_size = buf.get_u32_le();
        let type_id = buf.get_u8();
        let flags = buf.get_u16_le();
        let decimals = buf.get_u8();

        Ok(Self {
            catalog,
            schema,
            table_alias,
            table,
            alias,
            name,
            collation,
            max_size,
            r#type: ColumnType::try_from_u16(type_id)?,
            flags: ColumnFlags::from_bits_truncate(flags),
            decimals,
        })
    }
}

impl ColumnType {
    pub(crate) fn name(self, flags: ColumnFlags, max_size: Option<u32>) -> &'static str {
        let is_binary = flags.contains(ColumnFlags::BINARY);
        let is_unsigned = flags.contains(ColumnFlags::UNSIGNED);
        let is_enum = flags.contains(ColumnFlags::ENUM);

        match self {
            ColumnType::Tiny if max_size == Some(1) => "BOOLEAN",
            ColumnType::Tiny if is_unsigned => "TINYINT UNSIGNED",
            ColumnType::Short if is_unsigned => "SMALLINT UNSIGNED",
            ColumnType::Long if is_unsigned => "INT UNSIGNED",
            ColumnType::Int24 if is_unsigned => "MEDIUMINT UNSIGNED",
            ColumnType::LongLong if is_unsigned => "BIGINT UNSIGNED",
            ColumnType::Tiny => "TINYINT",
            ColumnType::Short => "SMALLINT",
            ColumnType::Long => "INT",
            ColumnType::Int24 => "MEDIUMINT",
            ColumnType::LongLong => "BIGINT",
            ColumnType::Float => "FLOAT",
            ColumnType::Double => "DOUBLE",
            ColumnType::Null => "NULL",
            ColumnType::Timestamp => "TIMESTAMP",
            ColumnType::Date => "DATE",
            ColumnType::Time => "TIME",
            ColumnType::Datetime => "DATETIME",
            ColumnType::Year => "YEAR",
            ColumnType::Bit => "BIT",
            ColumnType::Enum => "ENUM",
            ColumnType::Set => "SET",
            ColumnType::Decimal | ColumnType::NewDecimal => "DECIMAL",
            ColumnType::Geometry => "GEOMETRY",
            ColumnType::Json => "JSON",

            ColumnType::String if is_binary => "BINARY",
            ColumnType::String if is_enum => "ENUM",
            ColumnType::VarChar | ColumnType::VarString if is_binary => "VARBINARY",

            ColumnType::String => "CHAR",
            ColumnType::VarChar | ColumnType::VarString => "VARCHAR",

            ColumnType::TinyBlob if is_binary => "TINYBLOB",
            ColumnType::TinyBlob => "TINYTEXT",

            ColumnType::Blob if is_binary => "BLOB",
            ColumnType::Blob => "TEXT",

            ColumnType::MediumBlob if is_binary => "MEDIUMBLOB",
            ColumnType::MediumBlob => "MEDIUMTEXT",

            ColumnType::LongBlob if is_binary => "LONGBLOB",
            ColumnType::LongBlob => "LONGTEXT",
        }
    }

    pub(crate) fn try_from_u16(id: u8) -> Result<Self, Error> {
        Ok(match id {
            0x00 => ColumnType::Decimal,
            0x01 => ColumnType::Tiny,
            0x02 => ColumnType::Short,
            0x03 => ColumnType::Long,
            0x04 => ColumnType::Float,
            0x05 => ColumnType::Double,
            0x06 => ColumnType::Null,
            0x07 => ColumnType::Timestamp,
            0x08 => ColumnType::LongLong,
            0x09 => ColumnType::Int24,
            0x0a => ColumnType::Date,
            0x0b => ColumnType::Time,
            0x0c => ColumnType::Datetime,
            0x0d => ColumnType::Year,
            // [internal] 0x0e => ColumnType::NewDate,
            0x0f => ColumnType::VarChar,
            0x10 => ColumnType::Bit,
            // [internal] 0x11 => ColumnType::Timestamp2,
            // [internal] 0x12 => ColumnType::Datetime2,
            // [internal] 0x13 => ColumnType::Time2,
            0xf5 => ColumnType::Json,
            0xf6 => ColumnType::NewDecimal,
            0xf7 => ColumnType::Enum,
            0xf8 => ColumnType::Set,
            0xf9 => ColumnType::TinyBlob,
            0xfa => ColumnType::MediumBlob,
            0xfb => ColumnType::LongBlob,
            0xfc => ColumnType::Blob,
            0xfd => ColumnType::VarString,
            0xfe => ColumnType::String,
            0xff => ColumnType::Geometry,

            _ => {
                return Err(err_protocol!("unknown column type 0x{:02x}", id));
            }
        })
    }
}

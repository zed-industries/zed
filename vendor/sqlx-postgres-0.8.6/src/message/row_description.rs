use sqlx_core::bytes::{Buf, Bytes};

use crate::error::Error;
use crate::io::BufExt;
use crate::message::{BackendMessage, BackendMessageFormat};
use crate::types::Oid;

#[derive(Debug)]
pub struct RowDescription {
    pub fields: Vec<Field>,
}

#[derive(Debug)]
pub struct Field {
    /// The name of the field.
    pub name: String,

    /// If the field can be identified as a column of a specific table, the
    /// object ID of the table; otherwise zero.
    pub relation_id: Option<Oid>,

    /// If the field can be identified as a column of a specific table, the attribute number of
    /// the column; otherwise zero.
    pub relation_attribute_no: Option<i16>,

    /// The object ID of the field's data type.
    pub data_type_id: Oid,

    /// The data type size (see pg_type.typlen). Note that negative values denote
    /// variable-width types.
    #[allow(dead_code)]
    pub data_type_size: i16,

    /// The type modifier (see pg_attribute.atttypmod). The meaning of the
    /// modifier is type-specific.
    #[allow(dead_code)]
    pub type_modifier: i32,

    /// The format code being used for the field.
    #[allow(dead_code)]
    pub format: i16,
}

impl BackendMessage for RowDescription {
    const FORMAT: BackendMessageFormat = BackendMessageFormat::RowDescription;

    fn decode_body(mut buf: Bytes) -> Result<Self, Error> {
        if buf.len() < 2 {
            return Err(err_protocol!(
                "expected at least 2 bytes, got {}",
                buf.len()
            ));
        }

        let cnt = buf.get_u16();
        let mut fields = Vec::with_capacity(cnt as usize);

        for _ in 0..cnt {
            let name = buf.get_str_nul()?.to_owned();

            if buf.len() < 18 {
                return Err(err_protocol!(
                    "expected at least 18 bytes after field name {name:?}, got {}",
                    buf.len()
                ));
            }

            let relation_id = buf.get_u32();
            let relation_attribute_no = buf.get_i16();
            let data_type_id = Oid(buf.get_u32());
            let data_type_size = buf.get_i16();
            let type_modifier = buf.get_i32();
            let format = buf.get_i16();

            fields.push(Field {
                name,
                relation_id: if relation_id == 0 {
                    None
                } else {
                    Some(Oid(relation_id))
                },
                relation_attribute_no: if relation_attribute_no == 0 {
                    None
                } else {
                    Some(relation_attribute_no)
                },
                data_type_id,
                data_type_size,
                type_modifier,
                format,
            })
        }

        Ok(Self { fields })
    }
}

// TODO: Unit Test RowDescription
// TODO: Benchmark RowDescription

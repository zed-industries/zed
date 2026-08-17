use sqlx_core::bytes::Buf;

use crate::decode::Decode;
use crate::encode::Encode;
use crate::error::{mismatched_types, BoxDynError};
use crate::type_info::TypeInfo;
use crate::type_info::{PgType, PgTypeKind};
use crate::types::Oid;
use crate::types::Type;
use crate::{PgArgumentBuffer, PgTypeInfo, PgValueFormat, PgValueRef, Postgres};

#[doc(hidden)]
pub struct PgRecordEncoder<'a> {
    buf: &'a mut PgArgumentBuffer,
    off: usize,
    num: u32,
}

impl<'a> PgRecordEncoder<'a> {
    #[doc(hidden)]
    pub fn new(buf: &'a mut PgArgumentBuffer) -> Self {
        let off = buf.len();

        // reserve space for a field count
        buf.extend(&(0_u32).to_be_bytes());

        Self { buf, off, num: 0 }
    }

    #[doc(hidden)]
    pub fn finish(&mut self) {
        // fill in the record length
        self.buf[self.off..(self.off + 4)].copy_from_slice(&self.num.to_be_bytes());
    }

    #[doc(hidden)]
    pub fn encode<'q, T>(&mut self, value: T) -> Result<&mut Self, BoxDynError>
    where
        'a: 'q,
        T: Encode<'q, Postgres> + Type<Postgres>,
    {
        let ty = value.produces().unwrap_or_else(T::type_info);

        match ty.0 {
            // push a hole for this type ID
            // to be filled in on query execution
            PgType::DeclareWithName(name) => self.buf.patch_type_by_name(&name),
            PgType::DeclareArrayOf(array) => self.buf.patch_array_type(array),
            // write type id
            pg_type => self.buf.extend(&pg_type.oid().0.to_be_bytes()),
        }

        self.buf.encode(value)?;
        self.num += 1;

        Ok(self)
    }
}

#[doc(hidden)]
pub struct PgRecordDecoder<'r> {
    buf: &'r [u8],
    typ: PgTypeInfo,
    fmt: PgValueFormat,
    ind: usize,
}

impl<'r> PgRecordDecoder<'r> {
    #[doc(hidden)]
    pub fn new(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
        let fmt = value.format();
        let mut buf = value.as_bytes()?;
        let typ = value.type_info;

        match fmt {
            PgValueFormat::Binary => {
                let _len = buf.get_u32();
            }

            PgValueFormat::Text => {
                // remove the enclosing `(` .. `)`
                buf = &buf[1..(buf.len() - 1)];
            }
        }

        Ok(Self {
            buf,
            fmt,
            typ,
            ind: 0,
        })
    }

    #[doc(hidden)]
    pub fn try_decode<T>(&mut self) -> Result<T, BoxDynError>
    where
        T: for<'a> Decode<'a, Postgres> + Type<Postgres>,
    {
        if self.buf.is_empty() {
            return Err(format!("no field `{0}` found on record", self.ind).into());
        }

        match self.fmt {
            PgValueFormat::Binary => {
                let element_type_oid = Oid(self.buf.get_u32());
                let element_type_opt = match self.typ.0.kind() {
                    PgTypeKind::Simple if self.typ.0 == PgType::Record => {
                        PgTypeInfo::try_from_oid(element_type_oid)
                    }

                    PgTypeKind::Composite(fields) => {
                        let ty = fields[self.ind].1.clone();
                        if ty.0.oid() != element_type_oid {
                            return Err("unexpected mismatch of composite type information".into());
                        }

                        Some(ty)
                    }

                    _ => {
                        return Err(
                            "unexpected non-composite type being decoded as a composite type"
                                .into(),
                        );
                    }
                };

                if let Some(ty) = &element_type_opt {
                    if !ty.is_null() && !T::compatible(ty) {
                        return Err(mismatched_types::<Postgres, T>(ty));
                    }
                }

                let element_type =
                    element_type_opt
                        .ok_or_else(|| BoxDynError::from(format!("custom types in records are not fully supported yet: failed to retrieve type info for field {} with type oid {}", self.ind, element_type_oid.0)))?;

                self.ind += 1;

                T::decode(PgValueRef::get(&mut self.buf, self.fmt, element_type)?)
            }

            PgValueFormat::Text => {
                let mut element = String::new();
                let mut quoted = false;
                let mut in_quotes = false;
                let mut in_escape = false;
                let mut prev_ch = '\0';

                while !self.buf.is_empty() {
                    let ch = self.buf.get_u8() as char;
                    match ch {
                        _ if in_escape => {
                            element.push(ch);
                            in_escape = false;
                        }

                        '"' if in_quotes => {
                            in_quotes = false;
                        }

                        '"' => {
                            in_quotes = true;
                            quoted = true;

                            if prev_ch == '"' {
                                element.push('"')
                            }
                        }

                        '\\' if !in_escape => {
                            in_escape = true;
                        }

                        ',' if !in_quotes => break,

                        _ => {
                            element.push(ch);
                        }
                    }
                    prev_ch = ch;
                }

                let buf = if element.is_empty() && !quoted {
                    // completely empty input means NULL
                    None
                } else {
                    Some(element.as_bytes())
                };

                // NOTE: we do not call [`accepts`] or give a chance to from a user as
                //       TEXT sequences are not strongly typed

                T::decode(PgValueRef {
                    // NOTE: We pass `0` as the type ID because we don't have a reasonable value
                    //       we could use.
                    type_info: PgTypeInfo::with_oid(Oid(0)),
                    format: self.fmt,
                    value: buf,
                    row: None,
                })
            }
        }
    }
}

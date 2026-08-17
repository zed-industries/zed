use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{Error, SeqAccess, Visitor},
    ser::{SerializeSeq, SerializeStruct},
};
use std::{borrow::Cow, num::NonZeroU32};
use zbus_names::{BusName, ErrorName, InterfaceName, MemberName, UniqueName};
use zvariant::{ObjectPath, Signature, Type, Value};

use crate::message::{FieldCode, Header, Message};

/// A collection of [`Field`] instances.
///
/// [`Field`]: enum.Field.html
#[derive(Debug, Default, Clone, Type)]
#[zvariant(signature = "a(yv)")]
pub(crate) struct Fields<'f> {
    pub path: Option<ObjectPath<'f>>,
    pub interface: Option<InterfaceName<'f>>,
    pub member: Option<MemberName<'f>>,
    pub error_name: Option<ErrorName<'f>>,
    pub reply_serial: Option<NonZeroU32>,
    pub destination: Option<BusName<'f>>,
    pub sender: Option<UniqueName<'f>>,
    pub signature: Cow<'f, Signature>,
    pub unix_fds: Option<u32>,
}

impl Fields<'_> {
    /// Create an empty collection of fields.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serialize for Fields<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(None)?;
        if let Some(path) = &self.path {
            seq.serialize_element(&(FieldCode::Path, Value::from(path)))?;
        }
        if let Some(interface) = &self.interface {
            seq.serialize_element(&(FieldCode::Interface, Value::from(interface.as_str())))?;
        }
        if let Some(member) = &self.member {
            seq.serialize_element(&(FieldCode::Member, Value::from(member.as_str())))?;
        }
        if let Some(error_name) = &self.error_name {
            seq.serialize_element(&(FieldCode::ErrorName, Value::from(error_name.as_str())))?;
        }
        if let Some(reply_serial) = self.reply_serial {
            seq.serialize_element(&(FieldCode::ReplySerial, Value::from(reply_serial.get())))?;
        }
        if let Some(destination) = &self.destination {
            seq.serialize_element(&(FieldCode::Destination, Value::from(destination.as_str())))?;
        }
        if let Some(sender) = &self.sender {
            seq.serialize_element(&(FieldCode::Sender, Value::from(sender.as_str())))?;
        }
        if !matches!(&*self.signature, Signature::Unit) {
            seq.serialize_element(&(FieldCode::Signature, SignatureSerializer(&self.signature)))?;
        }
        if let Some(unix_fds) = self.unix_fds {
            seq.serialize_element(&(FieldCode::UnixFDs, Value::from(unix_fds)))?;
        }
        seq.end()
    }
}

/// Our special serializer for [`Value::Signature`].
///
/// Normally `Value` would use the default serializer for `Signature`, which will include the `()`
/// for strucutures but for body signature, that's what what the D-Bus expects so we do the same as
/// `Value` here, except we serialize signature value as string w/o the `()`.
#[derive(Debug, Type)]
#[zvariant(signature = "v")]
struct SignatureSerializer<'a>(&'a Signature);

impl Serialize for SignatureSerializer<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut structure = serializer.serialize_struct("Variant", 2)?;

        structure.serialize_field("signature", &Signature::Signature)?;

        let signature_str = self.0.to_string_no_parens();
        structure.serialize_field("value", &signature_str)?;

        structure.end()
    }
}

impl<'de: 'f, 'f> Deserialize<'de> for Fields<'f> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(FieldsVisitor)
    }
}

struct FieldsVisitor;

impl<'de> Visitor<'de> for FieldsVisitor {
    type Value = Fields<'de>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("D-Bus message header fields")
    }

    fn visit_seq<V>(self, mut visitor: V) -> Result<Fields<'de>, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let mut fields = Fields::new();
        while let Some((code, value)) = visitor.next_element::<(FieldCode, Value<'de>)>()? {
            match code {
                FieldCode::Path => {
                    fields.path = Some(ObjectPath::try_from(value).map_err(V::Error::custom)?)
                }
                FieldCode::Interface => {
                    fields.interface =
                        Some(InterfaceName::try_from(value).map_err(V::Error::custom)?)
                }
                FieldCode::Member => {
                    fields.member = Some(MemberName::try_from(value).map_err(V::Error::custom)?)
                }
                FieldCode::ErrorName => {
                    fields.error_name = Some(ErrorName::try_from(value).map_err(V::Error::custom)?)
                }
                FieldCode::ReplySerial => {
                    let value = u32::try_from(value)
                        .map_err(V::Error::custom)
                        .and_then(|v| v.try_into().map_err(V::Error::custom))?;
                    fields.reply_serial = Some(value)
                }
                FieldCode::Destination => {
                    fields.destination = Some(BusName::try_from(value).map_err(V::Error::custom)?)
                }
                FieldCode::Sender => {
                    fields.sender = Some(UniqueName::try_from(value).map_err(V::Error::custom)?)
                }
                FieldCode::Signature => {
                    fields.signature =
                        Cow::Owned(Signature::try_from(value).map_err(V::Error::custom)?)
                }
                FieldCode::UnixFDs => {
                    fields.unix_fds = Some(u32::try_from(value).map_err(V::Error::custom)?)
                }
            }
        }

        Ok(fields)
    }
}

/// A byte range of a field in a Message, used in [`QuickFields`].
///
/// Some invalid encodings (end = 0) are used to indicate "not cached" and "not present".
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct FieldPos {
    start: u32,
    end: u32,
}

impl FieldPos {
    pub fn new_not_present() -> Self {
        Self { start: 1, end: 0 }
    }

    pub fn build(msg_buf: &[u8], field_buf: &str) -> Option<Self> {
        let buf_start = msg_buf.as_ptr() as usize;
        let field_start = field_buf.as_ptr() as usize;
        let offset = field_start.checked_sub(buf_start)?;
        if offset <= msg_buf.len() && offset + field_buf.len() <= msg_buf.len() {
            Some(Self {
                start: offset.try_into().ok()?,
                end: (offset + field_buf.len()).try_into().ok()?,
            })
        } else {
            None
        }
    }

    pub fn new<T>(msg_buf: &[u8], field: Option<&T>) -> Self
    where
        T: std::ops::Deref<Target = str>,
    {
        field
            .and_then(|f| Self::build(msg_buf, f.deref()))
            .unwrap_or_else(Self::new_not_present)
    }

    /// Reassemble a previously cached field.
    ///
    /// **NOTE**: The caller must ensure that the `msg_buff` is the same one `build` was called for.
    /// Otherwise, you'll get a panic.
    pub fn read<'m, T>(&self, msg_buf: &'m [u8]) -> Option<T>
    where
        T: TryFrom<&'m str>,
        T::Error: std::fmt::Debug,
    {
        match self {
            Self {
                start: 0..=1,
                end: 0,
            } => None,
            Self { start, end } => {
                let s = std::str::from_utf8(&msg_buf[(*start as usize)..(*end as usize)])
                    .expect("Invalid utf8 when reconstructing string");
                // We already check the fields during the construction of `Self`.
                T::try_from(s)
                    .map(Some)
                    .expect("Invalid field reconstruction")
            }
        }
    }
}

/// A cache of the Message header fields.
#[derive(Debug, Default, Clone)]
pub(crate) struct QuickFields {
    path: FieldPos,
    interface: FieldPos,
    member: FieldPos,
    error_name: FieldPos,
    reply_serial: Option<NonZeroU32>,
    destination: FieldPos,
    sender: FieldPos,
    signature: Signature,
    unix_fds: Option<u32>,
}

impl QuickFields {
    pub fn new(buf: &[u8], header: &Header<'_>) -> Self {
        Self {
            path: FieldPos::new(buf, header.path()),
            interface: FieldPos::new(buf, header.interface()),
            member: FieldPos::new(buf, header.member()),
            error_name: FieldPos::new(buf, header.error_name()),
            reply_serial: header.reply_serial(),
            destination: FieldPos::new(buf, header.destination()),
            sender: FieldPos::new(buf, header.sender()),
            signature: header.signature().clone(),
            unix_fds: header.unix_fds(),
        }
    }

    pub fn path<'m>(&self, msg: &'m Message) -> Option<ObjectPath<'m>> {
        self.path.read(msg.data())
    }

    pub fn interface<'m>(&self, msg: &'m Message) -> Option<InterfaceName<'m>> {
        self.interface.read(msg.data())
    }

    pub fn member<'m>(&self, msg: &'m Message) -> Option<MemberName<'m>> {
        self.member.read(msg.data())
    }

    pub fn error_name<'m>(&self, msg: &'m Message) -> Option<ErrorName<'m>> {
        self.error_name.read(msg.data())
    }

    pub fn reply_serial(&self) -> Option<NonZeroU32> {
        self.reply_serial
    }

    pub fn destination<'m>(&self, msg: &'m Message) -> Option<BusName<'m>> {
        self.destination.read(msg.data())
    }

    pub fn sender<'m>(&self, msg: &'m Message) -> Option<UniqueName<'m>> {
        self.sender.read(msg.data())
    }

    pub fn signature(&self) -> &Signature {
        &self.signature
    }

    pub fn unix_fds(&self) -> Option<u32> {
        self.unix_fds
    }
}

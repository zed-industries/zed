use crate::reflect::message::message_ref::MessageRef;
use crate::reflect::runtime_types::RuntimeTypeTrait;
use crate::reflect::value::value_ref::ReflectValueMut;
use crate::reflect::value::value_ref::ReflectValueRef;
use crate::reflect::EnumDescriptor;
use crate::reflect::EnumValueDescriptor;
use crate::reflect::ProtobufValue;
use crate::reflect::RuntimeType;
use crate::MessageDyn;

/// Owner value of any elementary type
#[derive(Debug, Clone)]
pub enum ReflectValueBox {
    /// `u32`
    U32(u32),
    /// `u64`
    U64(u64),
    /// `i32`
    I32(i32),
    /// `i64`
    I64(i64),
    /// `f32`
    F32(f32),
    /// `f64`
    F64(f64),
    /// `bool`
    Bool(bool),
    /// `string`
    String(String),
    /// `bytes`
    Bytes(Vec<u8>),
    /// `enum`
    Enum(EnumDescriptor, i32),
    /// `message`
    Message(Box<dyn MessageDyn>),
}

impl From<u32> for ReflectValueBox {
    fn from(v: u32) -> Self {
        ReflectValueBox::U32(v)
    }
}

impl From<u64> for ReflectValueBox {
    fn from(v: u64) -> Self {
        ReflectValueBox::U64(v)
    }
}

impl From<i32> for ReflectValueBox {
    fn from(v: i32) -> Self {
        ReflectValueBox::I32(v)
    }
}

impl From<i64> for ReflectValueBox {
    fn from(v: i64) -> Self {
        ReflectValueBox::I64(v)
    }
}

impl From<f32> for ReflectValueBox {
    fn from(v: f32) -> Self {
        ReflectValueBox::F32(v)
    }
}

impl From<f64> for ReflectValueBox {
    fn from(v: f64) -> Self {
        ReflectValueBox::F64(v)
    }
}

impl From<bool> for ReflectValueBox {
    fn from(v: bool) -> Self {
        ReflectValueBox::Bool(v)
    }
}

impl From<String> for ReflectValueBox {
    fn from(v: String) -> Self {
        ReflectValueBox::String(v)
    }
}

impl From<Vec<u8>> for ReflectValueBox {
    fn from(v: Vec<u8>) -> Self {
        ReflectValueBox::Bytes(v)
    }
}

impl<'a> From<&'a EnumValueDescriptor> for ReflectValueBox {
    fn from(v: &'a EnumValueDescriptor) -> Self {
        ReflectValueBox::from(v.clone())
    }
}

impl From<EnumValueDescriptor> for ReflectValueBox {
    fn from(v: EnumValueDescriptor) -> Self {
        let number = v.value();
        ReflectValueBox::Enum(v.enum_descriptor, number)
    }
}

impl From<Box<dyn MessageDyn>> for ReflectValueBox {
    fn from(v: Box<dyn MessageDyn>) -> Self {
        ReflectValueBox::Message(v)
    }
}

fn _assert_value_box_send_sync() {
    fn _assert_send_sync<T: Send + Sync>() {}
    _assert_send_sync::<ReflectValueBox>();
}

impl ReflectValueBox {
    /// Type of this value.
    pub fn get_type(&self) -> RuntimeType {
        self.as_value_ref().get_type()
    }

    /// As ref
    pub fn as_value_ref(&self) -> ReflectValueRef {
        match self {
            ReflectValueBox::U32(v) => ReflectValueRef::U32(*v),
            ReflectValueBox::U64(v) => ReflectValueRef::U64(*v),
            ReflectValueBox::I32(v) => ReflectValueRef::I32(*v),
            ReflectValueBox::I64(v) => ReflectValueRef::I64(*v),
            ReflectValueBox::F32(v) => ReflectValueRef::F32(*v),
            ReflectValueBox::F64(v) => ReflectValueRef::F64(*v),
            ReflectValueBox::Bool(v) => ReflectValueRef::Bool(*v),
            ReflectValueBox::String(ref v) => ReflectValueRef::String(v.as_str()),
            ReflectValueBox::Bytes(ref v) => ReflectValueRef::Bytes(v.as_slice()),
            ReflectValueBox::Enum(d, v) => ReflectValueRef::Enum(d.clone(), *v),
            ReflectValueBox::Message(v) => ReflectValueRef::Message(MessageRef::from(&**v)),
        }
    }

    pub(crate) fn as_value_mut(&mut self) -> ReflectValueMut {
        match self {
            ReflectValueBox::Message(m) => ReflectValueMut::Message(&mut **m),
            _ => panic!(
                "ReflectValueMut cannot be constructed from {:?}",
                self.get_type()
            ),
        }
    }

    /// Downcast to real typed value.
    ///
    /// For `enum` `V` can be either `V: ProtobufEnum` or `V: ProtobufEnumOrUnknown<E>`.
    pub fn downcast<V: ProtobufValue>(self) -> Result<V, Self> {
        V::RuntimeType::from_value_box(self)
    }
}

impl<'a> PartialEq for ReflectValueBox {
    fn eq(&self, other: &Self) -> bool {
        self.as_value_ref() == other.as_value_ref()
    }
}

impl<'a> PartialEq<ReflectValueBox> for ReflectValueRef<'a> {
    fn eq(&self, other: &ReflectValueBox) -> bool {
        *self == other.as_value_ref()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn reflect_value_box_downcast_primitive() {
        assert_eq!(Ok(10), ReflectValueBox::U32(10).downcast::<u32>());
        assert_eq!(
            Err(ReflectValueBox::I32(10)),
            ReflectValueBox::I32(10).downcast::<u32>()
        );
    }

    #[test]
    fn reflect_value_box_downcast_string() {
        assert_eq!(
            Ok("aa".to_owned()),
            ReflectValueBox::String("aa".to_owned()).downcast::<String>()
        );
        assert_eq!(
            Err(ReflectValueBox::String("aa".to_owned())),
            ReflectValueBox::String("aa".to_owned()).downcast::<u32>()
        );
        assert_eq!(
            Err(ReflectValueBox::Bool(false)),
            ReflectValueBox::Bool(false).downcast::<String>()
        );
    }
}

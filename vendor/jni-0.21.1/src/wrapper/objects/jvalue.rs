use std::convert::TryFrom;
use std::fmt::Debug;

use log::trace;

use crate::{errors::*, objects::JObject, signature::Primitive, sys::*};

/// Rusty version of the JNI C `jvalue` enum. Used in Java method call arguments
/// and returns.
///
/// `JValueGen` is a generic type, meant to represent both owned and borrowed
/// JNI values. The type parameter `O` refers to what kind of object reference
/// the `JValueGen` can hold, which is either:
///
/// * an owned [`JObject`], used for values returned from a Java method call,
///   or
/// * a borrowed `&JObject`, used for parameters passed to a Java method call.
///
/// These two cases are represented by the type aliases [`JValueOwned`] and
/// [`JValue`], respectively.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug)]
pub enum JValueGen<O> {
    Object(O),
    Byte(jbyte),
    Char(jchar),
    Short(jshort),
    Int(jint),
    Long(jlong),
    Bool(jboolean),
    Float(jfloat),
    Double(jdouble),
    Void,
}

/// An <dfn>owned</dfn> [`JValueGen`].
///
/// This type is used for values returned from Java method calls. If the Java
/// method returns an object reference, it will take the form of an owned
/// [`JObject`].
pub type JValueOwned<'local> = JValueGen<JObject<'local>>;

/// A <dfn>reference</dfn> [`JValueGen`].
///
/// This type is used for parameters passed to Java method calls. If the Java
/// method is to be passed an object reference, it takes the form of a borrowed
/// <code>&[JObject]</code>.
pub type JValue<'local, 'obj_ref> = JValueGen<&'obj_ref JObject<'local>>;

impl<O> JValueGen<O> {
    /// Convert the enum to its jni-compatible equivalent.
    pub fn as_jni<'local>(&self) -> jvalue
    where
        O: AsRef<JObject<'local>> + Debug,
    {
        let val: jvalue = match self {
            JValueGen::Object(obj) => jvalue {
                l: obj.as_ref().as_raw(),
            },
            JValueGen::Byte(byte) => jvalue { b: *byte },
            JValueGen::Char(char) => jvalue { c: *char },
            JValueGen::Short(short) => jvalue { s: *short },
            JValueGen::Int(int) => jvalue { i: *int },
            JValueGen::Long(long) => jvalue { j: *long },
            JValueGen::Bool(boolean) => jvalue { b: *boolean as i8 },
            JValueGen::Float(float) => jvalue { f: *float },
            JValueGen::Double(double) => jvalue { d: *double },
            JValueGen::Void => jvalue {
                l: ::std::ptr::null_mut(),
            },
        };
        trace!("converted {:?} to jvalue {:?}", self, unsafe {
            ::std::mem::transmute::<_, u64>(val)
        });
        val
    }

    /// Convert the enum to its jni-compatible equivalent.
    #[deprecated = "Use `as_jni` instead."]
    pub fn to_jni<'local>(self) -> jvalue
    where
        O: AsRef<JObject<'local>> + Debug,
    {
        self.as_jni()
    }

    /// Get the type name for the enum variant.
    pub fn type_name(&self) -> &'static str {
        match *self {
            JValueGen::Void => "void",
            JValueGen::Object(_) => "object",
            JValueGen::Byte(_) => "byte",
            JValueGen::Char(_) => "char",
            JValueGen::Short(_) => "short",
            JValueGen::Int(_) => "int",
            JValueGen::Long(_) => "long",
            JValueGen::Bool(_) => "bool",
            JValueGen::Float(_) => "float",
            JValueGen::Double(_) => "double",
        }
    }

    /// Get the primitive type for the enum variant. If it's not a primitive
    /// (i.e. an Object), returns None.
    pub fn primitive_type(&self) -> Option<Primitive> {
        Some(match *self {
            JValueGen::Object(_) => return None,
            JValueGen::Void => Primitive::Void,
            JValueGen::Byte(_) => Primitive::Byte,
            JValueGen::Char(_) => Primitive::Char,
            JValueGen::Short(_) => Primitive::Short,
            JValueGen::Int(_) => Primitive::Int,
            JValueGen::Long(_) => Primitive::Long,
            JValueGen::Bool(_) => Primitive::Boolean,
            JValueGen::Float(_) => Primitive::Float,
            JValueGen::Double(_) => Primitive::Double,
        })
    }

    /// Try to unwrap to an Object.
    pub fn l(self) -> Result<O> {
        match self {
            JValueGen::Object(obj) => Ok(obj),
            _ => Err(Error::WrongJValueType("object", self.type_name())),
        }
    }

    /// Try to unwrap to a boolean.
    pub fn z(self) -> Result<bool> {
        match self {
            JValueGen::Bool(b) => Ok(b == JNI_TRUE),
            _ => Err(Error::WrongJValueType("bool", self.type_name())),
        }
    }

    /// Try to unwrap to a byte.
    pub fn b(self) -> Result<jbyte> {
        match self {
            JValueGen::Byte(b) => Ok(b),
            _ => Err(Error::WrongJValueType("jbyte", self.type_name())),
        }
    }

    /// Try to unwrap to a char.
    pub fn c(self) -> Result<jchar> {
        match self {
            JValueGen::Char(b) => Ok(b),
            _ => Err(Error::WrongJValueType("jchar", self.type_name())),
        }
    }

    /// Try to unwrap to a double.
    pub fn d(self) -> Result<jdouble> {
        match self {
            JValueGen::Double(b) => Ok(b),
            _ => Err(Error::WrongJValueType("jdouble", self.type_name())),
        }
    }

    /// Try to unwrap to a float.
    pub fn f(self) -> Result<jfloat> {
        match self {
            JValueGen::Float(b) => Ok(b),
            _ => Err(Error::WrongJValueType("jfloat", self.type_name())),
        }
    }

    /// Try to unwrap to an int.
    pub fn i(self) -> Result<jint> {
        match self {
            JValueGen::Int(b) => Ok(b),
            _ => Err(Error::WrongJValueType("jint", self.type_name())),
        }
    }

    /// Try to unwrap to a long.
    pub fn j(self) -> Result<jlong> {
        match self {
            JValueGen::Long(b) => Ok(b),
            _ => Err(Error::WrongJValueType("jlong", self.type_name())),
        }
    }

    /// Try to unwrap to a short.
    pub fn s(self) -> Result<jshort> {
        match self {
            JValueGen::Short(b) => Ok(b),
            _ => Err(Error::WrongJValueType("jshort", self.type_name())),
        }
    }

    /// Try to unwrap to a void.
    pub fn v(self) -> Result<()> {
        match self {
            JValueGen::Void => Ok(()),
            _ => Err(Error::WrongJValueType("void", self.type_name())),
        }
    }

    /// Copies or borrows the value in this `JValue`.
    ///
    /// If the value is a primitive type, it is copied. If the value is an
    /// object reference, it is borrowed.
    pub fn borrow(&self) -> JValueGen<&O> {
        match self {
            JValueGen::Object(o) => JValueGen::Object(o),
            JValueGen::Byte(v) => JValueGen::Byte(*v),
            JValueGen::Char(v) => JValueGen::Char(*v),
            JValueGen::Short(v) => JValueGen::Short(*v),
            JValueGen::Int(v) => JValueGen::Int(*v),
            JValueGen::Long(v) => JValueGen::Long(*v),
            JValueGen::Bool(v) => JValueGen::Bool(*v),
            JValueGen::Float(v) => JValueGen::Float(*v),
            JValueGen::Double(v) => JValueGen::Double(*v),
            JValueGen::Void => JValueGen::Void,
        }
    }
}

impl<'obj_ref, O> From<&'obj_ref JValueGen<O>> for JValueGen<&'obj_ref O> {
    fn from(other: &'obj_ref JValueGen<O>) -> Self {
        other.borrow()
    }
}

impl<'local, T: Into<JObject<'local>>> From<T> for JValueOwned<'local> {
    fn from(other: T) -> Self {
        Self::Object(other.into())
    }
}

impl<'local: 'obj_ref, 'obj_ref, T: AsRef<JObject<'local>>> From<&'obj_ref T>
    for JValue<'local, 'obj_ref>
{
    fn from(other: &'obj_ref T) -> Self {
        Self::Object(other.as_ref())
    }
}

impl<'local> TryFrom<JValueOwned<'local>> for JObject<'local> {
    type Error = Error;

    fn try_from(value: JValueOwned<'local>) -> Result<Self> {
        match value {
            JValueGen::Object(o) => Ok(o),
            _ => Err(Error::WrongJValueType("object", value.type_name())),
        }
    }
}

impl<O> From<bool> for JValueGen<O> {
    fn from(other: bool) -> Self {
        JValueGen::Bool(if other { JNI_TRUE } else { JNI_FALSE })
    }
}

// jbool
impl<O> From<jboolean> for JValueGen<O> {
    fn from(other: jboolean) -> Self {
        JValueGen::Bool(other)
    }
}

impl<O> TryFrom<JValueGen<O>> for jboolean {
    type Error = Error;

    fn try_from(value: JValueGen<O>) -> Result<Self> {
        match value {
            JValueGen::Bool(b) => Ok(b),
            _ => Err(Error::WrongJValueType("bool", value.type_name())),
        }
    }
}

// jchar
impl<O> From<jchar> for JValueGen<O> {
    fn from(other: jchar) -> Self {
        JValueGen::Char(other)
    }
}

impl<O> TryFrom<JValueGen<O>> for jchar {
    type Error = Error;

    fn try_from(value: JValueGen<O>) -> Result<Self> {
        match value {
            JValueGen::Char(c) => Ok(c),
            _ => Err(Error::WrongJValueType("char", value.type_name())),
        }
    }
}

// jshort
impl<O> From<jshort> for JValueGen<O> {
    fn from(other: jshort) -> Self {
        JValueGen::Short(other)
    }
}

impl<O> TryFrom<JValueGen<O>> for jshort {
    type Error = Error;

    fn try_from(value: JValueGen<O>) -> Result<Self> {
        match value {
            JValueGen::Short(s) => Ok(s),
            _ => Err(Error::WrongJValueType("short", value.type_name())),
        }
    }
}

// jfloat
impl<O> From<jfloat> for JValueGen<O> {
    fn from(other: jfloat) -> Self {
        JValueGen::Float(other)
    }
}

impl<O> TryFrom<JValueGen<O>> for jfloat {
    type Error = Error;

    fn try_from(value: JValueGen<O>) -> Result<Self> {
        match value {
            JValueGen::Float(f) => Ok(f),
            _ => Err(Error::WrongJValueType("float", value.type_name())),
        }
    }
}

// jdouble
impl<O> From<jdouble> for JValueGen<O> {
    fn from(other: jdouble) -> Self {
        JValueGen::Double(other)
    }
}

impl<O> TryFrom<JValueGen<O>> for jdouble {
    type Error = Error;

    fn try_from(value: JValueGen<O>) -> Result<Self> {
        match value {
            JValueGen::Double(d) => Ok(d),
            _ => Err(Error::WrongJValueType("double", value.type_name())),
        }
    }
}

// jint
impl<O> From<jint> for JValueGen<O> {
    fn from(other: jint) -> Self {
        JValueGen::Int(other)
    }
}

impl<O> TryFrom<JValueGen<O>> for jint {
    type Error = Error;

    fn try_from(value: JValueGen<O>) -> Result<Self> {
        match value {
            JValueGen::Int(i) => Ok(i),
            _ => Err(Error::WrongJValueType("int", value.type_name())),
        }
    }
}

// jlong
impl<O> From<jlong> for JValueGen<O> {
    fn from(other: jlong) -> Self {
        JValueGen::Long(other)
    }
}

impl<O> TryFrom<JValueGen<O>> for jlong {
    type Error = Error;

    fn try_from(value: JValueGen<O>) -> Result<Self> {
        match value {
            JValueGen::Long(l) => Ok(l),
            _ => Err(Error::WrongJValueType("long", value.type_name())),
        }
    }
}

// jbyte
impl<O> From<jbyte> for JValueGen<O> {
    fn from(other: jbyte) -> Self {
        JValueGen::Byte(other)
    }
}

impl<O> TryFrom<JValueGen<O>> for jbyte {
    type Error = Error;

    fn try_from(value: JValueGen<O>) -> Result<Self> {
        match value {
            JValueGen::Byte(b) => Ok(b),
            _ => Err(Error::WrongJValueType("byte", value.type_name())),
        }
    }
}

// jvoid
impl<O> From<()> for JValueGen<O> {
    fn from(_: ()) -> Self {
        JValueGen::Void
    }
}

impl<O> TryFrom<JValueGen<O>> for () {
    type Error = Error;

    fn try_from(value: JValueGen<O>) -> Result<Self> {
        match value {
            JValueGen::Void => Ok(()),
            _ => Err(Error::WrongJValueType("void", value.type_name())),
        }
    }
}

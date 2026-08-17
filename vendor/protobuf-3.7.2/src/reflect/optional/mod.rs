use crate::reflect::runtime_types::RuntimeTypeTrait;
use crate::reflect::ProtobufValue;
use crate::reflect::ReflectEq;
use crate::reflect::ReflectEqMode;
use crate::reflect::ReflectValueRef;
use crate::reflect::RuntimeType;

enum Impl<'a> {
    None(RuntimeType),
    Some(ReflectValueRef<'a>),
}

/// Singular field field and value type.
pub struct ReflectOptionalRef<'a>(Impl<'a>);

impl<'a> PartialEq for ReflectOptionalRef<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.reflect_eq(other, &ReflectEqMode::default())
    }
}

impl<'a> ReflectEq for ReflectOptionalRef<'a> {
    fn reflect_eq(&self, that: &Self, mode: &ReflectEqMode) -> bool {
        match (&self.0, &that.0) {
            (Impl::None(at), Impl::None(bt)) => at == bt,
            (Impl::Some(a), Impl::Some(b)) => a.reflect_eq(b, mode),
            (Impl::None(_), Impl::Some(_)) | (Impl::Some(_), Impl::None(_)) => false,
        }
    }
}

impl<'a> ReflectOptionalRef<'a> {
    /// No value.
    pub fn none(elem: RuntimeType) -> ReflectOptionalRef<'a> {
        ReflectOptionalRef(Impl::None(elem))
    }

    /// Has value.
    pub fn some(value: ReflectValueRef<'a>) -> ReflectOptionalRef<'a> {
        ReflectOptionalRef(Impl::Some(value))
    }

    pub(crate) fn none_from<V: ProtobufValue>() -> ReflectOptionalRef<'a> {
        ReflectOptionalRef::none(V::RuntimeType::runtime_type_box())
    }

    pub(crate) fn some_from<V: ProtobufValue>(value: &'a V) -> ReflectOptionalRef<'a> {
        ReflectOptionalRef::some(V::RuntimeType::as_ref(value))
    }

    pub(crate) fn new_filter_non_zero<V: ProtobufValue>(v: &'a V) -> ReflectOptionalRef<'a> {
        if V::RuntimeType::is_non_zero(v) {
            ReflectOptionalRef::some_from(v)
        } else {
            ReflectOptionalRef::none_from::<V>()
        }
    }

    pub(crate) fn new_from_option<V: ProtobufValue>(v: Option<&'a V>) -> ReflectOptionalRef<'a> {
        if let Some(v) = v {
            ReflectOptionalRef::some_from(v)
        } else {
            ReflectOptionalRef::none_from::<V>()
        }
    }

    /// Obtain the value, drop the type.
    pub fn value(&self) -> Option<ReflectValueRef<'a>> {
        match &self.0 {
            Impl::None(_) => None,
            Impl::Some(v) => Some(v.clone()),
        }
    }
}

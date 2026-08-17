use crate::message_dyn::MessageDyn;
use crate::reflect::acc::v2::AccessorV2;
use crate::reflect::ReflectFieldRef;

pub(crate) mod v2;

#[derive(Debug)]
pub(crate) enum GeneratedFieldAccessor {
    V2(AccessorV2),
}

/// Accessor object is constructed in generated code.
/// Should not be used directly.
#[derive(Debug)]
pub struct FieldAccessor {
    pub(crate) _name: &'static str,
    pub(crate) accessor: GeneratedFieldAccessor,
}

impl GeneratedFieldAccessor {
    pub(crate) fn get_reflect<'a>(&self, m: &'a dyn MessageDyn) -> ReflectFieldRef<'a> {
        match self {
            GeneratedFieldAccessor::V2(AccessorV2::Singular(ref a)) => {
                ReflectFieldRef::Optional(a.accessor.get_field(m))
            }
            GeneratedFieldAccessor::V2(AccessorV2::Repeated(ref a)) => {
                ReflectFieldRef::Repeated(a.accessor.get_repeated(m))
            }
            GeneratedFieldAccessor::V2(AccessorV2::Map(ref a)) => {
                ReflectFieldRef::Map(a.accessor.get_reflect(m))
            }
        }
    }
}

impl FieldAccessor {
    pub(crate) fn new(name: &'static str, accessor: AccessorV2) -> FieldAccessor {
        FieldAccessor {
            _name: name,
            accessor: GeneratedFieldAccessor::V2(accessor),
        }
    }
}

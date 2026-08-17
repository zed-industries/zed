use crate::reflect::acc::v2::singular::SingularFieldAccessorHolder;
use crate::reflect::acc::v2::AccessorV2;
use crate::reflect::acc::FieldAccessor;
use crate::reflect::runtime_types::RuntimeTypeWithDeref;
use crate::reflect::ProtobufValue;
use crate::EnumFull;
use crate::EnumOrUnknown;
use crate::MessageFull;

/// Make accessor for `oneof` `message` field
pub fn make_oneof_message_has_get_mut_set_accessor<M, F>(
    name: &'static str,
    has_field: fn(&M) -> bool,
    get_field: for<'a> fn(&'a M) -> &'a F,
    mut_field: for<'a> fn(&'a mut M) -> &'a mut F,
    set_field: fn(&mut M, F),
) -> FieldAccessor
where
    M: MessageFull,
    F: MessageFull,
{
    FieldAccessor::new(
        name,
        AccessorV2::Singular(SingularFieldAccessorHolder::new_has_get_mut_set(
            has_field, get_field, mut_field, set_field,
        )),
    )
}

/// Make accessor for `Copy` field
pub fn make_oneof_copy_has_get_set_simpler_accessors<M, V>(
    name: &'static str,
    has: fn(&M) -> bool,
    get: fn(&M) -> V,
    set: fn(&mut M, V),
) -> FieldAccessor
where
    M: MessageFull,
    V: ProtobufValue + Copy,
{
    FieldAccessor::new(
        name,
        AccessorV2::Singular(SingularFieldAccessorHolder::new_has_get_set(has, get, set)),
    )
}

/// Make accessor for `Copy` field
pub fn make_oneof_enum_accessors<M, E>(
    name: &'static str,
    get: fn(&M) -> Option<EnumOrUnknown<E>>,
    set: fn(&mut M, EnumOrUnknown<E>),
    // TODO: remove this
    _default_value: E,
) -> FieldAccessor
where
    M: MessageFull,
    E: EnumFull,
{
    FieldAccessor::new(
        name,
        AccessorV2::Singular(SingularFieldAccessorHolder::new_get_option_set_enum(
            get, set,
        )),
    )
}

/// Make accessor for `oneof` field
pub fn make_oneof_deref_has_get_set_simpler_accessor<M, F>(
    name: &'static str,
    has: fn(&M) -> bool,
    get: for<'a> fn(&'a M) -> &'a <F::RuntimeType as RuntimeTypeWithDeref>::DerefTarget,
    set: fn(&mut M, F),
) -> FieldAccessor
where
    M: MessageFull + 'static,
    F: ProtobufValue,
    F::RuntimeType: RuntimeTypeWithDeref,
{
    FieldAccessor::new(
        name,
        AccessorV2::Singular(SingularFieldAccessorHolder::new_has_get_set_deref(
            has, get, set,
        )),
    )
}

// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits and essential types intended for blanket imports.

pub use crate::{
    error::ErrorDomain,
    gobject::traits::{DynamicObjectRegisterExt, TypeModuleExt, TypePluginExt},
    object::{Cast, CastNone, IsA, ObjectClassExt, ObjectExt, ObjectType},
    param_spec::{HasParamSpec, ParamSpecBuilderExt, ParamSpecType},
    types::{StaticType, StaticTypeExt},
    value::{ToSendValue, ToValue, ValueType},
    variant::{FixedSizeVariantType, FromVariant, StaticVariantType, ToVariant},
};

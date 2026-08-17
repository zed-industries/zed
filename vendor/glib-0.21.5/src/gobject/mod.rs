// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! GObject bindings

#[allow(unused_imports)]
mod auto;
mod binding;
#[cfg(feature = "v2_72")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_72")))]
mod binding_group;
mod flags;
#[cfg(feature = "v2_74")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_74")))]
mod signal_group;

#[cfg(feature = "v2_72")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_72")))]
pub use binding_group::BindingGroupBuilder;

pub use self::{auto::*, flags::*};
//pub use self::auto::functions::*;

mod interface_info;
pub use interface_info::InterfaceInfo;

mod type_info;
pub use type_info::TypeInfo;

mod type_value_table;
pub use type_value_table::TypeValueTable;

mod type_module;
pub use self::type_module::TypeModule;

mod type_plugin;
pub use self::type_plugin::TypePlugin;

mod dynamic_object;

#[doc(hidden)]
pub mod traits {
    pub use super::dynamic_object::DynamicObjectRegisterExt;
    pub use super::type_module::TypeModuleExt;
    pub use super::type_plugin::TypePluginExt;
}

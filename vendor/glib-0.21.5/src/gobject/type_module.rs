// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{
    enums::{EnumValues, FlagsValues},
    gobject_ffi,
    prelude::*,
    translate::*,
    InterfaceInfo, TypeFlags, TypeInfo, TypePlugin,
};

crate::wrapper! {
    #[doc(alias = "GTypeModule")]
    pub struct TypeModule(Object<gobject_ffi::GTypeModule, gobject_ffi::GTypeModuleClass>) @implements TypePlugin;

    match fn {
        type_ => || gobject_ffi::g_type_module_get_type(),
    }
}

impl TypeModule {
    pub const NONE: Option<&'static TypeModule> = None;
}

pub trait TypeModuleExt: IsA<TypeModule> + 'static {
    #[doc(alias = "g_type_module_add_interface")]
    fn add_interface(
        &self,
        instance_type: crate::types::Type,
        interface_type: crate::types::Type,
        interface_info: &InterfaceInfo,
    ) {
        unsafe {
            gobject_ffi::g_type_module_add_interface(
                self.as_ref().to_glib_none().0,
                instance_type.into_glib(),
                interface_type.into_glib(),
                interface_info.as_ptr(),
            );
        }
    }

    #[doc(alias = "g_type_module_register_enum")]
    fn register_enum(
        &self,
        name: &str,
        const_static_values: &'static EnumValues,
    ) -> crate::types::Type {
        unsafe {
            from_glib(gobject_ffi::g_type_module_register_enum(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
                const_static_values.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_type_module_register_flags")]
    fn register_flags(
        &self,
        name: &str,
        const_static_values: &'static FlagsValues,
    ) -> crate::types::Type {
        unsafe {
            from_glib(gobject_ffi::g_type_module_register_flags(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
                const_static_values.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_type_module_register_type")]
    fn register_type(
        &self,
        parent_type: crate::types::Type,
        type_name: &str,
        type_info: &TypeInfo,
        flags: TypeFlags,
    ) -> crate::types::Type {
        unsafe {
            from_glib(gobject_ffi::g_type_module_register_type(
                self.as_ref().to_glib_none().0,
                parent_type.into_glib(),
                type_name.to_glib_none().0,
                type_info.as_ptr(),
                flags.into_glib(),
            ))
        }
    }

    #[doc(alias = "g_type_module_set_name")]
    fn set_name(&self, name: &str) {
        unsafe {
            gobject_ffi::g_type_module_set_name(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_type_module_unuse")]
    fn unuse(&self) {
        unsafe {
            gobject_ffi::g_type_module_unuse(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "g_type_module_use")]
    #[doc(alias = "use")]
    fn use_(&self) -> bool {
        unsafe {
            from_glib(gobject_ffi::g_type_module_use(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl<O: IsA<TypeModule>> TypeModuleExt for O {}

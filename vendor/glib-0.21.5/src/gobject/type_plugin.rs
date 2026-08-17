// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{gobject_ffi, prelude::*, translate::*, InterfaceInfo, TypeInfo, TypeValueTable};

crate::wrapper! {
    #[doc(alias = "GTypePlugin")]
    pub struct TypePlugin(Interface<gobject_ffi::GTypePlugin, gobject_ffi::GTypePluginClass>);

    match fn {
        type_ => || gobject_ffi::g_type_plugin_get_type(),
    }
}

impl TypePlugin {
    pub const NONE: Option<&'static TypePlugin> = None;
}

pub trait TypePluginExt: IsA<TypePlugin> + 'static {
    #[doc(alias = "g_type_plugin_complete_interface_info")]
    fn complete_interface_info(
        &self,
        instance_type: crate::types::Type,
        interface_type: crate::types::Type,
    ) -> InterfaceInfo {
        let info = InterfaceInfo::default();
        unsafe {
            gobject_ffi::g_type_plugin_complete_interface_info(
                self.as_ref().to_glib_none().0,
                instance_type.into_glib(),
                interface_type.into_glib(),
                info.as_ptr(),
            );
        }
        info
    }

    #[doc(alias = "g_type_plugin_complete_type_info")]
    fn complete_type_info(&self, g_type: crate::types::Type) -> (TypeInfo, TypeValueTable) {
        let info = TypeInfo::default();
        let value_table = TypeValueTable::default();
        unsafe {
            gobject_ffi::g_type_plugin_complete_type_info(
                self.as_ref().to_glib_none().0,
                g_type.into_glib(),
                info.as_ptr(),
                value_table.as_ptr(),
            );
        }
        (info, value_table)
    }

    #[doc(alias = "g_type_plugin_unuse")]
    fn unuse(&self) {
        unsafe {
            gobject_ffi::g_type_plugin_unuse(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "g_type_plugin_use")]
    #[doc(alias = "use")]
    fn use_(&self) {
        unsafe {
            gobject_ffi::g_type_plugin_use(self.as_ref().to_glib_none().0);
        }
    }
}

impl<O: IsA<TypePlugin>> TypePluginExt for O {}

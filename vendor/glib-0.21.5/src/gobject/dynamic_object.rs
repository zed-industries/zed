// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{
    enums::{EnumValues, FlagsValues},
    prelude::*,
    subclass::prelude::*,
    InterfaceInfo, Object, TypeFlags, TypeInfo, TypeModule, TypePlugin,
};

pub trait DynamicObjectRegisterExt: AsRef<TypePlugin> + 'static {
    fn add_dynamic_interface(
        &self,
        instance_type: crate::types::Type,
        interface_type: crate::types::Type,
        interface_info: &InterfaceInfo,
    );

    fn register_dynamic_enum(
        &self,
        name: &str,
        const_static_values: &'static EnumValues,
    ) -> crate::types::Type;

    fn register_dynamic_flags(
        &self,
        name: &str,
        const_static_values: &'static FlagsValues,
    ) -> crate::types::Type;

    fn register_dynamic_type(
        &self,
        parent_type: crate::types::Type,
        type_name: &str,
        type_info: &TypeInfo,
        flags: TypeFlags,
    ) -> crate::types::Type;
}

impl<O: IsA<Object> + IsA<TypePlugin> + ObjectSubclassIsExt> DynamicObjectRegisterExt for O
where
    O::Subclass: TypePluginRegisterImpl,
{
    fn add_dynamic_interface(
        &self,
        instance_type: crate::types::Type,
        interface_type: crate::types::Type,
        interface_info: &InterfaceInfo,
    ) {
        self.imp()
            .add_dynamic_interface(instance_type, interface_type, interface_info);
    }

    fn register_dynamic_enum(
        &self,
        name: &str,
        const_static_values: &'static EnumValues,
    ) -> crate::types::Type {
        self.imp().register_dynamic_enum(name, const_static_values)
    }

    fn register_dynamic_flags(
        &self,
        name: &str,
        const_static_values: &'static FlagsValues,
    ) -> crate::types::Type {
        self.imp().register_dynamic_flags(name, const_static_values)
    }

    fn register_dynamic_type(
        &self,
        parent_type: crate::types::Type,
        type_name: &str,
        type_info: &TypeInfo,
        flags: TypeFlags,
    ) -> crate::types::Type {
        self.imp()
            .register_dynamic_type(parent_type, type_name, type_info, flags)
    }
}

impl DynamicObjectRegisterExt for TypeModule {
    fn add_dynamic_interface(
        &self,
        instance_type: crate::types::Type,
        interface_type: crate::types::Type,
        interface_info: &InterfaceInfo,
    ) {
        <Self as TypeModuleExt>::add_interface(self, instance_type, interface_type, interface_info);
    }

    fn register_dynamic_enum(
        &self,
        name: &str,
        const_static_values: &'static EnumValues,
    ) -> crate::types::Type {
        <Self as TypeModuleExt>::register_enum(self, name, const_static_values)
    }

    fn register_dynamic_flags(
        &self,
        name: &str,
        const_static_values: &'static FlagsValues,
    ) -> crate::types::Type {
        <Self as TypeModuleExt>::register_flags(self, name, const_static_values)
    }

    fn register_dynamic_type(
        &self,
        parent_type: crate::types::Type,
        type_name: &str,
        type_info: &TypeInfo,
        flags: TypeFlags,
    ) -> crate::types::Type {
        <Self as TypeModuleExt>::register_type(self, parent_type, type_name, type_info, flags)
    }
}

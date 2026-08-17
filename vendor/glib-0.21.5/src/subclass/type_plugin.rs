// Take a look at the license at the top of the repository in the LICENSE file.

use crate::enums::{EnumValues, FlagsValues};
use crate::{
    ffi, gobject_ffi, prelude::*, subclass::prelude::*, translate::*, Interface, InterfaceInfo,
    Object, Type, TypeFlags, TypeInfo, TypePlugin, TypeValueTable,
};

pub trait TypePluginImpl: ObjectImpl + ObjectSubclass<Type: IsA<Object> + IsA<TypePlugin>> {
    fn use_plugin(&self) {
        self.parent_use_plugin();
    }

    fn unuse_plugin(&self) {
        self.parent_unuse_plugin();
    }

    fn complete_type_info(&self, type_: Type) -> (TypeInfo, TypeValueTable) {
        self.parent_complete_type_info(type_)
    }

    fn complete_interface_info(&self, instance_type: Type, interface_type: Type) -> InterfaceInfo {
        self.parent_complete_interface_info(instance_type, interface_type)
    }
}

pub trait TypePluginImplExt: TypePluginImpl {
    fn parent_use_plugin(&self);
    fn parent_unuse_plugin(&self);
    fn parent_complete_type_info(&self, type_: Type) -> (TypeInfo, TypeValueTable);
    fn parent_complete_interface_info(
        &self,
        instance_type: Type,
        interface_type: Type,
    ) -> InterfaceInfo;
}

impl<T: TypePluginImpl> TypePluginImplExt for T {
    fn parent_use_plugin(&self) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<TypePlugin>()
                as *const gobject_ffi::GTypePluginClass;

            let f = (*parent_iface)
                .use_plugin
                .expect("no parent \"use_plugin\" implementation");

            f(self.obj().unsafe_cast_ref::<TypePlugin>().to_glib_none().0)
        }
    }

    fn parent_unuse_plugin(&self) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<TypePlugin>()
                as *const gobject_ffi::GTypePluginClass;

            let f = (*parent_iface)
                .unuse_plugin
                .expect("no parent \"unuse_plugin\" implementation");

            f(self.obj().unsafe_cast_ref::<TypePlugin>().to_glib_none().0)
        }
    }

    fn parent_complete_type_info(&self, type_: Type) -> (TypeInfo, TypeValueTable) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<TypePlugin>()
                as *const gobject_ffi::GTypePluginClass;

            let f = (*parent_iface)
                .complete_type_info
                .expect("no parent \"complete_type_info\" implementation");

            let info = TypeInfo::default();
            let value_table = TypeValueTable::default();
            f(
                self.obj().unsafe_cast_ref::<TypePlugin>().to_glib_none().0,
                type_.into_glib(),
                info.as_ptr(),
                value_table.as_ptr(),
            );

            (info, value_table)
        }
    }

    fn parent_complete_interface_info(
        &self,
        instance_type: Type,
        interface_type: Type,
    ) -> InterfaceInfo {
        let info = InterfaceInfo::default();
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<TypePlugin>()
                as *const gobject_ffi::GTypePluginClass;

            let f = (*parent_iface)
                .complete_interface_info
                .expect("no parent \"complete_interface_info\" implementation");

            f(
                self.obj().unsafe_cast_ref::<TypePlugin>().to_glib_none().0,
                instance_type.into_glib(),
                interface_type.into_glib(),
                info.as_ptr(),
            )
        }
        info
    }
}

unsafe impl<T: TypePluginImpl> IsImplementable<T> for TypePlugin {
    fn interface_init(iface: &mut Interface<Self>) {
        let iface = iface.as_mut();

        iface.use_plugin = Some(use_plugin::<T>);
        iface.unuse_plugin = Some(unuse_plugin::<T>);
        iface.complete_type_info = Some(complete_type_info::<T>);
        iface.complete_interface_info = Some(complete_interface_info::<T>);
    }
}

unsafe extern "C" fn use_plugin<T: TypePluginImpl>(type_plugin: *mut gobject_ffi::GTypePlugin) {
    let instance = &*(type_plugin as *mut T::Instance);
    let imp = instance.imp();

    imp.use_plugin();
}

unsafe extern "C" fn unuse_plugin<T: TypePluginImpl>(type_plugin: *mut gobject_ffi::GTypePlugin) {
    let instance = &*(type_plugin as *mut T::Instance);
    let imp = instance.imp();

    imp.unuse_plugin();
}

unsafe extern "C" fn complete_type_info<T: TypePluginImpl>(
    type_plugin: *mut gobject_ffi::GTypePlugin,
    gtype: ffi::GType,
    info_ptr: *mut gobject_ffi::GTypeInfo,
    value_table_ptr: *mut gobject_ffi::GTypeValueTable,
) {
    assert!(!info_ptr.is_null());
    assert!(!value_table_ptr.is_null());
    let instance = &*(type_plugin as *mut T::Instance);
    let imp = instance.imp();
    let type_ = Type::from_glib(gtype);
    let info = TypeInfo::from_glib_ptr_borrow_mut(info_ptr);
    let value_table = TypeValueTable::from_glib_ptr_borrow_mut(value_table_ptr);

    let (info_, value_table_) = imp.complete_type_info(type_);

    *info = info_;
    *value_table = value_table_;
}

unsafe extern "C" fn complete_interface_info<T: TypePluginImpl>(
    type_plugin: *mut gobject_ffi::GTypePlugin,
    instance_gtype: ffi::GType,
    interface_gtype: ffi::GType,
    info_ptr: *mut gobject_ffi::GInterfaceInfo,
) {
    assert!(!info_ptr.is_null());
    let instance = &*(type_plugin as *mut T::Instance);
    let imp = instance.imp();
    let instance_type = Type::from_glib(instance_gtype);
    let interface_type = Type::from_glib(interface_gtype);
    let info = InterfaceInfo::from_glib_ptr_borrow_mut(info_ptr);

    let info_ = imp.complete_interface_info(instance_type, interface_type);
    *info = info_;
}

pub trait TypePluginRegisterImpl:
    TypePluginImpl + ObjectSubclass<Type: IsA<Object> + IsA<TypePlugin>>
{
    fn add_dynamic_interface(
        &self,
        _instance_type: Type,
        _interface_type: Type,
        _interface_info: &InterfaceInfo,
    ) {
        unimplemented!()
    }
    fn register_dynamic_enum(
        &self,
        _name: &str,
        _const_static_values: &'static EnumValues,
    ) -> Type {
        unimplemented!()
    }
    fn register_dynamic_flags(
        &self,
        _name: &str,
        _const_static_values: &'static FlagsValues,
    ) -> Type {
        unimplemented!()
    }
    fn register_dynamic_type(
        &self,
        _parent_type: Type,
        _type_name: &str,
        _type_info: &TypeInfo,
        _flags: TypeFlags,
    ) -> Type {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{self as glib, prelude::TypePluginExt};

    use super::*;

    mod imp {
        use super::*;

        #[derive(Default)]
        pub struct SimplePlugin {
            type_info: std::cell::Cell<Option<TypeInfo>>,
        }

        #[crate::object_subclass]
        impl ObjectSubclass for SimplePlugin {
            const NAME: &'static str = "SimplePlugin";
            type Type = super::SimplePlugin;
            type Interfaces = (TypePlugin,);
        }

        impl ObjectImpl for SimplePlugin {}

        impl TypePluginImpl for SimplePlugin {
            fn use_plugin(&self) {
                // registers types on implementation load
                SimplePluginType::on_implementation_load(self.obj().as_ref());
            }

            fn unuse_plugin(&self) {
                // unregisters types on implementation unload
                SimplePluginType::on_implementation_unload(self.obj().as_ref());
            }

            fn complete_type_info(&self, _type_: Type) -> (TypeInfo, TypeValueTable) {
                assert!(self.type_info.get().is_some());
                // returns type info
                (self.type_info.get().unwrap(), TypeValueTable::default())
            }
        }

        impl TypePluginRegisterImpl for SimplePlugin {
            fn register_dynamic_type(
                &self,
                parent_type: Type,
                type_name: &str,
                type_info: &TypeInfo,
                flags: TypeFlags,
            ) -> Type {
                let type_ = Type::from_name(type_name).unwrap_or_else(|| {
                    Type::register_dynamic(
                        parent_type,
                        type_name,
                        self.obj().upcast_ref::<TypePlugin>(),
                        flags,
                    )
                });
                if type_.is_valid() {
                    // save type info
                    self.type_info.set(Some(*type_info));
                }
                type_
            }
        }

        #[derive(Default)]
        pub struct SimplePluginType;

        #[crate::object_subclass]
        #[object_subclass_dynamic(plugin_type = super::SimplePlugin)]
        impl ObjectSubclass for SimplePluginType {
            const NAME: &'static str = "SimplePluginType";
            type Type = super::SimplePluginType;
        }

        impl ObjectImpl for SimplePluginType {}
    }

    crate::wrapper! {
        pub struct SimplePlugin(ObjectSubclass<imp::SimplePlugin>)
        @implements TypePlugin;
    }

    crate::wrapper! {
        pub struct SimplePluginType(ObjectSubclass<imp::SimplePluginType>);
    }

    #[test]
    fn test_plugin() {
        assert!(!imp::SimplePluginType::type_().is_valid());
        let simple_plugin = crate::Object::new::<SimplePlugin>();
        // simulates the GLib type system to use the plugin.
        TypePluginExt::use_(&simple_plugin);
        assert!(imp::SimplePluginType::type_().is_valid());
        TypePluginExt::unuse(&simple_plugin);
    }
}

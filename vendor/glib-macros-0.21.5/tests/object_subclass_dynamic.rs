// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, subclass::prelude::*};

mod static_ {
    use super::*;

    pub mod imp {
        use super::*;

        // impl for an object interface to register as a static type.
        #[derive(Clone, Copy)]
        #[repr(C)]
        pub struct MyStaticInterfaceClass {
            parent: glib::gobject_ffi::GTypeInterface,
        }

        unsafe impl InterfaceStruct for MyStaticInterfaceClass {
            type Type = MyStaticInterface;
        }

        pub enum MyStaticInterface {}

        #[glib::object_interface]
        impl ObjectInterface for MyStaticInterface {
            const NAME: &'static str = "MyStaticInterface";

            type Interface = MyStaticInterfaceClass;
        }

        pub trait MyStaticInterfaceImpl:
            ObjectImpl + ObjectSubclass<Type: IsA<super::MyStaticInterface>>
        {
        }

        // impl for an object subclass to register as a static type and that implements `MyStaticInterface`.
        #[derive(Default)]
        pub struct MyStaticType;

        #[glib::object_subclass]
        impl ObjectSubclass for MyStaticType {
            const NAME: &'static str = "MyStaticType";
            type Type = super::MyStaticType;
            type Interfaces = (super::MyStaticInterface,);
        }

        impl ObjectImpl for MyStaticType {}

        impl MyStaticInterfaceImpl for MyStaticType {}

        pub trait MyStaticTypeImpl:
            ObjectImpl + ObjectSubclass<Type: IsA<super::MyStaticType>>
        {
        }
    }

    // an object interface to register as a static type.
    glib::wrapper! {
        pub struct MyStaticInterface(ObjectInterface<imp::MyStaticInterface>);
    }

    unsafe impl<T: imp::MyStaticInterfaceImpl> IsImplementable<T> for MyStaticInterface {}

    // an object subclass to register as a static type and that implements `MyStaticInterface`.
    glib::wrapper! {
        pub struct MyStaticType(ObjectSubclass<imp::MyStaticType>) @implements MyStaticInterface;
    }

    unsafe impl<T: imp::MyStaticTypeImpl> IsSubclassable<T> for MyStaticType {}
}

use static_::{
    imp::{MyStaticInterfaceImpl, MyStaticTypeImpl},
    *,
};

mod module {
    use super::*;

    mod imp {
        use super::*;

        // impl for a object interface to register as a dynamic type and that extends `MyStaticInterface`.
        #[derive(Clone, Copy)]
        #[repr(C)]
        pub struct MyModuleInterfaceClass {
            parent: glib::gobject_ffi::GTypeInterface,
        }

        unsafe impl InterfaceStruct for MyModuleInterfaceClass {
            type Type = MyModuleInterface;
        }

        pub enum MyModuleInterface {}

        #[glib::object_interface]
        #[object_interface_dynamic]
        impl ObjectInterface for MyModuleInterface {
            const NAME: &'static str = "MyModuleInterface";
            type Prerequisites = (MyStaticInterface,);
            type Interface = MyModuleInterfaceClass;
        }

        pub trait MyModuleInterfaceImpl:
            ObjectImpl + ObjectSubclass<Type: IsA<super::MyModuleInterface>>
        {
        }

        // impl for an object subclass to register as a dynamic type and that extends `MyStaticType` and that implements `MyStaticInterface` and `MyModuleInterface`.
        #[derive(Default)]
        pub struct MyModuleType;

        #[glib::object_subclass]
        #[object_subclass_dynamic]
        impl ObjectSubclass for MyModuleType {
            const NAME: &'static str = "MyModuleType";
            type Type = super::MyModuleType;
            type ParentType = MyStaticType;
            type Interfaces = (MyStaticInterface, super::MyModuleInterface);
        }

        impl ObjectImpl for MyModuleType {}

        impl MyStaticTypeImpl for MyModuleType {}

        impl MyStaticInterfaceImpl for MyModuleType {}

        impl MyModuleInterfaceImpl for MyModuleType {}

        // impl for an object interface to lazy register as a dynamic type and that extends `MyStaticInterface`.
        #[derive(Clone, Copy)]
        #[repr(C)]
        pub struct MyModuleInterfaceLazyClass {
            parent: glib::gobject_ffi::GTypeInterface,
        }

        unsafe impl InterfaceStruct for MyModuleInterfaceLazyClass {
            type Type = MyModuleInterfaceLazy;
        }

        pub enum MyModuleInterfaceLazy {}

        #[glib::object_interface]
        #[object_interface_dynamic(lazy_registration = true)]
        impl ObjectInterface for MyModuleInterfaceLazy {
            const NAME: &'static str = "MyModuleInterfaceLazy";
            type Prerequisites = (MyStaticInterface,);
            type Interface = MyModuleInterfaceLazyClass;
        }

        pub trait MyModuleInterfaceLazyImpl:
            ObjectImpl + ObjectSubclass<Type: IsA<super::MyModuleInterfaceLazy>>
        {
        }

        // impl for an object subclass to lazy register as a dynamic type and that extends `MyStaticType` and that implements `MyStaticInterface` and `MyModuleInterfaceLazy`.
        #[derive(Default)]
        pub struct MyModuleTypeLazy;

        #[glib::object_subclass]
        #[object_subclass_dynamic(lazy_registration = true)]
        impl ObjectSubclass for MyModuleTypeLazy {
            const NAME: &'static str = "MyModuleTypeLazy";
            type Type = super::MyModuleTypeLazy;
            type ParentType = MyStaticType;
            type Interfaces = (MyStaticInterface, super::MyModuleInterfaceLazy);
        }

        impl ObjectImpl for MyModuleTypeLazy {}

        impl MyStaticTypeImpl for MyModuleTypeLazy {}

        impl MyStaticInterfaceImpl for MyModuleTypeLazy {}

        impl MyModuleInterfaceLazyImpl for MyModuleTypeLazy {}

        // impl for a type module (must extend `glib::TypeModule` and must implement `glib::TypePlugin`).
        #[derive(Default)]
        pub struct MyModule;

        #[glib::object_subclass]
        impl ObjectSubclass for MyModule {
            const NAME: &'static str = "MyModule";
            type Type = super::MyModule;
            type ParentType = glib::TypeModule;
            type Interfaces = (glib::TypePlugin,);
        }

        impl ObjectImpl for MyModule {}

        impl TypePluginImpl for MyModule {}

        impl TypeModuleImpl for MyModule {
            fn load(&self) -> bool {
                // registers object subclasses and interfaces as dynamic types.
                let my_module = self.obj();
                let type_module: &glib::TypeModule = my_module.upcast_ref();
                MyModuleInterface::on_implementation_load(type_module)
                    && MyModuleType::on_implementation_load(type_module)
                    && MyModuleInterfaceLazy::on_implementation_load(type_module)
                    && MyModuleTypeLazy::on_implementation_load(type_module)
            }

            fn unload(&self) {
                // marks object subclasses and interfaces as unregistered.
                let my_module = self.obj();
                let type_module: &glib::TypeModule = my_module.upcast_ref();
                MyModuleTypeLazy::on_implementation_unload(type_module);
                MyModuleInterfaceLazy::on_implementation_unload(type_module);
                MyModuleType::on_implementation_unload(type_module);
                MyModuleInterface::on_implementation_unload(type_module);
            }
        }
    }

    // an object interface to register as a dynamic type and that extends `MyStaticInterface`.
    glib::wrapper! {
        pub struct MyModuleInterface(ObjectInterface<imp::MyModuleInterface>) @requires MyStaticInterface;
    }

    unsafe impl<T: imp::MyModuleInterfaceImpl> IsImplementable<T> for MyModuleInterface {}

    // an object subclass to register as a dynamic type and that extends `MyStaticType` and that implements `MyStaticInterface` and `MyModuleInterface`.
    glib::wrapper! {
        pub struct MyModuleType(ObjectSubclass<imp::MyModuleType>) @extends MyStaticType, @implements MyStaticInterface, MyModuleInterface;
    }

    // an object interface to lazy register as a dynamic type and that extends `MyStaticInterface`.
    glib::wrapper! {
        pub struct MyModuleInterfaceLazy(ObjectInterface<imp::MyModuleInterfaceLazy>) @requires MyStaticInterface;
    }

    unsafe impl<T: imp::MyModuleInterfaceLazyImpl> IsImplementable<T> for MyModuleInterfaceLazy {}

    // an object subclass to lazy register as a dynamic type and that extends `MyStaticType` that implements `MyStaticInterface` and `MyModuleInterfaceLazy`.
    glib::wrapper! {
        pub struct MyModuleTypeLazy(ObjectSubclass<imp::MyModuleTypeLazy>) @extends MyStaticType, @implements MyStaticInterface, MyModuleInterfaceLazy;
    }

    // a module (must extend `glib::TypeModule` and must implement `glib::TypePlugin`).
    glib::wrapper! {
        pub struct MyModule(ObjectSubclass<imp::MyModule>)
        @extends glib::TypeModule, @implements glib::TypePlugin;
    }

    #[test]
    fn dynamic_object_subclasses() {
        use glib::prelude::TypeModuleExt;

        // checks types of object subclasses and of object interfaces to register as dynamic types are invalid (module is not loaded yet).
        assert!(!imp::MyModuleInterface::type_().is_valid());
        assert!(!imp::MyModuleType::type_().is_valid());
        assert!(!imp::MyModuleInterfaceLazy::type_().is_valid());
        assert!(!imp::MyModuleTypeLazy::type_().is_valid());

        // simulates the GLib type system to load/unload the module.
        let module = glib::Object::new::<MyModule>();
        TypeModuleExt::use_(&module);
        TypeModuleExt::unuse(&module);

        // checks types of object subclasses and of object interfaces registered as dynamic types are valid (module is unloaded).
        assert!(imp::MyModuleInterface::type_().is_valid());
        assert!(imp::MyModuleType::type_().is_valid());
        // checks types of object subclasses and of object interfaces that are lazy registered as dynamic types are still invalid (module is unloaded).
        assert!(!imp::MyModuleInterfaceLazy::type_().is_valid());
        assert!(!imp::MyModuleTypeLazy::type_().is_valid());

        // simulates the GLib type system to load the module.
        TypeModuleExt::use_(&module);

        // checks types of object subclasses and of object interfaces registered as dynamic types are valid (module is loaded).
        let iface_type = imp::MyModuleInterface::type_();
        assert!(iface_type.is_valid());
        let obj_type = imp::MyModuleType::type_();
        assert!(obj_type.is_valid());
        let iface_lazy_type = imp::MyModuleInterfaceLazy::type_();
        assert!(iface_lazy_type.is_valid());
        let obj_lazy_type = imp::MyModuleTypeLazy::type_();
        assert!(obj_lazy_type.is_valid());

        // checks plugin of object subclasses and of object interfaces is `MyModule`.
        assert_eq!(
            iface_type.plugin().as_ref(),
            Some(module.upcast_ref::<glib::TypePlugin>())
        );
        assert_eq!(
            obj_type.plugin().as_ref(),
            Some(module.upcast_ref::<glib::TypePlugin>())
        );
        assert_eq!(
            iface_lazy_type.plugin().as_ref(),
            Some(module.upcast_ref::<glib::TypePlugin>())
        );
        assert_eq!(
            obj_lazy_type.plugin().as_ref(),
            Some(module.upcast_ref::<glib::TypePlugin>())
        );

        // simulates the GLib type system to unload the module.
        TypeModuleExt::unuse(&module);

        // checks types of object subclasses and of object interfaces registered as dynamic types are still valid (should have been marked as unloaded by the GLib type system but this cannot be checked).
        assert!(imp::MyModuleInterface::type_().is_valid());
        assert!(imp::MyModuleType::type_().is_valid());
        assert!(imp::MyModuleInterfaceLazy::type_().is_valid());
        assert!(imp::MyModuleTypeLazy::type_().is_valid());

        // simulates the GLib type system to reload the module.
        TypeModuleExt::use_(&module);

        // checks types of object subclasses and of object interfaces registered as dynamic types are still valid (should have been marked as unloaded by the GLib type system but this cannot be checked).
        assert!(imp::MyModuleInterface::type_().is_valid());
        assert!(imp::MyModuleType::type_().is_valid());
        assert!(imp::MyModuleInterfaceLazy::type_().is_valid());
        assert!(imp::MyModuleTypeLazy::type_().is_valid());

        // simulates the GLib type system to unload the module.
        TypeModuleExt::unuse(&module);
    }
}

pub mod plugin {
    use super::*;

    pub mod imp {
        use super::*;
        use std::cell::Cell;

        // impl for a object interface to register as a dynamic type and that extends `MyStaticInterface`.
        #[derive(Clone, Copy)]
        #[repr(C)]
        pub struct MyPluginInterfaceClass {
            parent: glib::gobject_ffi::GTypeInterface,
        }

        unsafe impl InterfaceStruct for MyPluginInterfaceClass {
            type Type = MyPluginInterface;
        }

        pub enum MyPluginInterface {}

        #[glib::object_interface]
        #[object_interface_dynamic(plugin_type = super::MyPlugin)]
        impl ObjectInterface for MyPluginInterface {
            const NAME: &'static str = "MyPluginInterface";
            type Prerequisites = (MyStaticInterface,);
            type Interface = MyPluginInterfaceClass;
        }

        pub trait MyPluginInterfaceImpl:
            ObjectImpl + ObjectSubclass<Type: IsA<super::MyPluginInterface>>
        {
        }

        // impl for an object subclass to register as a dynamic type and that extends `MyStaticType` and that implements `MyStaticInterface` and `MyPluginInterface`.
        #[derive(Default)]
        pub struct MyPluginType;

        #[glib::object_subclass]
        #[object_subclass_dynamic(plugin_type = super::MyPlugin)]
        impl ObjectSubclass for MyPluginType {
            const NAME: &'static str = "MyPluginType";
            type Type = super::MyPluginType;
            type ParentType = MyStaticType;
            type Interfaces = (MyStaticInterface, super::MyPluginInterface);
        }

        impl ObjectImpl for MyPluginType {}

        impl MyStaticTypeImpl for MyPluginType {}

        impl MyStaticInterfaceImpl for MyPluginType {}

        impl MyPluginInterfaceImpl for MyPluginType {}

        // impl for an object interface to lazy register as a dynamic type and that extends `MyStaticInterface`.
        #[derive(Clone, Copy)]
        #[repr(C)]
        pub struct MyPluginInterfaceLazyClass {
            parent: glib::gobject_ffi::GTypeInterface,
        }

        unsafe impl InterfaceStruct for MyPluginInterfaceLazyClass {
            type Type = MyPluginInterfaceLazy;
        }

        pub enum MyPluginInterfaceLazy {}

        #[glib::object_interface]
        #[object_interface_dynamic(plugin_type = super::MyPlugin, lazy_registration = true)]
        impl ObjectInterface for MyPluginInterfaceLazy {
            const NAME: &'static str = "MyPluginInterfaceLazy";
            type Prerequisites = (MyStaticInterface,);
            type Interface = MyPluginInterfaceLazyClass;
        }

        pub trait MyPluginInterfaceLazyImpl:
            ObjectImpl + ObjectSubclass<Type: IsA<super::MyPluginInterfaceLazy>>
        {
        }

        // impl for an object subclass to lazy register as a dynamic type and that extends `MyStaticType` and that implements `MyStaticInterface` and `MyPluginInterfaceLazy`.
        #[derive(Default)]
        pub struct MyPluginTypeLazy;

        #[glib::object_subclass]
        #[object_subclass_dynamic(plugin_type = super::MyPlugin, lazy_registration = true)]
        impl ObjectSubclass for MyPluginTypeLazy {
            const NAME: &'static str = "MyPluginTypeLazy";
            type Type = super::MyPluginTypeLazy;
            type ParentType = MyStaticType;
            type Interfaces = (MyStaticInterface, super::MyPluginInterfaceLazy);
        }

        impl ObjectImpl for MyPluginTypeLazy {}

        impl MyStaticTypeImpl for MyPluginTypeLazy {}

        impl MyStaticInterfaceImpl for MyPluginTypeLazy {}

        impl MyPluginInterfaceLazyImpl for MyPluginTypeLazy {}

        // impl for a type plugin (must implement `glib::TypePlugin`).
        #[derive(Default)]
        pub struct MyPlugin {
            my_interface_type_info: Cell<Option<glib::TypeInfo>>,
            my_interface_iface_info: Cell<Option<glib::InterfaceInfo>>,
            my_type_type_info: Cell<Option<glib::TypeInfo>>,
            my_interface_lazy_type_info: Cell<Option<glib::TypeInfo>>,
            my_interface_lazy_iface_info: Cell<Option<glib::InterfaceInfo>>,
            my_type_lazy_type_info: Cell<Option<glib::TypeInfo>>,
        }

        #[glib::object_subclass]
        impl ObjectSubclass for MyPlugin {
            const NAME: &'static str = "MyPlugin";
            type Type = super::MyPlugin;
            type Interfaces = (glib::TypePlugin,);
        }

        impl ObjectImpl for MyPlugin {}

        impl TypePluginImpl for MyPlugin {
            fn use_plugin(&self) {
                // registers object subclasses and interfaces as dynamic types.
                let my_plugin = self.obj();
                MyPluginInterface::on_implementation_load(my_plugin.as_ref());
                MyPluginType::on_implementation_load(my_plugin.as_ref());
                MyPluginInterfaceLazy::on_implementation_load(my_plugin.as_ref());
                MyPluginTypeLazy::on_implementation_load(my_plugin.as_ref());
            }

            fn unuse_plugin(&self) {
                // marks object subclasses and interfaces as unregistered.
                let my_plugin = self.obj();
                MyPluginTypeLazy::on_implementation_unload(my_plugin.as_ref());
                MyPluginInterfaceLazy::on_implementation_unload(my_plugin.as_ref());
                MyPluginType::on_implementation_unload(my_plugin.as_ref());
                MyPluginInterface::on_implementation_unload(my_plugin.as_ref());
            }

            fn complete_type_info(
                &self,
                type_: glib::Type,
            ) -> (glib::TypeInfo, glib::TypeValueTable) {
                let info = match type_ {
                    type_ if type_ == MyPluginType::type_() => self.my_type_type_info.get(),
                    type_ if type_ == MyPluginInterface::type_() => {
                        self.my_interface_type_info.get()
                    }
                    type_ if type_ == MyPluginTypeLazy::type_() => {
                        self.my_type_lazy_type_info.get()
                    }
                    type_ if type_ == MyPluginInterfaceLazy::type_() => {
                        self.my_interface_lazy_type_info.get()
                    }
                    _ => panic!("unexpected"),
                };
                match info {
                    Some(info) => (info, glib::TypeValueTable::default()),
                    _ => panic!("unexpected"),
                }
            }

            fn complete_interface_info(
                &self,
                _instance_type: glib::Type,
                interface_type: glib::Type,
            ) -> glib::InterfaceInfo {
                let info = match interface_type {
                    type_ if type_ == MyPluginInterface::type_() => {
                        self.my_interface_iface_info.get()
                    }
                    type_ if type_ == MyPluginInterfaceLazy::type_() => {
                        self.my_interface_lazy_iface_info.get()
                    }
                    _ => panic!("unexpected"),
                };
                match info {
                    Some(info) => info,
                    _ => panic!("unexpected"),
                }
            }
        }

        impl TypePluginRegisterImpl for MyPlugin {
            fn add_dynamic_interface(
                &self,
                instance_type: glib::Type,
                interface_type: glib::Type,
                interface_info: &glib::InterfaceInfo,
            ) {
                if !instance_type.is_a(interface_type) {
                    instance_type.add_interface_dynamic(
                        interface_type,
                        self.obj().upcast_ref::<glib::TypePlugin>(),
                    );
                }
                match interface_type {
                    type_ if type_ == imp::MyPluginInterface::type_() => {
                        self.my_interface_iface_info.set(Some(*interface_info))
                    }
                    type_ if type_ == imp::MyPluginInterfaceLazy::type_() => {
                        self.my_interface_lazy_iface_info.set(Some(*interface_info))
                    }
                    _ => panic!("unexpected"),
                };
            }

            fn register_dynamic_type(
                &self,
                parent_type: glib::Type,
                type_name: &str,
                type_info: &glib::TypeInfo,
                flags: glib::TypeFlags,
            ) -> glib::Type {
                let type_ = glib::Type::from_name(type_name).unwrap_or_else(|| {
                    glib::Type::register_dynamic(
                        parent_type,
                        type_name,
                        self.obj().upcast_ref::<glib::TypePlugin>(),
                        flags,
                    )
                });
                if type_.is_valid() {
                    match type_name {
                        imp::MyPluginType::NAME => self.my_type_type_info.set(Some(*type_info)),
                        imp::MyPluginInterface::NAME => {
                            self.my_interface_type_info.set(Some(*type_info))
                        }
                        imp::MyPluginTypeLazy::NAME => {
                            self.my_type_lazy_type_info.set(Some(*type_info))
                        }
                        imp::MyPluginInterfaceLazy::NAME => {
                            self.my_interface_lazy_type_info.set(Some(*type_info))
                        }
                        _ => panic!("unexpected"),
                    };
                }
                type_
            }
        }
    }

    // an object interface to register as a dynamic type and that extends `MyStaticInterface`.
    glib::wrapper! {
        pub struct MyPluginInterface(ObjectInterface<imp::MyPluginInterface>) @requires MyStaticInterface;
    }

    unsafe impl<T: imp::MyPluginInterfaceImpl> IsImplementable<T> for MyPluginInterface {}

    // an object subclass to register as a dynamic type and that extends `MyStaticType` and that implements `MyStaticInterface` and `MyPluginInterface`.
    glib::wrapper! {
        pub struct MyPluginType(ObjectSubclass<imp::MyPluginType>) @extends MyStaticType, @implements MyPluginInterface, MyStaticInterface;
    }

    // an object interface to lazy register as a dynamic type and that extends `MyStaticInterface`.
    glib::wrapper! {
        pub struct MyPluginInterfaceLazy(ObjectInterface<imp::MyPluginInterfaceLazy>) @requires MyStaticInterface;
    }

    unsafe impl<T: imp::MyPluginInterfaceLazyImpl> IsImplementable<T> for MyPluginInterfaceLazy {}

    // an object subclass to lazy register as a dynamic type and that extends `MyStaticType` that implements `MyStaticInterface` and `MyPluginInterfaceLazy`.
    glib::wrapper! {
        pub struct MyPluginTypeLazy(ObjectSubclass<imp::MyPluginTypeLazy>) @extends MyStaticType, @implements MyPluginInterfaceLazy, MyStaticInterface;
    }

    // a plugin (must implement `glib::TypePlugin`).
    glib::wrapper! {
        pub struct MyPlugin(ObjectSubclass<imp::MyPlugin>) @implements glib::TypePlugin;
    }

    #[test]
    fn dynamic_object_subclasses() {
        use glib::prelude::TypePluginExt;

        // checks types of object subclasses and of object interfaces to register as dynamic types are invalid (plugin is not used yet).
        assert!(!imp::MyPluginInterface::type_().is_valid());
        assert!(!imp::MyPluginType::type_().is_valid());
        assert!(!imp::MyPluginInterfaceLazy::type_().is_valid());
        assert!(!imp::MyPluginTypeLazy::type_().is_valid());

        // simulates the GLib type system to use/unuse the plugin.
        let plugin = glib::Object::new::<MyPlugin>();
        TypePluginExt::use_(&plugin);
        TypePluginExt::unuse(&plugin);

        // checks types of object subclasses and of object interfaces registered as dynamic types are valid (plugin is unused).
        assert!(imp::MyPluginInterface::type_().is_valid());
        assert!(imp::MyPluginType::type_().is_valid());
        // check types of object subclasses and of object interfaces that are lazy registered as dynamic types are still invalid (plugin is unused)
        assert!(!imp::MyPluginInterfaceLazy::type_().is_valid());
        assert!(!imp::MyPluginTypeLazy::type_().is_valid());

        // simulates the GLib type system to use the plugin.
        TypePluginExt::use_(&plugin);

        // checks types of object subclasses and of object interfaces registered as dynamic types are valid (plugin is used).
        let iface_type = imp::MyPluginInterface::type_();
        assert!(iface_type.is_valid());
        let obj_type = imp::MyPluginType::type_();
        assert!(obj_type.is_valid());
        let iface_lazy_type = imp::MyPluginInterfaceLazy::type_();
        assert!(iface_lazy_type.is_valid());
        let obj_lazy_type = imp::MyPluginTypeLazy::type_();
        assert!(obj_lazy_type.is_valid());

        // checks plugin of object subclasses and of object interfaces is `MyPlugin`.
        assert_eq!(
            iface_type.plugin().as_ref(),
            Some(plugin.upcast_ref::<glib::TypePlugin>())
        );
        assert_eq!(
            obj_type.plugin().as_ref(),
            Some(plugin.upcast_ref::<glib::TypePlugin>())
        );
        assert_eq!(
            iface_lazy_type.plugin().as_ref(),
            Some(plugin.upcast_ref::<glib::TypePlugin>())
        );
        assert_eq!(
            obj_lazy_type.plugin().as_ref(),
            Some(plugin.upcast_ref::<glib::TypePlugin>())
        );

        // simulates the GLib type system to unuse the plugin.
        TypePluginExt::unuse(&plugin);

        // checks types of object subclasses and of object interfaces registered as dynamic types are still valid.
        assert!(imp::MyPluginInterface::type_().is_valid());
        assert!(imp::MyPluginType::type_().is_valid());
        assert!(imp::MyPluginInterfaceLazy::type_().is_valid());
        assert!(imp::MyPluginTypeLazy::type_().is_valid());

        // simulates the GLib type system to reuse the plugin.
        TypePluginExt::use_(&plugin);

        // checks types of object subclasses and of object interfaces registered as dynamic types are still valid.
        assert!(imp::MyPluginInterface::type_().is_valid());
        assert!(imp::MyPluginType::type_().is_valid());
        assert!(imp::MyPluginInterfaceLazy::type_().is_valid());
        assert!(imp::MyPluginTypeLazy::type_().is_valid());

        // simulates the GLib type system to unuse the plugin.
        TypePluginExt::unuse(&plugin);
    }
}

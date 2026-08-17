// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, subclass::prelude::*};

mod module {
    use super::*;

    mod imp {
        use super::*;

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
                // registers enums as dynamic types.
                let my_module = self.obj();
                let type_module: &glib::TypeModule = my_module.upcast_ref();
                super::MyModuleEnum::on_implementation_load(type_module)
                    && super::MyModuleEnumLazy::on_implementation_load(type_module)
            }

            fn unload(&self) {
                // marks the enums as unregistered.
                let my_module = self.obj();
                let type_module: &glib::TypeModule = my_module.upcast_ref();
                super::MyModuleEnumLazy::on_implementation_unload(type_module);
                super::MyModuleEnum::on_implementation_unload(type_module);
            }
        }
    }

    // an enum to register as a dynamic type.
    #[derive(Debug, Eq, PartialEq, Clone, Copy, glib::Enum)]
    #[repr(u32)]
    #[enum_type(name = "MyModuleEnum")]
    #[enum_dynamic]
    pub enum MyModuleEnum {
        #[enum_value(name = "Foo")]
        Foo,
        Bar,
    }

    // an enum to lazy register as a dynamic type.
    #[derive(Debug, Eq, PartialEq, Clone, Copy, glib::Enum)]
    #[repr(u32)]
    #[enum_type(name = "MyModuleEnumLazy")]
    #[enum_dynamic(lazy_registration = true)]
    pub enum MyModuleEnumLazy {
        #[enum_value(name = "Foo")]
        Foo,
        Bar,
    }

    // a module (must extend `glib::TypeModule` and must implement `glib::TypePlugin`).
    glib::wrapper! {
        pub struct MyModule(ObjectSubclass<imp::MyModule>)
        @extends glib::TypeModule, @implements glib::TypePlugin;
    }

    #[test]
    fn dynamic_enums() {
        // 1st: creates a single module to test with.
        let module = glib::Object::new::<MyModule>();
        // 1st: uses it to test lifecycle of enums registered as dynamic types.
        dynamic_enums_lifecycle(&module);
        // 2nd: uses it to test behavior of enums registered as dynamic types.
        dynamic_enums_behavior(&module);
    }

    // tests lifecycle of enums registered as dynamic types within a module.
    fn dynamic_enums_lifecycle(module: &MyModule) {
        // checks types of enums to register as dynamic types are invalid (module is not loaded yet).
        assert!(!MyModuleEnum::static_type().is_valid());
        assert!(!MyModuleEnumLazy::static_type().is_valid());

        // simulates the GLib type system to load/unload the module.
        TypeModuleExt::use_(module);
        TypeModuleExt::unuse(module);

        // checks types of enums registered as dynamic types are valid (module is unloaded).
        assert!(MyModuleEnum::static_type().is_valid());
        // checks types of enums that are lazy registered as dynamic types are valid (module is unloaded).
        assert!(!MyModuleEnumLazy::static_type().is_valid());

        // simulates the GLib type system to load the module.
        TypeModuleExt::use_(module);

        // checks types of enums registered as dynamic types are valid (module is loaded).
        let enum_type = MyModuleEnum::static_type();
        assert!(enum_type.is_valid());
        let enum_lazy_type = MyModuleEnumLazy::static_type();
        assert!(enum_lazy_type.is_valid());

        // checks plugin of enums registered as dynamic types is `MyModule`.
        assert_eq!(
            enum_type.plugin().as_ref(),
            Some(module.upcast_ref::<glib::TypePlugin>())
        );
        assert_eq!(
            enum_lazy_type.plugin().as_ref(),
            Some(module.upcast_ref::<glib::TypePlugin>())
        );

        // simulates the GLib type system to unload the module.
        TypeModuleExt::unuse(module);

        // checks types of enums registered as dynamic types are still valid (should have been marked as unloaded by the GLib type system but this cannot be checked).
        assert!(MyModuleEnum::static_type().is_valid());
        assert!(MyModuleEnumLazy::static_type().is_valid());

        // simulates the GLib type system to reload the module.
        TypeModuleExt::use_(module);

        // checks types of enums registered as dynamic types are still valid (should have been marked as loaded by the GLib type system but this cannot be checked).
        assert!(MyModuleEnum::static_type().is_valid());
        assert!(MyModuleEnumLazy::static_type().is_valid());

        // simulates the GLib type system to unload the module.
        TypeModuleExt::unuse(module);
    }

    // tests behavior of enums registered as dynamic types within a module.
    fn dynamic_enums_behavior(module: &MyModule) {
        use glib::prelude::*;
        use glib::translate::{FromGlib, IntoGlib};

        // simulates the GLib type system to load the module.
        TypeModuleExt::use_(module);

        assert_eq!(MyModuleEnum::Foo.into_glib(), 0);
        assert_eq!(MyModuleEnum::Bar.into_glib(), 1);

        assert_eq!(unsafe { MyModuleEnum::from_glib(0) }, MyModuleEnum::Foo);
        assert_eq!(unsafe { MyModuleEnum::from_glib(1) }, MyModuleEnum::Bar);

        let t = MyModuleEnum::static_type();
        assert!(t.is_a(glib::Type::ENUM));
        assert_eq!(t.name(), "MyModuleEnum");

        let e = glib::EnumClass::with_type(t).expect("EnumClass::new failed");

        let values = e.values();
        assert_eq!(values.len(), 2);
        assert_eq!(values[0].name(), "Foo");
        assert_eq!(values[0].nick(), "foo");
        assert_eq!(values[1].name(), "Bar");
        assert_eq!(values[1].nick(), "bar");

        let v = e.value(0).expect("EnumClass::get_value(0) failed");
        assert_eq!(v.name(), "Foo");
        assert_eq!(v.nick(), "foo");
        let v = e.value(1).expect("EnumClass::get_value(1) failed");
        assert_eq!(v.name(), "Bar");
        assert_eq!(v.nick(), "bar");
        assert_eq!(e.value(2), None);

        // within enums registered as dynamic types, values are usables only if
        // at least one type class ref exists (see `glib::EnumClass`).
        assert_eq!(
            MyModuleEnum::Foo.to_value().get::<MyModuleEnum>(),
            Ok(MyModuleEnum::Foo)
        );
        assert_eq!(
            MyModuleEnum::Bar.to_value().get::<MyModuleEnum>(),
            Ok(MyModuleEnum::Bar)
        );

        assert_eq!(MyModuleEnumLazy::Foo.into_glib(), 0);
        assert_eq!(MyModuleEnumLazy::Bar.into_glib(), 1);

        assert_eq!(
            unsafe { MyModuleEnumLazy::from_glib(0) },
            MyModuleEnumLazy::Foo
        );
        assert_eq!(
            unsafe { MyModuleEnumLazy::from_glib(1) },
            MyModuleEnumLazy::Bar
        );

        let t = MyModuleEnumLazy::static_type();
        assert!(t.is_a(glib::Type::ENUM));
        assert_eq!(t.name(), "MyModuleEnumLazy");

        let e = glib::EnumClass::with_type(t).expect("EnumClass::new failed");

        let values = e.values();
        assert_eq!(values.len(), 2);
        assert_eq!(values[0].name(), "Foo");
        assert_eq!(values[0].nick(), "foo");
        assert_eq!(values[1].name(), "Bar");
        assert_eq!(values[1].nick(), "bar");

        let v = e.value(0).expect("EnumClass::get_value(0) failed");
        assert_eq!(v.name(), "Foo");
        assert_eq!(v.nick(), "foo");
        let v = e.value(1).expect("EnumClass::get_value(1) failed");
        assert_eq!(v.name(), "Bar");
        assert_eq!(v.nick(), "bar");
        assert_eq!(e.value(2), None);

        // within enums registered as dynamic types, values are usables only if
        // at least one type class ref exists (see `glib::EnumClass`).
        assert_eq!(
            MyModuleEnumLazy::Foo.to_value().get::<MyModuleEnumLazy>(),
            Ok(MyModuleEnumLazy::Foo)
        );
        assert_eq!(
            MyModuleEnumLazy::Bar.to_value().get::<MyModuleEnumLazy>(),
            Ok(MyModuleEnumLazy::Bar)
        );

        // simulates the GLib type system to unload the module.
        TypeModuleExt::unuse(module);
    }
}

pub mod plugin {
    use super::*;

    pub mod imp {
        use glib::EnumClass;

        use super::*;
        use std::cell::Cell;

        // impl for a type plugin (must implement `glib::TypePlugin`).
        #[derive(Default)]
        pub struct MyPlugin {
            my_enum_type_values: Cell<Option<&'static glib::enums::EnumValues>>,
            my_enum_lazy_type_values: Cell<Option<&'static glib::enums::EnumValues>>,
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
                // register enums as dynamic types.
                let my_plugin = self.obj();
                super::MyPluginEnum::on_implementation_load(my_plugin.as_ref());
                super::MyPluginEnumLazy::on_implementation_load(my_plugin.as_ref());
            }

            fn unuse_plugin(&self) {
                // marks enums as unregistered.
                let my_plugin = self.obj();
                super::MyPluginEnumLazy::on_implementation_unload(my_plugin.as_ref());
                super::MyPluginEnum::on_implementation_unload(my_plugin.as_ref());
            }

            fn complete_type_info(
                &self,
                type_: glib::Type,
            ) -> (glib::TypeInfo, glib::TypeValueTable) {
                let enum_type_values = match type_ {
                    type_ if type_ == super::MyPluginEnum::static_type() => {
                        self.my_enum_type_values.get()
                    }
                    type_ if type_ == super::MyPluginEnumLazy::static_type() => {
                        self.my_enum_lazy_type_values.get()
                    }
                    _ => panic!("unexpected type"),
                }
                .expect("enum type values");
                let type_info = EnumClass::complete_type_info(type_, enum_type_values)
                    .expect("EnumClass::complete_type_info failed");
                (type_info, glib::TypeValueTable::default())
            }
        }

        impl TypePluginRegisterImpl for MyPlugin {
            fn register_dynamic_enum(
                &self,
                type_name: &str,
                const_static_values: &'static glib::enums::EnumValues,
            ) -> glib::Type {
                let type_ = glib::Type::from_name(type_name).unwrap_or_else(|| {
                    glib::Type::register_dynamic(
                        glib::Type::ENUM,
                        type_name,
                        self.obj().upcast_ref::<glib::TypePlugin>(),
                        glib::TypeFlags::NONE,
                    )
                });
                if type_.is_valid() {
                    match type_name {
                        "MyPluginEnum" => self.my_enum_type_values.set(Some(const_static_values)),
                        "MyPluginEnumLazy" => {
                            self.my_enum_lazy_type_values.set(Some(const_static_values))
                        }
                        _ => panic!("unexpected"),
                    };
                }
                type_
            }
        }
    }

    // an enum to register as a dynamic type.
    #[derive(Debug, Eq, PartialEq, Clone, Copy, glib::Enum)]
    #[repr(u32)]
    #[enum_type(name = "MyPluginEnum")]
    #[enum_dynamic(plugin_type = MyPlugin)]
    pub enum MyPluginEnum {
        #[enum_value(name = "Foo")]
        Foo,
        Bar,
    }

    // an enum to lazy register as a dynamic type.
    #[derive(Debug, Eq, PartialEq, Clone, Copy, glib::Enum)]
    #[repr(u32)]
    #[enum_type(name = "MyPluginEnumLazy")]
    #[enum_dynamic(plugin_type = MyPlugin, lazy_registration = true)]
    pub enum MyPluginEnumLazy {
        #[enum_value(name = "Foo")]
        Foo,
        Bar,
    }

    // a plugin (must implement `glib::TypePlugin`).
    glib::wrapper! {
        pub struct MyPlugin(ObjectSubclass<imp::MyPlugin>) @implements glib::TypePlugin;
    }

    #[test]
    fn dynamic_enums() {
        // 1st: creates a single plugin to test with.
        let plugin = glib::Object::new::<MyPlugin>();
        // 1st: uses it to test lifecycle of enums registered as dynamic types.
        dynamic_enums_lifecycle(&plugin);
        // 2nd: uses it to test behavior of enums registered as dynamic types.
        dynamic_enums_behavior(&plugin);
    }

    // tests lifecycle of enums registered as dynamic types within a plugin.
    fn dynamic_enums_lifecycle(plugin: &MyPlugin) {
        use glib::prelude::*;

        // checks types of enums to register as dynamic types are invalid (plugin is not used yet).
        assert!(!MyPluginEnum::static_type().is_valid());
        assert!(!MyPluginEnumLazy::static_type().is_valid());

        // simulates the GLib type system to use/unuse the plugin.
        TypePluginExt::use_(plugin);
        TypePluginExt::unuse(plugin);

        // checks types of enums registered as dynamic types are valid (plugin is unused).
        assert!(MyPluginEnum::static_type().is_valid());
        // checks types of enums that are lazy registered as dynamic types are still invalid (plugin is unused).
        assert!(!MyPluginEnumLazy::static_type().is_valid());

        // simulates the GLib type system to use the plugin.
        TypePluginExt::use_(plugin);

        // checks types of enums registered as dynamic types are valid (plugin is used).
        let enum_type = MyPluginEnum::static_type();
        assert!(enum_type.is_valid());
        let enum_lazy_type = MyPluginEnumLazy::static_type();
        assert!(enum_lazy_type.is_valid());

        // checks plugin of enums registered as dynamic types is `MyPlugin`.
        assert_eq!(
            enum_type.plugin().as_ref(),
            Some(plugin.upcast_ref::<glib::TypePlugin>())
        );
        assert_eq!(
            enum_lazy_type.plugin().as_ref(),
            Some(plugin.upcast_ref::<glib::TypePlugin>())
        );

        // simulates the GLib type system to unuse the plugin.
        TypePluginExt::unuse(plugin);

        // checks types of enums registered as dynamic types are still valid.
        assert!(MyPluginEnum::static_type().is_valid());
        assert!(MyPluginEnumLazy::static_type().is_valid());

        // simulates the GLib type system to reuse the plugin.
        TypePluginExt::use_(plugin);

        // checks types of enums registered as dynamic types are still valid.
        assert!(MyPluginEnum::static_type().is_valid());
        assert!(MyPluginEnumLazy::static_type().is_valid());

        // simulates the GLib type system to unuse the plugin.
        TypePluginExt::unuse(plugin);
    }

    // tests behavior of enums registered as dynamic types within a plugin.
    fn dynamic_enums_behavior(plugin: &MyPlugin) {
        use glib::prelude::*;
        use glib::translate::{FromGlib, IntoGlib};

        // simulates the GLib type system to use the plugin.
        TypePluginExt::use_(plugin);

        assert_eq!(MyPluginEnum::Foo.into_glib(), 0);
        assert_eq!(MyPluginEnum::Bar.into_glib(), 1);

        assert_eq!(unsafe { MyPluginEnum::from_glib(0) }, MyPluginEnum::Foo);
        assert_eq!(unsafe { MyPluginEnum::from_glib(1) }, MyPluginEnum::Bar);

        let t = MyPluginEnum::static_type();
        assert!(t.is_a(glib::Type::ENUM));
        assert_eq!(t.name(), "MyPluginEnum");

        let e = glib::EnumClass::with_type(t).expect("EnumClass::new failed");

        let values = e.values();
        assert_eq!(values.len(), 2);
        assert_eq!(values[0].name(), "Foo");
        assert_eq!(values[0].nick(), "foo");
        assert_eq!(values[1].name(), "Bar");
        assert_eq!(values[1].nick(), "bar");

        let v = e.value(0).expect("EnumClass::get_value(0) failed");
        assert_eq!(v.name(), "Foo");
        assert_eq!(v.nick(), "foo");
        let v = e.value(1).expect("EnumClass::get_value(1) failed");
        assert_eq!(v.name(), "Bar");
        assert_eq!(v.nick(), "bar");
        assert_eq!(e.value(2), None);

        // within enums registered as dynamic types, values are usables only if
        // at least one type class ref exists (see `glib::EnumClass`).
        assert_eq!(
            MyPluginEnum::Foo.to_value().get::<MyPluginEnum>(),
            Ok(MyPluginEnum::Foo)
        );
        assert_eq!(
            MyPluginEnum::Bar.to_value().get::<MyPluginEnum>(),
            Ok(MyPluginEnum::Bar)
        );

        assert_eq!(MyPluginEnumLazy::Foo.into_glib(), 0);
        assert_eq!(MyPluginEnumLazy::Bar.into_glib(), 1);

        assert_eq!(
            unsafe { MyPluginEnumLazy::from_glib(0) },
            MyPluginEnumLazy::Foo
        );
        assert_eq!(
            unsafe { MyPluginEnumLazy::from_glib(1) },
            MyPluginEnumLazy::Bar
        );

        let t = MyPluginEnumLazy::static_type();
        assert!(t.is_a(glib::Type::ENUM));
        assert_eq!(t.name(), "MyPluginEnumLazy");

        let e = glib::EnumClass::with_type(t).expect("EnumClass::new failed");

        let values = e.values();
        assert_eq!(values.len(), 2);
        assert_eq!(values[0].name(), "Foo");
        assert_eq!(values[0].nick(), "foo");
        assert_eq!(values[1].name(), "Bar");
        assert_eq!(values[1].nick(), "bar");

        let v = e.value(0).expect("EnumClass::get_value(0) failed");
        assert_eq!(v.name(), "Foo");
        assert_eq!(v.nick(), "foo");
        let v = e.value(1).expect("EnumClass::get_value(1) failed");
        assert_eq!(v.name(), "Bar");
        assert_eq!(v.nick(), "bar");
        assert_eq!(e.value(2), None);

        // within enums registered as dynamic types, values are usables only if
        // at least one type class ref exists (see `glib::EnumClass`).
        assert_eq!(
            MyPluginEnumLazy::Foo.to_value().get::<MyPluginEnumLazy>(),
            Ok(MyPluginEnumLazy::Foo)
        );
        assert_eq!(
            MyPluginEnumLazy::Bar.to_value().get::<MyPluginEnumLazy>(),
            Ok(MyPluginEnumLazy::Bar)
        );

        // simulates the GLib type system to unuse the plugin.
        TypePluginExt::unuse(plugin);
    }
}

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
                // registers flags as dynamic types.
                let my_module = self.obj();
                let type_module: &glib::TypeModule = my_module.upcast_ref();
                super::MyModuleFlags::on_implementation_load(type_module)
                    && super::MyModuleFlagsLazy::on_implementation_load(type_module)
            }

            fn unload(&self) {
                // marks the flags as unregistered.
                let my_module = self.obj();
                let type_module: &glib::TypeModule = my_module.upcast_ref();
                super::MyModuleFlagsLazy::on_implementation_unload(type_module);
                super::MyModuleFlags::on_implementation_unload(type_module);
            }
        }
    }

    // flags to register as a dynamic type.
    #[glib::flags(name = "MyModuleFlags")]
    #[flags_dynamic]
    enum MyModuleFlags {
        #[flags_value(name = "Flag A", nick = "nick-a")]
        A = 0b00000001,
        #[flags_value(name = "Flag B")]
        B = 0b00000010,
        #[flags_value(skip)]
        AB = Self::A.bits() | Self::B.bits(),
        C = 0b00000100,
    }

    // flags to lazy register as a dynamic type.
    #[glib::flags(name = "MyModuleFlagsLazy")]
    #[flags_dynamic(lazy_registration = true)]
    enum MyModuleFlagsLazy {
        #[flags_value(name = "Flag A", nick = "nick-a")]
        A = 0b00000001,
        #[flags_value(name = "Flag B")]
        B = 0b00000010,
        #[flags_value(skip)]
        AB = Self::A.bits() | Self::B.bits(),
        C = 0b00000100,
    }

    // a module (must extend `glib::TypeModule` and must implement `glib::TypePlugin`).
    glib::wrapper! {
        pub struct MyModule(ObjectSubclass<imp::MyModule>)
        @extends glib::TypeModule, @implements glib::TypePlugin;
    }

    #[test]
    fn dynamic_flags() {
        // 1st: creates a single module to test with.
        let module = glib::Object::new::<MyModule>();
        // 1st: uses it to test lifecycle of flags registered as dynamic types.
        dynamic_flags_lifecycle(&module);
        // 2nd: uses it to test behavior of flags registered as dynamic types.
        dynamic_flags_behavior(&module);
    }

    // tests lifecycle of flags registered as dynamic types within a module.
    fn dynamic_flags_lifecycle(module: &MyModule) {
        // checks types of flags to register as dynamic types are invalid (module is not loaded yet).
        assert!(!MyModuleFlags::static_type().is_valid());
        assert!(!MyModuleFlagsLazy::static_type().is_valid());

        // simulates the GLib type system to load/unload the module.
        TypeModuleExt::use_(module);
        TypeModuleExt::unuse(module);

        // checks types of flags registered as dynamic types are valid (module is unloaded).
        assert!(MyModuleFlags::static_type().is_valid());
        // checks types of flags that are lazy registered as dynamic types are valid (module is unloaded).
        assert!(!MyModuleFlagsLazy::static_type().is_valid());

        // simulates the GLib type system to load the module.
        TypeModuleExt::use_(module);

        // checks types of flags registered as dynamic types are valid (module is loaded).
        let flags_type = MyModuleFlags::static_type();
        assert!(flags_type.is_valid());
        let flags_lazy_type = MyModuleFlagsLazy::static_type();
        assert!(flags_lazy_type.is_valid());

        // checks plugin of flags registered as dynamic types is `MyModule`.
        assert_eq!(
            flags_type.plugin().as_ref(),
            Some(module.upcast_ref::<glib::TypePlugin>())
        );
        assert_eq!(
            flags_lazy_type.plugin().as_ref(),
            Some(module.upcast_ref::<glib::TypePlugin>())
        );

        // simulates the GLib type system to unload the module.
        TypeModuleExt::unuse(module);

        // checks types of flags registered as dynamic types are still valid (should have been marked as unloaded by the GLib type system but this cannot be checked).
        assert!(MyModuleFlags::static_type().is_valid());
        assert!(MyModuleFlagsLazy::static_type().is_valid());

        // simulates the GLib type system to reload the module.
        TypeModuleExt::use_(module);

        // checks types of flags registered as dynamic types are still valid (should have been marked as loaded by the GLib type system but this cannot be checked).
        assert!(MyModuleFlags::static_type().is_valid());
        assert!(MyModuleFlagsLazy::static_type().is_valid());

        // simulates the GLib type system to unload the module.
        TypeModuleExt::unuse(module);
    }

    // tests behavior of flags registered as dynamic types within a module.
    fn dynamic_flags_behavior(module: &MyModule) {
        use glib::prelude::*;
        use glib::translate::{FromGlib, IntoGlib};

        // simulates the GLib type system to load the module.
        TypeModuleExt::use_(module);

        assert_eq!(MyModuleFlags::A.bits(), 1);
        assert_eq!(MyModuleFlags::B.bits(), 2);
        assert_eq!(MyModuleFlags::AB.bits(), 3);

        assert_eq!(MyModuleFlags::empty().into_glib(), 0);
        assert_eq!(MyModuleFlags::A.into_glib(), 1);
        assert_eq!(MyModuleFlags::B.into_glib(), 2);
        assert_eq!(MyModuleFlags::AB.into_glib(), 3);

        assert_eq!(
            unsafe { MyModuleFlags::from_glib(0) },
            MyModuleFlags::empty()
        );
        assert_eq!(unsafe { MyModuleFlags::from_glib(1) }, MyModuleFlags::A);
        assert_eq!(unsafe { MyModuleFlags::from_glib(2) }, MyModuleFlags::B);
        assert_eq!(unsafe { MyModuleFlags::from_glib(3) }, MyModuleFlags::AB);

        let t = MyModuleFlags::static_type();
        assert!(t.is_a(glib::Type::FLAGS));
        assert_eq!(t.name(), "MyModuleFlags");

        let e = glib::FlagsClass::with_type(t).expect("FlagsClass::new failed");
        let values = e.values();
        assert_eq!(values.len(), 3);
        assert_eq!(values[0].name(), "Flag A");
        assert_eq!(values[0].nick(), "nick-a");
        assert_eq!(values[1].name(), "Flag B");
        assert_eq!(values[1].nick(), "b");
        assert_eq!(values[2].name(), "C");
        assert_eq!(values[2].nick(), "c");

        let v = e.value(1).expect("FlagsClass::get_value(1) failed");
        assert_eq!(v.name(), "Flag A");
        assert_eq!(v.nick(), "nick-a");
        let v = e.value(2).expect("FlagsClass::get_value(2) failed");
        assert_eq!(v.name(), "Flag B");
        assert_eq!(v.nick(), "b");
        let v = e.value(4).expect("FlagsClass::get_value(4) failed");
        assert_eq!(v.name(), "C");
        assert_eq!(v.nick(), "c");

        // within flags registered as dynamic types, values are usables only if
        // at least one type class ref exists (see `glib::FlagsClass`).
        assert_eq!(
            MyModuleFlags::empty().to_value().get::<MyModuleFlags>(),
            Ok(MyModuleFlags::empty())
        );

        assert_eq!(
            MyModuleFlags::A.to_value().get::<MyModuleFlags>(),
            Ok(MyModuleFlags::A)
        );
        assert_eq!(
            MyModuleFlags::B.to_value().get::<MyModuleFlags>(),
            Ok(MyModuleFlags::B)
        );
        assert_eq!(
            MyModuleFlags::AB.to_value().get::<MyModuleFlags>(),
            Ok(MyModuleFlags::AB)
        );

        assert!(e.value_by_name("Flag A").is_some());
        assert!(e.value_by_name("Flag B").is_some());
        assert!(e.value_by_name("AB").is_none());
        assert!(e.value_by_name("C").is_some());

        assert!(e.value_by_nick("nick-a").is_some());
        assert!(e.value_by_nick("b").is_some());
        assert!(e.value_by_nick("ab").is_none());
        assert!(e.value_by_nick("c").is_some());

        assert_eq!(MyModuleFlagsLazy::A.bits(), 1);
        assert_eq!(MyModuleFlagsLazy::B.bits(), 2);
        assert_eq!(MyModuleFlagsLazy::AB.bits(), 3);

        assert_eq!(MyModuleFlagsLazy::empty().into_glib(), 0);
        assert_eq!(MyModuleFlagsLazy::A.into_glib(), 1);
        assert_eq!(MyModuleFlagsLazy::B.into_glib(), 2);
        assert_eq!(MyModuleFlagsLazy::AB.into_glib(), 3);

        assert_eq!(
            unsafe { MyModuleFlagsLazy::from_glib(0) },
            MyModuleFlagsLazy::empty()
        );
        assert_eq!(
            unsafe { MyModuleFlagsLazy::from_glib(1) },
            MyModuleFlagsLazy::A
        );
        assert_eq!(
            unsafe { MyModuleFlagsLazy::from_glib(2) },
            MyModuleFlagsLazy::B
        );
        assert_eq!(
            unsafe { MyModuleFlagsLazy::from_glib(3) },
            MyModuleFlagsLazy::AB
        );

        let t = MyModuleFlagsLazy::static_type();
        assert!(t.is_a(glib::Type::FLAGS));
        assert_eq!(t.name(), "MyModuleFlagsLazy");

        let e = glib::FlagsClass::with_type(t).expect("FlagsClass::new failed");
        let values = e.values();
        assert_eq!(values.len(), 3);
        assert_eq!(values[0].name(), "Flag A");
        assert_eq!(values[0].nick(), "nick-a");
        assert_eq!(values[1].name(), "Flag B");
        assert_eq!(values[1].nick(), "b");
        assert_eq!(values[2].name(), "C");
        assert_eq!(values[2].nick(), "c");

        let v = e.value(1).expect("FlagsClass::get_value(1) failed");
        assert_eq!(v.name(), "Flag A");
        assert_eq!(v.nick(), "nick-a");
        let v = e.value(2).expect("FlagsClass::get_value(2) failed");
        assert_eq!(v.name(), "Flag B");
        assert_eq!(v.nick(), "b");
        let v = e.value(4).expect("FlagsClass::get_value(4) failed");
        assert_eq!(v.name(), "C");
        assert_eq!(v.nick(), "c");

        // within flags registered as dynamic types, values are usables only if
        // at least one type class ref exists (see `glib::FlagsClass`).
        assert_eq!(
            MyModuleFlagsLazy::empty()
                .to_value()
                .get::<MyModuleFlagsLazy>(),
            Ok(MyModuleFlagsLazy::empty())
        );

        assert_eq!(
            MyModuleFlagsLazy::A.to_value().get::<MyModuleFlagsLazy>(),
            Ok(MyModuleFlagsLazy::A)
        );
        assert_eq!(
            MyModuleFlagsLazy::B.to_value().get::<MyModuleFlagsLazy>(),
            Ok(MyModuleFlagsLazy::B)
        );
        assert_eq!(
            MyModuleFlagsLazy::AB.to_value().get::<MyModuleFlagsLazy>(),
            Ok(MyModuleFlagsLazy::AB)
        );

        assert!(e.value_by_name("Flag A").is_some());
        assert!(e.value_by_name("Flag B").is_some());
        assert!(e.value_by_name("AB").is_none());
        assert!(e.value_by_name("C").is_some());

        assert!(e.value_by_nick("nick-a").is_some());
        assert!(e.value_by_nick("b").is_some());
        assert!(e.value_by_nick("ab").is_none());
        assert!(e.value_by_nick("c").is_some());

        // simulates the GLib type system to unload the module.
        TypeModuleExt::unuse(module);
    }
}

pub mod plugin {
    use super::*;

    pub mod imp {
        use glib::FlagsClass;

        use super::*;
        use std::cell::Cell;

        // impl for a type plugin (must implement `glib::TypePlugin`).
        #[derive(Default)]
        pub struct MyPlugin {
            my_flags_type_values: Cell<Option<&'static glib::enums::FlagsValues>>,
            my_flags_lazy_type_values: Cell<Option<&'static glib::enums::FlagsValues>>,
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
                // register flags as dynamic types.
                let my_plugin = self.obj();
                super::MyPluginFlags::on_implementation_load(my_plugin.as_ref());
                super::MyPluginFlagsLazy::on_implementation_load(my_plugin.as_ref());
            }

            fn unuse_plugin(&self) {
                // marks flags as unregistered.
                let my_plugin = self.obj();
                super::MyPluginFlagsLazy::on_implementation_unload(my_plugin.as_ref());
                super::MyPluginFlags::on_implementation_unload(my_plugin.as_ref());
            }

            fn complete_type_info(
                &self,
                type_: glib::Type,
            ) -> (glib::TypeInfo, glib::TypeValueTable) {
                let flags_type_values = match type_ {
                    type_ if type_ == super::MyPluginFlags::static_type() => {
                        self.my_flags_type_values.get()
                    }
                    type_ if type_ == super::MyPluginFlagsLazy::static_type() => {
                        self.my_flags_lazy_type_values.get()
                    }
                    _ => panic!("unexpected type"),
                }
                .expect("flags type values");
                let type_info = FlagsClass::complete_type_info(type_, flags_type_values)
                    .expect("FlagsClass::type_info failed");
                (type_info, glib::TypeValueTable::default())
            }
        }

        impl TypePluginRegisterImpl for MyPlugin {
            fn register_dynamic_flags(
                &self,
                type_name: &str,
                const_static_values: &'static glib::enums::FlagsValues,
            ) -> glib::Type {
                let type_ = glib::Type::from_name(type_name).unwrap_or_else(|| {
                    glib::Type::register_dynamic(
                        glib::Type::FLAGS,
                        type_name,
                        self.obj().upcast_ref::<glib::TypePlugin>(),
                        glib::TypeFlags::NONE,
                    )
                });
                if type_.is_valid() {
                    match type_name {
                        "MyPluginFlags" => self.my_flags_type_values.set(Some(const_static_values)),
                        "MyPluginFlagsLazy" => self
                            .my_flags_lazy_type_values
                            .set(Some(const_static_values)),
                        _ => panic!("unexpected"),
                    };
                }
                type_
            }
        }
    }

    // flags to register as a dynamic type.
    #[glib::flags(name = "MyPluginFlags")]
    #[flags_dynamic(plugin_type = MyPlugin)]
    enum MyPluginFlags {
        #[flags_value(name = "Flag A", nick = "nick-a")]
        A = 0b00000001,
        #[flags_value(name = "Flag B")]
        B = 0b00000010,
        #[flags_value(skip)]
        AB = Self::A.bits() | Self::B.bits(),
        C = 0b00000100,
    }

    // flags to lazy register as a dynamic type.
    #[glib::flags(name = "MyPluginFlagsLazy")]
    #[flags_dynamic(plugin_type = MyPlugin, lazy_registration = true)]
    enum MyPluginFlagsLazy {
        #[flags_value(name = "Flag A", nick = "nick-a")]
        A = 0b00000001,
        #[flags_value(name = "Flag B")]
        B = 0b00000010,
        #[flags_value(skip)]
        AB = Self::A.bits() | Self::B.bits(),
        C = 0b00000100,
    }

    // a plugin (must implement `glib::TypePlugin`).
    glib::wrapper! {
        pub struct MyPlugin(ObjectSubclass<imp::MyPlugin>) @implements glib::TypePlugin;
    }

    #[test]
    fn dynamic_flags() {
        // 1st: creates a single plugin to test with.
        let plugin = glib::Object::new::<MyPlugin>();
        // 1st: uses it to test lifecycle of flags registered as dynamic types.
        dynamic_flags_lifecycle(&plugin);
        // 2nd: uses it to test behavior of flags registered as dynamic types.
        dynamic_flags_behavior(&plugin);
    }

    // tests lifecycle of flags registered as dynamic types within a plugin.
    fn dynamic_flags_lifecycle(plugin: &MyPlugin) {
        use glib::prelude::*;

        // checks types of flags to register as dynamic types are invalid (plugin is not used yet).
        assert!(!MyPluginFlags::static_type().is_valid());
        assert!(!MyPluginFlagsLazy::static_type().is_valid());

        // simulates the GLib type system to use/unuse the plugin.
        TypePluginExt::use_(plugin);
        TypePluginExt::unuse(plugin);

        // checks types of flags registered as dynamic types are valid (plugin is unused).
        assert!(MyPluginFlags::static_type().is_valid());
        // checks types of flags that are lazy registered as dynamic types are still invalid (plugin is unused).
        assert!(!MyPluginFlagsLazy::static_type().is_valid());

        // simulates the GLib type system to use the plugin.
        TypePluginExt::use_(plugin);

        // checks types of flags registered as dynamic types are valid (plugin is used).
        let flags_type = MyPluginFlags::static_type();
        assert!(flags_type.is_valid());
        let flags_lazy_type = MyPluginFlagsLazy::static_type();
        assert!(flags_lazy_type.is_valid());

        // checks plugin of flags registered as dynamic types is `MyPlugin`.
        assert_eq!(
            flags_type.plugin().as_ref(),
            Some(plugin.upcast_ref::<glib::TypePlugin>())
        );
        assert_eq!(
            flags_lazy_type.plugin().as_ref(),
            Some(plugin.upcast_ref::<glib::TypePlugin>())
        );

        // simulates the GLib type system to unuse the plugin.
        TypePluginExt::unuse(plugin);

        // checks types of flags registered as dynamic types are still valid.
        assert!(MyPluginFlags::static_type().is_valid());
        assert!(MyPluginFlagsLazy::static_type().is_valid());

        // simulates the GLib type system to reuse the plugin.
        TypePluginExt::use_(plugin);

        // checks types of flags registered as dynamic types are still valid.
        assert!(MyPluginFlags::static_type().is_valid());
        assert!(MyPluginFlagsLazy::static_type().is_valid());

        // simulates the GLib type system to unuse the plugin.
        TypePluginExt::unuse(plugin);
    }

    // tests behavior of flags registered as dynamic types within a plugin.
    fn dynamic_flags_behavior(plugin: &MyPlugin) {
        use glib::prelude::*;
        use glib::translate::{FromGlib, IntoGlib};

        // simulates the GLib type system to use the plugin.
        TypePluginExt::use_(plugin);

        assert_eq!(MyPluginFlags::A.bits(), 1);
        assert_eq!(MyPluginFlags::B.bits(), 2);
        assert_eq!(MyPluginFlags::AB.bits(), 3);

        assert_eq!(MyPluginFlags::empty().into_glib(), 0);
        assert_eq!(MyPluginFlags::A.into_glib(), 1);
        assert_eq!(MyPluginFlags::B.into_glib(), 2);
        assert_eq!(MyPluginFlags::AB.into_glib(), 3);

        assert_eq!(
            unsafe { MyPluginFlags::from_glib(0) },
            MyPluginFlags::empty()
        );
        assert_eq!(unsafe { MyPluginFlags::from_glib(1) }, MyPluginFlags::A);
        assert_eq!(unsafe { MyPluginFlags::from_glib(2) }, MyPluginFlags::B);
        assert_eq!(unsafe { MyPluginFlags::from_glib(3) }, MyPluginFlags::AB);

        let t = MyPluginFlags::static_type();
        assert!(t.is_a(glib::Type::FLAGS));
        assert_eq!(t.name(), "MyPluginFlags");

        let e = glib::FlagsClass::with_type(t).expect("FlagsClass::new failed");
        let values = e.values();
        assert_eq!(values.len(), 3);
        assert_eq!(values[0].name(), "Flag A");
        assert_eq!(values[0].nick(), "nick-a");
        assert_eq!(values[1].name(), "Flag B");
        assert_eq!(values[1].nick(), "b");
        assert_eq!(values[2].name(), "C");
        assert_eq!(values[2].nick(), "c");

        let v = e.value(1).expect("FlagsClass::get_value(1) failed");
        assert_eq!(v.name(), "Flag A");
        assert_eq!(v.nick(), "nick-a");
        let v = e.value(2).expect("FlagsClass::get_value(2) failed");
        assert_eq!(v.name(), "Flag B");
        assert_eq!(v.nick(), "b");
        let v = e.value(4).expect("FlagsClass::get_value(4) failed");
        assert_eq!(v.name(), "C");
        assert_eq!(v.nick(), "c");

        // within flags registered as dynamic types, values are usables only if
        // at least one type class ref exists (see `glib::FlagsClass`).
        assert_eq!(
            MyPluginFlags::empty().to_value().get::<MyPluginFlags>(),
            Ok(MyPluginFlags::empty())
        );

        assert_eq!(
            MyPluginFlags::A.to_value().get::<MyPluginFlags>(),
            Ok(MyPluginFlags::A)
        );
        assert_eq!(
            MyPluginFlags::B.to_value().get::<MyPluginFlags>(),
            Ok(MyPluginFlags::B)
        );
        assert_eq!(
            MyPluginFlags::AB.to_value().get::<MyPluginFlags>(),
            Ok(MyPluginFlags::AB)
        );

        assert!(e.value_by_name("Flag A").is_some());
        assert!(e.value_by_name("Flag B").is_some());
        assert!(e.value_by_name("AB").is_none());
        assert!(e.value_by_name("C").is_some());

        assert!(e.value_by_nick("nick-a").is_some());
        assert!(e.value_by_nick("b").is_some());
        assert!(e.value_by_nick("ab").is_none());
        assert!(e.value_by_nick("c").is_some());

        assert_eq!(MyPluginFlagsLazy::A.bits(), 1);
        assert_eq!(MyPluginFlagsLazy::B.bits(), 2);
        assert_eq!(MyPluginFlagsLazy::AB.bits(), 3);

        assert_eq!(MyPluginFlagsLazy::empty().into_glib(), 0);
        assert_eq!(MyPluginFlagsLazy::A.into_glib(), 1);
        assert_eq!(MyPluginFlagsLazy::B.into_glib(), 2);
        assert_eq!(MyPluginFlagsLazy::AB.into_glib(), 3);

        assert_eq!(
            unsafe { MyPluginFlagsLazy::from_glib(0) },
            MyPluginFlagsLazy::empty()
        );
        assert_eq!(
            unsafe { MyPluginFlagsLazy::from_glib(1) },
            MyPluginFlagsLazy::A
        );
        assert_eq!(
            unsafe { MyPluginFlagsLazy::from_glib(2) },
            MyPluginFlagsLazy::B
        );
        assert_eq!(
            unsafe { MyPluginFlagsLazy::from_glib(3) },
            MyPluginFlagsLazy::AB
        );

        let t = MyPluginFlagsLazy::static_type();
        assert!(t.is_a(glib::Type::FLAGS));
        assert_eq!(t.name(), "MyPluginFlagsLazy");

        let e = glib::FlagsClass::with_type(t).expect("FlagsClass::new failed");
        let values = e.values();
        assert_eq!(values.len(), 3);
        assert_eq!(values[0].name(), "Flag A");
        assert_eq!(values[0].nick(), "nick-a");
        assert_eq!(values[1].name(), "Flag B");
        assert_eq!(values[1].nick(), "b");
        assert_eq!(values[2].name(), "C");
        assert_eq!(values[2].nick(), "c");

        let v = e.value(1).expect("FlagsClass::get_value(1) failed");
        assert_eq!(v.name(), "Flag A");
        assert_eq!(v.nick(), "nick-a");
        let v = e.value(2).expect("FlagsClass::get_value(2) failed");
        assert_eq!(v.name(), "Flag B");
        assert_eq!(v.nick(), "b");
        let v = e.value(4).expect("FlagsClass::get_value(4) failed");
        assert_eq!(v.name(), "C");
        assert_eq!(v.nick(), "c");

        // within flags registered as dynamic types, values are usables only if
        // at least one type class ref exists (see `glib::FlagsClass`).
        assert_eq!(
            MyPluginFlagsLazy::empty()
                .to_value()
                .get::<MyPluginFlagsLazy>(),
            Ok(MyPluginFlagsLazy::empty())
        );

        assert_eq!(
            MyPluginFlagsLazy::A.to_value().get::<MyPluginFlagsLazy>(),
            Ok(MyPluginFlagsLazy::A)
        );
        assert_eq!(
            MyPluginFlagsLazy::B.to_value().get::<MyPluginFlagsLazy>(),
            Ok(MyPluginFlagsLazy::B)
        );
        assert_eq!(
            MyPluginFlagsLazy::AB.to_value().get::<MyPluginFlagsLazy>(),
            Ok(MyPluginFlagsLazy::AB)
        );

        assert!(e.value_by_name("Flag A").is_some());
        assert!(e.value_by_name("Flag B").is_some());
        assert!(e.value_by_name("AB").is_none());
        assert!(e.value_by_name("C").is_some());

        assert!(e.value_by_nick("nick-a").is_some());
        assert!(e.value_by_nick("b").is_some());
        assert!(e.value_by_nick("ab").is_none());
        assert!(e.value_by_nick("c").is_some());

        // simulates the GLib type system to unuse the plugin.
        TypePluginExt::unuse(plugin);
    }
}

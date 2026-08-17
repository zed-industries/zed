// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{prelude::*, Binding, Object};

impl Binding {
    #[doc(alias = "get_source")]
    pub fn source(&self) -> Option<Object> {
        self.property("source")
    }

    #[doc(alias = "get_target")]
    pub fn target(&self) -> Option<Object> {
        self.property("target")
    }
}

#[cfg(test)]
mod test {
    use crate::{prelude::*, subclass::prelude::*};

    #[test]
    fn binding() {
        let source = TestObject::default();
        let target = TestObject::default();

        assert!(source.find_property("name").is_some());
        source
            .bind_property("name", &target, "name")
            .bidirectional()
            .build();

        source.set_name("test_source_name");
        assert_eq!(source.name(), target.name());

        target.set_name("test_target_name");
        assert_eq!(source.name(), target.name());
    }

    #[test]
    fn binding_to_transform_with_values() {
        let source = TestObject::default();
        let target = TestObject::default();

        source
            .bind_property("name", &target, "name")
            .sync_create()
            .transform_to_with_values(|_binding, value| {
                let value = value.get::<&str>().unwrap();
                Some(format!("{value} World").to_value())
            })
            .transform_from_with_values(|_binding, value| {
                let value = value.get::<&str>().unwrap();
                Some(format!("{value} World").to_value())
            })
            .build();

        source.set_name("Hello");
        assert_eq!(target.name(), "Hello World");
    }

    #[test]
    fn binding_from_transform_with_values() {
        let source = TestObject::default();
        let target = TestObject::default();

        source
            .bind_property("name", &target, "name")
            .sync_create()
            .bidirectional()
            .transform_to_with_values(|_binding, value| {
                let value = value.get::<&str>().unwrap();
                Some(format!("{value} World").to_value())
            })
            .transform_from_with_values(|_binding, value| {
                let value = value.get::<&str>().unwrap();
                Some(format!("{value} World").to_value())
            })
            .build();

        target.set_name("Hello");
        assert_eq!(source.name(), "Hello World");
    }

    #[test]
    fn binding_to_transform_ref() {
        let source = TestObject::default();
        let target = TestObject::default();

        source
            .bind_property("name", &target, "name")
            .sync_create()
            .transform_to(|_binding, value: &str| Some(format!("{value} World")))
            .transform_from(|_binding, value: &str| Some(format!("{value} World")))
            .build();

        source.set_name("Hello");
        assert_eq!(target.name(), "Hello World");
    }

    #[test]
    fn binding_to_transform_owned_ref() {
        let source = TestObject::default();
        let target = TestObject::default();

        source
            .bind_property("name", &target, "name")
            .sync_create()
            .transform_to(|_binding, value: String| Some(format!("{value} World")))
            .transform_from(|_binding, value: &str| Some(format!("{value} World")))
            .build();

        source.set_name("Hello");
        assert_eq!(target.name(), "Hello World");
    }

    #[test]
    fn binding_from_transform() {
        let source = TestObject::default();
        let target = TestObject::default();

        source
            .bind_property("name", &target, "name")
            .sync_create()
            .bidirectional()
            .transform_to(|_binding, value: &str| Some(format!("{value} World")))
            .transform_from(|_binding, value: &str| Some(format!("{value} World")))
            .build();

        target.set_name("Hello");
        assert_eq!(source.name(), "Hello World");
    }

    #[test]
    fn binding_to_transform_with_values_change_type() {
        let source = TestObject::default();
        let target = TestObject::default();

        source
            .bind_property("name", &target, "enabled")
            .sync_create()
            .transform_to_with_values(|_binding, value| {
                let value = value.get::<&str>().unwrap();
                Some((value == "Hello").to_value())
            })
            .transform_from_with_values(|_binding, value| {
                let value = value.get::<bool>().unwrap();
                Some((if value { "Hello" } else { "World" }).to_value())
            })
            .build();

        source.set_name("Hello");
        assert!(target.enabled());

        source.set_name("Hello World");
        assert!(!target.enabled());
    }

    #[test]
    fn binding_from_transform_values_change_type() {
        let source = TestObject::default();
        let target = TestObject::default();

        source
            .bind_property("name", &target, "enabled")
            .sync_create()
            .bidirectional()
            .transform_to_with_values(|_binding, value| {
                let value = value.get::<&str>().unwrap();
                Some((value == "Hello").to_value())
            })
            .transform_from_with_values(|_binding, value| {
                let value = value.get::<bool>().unwrap();
                Some((if value { "Hello" } else { "World" }).to_value())
            })
            .build();

        target.set_enabled(true);
        assert_eq!(source.name(), "Hello");
        target.set_enabled(false);
        assert_eq!(source.name(), "World");
    }

    #[test]
    fn binding_to_transform_change_type() {
        let source = TestObject::default();
        let target = TestObject::default();

        source
            .bind_property("name", &target, "enabled")
            .sync_create()
            .transform_to(|_binding, value: &str| Some(value == "Hello"))
            .transform_from(|_binding, value: bool| Some(if value { "Hello" } else { "World" }))
            .build();

        source.set_name("Hello");
        assert!(target.enabled());

        source.set_name("Hello World");
        assert!(!target.enabled());
    }

    #[test]
    fn binding_from_transform_change_type() {
        let source = TestObject::default();
        let target = TestObject::default();

        source
            .bind_property("name", &target, "enabled")
            .sync_create()
            .bidirectional()
            .transform_to(|_binding, value: &str| Some(value == "Hello"))
            .transform_from(|_binding, value: bool| Some(if value { "Hello" } else { "World" }))
            .build();

        target.set_enabled(true);
        assert_eq!(source.name(), "Hello");
        target.set_enabled(false);
        assert_eq!(source.name(), "World");
    }

    mod imp {
        use std::{cell::RefCell, sync::OnceLock};

        use super::*;
        use crate as glib;

        #[derive(Debug, Default)]
        pub struct TestObject {
            pub name: RefCell<String>,
            pub enabled: RefCell<bool>,
        }

        #[crate::object_subclass]
        impl ObjectSubclass for TestObject {
            const NAME: &'static str = "TestBinding";
            type Type = super::TestObject;
        }

        impl ObjectImpl for TestObject {
            fn properties() -> &'static [crate::ParamSpec] {
                static PROPERTIES: OnceLock<Vec<crate::ParamSpec>> = OnceLock::new();
                PROPERTIES.get_or_init(|| {
                    vec![
                        crate::ParamSpecString::builder("name")
                            .explicit_notify()
                            .build(),
                        crate::ParamSpecBoolean::builder("enabled")
                            .explicit_notify()
                            .build(),
                    ]
                })
            }

            fn property(&self, _id: usize, pspec: &crate::ParamSpec) -> crate::Value {
                let obj = self.obj();
                match pspec.name() {
                    "name" => obj.name().to_value(),
                    "enabled" => obj.enabled().to_value(),
                    _ => unimplemented!(),
                }
            }

            fn set_property(&self, _id: usize, value: &crate::Value, pspec: &crate::ParamSpec) {
                let obj = self.obj();
                match pspec.name() {
                    "name" => obj.set_name(value.get().unwrap()),
                    "enabled" => obj.set_enabled(value.get().unwrap()),
                    _ => unimplemented!(),
                };
            }
        }
    }

    crate::wrapper! {
        pub struct TestObject(ObjectSubclass<imp::TestObject>);
    }

    impl Default for TestObject {
        fn default() -> Self {
            crate::Object::new()
        }
    }

    impl TestObject {
        fn name(&self) -> String {
            self.imp().name.borrow().clone()
        }

        fn set_name(&self, name: &str) {
            if name != self.imp().name.replace(name.to_string()).as_str() {
                self.notify("name");
            }
        }

        fn enabled(&self) -> bool {
            *self.imp().enabled.borrow()
        }

        fn set_enabled(&self, enabled: bool) {
            if enabled != self.imp().enabled.replace(enabled) {
                self.notify("enabled");
            }
        }
    }
}

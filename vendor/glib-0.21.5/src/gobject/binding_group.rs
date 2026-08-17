// Take a look at the license at the top of the repository in the LICENSE file.

use std::{fmt, ptr};

use crate::{
    ffi, gobject_ffi, object::ObjectRef, prelude::*, translate::*, Binding, BindingFlags,
    BindingGroup, BoolError, Object, ParamSpec, Value,
};

impl BindingGroup {
    #[doc(alias = "bind_with_closures")]
    pub fn bind<'a, O: ObjectType>(
        &'a self,
        source_property: &'a str,
        target: &'a O,
        target_property: &'a str,
    ) -> BindingGroupBuilder<'a> {
        BindingGroupBuilder::new(self, source_property, target, target_property)
    }
}

type TransformFn = Option<Box<dyn Fn(&Binding, &Value) -> Option<Value> + Send + Sync + 'static>>;

// rustdoc-stripper-ignore-next
/// Builder for binding group bindings.
#[must_use = "The builder must be built to be used"]
pub struct BindingGroupBuilder<'a> {
    group: &'a BindingGroup,
    source_property: &'a str,
    target: &'a ObjectRef,
    target_property: &'a str,
    flags: BindingFlags,
    transform_to: TransformFn,
    transform_from: TransformFn,
}

impl fmt::Debug for BindingGroupBuilder<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BindingGroupBuilder")
            .field("group", &self.group)
            .field("source_property", &self.source_property)
            .field("target", &self.target)
            .field("target_property", &self.target_property)
            .field("flags", &self.flags)
            .finish()
    }
}

impl<'a> BindingGroupBuilder<'a> {
    fn new(
        group: &'a BindingGroup,
        source_property: &'a str,
        target: &'a impl ObjectType,
        target_property: &'a str,
    ) -> Self {
        Self {
            group,
            source_property,
            target: target.as_object_ref(),
            target_property,
            flags: BindingFlags::DEFAULT,
            transform_to: None,
            transform_from: None,
        }
    }

    // rustdoc-stripper-ignore-next
    /// Transform changed property values from the target object to the source object with the given closure.
    pub fn transform_from<F: Fn(&Binding, &Value) -> Option<Value> + Send + Sync + 'static>(
        self,
        func: F,
    ) -> Self {
        Self {
            transform_from: Some(Box::new(func)),
            ..self
        }
    }

    // rustdoc-stripper-ignore-next
    /// Transform changed property values from the source object to the target object with the given closure.
    pub fn transform_to<F: Fn(&Binding, &Value) -> Option<Value> + Send + Sync + 'static>(
        self,
        func: F,
    ) -> Self {
        Self {
            transform_to: Some(Box::new(func)),
            ..self
        }
    }

    // rustdoc-stripper-ignore-next
    /// Bind the properties with the given flags.
    pub fn flags(self, flags: BindingFlags) -> Self {
        Self { flags, ..self }
    }

    // rustdoc-stripper-ignore-next
    /// Set the binding flags to [`BIDIRECTIONAL`][crate::BindingFlags::BIDIRECTIONAL].
    pub fn bidirectional(mut self) -> Self {
        self.flags |= crate::BindingFlags::BIDIRECTIONAL;
        self
    }

    // rustdoc-stripper-ignore-next
    /// Set the binding flags to [`SYNC_CREATE`][crate::BindingFlags::SYNC_CREATE].
    pub fn sync_create(mut self) -> Self {
        self.flags |= crate::BindingFlags::SYNC_CREATE;
        self
    }

    // rustdoc-stripper-ignore-next
    /// Set the binding flags to [`INVERT_BOOLEAN`][crate::BindingFlags::INVERT_BOOLEAN].
    pub fn invert_boolean(mut self) -> Self {
        self.flags |= crate::BindingFlags::INVERT_BOOLEAN;
        self
    }

    // rustdoc-stripper-ignore-next
    /// Establish the property binding.
    ///
    /// This fails if the provided properties do not exist.
    pub fn try_build(self) -> Result<(), BoolError> {
        unsafe extern "C" fn transform_to_trampoline(
            binding: *mut gobject_ffi::GBinding,
            from_value: *const gobject_ffi::GValue,
            to_value: *mut gobject_ffi::GValue,
            user_data: ffi::gpointer,
        ) -> ffi::gboolean {
            let transform_data =
                &*(user_data as *const (TransformFn, TransformFn, String, ParamSpec));

            match (transform_data.0.as_ref().unwrap())(
                &from_glib_borrow(binding),
                &*(from_value as *const Value),
            ) {
                None => false,
                Some(res) => {
                    assert!(
                        res.type_().is_a(transform_data.3.value_type()),
                        "Target property {} expected type {} but transform_to function returned {}",
                        transform_data.3.name(),
                        transform_data.3.value_type(),
                        res.type_()
                    );
                    *to_value = res.into_raw();
                    true
                }
            }
            .into_glib()
        }

        unsafe extern "C" fn transform_from_trampoline(
            binding: *mut gobject_ffi::GBinding,
            from_value: *const gobject_ffi::GValue,
            to_value: *mut gobject_ffi::GValue,
            user_data: ffi::gpointer,
        ) -> ffi::gboolean {
            let transform_data =
                &*(user_data as *const (TransformFn, TransformFn, String, ParamSpec));
            let binding = from_glib_borrow(binding);

            match (transform_data.1.as_ref().unwrap())(
                &binding,
                &*(from_value as *const Value),
            ) {
                None => false,
                Some(res) => {
                    let pspec_name = transform_data.2.clone();
                    let source = binding.source().unwrap();
                    let pspec = source.find_property(&pspec_name);
                    assert!(pspec.is_some(), "Source object does not have a property {pspec_name}");
                    let pspec = pspec.unwrap();

                    assert!(
                        res.type_().is_a(pspec.value_type()),
                        "Source property {pspec_name} expected type {} but transform_from function returned {}",
                        pspec.value_type(),
                        res.type_()
                    );
                    *to_value = res.into_raw();
                    true
                }
            }
            .into_glib()
        }

        unsafe extern "C" fn free_transform_data(data: ffi::gpointer) {
            let _ = Box::from_raw(data as *mut (TransformFn, TransformFn, String, ParamSpec));
        }

        let mut _source_property_name_cstr = None;
        let source_property_name = if let Some(source) = self.group.source() {
            let source_property = source.find_property(self.source_property).ok_or_else(|| {
                bool_error!(
                    "Source property {} on type {} not found",
                    self.source_property,
                    source.type_()
                )
            })?;

            // This is NUL-terminated from the C side
            source_property.name().as_ptr()
        } else {
            // This is a Rust &str and needs to be NUL-terminated first
            let source_property_name = std::ffi::CString::new(self.source_property).unwrap();
            let source_property_name_ptr = source_property_name.as_ptr() as *const u8;
            _source_property_name_cstr = Some(source_property_name);

            source_property_name_ptr
        };

        unsafe {
            let target: Object = from_glib_none(self.target.clone().to_glib_none().0);

            let target_property = target.find_property(self.target_property).ok_or_else(|| {
                bool_error!(
                    "Target property {} on type {} not found",
                    self.target_property,
                    target.type_()
                )
            })?;

            let target_property_name = target_property.name().as_ptr();

            let have_transform_to = self.transform_to.is_some();
            let have_transform_from = self.transform_from.is_some();
            let transform_data = if have_transform_to || have_transform_from {
                Box::into_raw(Box::new((
                    self.transform_to,
                    self.transform_from,
                    String::from_glib_none(source_property_name as *const _),
                    target_property,
                )))
            } else {
                ptr::null_mut()
            };

            gobject_ffi::g_binding_group_bind_full(
                self.group.to_glib_none().0,
                source_property_name as *const _,
                target.to_glib_none().0,
                target_property_name as *const _,
                self.flags.into_glib(),
                if have_transform_to {
                    Some(transform_to_trampoline)
                } else {
                    None
                },
                if have_transform_from {
                    Some(transform_from_trampoline)
                } else {
                    None
                },
                transform_data as ffi::gpointer,
                if transform_data.is_null() {
                    None
                } else {
                    Some(free_transform_data)
                },
            );
        }

        Ok(())
    }

    // rustdoc-stripper-ignore-next
    /// Similar to `try_build` but panics instead of failing.
    pub fn build(self) {
        self.try_build().unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::{prelude::*, subclass::prelude::*};

    #[test]
    fn binding_without_source() {
        let binding_group = crate::BindingGroup::new();

        let source = TestObject::default();
        let target = TestObject::default();

        assert!(source.find_property("name").is_some());
        binding_group
            .bind("name", &target, "name")
            .bidirectional()
            .build();

        binding_group.set_source(Some(&source));

        source.set_name("test_source_name");
        assert_eq!(source.name(), target.name());

        target.set_name("test_target_name");
        assert_eq!(source.name(), target.name());
    }

    #[test]
    fn binding_with_source() {
        let binding_group = crate::BindingGroup::new();

        let source = TestObject::default();
        let target = TestObject::default();

        binding_group.set_source(Some(&source));

        binding_group.bind("name", &target, "name").build();

        source.set_name("test_source_name");
        assert_eq!(source.name(), target.name());
    }

    #[test]
    fn binding_to_transform() {
        let binding_group = crate::BindingGroup::new();

        let source = TestObject::default();
        let target = TestObject::default();

        binding_group.set_source(Some(&source));
        binding_group
            .bind("name", &target, "name")
            .sync_create()
            .transform_to(|_binding, value| {
                let value = value.get::<&str>().unwrap();
                Some(format!("{value} World").to_value())
            })
            .transform_from(|_binding, value| {
                let value = value.get::<&str>().unwrap();
                Some(format!("{value} World").to_value())
            })
            .build();

        source.set_name("Hello");
        assert_eq!(target.name(), "Hello World");
    }

    #[test]
    fn binding_from_transform() {
        let binding_group = crate::BindingGroup::new();

        let source = TestObject::default();
        let target = TestObject::default();

        binding_group.set_source(Some(&source));
        binding_group
            .bind("name", &target, "name")
            .sync_create()
            .bidirectional()
            .transform_to(|_binding, value| {
                let value = value.get::<&str>().unwrap();
                Some(format!("{value} World").to_value())
            })
            .transform_from(|_binding, value| {
                let value = value.get::<&str>().unwrap();
                Some(format!("{value} World").to_value())
            })
            .build();

        target.set_name("Hello");
        assert_eq!(source.name(), "Hello World");
    }

    #[test]
    fn binding_to_transform_change_type() {
        let binding_group = crate::BindingGroup::new();

        let source = TestObject::default();
        let target = TestObject::default();

        binding_group.set_source(Some(&source));
        binding_group
            .bind("name", &target, "enabled")
            .sync_create()
            .transform_to(|_binding, value| {
                let value = value.get::<&str>().unwrap();
                Some((value == "Hello").to_value())
            })
            .transform_from(|_binding, value| {
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
    fn binding_from_transform_change_type() {
        let binding_group = crate::BindingGroup::new();

        let source = TestObject::default();
        let target = TestObject::default();

        binding_group.set_source(Some(&source));
        binding_group
            .bind("name", &target, "enabled")
            .sync_create()
            .bidirectional()
            .transform_to(|_binding, value| {
                let value = value.get::<&str>().unwrap();
                Some((value == "Hello").to_value())
            })
            .transform_from(|_binding, value| {
                let value = value.get::<bool>().unwrap();
                Some((if value { "Hello" } else { "World" }).to_value())
            })
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
            const NAME: &'static str = "TestBindingGroup";
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

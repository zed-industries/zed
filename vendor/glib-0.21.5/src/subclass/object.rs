// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Module that contains all types needed for creating a direct subclass of `GObject`
//! or implementing virtual methods of it.

use std::{mem, ptr};

use crate::{
    ffi, gobject_ffi,
    prelude::*,
    subclass::{prelude::*, Signal},
    translate::*,
    Object, ParamSpec, Slice, Value,
};

// rustdoc-stripper-ignore-next
/// Trait for implementors of `glib::Object` subclasses.
///
/// This allows overriding the virtual methods of `glib::Object`. Except for
/// `finalize` as implementing `Drop` would allow the same behavior.
pub trait ObjectImpl: ObjectSubclass<Type: IsA<Object>> {
    // rustdoc-stripper-ignore-next
    /// Properties installed for this type.
    fn properties() -> &'static [ParamSpec] {
        &[]
    }

    // rustdoc-stripper-ignore-next
    /// Signals installed for this type.
    fn signals() -> &'static [Signal] {
        &[]
    }

    // rustdoc-stripper-ignore-next
    /// Property setter.
    ///
    /// This is called whenever the property of this specific subclass with the
    /// given index is set. The new value is passed as `glib::Value`.
    ///
    /// `value` is guaranteed to be of the correct type for the given property.
    fn set_property(&self, _id: usize, _value: &Value, _pspec: &ParamSpec) {
        unimplemented!()
    }

    // rustdoc-stripper-ignore-next
    /// Property getter.
    ///
    /// This is called whenever the property value of the specific subclass with the
    /// given index should be returned.
    ///
    /// The returned `Value` must be of the correct type for the given property.
    #[doc(alias = "get_property")]
    fn property(&self, _id: usize, _pspec: &ParamSpec) -> Value {
        unimplemented!()
    }

    // rustdoc-stripper-ignore-next
    /// Constructed.
    ///
    /// This is called once construction of the instance is finished.
    ///
    /// Should chain up to the parent class' implementation.
    fn constructed(&self) {
        self.parent_constructed();
    }

    // rustdoc-stripper-ignore-next
    /// Disposes of the object.
    ///
    /// When `dispose()` ends, the object should not hold any reference to any other member object.
    /// The object is also expected to be able to answer client method invocations (with possibly an
    /// error code but no memory violation) until it is dropped. `dispose()` can be executed more
    /// than once.
    fn dispose(&self) {}

    // rustdoc-stripper-ignore-next
    /// Function to be called when property change is notified for with
    /// `self.notify("property")`.
    fn notify(&self, pspec: &ParamSpec) {
        self.parent_notify(pspec)
    }

    fn dispatch_properties_changed(&self, pspecs: &[ParamSpec]) {
        self.parent_dispatch_properties_changed(pspecs)
    }
}

#[doc(alias = "get_property")]
unsafe extern "C" fn property<T: ObjectImpl>(
    obj: *mut gobject_ffi::GObject,
    id: u32,
    value: *mut gobject_ffi::GValue,
    pspec: *mut gobject_ffi::GParamSpec,
) {
    let instance = &*(obj as *mut T::Instance);
    let imp = instance.imp();

    let v = imp.property(id as usize, &from_glib_borrow(pspec));

    // We first unset the value we get passed in, in case it contained
    // any previous data. Then we directly overwrite it with our new
    // value, and pass ownership of the contained data to the C GValue
    // by forgetting it on the Rust side.
    //
    // Without this, by using the GValue API, we would have to create
    // a copy of the value when setting it on the destination just to
    // immediately free the original value afterwards.
    gobject_ffi::g_value_unset(value);
    let v = mem::ManuallyDrop::new(v);
    ptr::write(value, ptr::read(v.to_glib_none().0));
}

unsafe extern "C" fn set_property<T: ObjectImpl>(
    obj: *mut gobject_ffi::GObject,
    id: u32,
    value: *mut gobject_ffi::GValue,
    pspec: *mut gobject_ffi::GParamSpec,
) {
    let instance = &*(obj as *mut T::Instance);
    let imp = instance.imp();
    imp.set_property(
        id as usize,
        &*(value as *mut Value),
        &from_glib_borrow(pspec),
    );
}

unsafe extern "C" fn constructed<T: ObjectImpl>(obj: *mut gobject_ffi::GObject) {
    let instance = &*(obj as *mut T::Instance);
    let imp = instance.imp();

    imp.constructed();
}

unsafe extern "C" fn notify<T: ObjectImpl>(
    obj: *mut gobject_ffi::GObject,
    pspec: *mut gobject_ffi::GParamSpec,
) {
    let instance = &*(obj as *mut T::Instance);
    let imp = instance.imp();
    imp.notify(&from_glib_borrow(pspec));
}

unsafe extern "C" fn dispatch_properties_changed<T: ObjectImpl>(
    obj: *mut gobject_ffi::GObject,
    n_pspecs: u32,
    pspecs: *mut *mut gobject_ffi::GParamSpec,
) {
    let instance = &*(obj as *mut T::Instance);
    let imp = instance.imp();
    imp.dispatch_properties_changed(Slice::from_glib_borrow_num(pspecs, n_pspecs as _));
}

unsafe extern "C" fn dispose<T: ObjectImpl>(obj: *mut gobject_ffi::GObject) {
    let instance = &*(obj as *mut T::Instance);
    let imp = instance.imp();

    imp.dispose();

    // Chain up to the parent's dispose.
    let data = T::type_data();
    let parent_class = data.as_ref().parent_class() as *mut gobject_ffi::GObjectClass;
    if let Some(ref func) = (*parent_class).dispose {
        func(obj);
    }
}

// rustdoc-stripper-ignore-next
/// Trait containing only the property related functions of [`ObjectImpl`].
/// Implemented by the [`Properties`](crate::Properties) macro.
/// When implementing `ObjectImpl` you may want to delegate the function calls to this trait.
pub trait DerivedObjectProperties: ObjectSubclass {
    // rustdoc-stripper-ignore-next
    /// Properties installed for this type.
    fn derived_properties() -> &'static [ParamSpec] {
        &[]
    }

    // rustdoc-stripper-ignore-next
    /// Similar to [`ObjectImpl`](trait.ObjectImpl.html) but auto-generated by the [`Properties`](crate::Properties) macro
    /// to allow handling more complex use-cases.
    fn derived_set_property(&self, _id: usize, _value: &Value, _pspec: &ParamSpec) {
        unimplemented!()
    }

    // rustdoc-stripper-ignore-next
    /// Similar to [`ObjectImpl`](trait.ObjectImpl.html) but auto-generated by the [`Properties`](crate::Properties) macro
    /// to allow handling more complex use-cases.
    fn derived_property(&self, _id: usize, _pspec: &ParamSpec) -> Value {
        unimplemented!()
    }
}

// rustdoc-stripper-ignore-next
/// Extension trait for `glib::Object`'s class struct.
///
/// This contains various class methods and allows subclasses to override signal class handlers.
pub unsafe trait ObjectClassSubclassExt: Sized + 'static {
    fn override_signal_class_handler<F>(&mut self, name: &str, class_handler: F)
    where
        F: Fn(&super::SignalClassHandlerToken, &[Value]) -> Option<Value> + Send + Sync + 'static,
    {
        unsafe {
            super::types::signal_override_class_handler(
                name,
                *(self as *mut _ as *mut ffi::GType),
                class_handler,
            );
        }
    }
}

unsafe impl ObjectClassSubclassExt for crate::Class<Object> {}

unsafe impl<T: ObjectImpl> IsSubclassable<T> for Object {
    fn class_init(class: &mut crate::Class<Self>) {
        let klass = class.as_mut();
        klass.set_property = Some(set_property::<T>);
        klass.get_property = Some(property::<T>);
        klass.constructed = Some(constructed::<T>);
        klass.notify = Some(notify::<T>);
        klass.dispatch_properties_changed = Some(dispatch_properties_changed::<T>);
        klass.dispose = Some(dispose::<T>);

        let pspecs = <T as ObjectImpl>::properties();
        if !pspecs.is_empty() {
            unsafe {
                let mut pspecs_ptrs = Vec::with_capacity(pspecs.len() + 1);

                pspecs_ptrs.push(ptr::null_mut());

                for pspec in pspecs {
                    pspecs_ptrs.push(pspec.to_glib_none().0);
                }

                gobject_ffi::g_object_class_install_properties(
                    klass,
                    pspecs_ptrs.len() as u32,
                    pspecs_ptrs.as_mut_ptr(),
                );
            }
        }

        let type_ = T::type_();
        let signals = <T as ObjectImpl>::signals();
        for signal in signals {
            signal.register(type_);
        }
    }

    #[inline]
    fn instance_init(_instance: &mut super::InitializingObject<T>) {}
}

pub trait ObjectImplExt: ObjectImpl {
    // rustdoc-stripper-ignore-next
    /// Chain up to the parent class' implementation of `glib::Object::constructed()`.
    #[inline]
    fn parent_constructed(&self) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut gobject_ffi::GObjectClass;

            if let Some(ref func) = (*parent_class).constructed {
                func(self.obj().unsafe_cast_ref::<Object>().to_glib_none().0);
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Chain up to the parent class' implementation of `glib::Object::notify()`.
    #[inline]
    fn parent_notify(&self, pspec: &ParamSpec) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut gobject_ffi::GObjectClass;

            if let Some(ref func) = (*parent_class).notify {
                func(
                    self.obj().unsafe_cast_ref::<Object>().to_glib_none().0,
                    pspec.to_glib_none().0,
                );
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Chain up to the parent class' implementation of `glib::Object::dispatch_properties_changed()`.
    #[inline]
    fn parent_dispatch_properties_changed(&self, pspecs: &[ParamSpec]) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut gobject_ffi::GObjectClass;

            if let Some(ref func) = (*parent_class).dispatch_properties_changed {
                func(
                    self.obj().unsafe_cast_ref::<Object>().to_glib_none().0,
                    pspecs.len() as _,
                    pspecs.as_ptr() as *mut _,
                );
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Chain up to parent class signal handler.
    fn signal_chain_from_overridden(
        &self,
        token: &super::SignalClassHandlerToken,
        values: &[Value],
    ) -> Option<Value> {
        unsafe {
            super::types::signal_chain_from_overridden(self.obj().as_ptr() as *mut _, token, values)
        }
    }
}

impl<T: ObjectImpl> ObjectImplExt for T {}

#[cfg(test)]
mod test {
    use std::cell::RefCell;

    use super::*;
    // We rename the current crate as glib, since the macros in glib-macros
    // generate the glib namespace through the crate_ident_new utility,
    // and that returns `glib` (and not `crate`) when called inside the glib crate
    use crate as glib;

    mod imp {
        use std::sync::OnceLock;

        use super::*;

        // A dummy `Object` to test setting an `Object` property and returning an `Object` in signals
        #[derive(Default)]
        pub struct ChildObject;

        #[glib::object_subclass]
        impl ObjectSubclass for ChildObject {
            const NAME: &'static str = "ChildObject";
            type Type = super::ChildObject;
        }

        impl ObjectImpl for ChildObject {}

        pub struct SimpleObject {
            name: RefCell<Option<String>>,
            construct_name: RefCell<Option<String>>,
            constructed: RefCell<bool>,
            answer: RefCell<i32>,
            array: RefCell<Vec<String>>,
        }

        impl Default for SimpleObject {
            fn default() -> Self {
                SimpleObject {
                    name: Default::default(),
                    construct_name: Default::default(),
                    constructed: Default::default(),
                    answer: RefCell::new(42i32),
                    array: RefCell::new(vec!["default0".to_string(), "default1".to_string()]),
                }
            }
        }

        #[glib::object_subclass]
        impl ObjectSubclass for SimpleObject {
            const NAME: &'static str = "SimpleObject";
            type Type = super::SimpleObject;
            type Interfaces = (super::Dummy,);
        }

        impl ObjectImpl for SimpleObject {
            fn properties() -> &'static [ParamSpec] {
                static PROPERTIES: OnceLock<Vec<ParamSpec>> = OnceLock::new();
                PROPERTIES.get_or_init(|| {
                    vec![
                        crate::ParamSpecString::builder("name").build(),
                        crate::ParamSpecString::builder("construct-name")
                            .construct_only()
                            .build(),
                        crate::ParamSpecBoolean::builder("constructed")
                            .read_only()
                            .build(),
                        crate::ParamSpecObject::builder::<super::ChildObject>("child").build(),
                        crate::ParamSpecInt::builder("answer")
                            .default_value(42i32)
                            .build(),
                        crate::ParamSpecValueArray::builder("array").build(),
                    ]
                })
            }

            fn signals() -> &'static [super::Signal] {
                static SIGNALS: OnceLock<Vec<super::Signal>> = OnceLock::new();
                SIGNALS.get_or_init(|| {
                    vec![
                        super::Signal::builder("name-changed")
                            .param_types([String::static_type()])
                            .build(),
                        super::Signal::builder("change-name")
                            .param_types([String::static_type()])
                            .return_type::<String>()
                            .action()
                            .class_handler(|args| {
                                let obj = args[0]
                                    .get::<super::SimpleObject>()
                                    .expect("Failed to get Object from args[0]");
                                let new_name = args[1]
                                    .get::<String>()
                                    .expect("Failed to get Object from args[1]");
                                let imp = obj.imp();

                                let old_name = imp.name.replace(Some(new_name));

                                obj.emit_by_name::<()>("name-changed", &[&*imp.name.borrow()]);

                                Some(old_name.to_value())
                            })
                            .build(),
                        super::Signal::builder("create-string")
                            .return_type::<String>()
                            .accumulator(|_hint, acc, val| {
                                // join all strings from signal handlers by newline
                                let mut acc = acc
                                    .get_owned::<Option<String>>()
                                    .unwrap()
                                    .map(|mut acc| {
                                        acc.push('\n');
                                        acc
                                    })
                                    .unwrap_or_default();
                                acc.push_str(val.get::<&str>().unwrap());
                                std::ops::ControlFlow::Continue(acc.to_value())
                            })
                            .build(),
                        super::Signal::builder("create-child-object")
                            .return_type::<super::ChildObject>()
                            .build(),
                        super::Signal::builder("return-string")
                            .return_type::<String>()
                            .action()
                            .class_handler(|args| {
                                let _obj = args[0]
                                    .get::<super::SimpleObject>()
                                    .expect("Failed to get Object from args[0]");
                                Some("base".to_value())
                            })
                            .build(),
                    ]
                })
            }

            fn set_property(&self, _id: usize, value: &Value, pspec: &crate::ParamSpec) {
                match pspec.name() {
                    "name" => {
                        let name = value
                            .get()
                            .expect("type conformity checked by 'Object::set_property'");
                        self.name.replace(name);
                        self.obj()
                            .emit_by_name::<()>("name-changed", &[&*self.name.borrow()]);
                    }
                    "construct-name" => {
                        let name = value
                            .get()
                            .expect("type conformity checked by 'Object::set_property'");
                        self.construct_name.replace(name);
                    }
                    "child" => {
                        // not stored, only used to test `set_property` with `Objects`
                    }
                    "answer" => {
                        let answer = value
                            .get()
                            .expect("type conformity checked by 'Object::set_property'");
                        self.answer.replace(answer);
                    }
                    "array" => {
                        let value = value
                            .get::<crate::ValueArray>()
                            .expect("type conformity checked by 'Object::set_property'");
                        let mut array = self.array.borrow_mut();
                        array.clear();
                        array.extend(value.iter().map(|v| v.get().unwrap()));
                    }
                    _ => unimplemented!(),
                }
            }

            fn property(&self, _id: usize, pspec: &crate::ParamSpec) -> Value {
                match pspec.name() {
                    "name" => self.name.borrow().to_value(),
                    "construct-name" => self.construct_name.borrow().to_value(),
                    "constructed" => self.constructed.borrow().to_value(),
                    "answer" => self.answer.borrow().to_value(),
                    "array" => crate::ValueArray::new(self.array.borrow().iter()).to_value(),
                    _ => unimplemented!(),
                }
            }

            fn constructed(&self) {
                self.parent_constructed();

                debug_assert_eq!(self as *const _, self.obj().imp() as *const _);

                *self.constructed.borrow_mut() = true;
            }
        }

        #[derive(Default)]
        pub struct SimpleSubObject;

        #[glib::object_subclass]
        impl ObjectSubclass for SimpleSubObject {
            const NAME: &'static str = "SimpleSubObject";
            type Type = super::SimpleSubObject;
            type ParentType = super::SimpleObject;

            fn class_init(class: &mut Self::Class) {
                class.override_signal_class_handler("return-string", |token, args| {
                    let obj = args[0]
                        .get::<super::SimpleSubObject>()
                        .expect("Failed to get Object from args[0]");

                    let res = obj.imp().signal_chain_from_overridden(token, args);
                    assert_eq!(res.unwrap().get::<&str>().unwrap(), "base");

                    Some("sub".to_value())
                });
            }
        }

        impl ObjectImpl for SimpleSubObject {}

        impl SimpleObjectImpl for SimpleSubObject {}

        #[derive(Clone, Copy)]
        #[repr(C)]
        pub struct DummyInterface {
            parent: gobject_ffi::GTypeInterface,
        }

        unsafe impl InterfaceStruct for DummyInterface {
            type Type = Dummy;
        }

        pub enum Dummy {}

        #[glib::object_interface]
        impl ObjectInterface for Dummy {
            const NAME: &'static str = "Dummy";
            type Interface = DummyInterface;
        }
    }

    wrapper! {
        pub struct ChildObject(ObjectSubclass<imp::ChildObject>);
    }

    wrapper! {
        pub struct SimpleObject(ObjectSubclass<imp::SimpleObject>);
    }

    pub trait SimpleObjectImpl: ObjectImpl {}

    unsafe impl<Obj: SimpleObjectImpl> IsSubclassable<Obj> for SimpleObject {}

    wrapper! {
        pub struct SimpleSubObject(ObjectSubclass<imp::SimpleSubObject>) @extends SimpleObject;
    }

    wrapper! {
        pub struct Dummy(ObjectInterface<imp::Dummy>);
    }

    unsafe impl<T: ObjectSubclass> IsImplementable<T> for Dummy {}

    #[test]
    fn test_create() {
        let type_ = SimpleObject::static_type();
        let obj = Object::with_type(type_);

        assert!(obj.type_().is_a(Dummy::static_type()));

        // Assert that the object representation is equivalent to the underlying C GObject pointer
        assert_eq!(
            mem::size_of::<SimpleObject>(),
            mem::size_of::<ffi::gpointer>()
        );
        assert_eq!(obj.as_ptr() as ffi::gpointer, unsafe {
            *(&obj as *const _ as *const ffi::gpointer)
        });

        assert!(obj.property::<bool>("constructed"));

        let weak = obj.downgrade();
        drop(obj);
        assert!(weak.upgrade().is_none());
    }

    #[test]
    fn test_sub_create() {
        let obj = Object::builder::<SimpleSubObject>().build();
        assert!(obj.type_().is_a(SimpleObject::static_type()));
        assert_eq!(obj.type_(), SimpleSubObject::static_type());
        assert!(obj.property::<bool>("constructed"));
    }

    #[test]
    fn test_properties() {
        let type_ = SimpleObject::static_type();
        let obj = Object::with_type(type_);

        assert!(obj.type_().is_a(Dummy::static_type()));

        let properties = obj.list_properties();
        assert_eq!(properties.len(), 6);
        assert_eq!(properties[0].name(), "name");
        assert_eq!(properties[1].name(), "construct-name");
        assert_eq!(properties[2].name(), "constructed");
        assert_eq!(properties[3].name(), "child");
        assert_eq!(properties[4].name(), "answer");
        assert_eq!(properties[5].name(), "array");
    }

    #[test]
    fn test_create_child_object() {
        let obj: ChildObject = Object::new();

        assert_eq!(&obj, obj.imp().obj().as_ref());
    }

    #[test]
    fn test_builder() {
        let obj = Object::builder::<SimpleObject>()
            .property("construct-name", "meh")
            .property("name", "initial")
            .build();

        assert_eq!(
            obj.property::<String>("construct-name"),
            String::from("meh")
        );

        assert_eq!(obj.property::<String>("name"), String::from("initial"));
    }

    #[test]
    fn test_set_property() {
        let obj = Object::builder::<SimpleObject>()
            .property("construct-name", "meh")
            .property("name", "initial")
            .build();

        assert_eq!(
            obj.property::<String>("construct-name"),
            String::from("meh")
        );

        assert_eq!(
            obj.property::<String>("construct-name"),
            String::from("meh")
        );

        assert_eq!(obj.property::<String>("name"), String::from("initial"));
        obj.set_property("name", "test");
        assert_eq!(obj.property::<String>("name"), String::from("test"));

        let child = Object::with_type(ChildObject::static_type());
        obj.set_property("child", &child);
    }

    #[test]
    fn builder_property_if() {
        use crate::ValueArray;

        let array = ["val0", "val1"];
        let obj = Object::builder::<SimpleObject>()
            .property_if("name", "some name", true)
            .property_if("answer", 21i32, 6 != 9)
            .property_if("array", ValueArray::new(["val0", "val1"]), array.len() == 2)
            .build();

        assert_eq!(obj.property::<String>("name").as_str(), "some name");
        assert_eq!(
            obj.property::<Option<String>>("name").as_deref(),
            Some("some name")
        );
        assert_eq!(obj.property::<i32>("answer"), 21);
        assert!(obj
            .property::<ValueArray>("array")
            .iter()
            .map(|val| val.get::<&str>().unwrap())
            .eq(array));

        let obj = Object::builder::<SimpleObject>()
            .property_if("name", "some name", false)
            .property_if("answer", 21i32, 6 == 9)
            .property_if("array", ValueArray::new(array), array.len() == 4)
            .build();

        assert!(obj.property::<Option<String>>("name").is_none());
        assert_eq!(obj.property::<i32>("answer"), 42);
        assert!(obj
            .property::<ValueArray>("array")
            .iter()
            .map(|val| val.get::<&str>().unwrap())
            .eq(["default0", "default1"]));
    }

    #[test]
    fn builder_property_if_some() {
        use crate::ValueArray;

        let array = ["val0", "val1"];
        let obj = Object::builder::<SimpleObject>()
            .property_if_some("name", Some("some name"))
            .property_if_some("answer", Some(21i32))
            .property_if_some("array", Some(ValueArray::new(array)))
            .build();

        assert_eq!(obj.property::<String>("name").as_str(), "some name");
        assert_eq!(
            obj.property::<Option<String>>("name").as_deref(),
            Some("some name")
        );
        assert_eq!(obj.property::<i32>("answer"), 21);
        assert!(obj
            .property::<ValueArray>("array")
            .iter()
            .map(|val| val.get::<&str>().unwrap())
            .eq(array));

        let obj = Object::builder::<SimpleObject>()
            .property_if_some("name", Option::<&str>::None)
            .property_if_some("answer", Option::<i32>::None)
            .property_if_some("array", Option::<ValueArray>::None)
            .build();

        assert!(obj.property::<Option<String>>("name").is_none());
        assert_eq!(obj.property::<i32>("answer"), 42);
        assert!(obj
            .property::<ValueArray>("array")
            .iter()
            .map(|val| val.get::<&str>().unwrap())
            .eq(["default0", "default1"]));
    }

    #[test]
    fn builder_property_from_iter() {
        use crate::ValueArray;

        let array = ["val0", "val1"];
        let obj = Object::builder::<SimpleObject>()
            .property_from_iter::<ValueArray>("array", &array)
            .build();

        assert!(obj
            .property::<ValueArray>("array")
            .iter()
            .map(|val| val.get::<&str>().unwrap())
            .eq(array));

        let obj = Object::builder::<SimpleObject>()
            .property_from_iter::<ValueArray>("array", Vec::<&str>::new())
            .build();

        assert!(obj.property::<ValueArray>("array").is_empty());
    }

    #[test]
    fn builder_property_if_not_empty() {
        use crate::ValueArray;

        let array = ["val0", "val1"];
        let obj = Object::builder::<SimpleObject>()
            .property_if_not_empty::<ValueArray>("array", &array)
            .build();

        assert!(obj
            .property::<ValueArray>("array")
            .iter()
            .map(|val| val.get::<&str>().unwrap())
            .eq(array));

        let empty_vec = Vec::<String>::new();
        let obj = Object::builder::<SimpleObject>()
            .property_if_not_empty::<ValueArray>("array", &empty_vec)
            .build();

        assert!(obj
            .property::<ValueArray>("array")
            .iter()
            .map(|val| val.get::<&str>().unwrap())
            .eq(["default0", "default1"]));
    }

    #[test]
    #[should_panic = "property 'construct-name' of type 'SimpleObject' is not writable"]
    fn test_set_property_non_writable() {
        let obj = Object::builder::<SimpleObject>()
            .property("construct-name", "meh")
            .property("name", "initial")
            .build();

        obj.set_property("construct-name", "test");
    }

    #[test]
    #[should_panic = "property 'test' of type 'SimpleObject' not found"]
    fn test_set_property_not_found() {
        let obj = Object::builder::<SimpleObject>()
            .property("construct-name", "meh")
            .property("name", "initial")
            .build();

        obj.set_property("test", true);
    }

    #[test]
    #[should_panic = "property 'constructed' of type 'SimpleObject' is not writable"]
    fn test_set_property_not_writable() {
        let obj = Object::builder::<SimpleObject>()
            .property("construct-name", "meh")
            .property("name", "initial")
            .build();

        obj.set_property("constructed", false);
    }

    #[test]
    #[should_panic = "property 'name' of type 'SimpleObject' can't be set from the given type (expected: 'gchararray', got: 'gboolean')"]
    fn test_set_property_wrong_type() {
        let obj = Object::builder::<SimpleObject>()
            .property("construct-name", "meh")
            .property("name", "initial")
            .build();

        obj.set_property("name", false);
    }

    #[test]
    #[should_panic = "property 'child' of type 'SimpleObject' can't be set from the given type (expected: 'ChildObject', got: 'SimpleObject')"]
    fn test_set_property_wrong_type_2() {
        let obj = Object::builder::<SimpleObject>()
            .property("construct-name", "meh")
            .property("name", "initial")
            .build();

        let other_obj = Object::with_type(SimpleObject::static_type());

        obj.set_property("child", &other_obj);
    }

    #[test]
    #[should_panic = "Can't set construct property 'construct-name' for type 'SimpleObject' twice"]
    fn test_construct_property_set_twice() {
        let _obj = Object::builder::<SimpleObject>()
            .property("construct-name", "meh")
            .property("construct-name", "meh2")
            .build();
    }

    #[test]
    fn test_signals() {
        use std::sync::{
            atomic::{AtomicBool, Ordering},
            Arc,
        };

        let obj = Object::builder::<SimpleObject>()
            .property("name", "old-name")
            .build();

        let name_changed_triggered = Arc::new(AtomicBool::new(false));
        let name_changed_clone = name_changed_triggered.clone();
        obj.connect("name-changed", false, move |args| {
            let _obj = args[0].get::<Object>().expect("Failed to get args[0]");
            let name = args[1].get::<&str>().expect("Failed to get args[1]");

            assert_eq!(name, "new-name");
            name_changed_clone.store(true, Ordering::Relaxed);

            None
        });

        assert_eq!(obj.property::<String>("name"), String::from("old-name"));
        assert!(!name_changed_triggered.load(Ordering::Relaxed));

        assert_eq!(
            obj.emit_by_name::<String>("change-name", &[&"new-name"]),
            "old-name"
        );
        assert!(name_changed_triggered.load(Ordering::Relaxed));

        assert_eq!(obj.emit_by_name::<String>("return-string", &[]), "base");
    }

    #[test]
    fn test_signal_return_expected_type() {
        let obj = Object::with_type(SimpleObject::static_type());

        obj.connect("create-string", false, move |_args| {
            Some("return value 1".to_value())
        });

        obj.connect("create-string", false, move |_args| {
            Some("return value 2".to_value())
        });

        let signal_id = imp::SimpleObject::signals()[2].signal_id();

        let value = obj.emit::<String>(signal_id, &[]);
        assert_eq!(value, "return value 1\nreturn value 2");
    }

    #[test]
    fn test_signal_override() {
        let obj = Object::builder::<SimpleSubObject>().build();

        assert_eq!(obj.emit_by_name::<String>("return-string", &[]), "sub");
    }

    #[test]
    fn test_callback_validity() {
        use std::sync::{
            atomic::{AtomicBool, Ordering},
            Arc,
        };

        let obj = Object::builder::<SimpleObject>()
            .property("name", "old-name")
            .build();

        let name_changed_triggered = Arc::new(AtomicBool::new(false));
        let name_changed_clone = name_changed_triggered.clone();

        obj.connect_notify(Some("name"), move |_, _| {
            name_changed_clone.store(true, Ordering::Relaxed);
        });
        obj.notify("name");
        assert!(name_changed_triggered.load(Ordering::Relaxed));
    }

    // Note: can't test type mismatch in signals since panics across FFI boundaries
    // are UB. See https://github.com/gtk-rs/glib/issues/518

    #[test]
    fn test_signal_return_expected_object_type() {
        let obj = Object::with_type(SimpleObject::static_type());

        obj.connect("create-child-object", false, move |_args| {
            Some(Object::with_type(ChildObject::static_type()).to_value())
        });
        let value: glib::Object = obj.emit_by_name("create-child-object", &[]);
        assert!(value.type_().is_a(ChildObject::static_type()));
    }
}

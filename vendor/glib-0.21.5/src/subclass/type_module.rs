// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{ffi, gobject_ffi, prelude::*, subclass::prelude::*, translate::*, Object, TypeModule};

pub trait TypeModuleImpl: ObjectImpl + ObjectSubclass<Type: IsA<Object> + IsA<TypeModule>> {
    // rustdoc-stripper-ignore-next
    /// Loads the module, registers one or more object subclasses using
    /// [`register_dynamic_type`] and registers one or more object interfaces
    /// using [`register_dynamic_interface`] (see [`TypeModule`]).
    ///
    /// [`register_dynamic_type`]: ../types/fn.register_dynamic_type.html
    /// [`register_dynamic_interface`]: ../interface/fn.register_dynamic_interface.html
    /// [`TypeModule`]: ../../gobject/auto/type_module/struct.TypeModule.html
    fn load(&self) -> bool;

    // rustdoc-stripper-ignore-next
    /// Unloads the module (see [`TypeModuleExt::unuse`]).
    ///
    /// [`TypeModuleExt::unuse`]: ../../gobject/auto/type_module/trait.TypeModuleExt.html#method.unuse
    // rustdoc-stripper-ignore-next-stop
    fn unload(&self);
}

pub trait TypeModuleImplExt: TypeModuleImpl {
    fn parent_load(&self) -> bool;
    fn parent_unload(&self);
}

impl<T: TypeModuleImpl> TypeModuleImplExt for T {
    fn parent_load(&self) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *const gobject_ffi::GTypeModuleClass;

            let f = (*parent_class)
                .load
                .expect("No parent class implementation for \"load\"");

            from_glib(f(self
                .obj()
                .unsafe_cast_ref::<TypeModule>()
                .to_glib_none()
                .0))
        }
    }

    fn parent_unload(&self) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *const gobject_ffi::GTypeModuleClass;

            let f = (*parent_class)
                .unload
                .expect("No parent class implementation for \"unload\"");

            f(self.obj().unsafe_cast_ref::<TypeModule>().to_glib_none().0);
        }
    }
}

unsafe impl<T: TypeModuleImpl> IsSubclassable<T> for TypeModule {
    fn class_init(class: &mut crate::Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.load = Some(load::<T>);
        klass.unload = Some(unload::<T>);
    }
}

unsafe extern "C" fn load<T: TypeModuleImpl>(
    type_module: *mut gobject_ffi::GTypeModule,
) -> ffi::gboolean {
    let instance = &*(type_module as *mut T::Instance);
    let imp = instance.imp();

    let res = imp.load();
    // GLib type system expects a module to never be disposed if types has been
    // successfully loaded.
    // The following code prevents the Rust wrapper (`glib::TypeModule` subclass)
    // to dispose the module when dropped by ensuring the reference count is > 1.
    // Nothing is done if loading types has failed, allowing application to drop
    // and dispose the invalid module.
    if res && (*(type_module as *const gobject_ffi::GObject)).ref_count == 1 {
        unsafe {
            gobject_ffi::g_object_ref(type_module as _);
        }
    }

    res.into_glib()
}

unsafe extern "C" fn unload<T: TypeModuleImpl>(type_module: *mut gobject_ffi::GTypeModule) {
    let instance = &*(type_module as *mut T::Instance);
    let imp = instance.imp();

    imp.unload();
}

#[cfg(test)]
mod tests {
    use crate as glib;

    use super::*;

    mod imp {
        use super::*;

        #[derive(Default)]
        pub struct SimpleModule;

        #[crate::object_subclass]
        impl ObjectSubclass for SimpleModule {
            const NAME: &'static str = "SimpleModule";
            type Type = super::SimpleModule;
            type ParentType = TypeModule;
            type Interfaces = (crate::TypePlugin,);
        }

        impl ObjectImpl for SimpleModule {}

        impl TypePluginImpl for SimpleModule {}

        impl TypeModuleImpl for SimpleModule {
            fn load(&self) -> bool {
                // register types on implementation load
                SimpleModuleType::on_implementation_load(self.obj().upcast_ref::<TypeModule>())
            }

            fn unload(&self) {
                // unregister types on implementation unload
                SimpleModuleType::on_implementation_unload(self.obj().upcast_ref::<TypeModule>());
            }
        }

        #[derive(Default)]
        pub struct SimpleModuleType;

        #[crate::object_subclass]
        #[object_subclass_dynamic]
        impl ObjectSubclass for SimpleModuleType {
            const NAME: &'static str = "SimpleModuleType";
            type Type = super::SimpleModuleType;
        }

        impl ObjectImpl for SimpleModuleType {}
    }

    crate::wrapper! {
        pub struct SimpleModule(ObjectSubclass<imp::SimpleModule>)
        @extends TypeModule, @implements crate::TypePlugin;
    }

    crate::wrapper! {
        pub struct SimpleModuleType(ObjectSubclass<imp::SimpleModuleType>);
    }

    #[test]
    fn test_module() {
        assert!(!imp::SimpleModuleType::type_().is_valid());
        let simple_module = glib::Object::new::<SimpleModule>();
        // simulates the GLib type system to load the module.
        assert!(TypeModuleExt::use_(&simple_module));
        assert!(imp::SimpleModuleType::type_().is_valid());
        TypeModuleExt::unuse(&simple_module);
    }
}

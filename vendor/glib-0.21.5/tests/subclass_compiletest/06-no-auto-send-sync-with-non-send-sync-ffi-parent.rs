glib::wrapper! {
    #[doc(alias = "GInitiallyUnowned")]
    pub struct InitiallyUnowned(Object<glib::gobject_ffi::GInitiallyUnowned, glib::gobject_ffi::GInitiallyUnownedClass>);

    match fn {
        type_ => || glib::gobject_ffi::g_initially_unowned_get_type(),
    }
}

pub trait InitiallyUnownedImpl:
    glib::subclass::prelude::ObjectImpl
    + glib::subclass::prelude::ObjectSubclass<Type: glib::prelude::IsA<InitiallyUnowned>>
{
}

unsafe impl<T: InitiallyUnownedImpl> glib::subclass::prelude::IsSubclassable<T>
    for InitiallyUnowned
{
}

mod imp_object {
    use glib::subclass::prelude::*;

    #[derive(Default)]
    pub struct TestObject {
        s: String,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TestObject {
        const NAME: &'static str = "TestObject";
        type Type = super::TestObject;
        type ParentType = super::InitiallyUnowned;
    }

    impl ObjectImpl for TestObject {}
    impl super::InitiallyUnownedImpl for TestObject {}
}

glib::wrapper! {
    pub struct TestObject(ObjectSubclass<imp_object::TestObject>) @extends InitiallyUnowned;
}

impl Default for TestObject {
    fn default() -> Self {
        glib::Object::new()
    }
}

fn main() {
    fn check<T: Send + Sync>(_obj: &T) {}

    let obj = TestObject::default();
    check(&obj);
}

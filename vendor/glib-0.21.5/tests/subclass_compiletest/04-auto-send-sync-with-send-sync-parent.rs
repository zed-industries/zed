mod imp_parent {
    use glib::subclass::prelude::*;

    #[derive(Default)]
    pub struct TestParent {
        s: String,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TestParent {
        const NAME: &'static str = "TestParent";
        type Type = super::TestParent;
    }

    impl ObjectImpl for TestParent {}
}

glib::wrapper! {
    pub struct TestParent(ObjectSubclass<imp_parent::TestParent>);
}

pub trait TestParentImpl:
    Send
    + Sync
    + glib::subclass::prelude::ObjectImpl
    + glib::subclass::prelude::ObjectSubclass<Type: glib::prelude::IsA<TestParent>>
{
}

unsafe impl<T: TestParentImpl> glib::subclass::prelude::IsSubclassable<T> for TestParent {}

impl Default for TestParent {
    fn default() -> Self {
        glib::Object::new()
    }
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
        type ParentType = super::TestParent;
    }

    impl ObjectImpl for TestObject {}
    impl super::TestParentImpl for TestObject {}
}

glib::wrapper! {
    pub struct TestObject(ObjectSubclass<imp_object::TestObject>) @extends TestParent;
}

impl Default for TestObject {
    fn default() -> Self {
        glib::Object::new()
    }
}

fn main() {
    fn check<T: Send + Sync>(_obj: &T) {}

    let obj = TestParent::default();
    check(&obj);

    let obj = TestObject::default();
    check(&obj);
}

mod imp {
    use glib::subclass::prelude::*;
    use std::cell::RefCell;

    #[derive(Default)]
    pub struct TestObject {
        s: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TestObject {
        const NAME: &'static str = "TestObject";
        type Type = super::TestObject;
    }

    impl ObjectImpl for TestObject {}
}

glib::wrapper! {
    pub struct TestObject(ObjectSubclass<imp::TestObject>);
}

impl Default for TestObject {
    fn default() -> Self {
        glib::Object::new()
    }
}

fn main() {
    fn check<T: Send>(_obj: &T) {}

    let obj = TestObject::default();
    check(&obj);
}

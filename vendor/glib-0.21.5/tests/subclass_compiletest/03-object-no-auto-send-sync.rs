fn main() {
    fn check<T: Send + Sync>(_obj: &T) {}

    let obj = glib::Object::new::<glib::Object>();
    check(&obj);
}

use std::marker::PhantomData;

use glib::{Downgrade, Object};

#[test]
fn derive_downgrade() {
    #[derive(Downgrade)]
    #[allow(dead_code)]
    pub struct NewType(Object);

    #[derive(Downgrade)]
    #[allow(dead_code)]
    pub struct Struct {
        o1: Object,
        o2: std::rc::Rc<u32>,
    }

    #[derive(Downgrade)]
    #[allow(dead_code)]
    pub enum Enum {
        None,
        Pair { x: Object, y: Object },
        Unit(),
        SingleUnnamed(Object),
        MultipleUnnamed(Object, Object, Object),
    }

    #[derive(Downgrade)]
    #[allow(dead_code)]
    pub struct TypedWrapper<T>(Object, PhantomData<T>);

    #[derive(Downgrade)]
    #[allow(dead_code)]
    pub enum TypedEnum<T> {
        This(Object, PhantomData<T>),
        That(Object, PhantomData<T>),
    }
}

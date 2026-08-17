use crate::prelude::*;
use core::pin::Pin;

#[test]
fn new_method_special_case() {
    struct Sut;

    #[bon]
    impl Sut {
        #[builder]
        fn new() {}
    }

    let _: SutBuilder = Sut::builder();
    let builder: SutBuilder<sut_builder::Empty> = Sut::builder();

    builder.build();
}

#[test]
fn builder_method_special_case() {
    struct Sut;

    #[bon]
    impl Sut {
        #[builder]
        fn builder() {}
    }

    let _: SutBuilder = Sut::builder();
    let builder: SutBuilder<sut_builder::Empty> = Sut::builder();

    builder.build();
}

#[test]
fn builder_start_fn_is_not_special_case() {
    struct Sut;

    #[bon]
    impl Sut {
        #[builder(start_fn = builder)]
        fn some_other_name() {}
    }

    let _: SutSomeOtherNameBuilder = Sut::builder();
    let builder: SutSomeOtherNameBuilder<sut_some_other_name_builder::Empty> = Sut::builder();

    builder.call();

    Sut::some_other_name();
}

#[test]
fn receiver_variations() {
    #[derive(Clone)]
    struct Sut {
        field: u32,
    }

    #[bon]
    impl Sut {
        #[builder]
        #[allow(clippy::use_self)]
        fn self_as_ref_sut(self: &Sut) -> u32 {
            self.field
        }

        #[builder]
        fn mut_self(mut self) -> Self {
            self.field += 1;
            self
        }

        #[builder]
        fn ref_mut_self(&mut self) {
            self.field += 1;
        }

        #[builder]
        fn mut_self_ref_self(#[allow(clippy::needless_arbitrary_self_type)] mut self: &Self) {
            #[allow(clippy::self_assignment, unused_assignments)]
            {
                self = &Self { field: 43 };
            }
        }

        #[builder]
        fn mut_self_as_ref_mut_self(
            #[allow(clippy::needless_arbitrary_self_type)] mut self: &mut Self,
        ) {
            self.field = 45;
            #[allow(clippy::self_assignment, unused_assignments, dead_code)]
            {
                self = self;
            }
        }

        #[builder]
        fn self_as_pin_mut_self(self: Pin<&mut Self>) {
            let _ = self;
        }

        #[builder]
        fn mut_self_as_pin_mut_self(mut self: Pin<&mut Self>) {
            #[allow(clippy::self_assignment, unused_assignments, dead_code)]
            {
                self = self;
            }
        }
    }

    let mut sut = Sut { field: 42 };

    Sut::self_as_ref_sut(&sut).call();
    sut.self_as_ref_sut().call();

    Sut::mut_self(sut.clone()).call();
    sut.clone().mut_self().call();

    Sut::mut_self_ref_self(&sut).call();
    sut.mut_self_ref_self().call();

    Sut::ref_mut_self(&mut sut).call();
    sut.ref_mut_self().call();
    assert_eq!(sut.field, 44);

    Sut::mut_self_as_ref_mut_self(&mut sut).call();
    sut.mut_self_as_ref_mut_self().call();
    assert_eq!(sut.field, 45);

    Sut::self_as_pin_mut_self(Pin::new(&mut sut)).call();
    Pin::new(&mut sut).self_as_pin_mut_self().call();

    Sut::mut_self_as_pin_mut_self(Pin::new(&mut sut)).call();
    Pin::new(&mut sut).mut_self_as_pin_mut_self().call();
}

#[cfg(feature = "alloc")]
#[test]
fn receiver_variations_alloc() {
    use core::pin::Pin;

    #[derive(Clone)]
    struct Sut {
        field: u32,
    }

    #[bon]
    impl Sut {
        #[builder]
        fn mut_self_as_box(mut self: Box<Self>) {
            self.field += 1;
            drop(self);
        }
        #[builder]
        fn self_as_box(self: Box<Self>) {
            drop(self);
        }
        #[builder]
        fn self_as_rc(self: Rc<Self>) {
            drop(self);
        }
        #[builder]
        fn self_as_arc(self: Arc<Self>) {
            drop(self);
        }
        #[builder]
        fn self_as_pin_box_self(self: Pin<Box<Self>>) {
            drop(self);
        }
        #[builder]
        #[allow(clippy::use_self)]
        fn self_as_pin_box_sut(self: Pin<Box<Sut>>) {
            drop(self);
        }
    }

    let sut = Sut { field: 42 };

    Sut::mut_self_as_box(Box::new(sut.clone())).call();
    Box::new(sut.clone()).mut_self_as_box().call();

    Sut::self_as_box(Box::new(sut.clone())).call();
    Box::new(sut.clone()).self_as_box().call();

    Sut::self_as_rc(Rc::new(sut.clone())).call();
    Rc::new(sut.clone()).self_as_rc().call();

    Sut::self_as_arc(Arc::new(sut.clone())).call();
    Arc::new(sut.clone()).self_as_arc().call();

    Sut::self_as_pin_box_self(Pin::new(Box::new(sut.clone()))).call();
    Pin::new(Box::new(sut.clone()))
        .self_as_pin_box_self()
        .call();

    Sut::self_as_pin_box_sut(Pin::new(Box::new(sut.clone()))).call();
    Pin::new(Box::new(sut)).self_as_pin_box_sut().call();
}

#[test]
fn constructor() {
    struct Counter {
        val: u32,
    }

    #[bon]
    impl Counter {
        #[builder(start_fn = builder)]
        fn new(initial: Option<u32>) -> Self {
            Self {
                val: initial.unwrap_or_default(),
            }
        }
    }

    let counter = Counter::builder().initial(3).build();

    assert_eq!(counter.val, 3);

    let counter = Counter::new(Some(32));

    assert_eq!(counter.val, 32);
}

#[test]
fn receiver() {
    #[derive(Clone)]
    struct Counter {
        val: u32,
    }

    #[bon]
    impl Counter {
        /// Docs on the method.
        /// Multiline
        #[builder]
        fn increment(&self, #[builder(default)] disabled: bool) -> Self {
            if disabled {
                return self.clone();
            }
            Self { val: self.val + 1 }
        }
    }

    let counter = Counter { val: 0 };
    let counter = counter.increment().call();

    assert_eq!(counter.val, 1);
}

#[test]
fn receiver_with_lifetimes() {
    struct Sut<'a, 'b> {
        a: &'a str,
        b: &'b str,
    }

    #[bon]
    impl Sut<'_, '_> {
        #[builder]
        fn method(&self, c: &str) -> usize {
            let Self { a, b } = self;

            a.len() + b.len() + c.len()
        }
    }

    let actual = Sut { a: "a", b: "b" }.method().c("c").call();
    assert_eq!(actual, 3);
}

#[test]
fn self_in_a_bunch_of_places() {
    struct Sut;

    #[bon]
    impl Sut
    where
        Self: Sized + 'static,
    {
        #[builder]
        fn method(&self, me: Option<Self>) -> impl Iterator<Item = Self>
        where
            Self: Sized,
        {
            let _ = self;
            me.into_iter()
        }
    }

    assert_eq!(Sut.method().me(Sut).call().count(), 1);
}

#[test]
fn impl_block_ty_contains_a_reference() {
    struct Sut<T>(T);

    #[bon]
    impl<T> Sut<&'_ T> {
        #[builder]
        fn get(&self) -> &T {
            self.0
        }
    }

    assert_eq!(Sut(&42).get().call(), &42);
}

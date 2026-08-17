// All the tests have their arguments used so they produce unused variable compiler warnings
#![allow(unused_variables)]

use std::{
    cell::RefCell,
    panic,
    rc::Rc,
    sync::{Arc, Mutex},
    thread,
};

use futures_executor::block_on;
use glib::clone;

struct State {
    count: i32,
    started: bool,
}

impl State {
    fn new() -> Self {
        Self {
            count: 0,
            started: false,
        }
    }
}

#[test]
fn clone_and_references() {
    let state = Rc::new(RefCell::new(State::new()));
    let ref_state = &state;
    assert!(!ref_state.borrow().started);

    let closure = {
        clone!(
            #[weak]
            ref_state,
            move || {
                ref_state.borrow_mut().started = true;
            }
        )
    };

    closure();
    assert!(ref_state.borrow().started);
}

#[test]
fn subfields_renaming() {
    struct Foo {
        v: Rc<usize>,
    }

    impl Foo {
        fn foo(&self) {
            let state = Rc::new(RefCell::new(State::new()));

            let closure = clone!(
                #[strong(rename_to = v)]
                self.v,
                #[weak(rename_to = hello)]
                state,
                move |_| {
                    println!("v: {v}");
                    hello.borrow_mut().started = true;
                }
            );
            closure(2);
        }
    }

    Foo { v: Rc::new(0) }.foo();
}

#[test]
fn renaming() {
    let state = Rc::new(RefCell::new(State::new()));
    assert!(!state.borrow().started);

    let closure = {
        clone!(
            #[weak(rename_to = hello)]
            state,
            move || {
                hello.borrow_mut().started = true;
            }
        )
    };

    closure();
    assert!(state.borrow().started);
}

#[test]
fn clone_closure() {
    let state = Rc::new(RefCell::new(State::new()));
    assert!(!state.borrow().started);

    let closure = {
        clone!(
            #[weak]
            state,
            move || {
                state.borrow_mut().started = true;
            }
        )
    };

    closure();

    assert!(state.borrow().started);
    assert_eq!(state.borrow().count, 0);

    let closure = {
        let state2 = Rc::new(RefCell::new(State::new()));
        assert!(state.borrow().started);

        clone!(
            #[weak]
            state,
            #[strong]
            state2,
            move || {
                state.borrow_mut().count += 1;
                state.borrow_mut().started = true;
                state2.borrow_mut().started = true;
            }
        )
    };

    closure();

    assert_eq!(state.borrow().count, 1);
    assert!(state.borrow().started);
}

#[test]
fn clone_default_value() {
    let closure = {
        let state = Rc::new(RefCell::new(State::new()));
        clone!(
            #[weak]
            state,
            #[upgrade_or]
            42,
            move |_| {
                state.borrow_mut().started = true;
                10
            }
        )
    };

    assert_eq!(42, closure(50));
}

#[test]
fn clone_panic() {
    let state = Arc::new(Mutex::new(State::new()));
    state.lock().expect("Failed to lock state mutex").count = 20;

    let closure = {
        let state2 = Arc::new(Mutex::new(State::new()));
        clone!(
            #[weak]
            state2,
            #[strong]
            state,
            #[upgrade_or_else]
            || panic!(),
            move |_| {
                state.lock().expect("Failed to lock state mutex").count = 21;
                state2.lock().expect("Failed to lock state2 mutex").started = true;
                10
            }
        )
    };

    let result = panic::catch_unwind(|| {
        closure(50);
    });

    assert!(result.is_err());

    assert_eq!(state.lock().expect("Failed to lock state mutex").count, 20);
}

#[test]
fn clone_import_rename() {
    import_rename::test();
}

mod import_rename {
    use glib::clone as clone_g;

    #[allow(unused_macros)]
    macro_rules! clone {
        ($($anything:tt)*) => {
            |_, _| panic!("The clone! macro doesn't support renaming")
        };
    }

    #[allow(unused_variables)]
    pub fn test() {
        let n = 2;

        let closure: Box<dyn Fn(u32, u32)> = Box::new(clone_g!(
            #[strong]
            n,
            move |_, _| println!("The clone! macro does support renaming")
        ));

        closure(0, 0);
    }
}

#[test]
fn test_clone_macro_self_rename() {
    #[derive(Debug)]
    struct Foo {
        v: u8,
    }

    impl Foo {
        #[allow(dead_code)]
        fn foo(&self) {
            let closure = clone!(
                #[strong(rename_to = this)]
                self,
                move |_x| {
                    println!("v: {this:?}");
                }
            );
            closure(0i8); // to prevent compiler error for unknown `x` type.
            let _ = clone!(
                #[strong(rename_to = this)]
                self,
                move || {
                    println!("v: {this:?}");
                }
            );
            let closure = clone!(
                #[strong(rename_to = this)]
                self,
                move |_x| println!("v: {this:?}")
            );
            closure(0i8); // to prevent compiler error for unknown `x` type.
            let _ = clone!(
                #[strong(rename_to = this)]
                self,
                move || println!("v: {this:?}")
            );

            // Fields now!
            let closure = clone!(
                #[strong(rename_to = v)]
                self.v,
                move |_x| {
                    println!("v: {v:?}");
                }
            );
            closure(0i8); // to prevent compiler error for unknown `x` type.
            let _ = clone!(
                #[strong(rename_to = v)]
                self.v,
                move || println!("v: {v:?}")
            );
        }
    }
}

#[test]
fn test_clone_macro_rename() {
    let v = Rc::new(1);

    let closure = clone!(
        #[weak(rename_to = y)]
        v,
        #[upgrade_or_panic]
        move |_x| {
            println!("v: {y}");
        }
    );
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = clone!(
        #[weak(rename_to = y)]
        v,
        #[upgrade_or_panic]
        move || println!("v: {y}")
    );

    let closure = clone!(
        #[strong(rename_to = y)]
        v,
        move |_x| {
            println!("v: {y}");
        }
    );
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = clone!(
        #[strong(rename_to = y)]
        v,
        move || println!("v: {y}")
    );

    let closure = clone!(
        #[weak(rename_to = y)]
        v,
        move |_x| {
            println!("v: {y}");
        }
    );
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = clone!(
        #[weak(rename_to = y)]
        v,
        move || println!("v: {y}")
    );

    let closure = clone!(
        #[strong(rename_to = y)]
        v,
        move |_x| {
            println!("v: {y}");
        }
    );
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = clone!(
        #[strong(rename_to = y)]
        v,
        move || println!("v: {y}")
    );

    let closure = clone!(
        #[weak(rename_to = y)]
        v,
        #[upgrade_or]
        true,
        move |_x| false
    );
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = clone!(
        #[weak(rename_to = y)]
        v,
        #[upgrade_or_else]
        || true,
        move || false
    );

    let closure = clone!(
        #[strong(rename_to = y)]
        v,
        move |_x| false
    );
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = clone!(
        #[strong(rename_to = y)]
        v,
        move || false
    );
}

#[test]
fn test_clone_macro_simple() {
    let v = Rc::new(1);

    let closure = clone!(
        #[weak]
        v,
        #[upgrade_or_panic]
        move |_x| {
            println!("v: {v}");
        }
    );
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = clone!(
        #[weak]
        v,
        #[upgrade_or_panic]
        move || println!("v: {v}")
    );

    let closure = clone!(
        #[strong]
        v,
        move |_x| {
            println!("v: {v}");
        }
    );
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = clone!(
        #[strong]
        v,
        move || println!("v: {v}")
    );

    let closure = clone!(
        #[weak]
        v,
        move |_x| {
            println!("v: {v}");
        }
    );
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = clone!(
        #[weak]
        v,
        move || println!("v: {v}")
    );

    let closure = clone!(
        #[strong]
        v,
        move |_x| {
            println!("v: {v}");
        }
    );
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = clone!(
        #[strong]
        v,
        move || println!("v: {v}")
    );

    let closure = clone!(
        #[weak]
        v,
        #[upgrade_or]
        true,
        move |_x| false
    );
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = clone!(
        #[weak]
        v,
        #[upgrade_or_else]
        || true,
        move || false
    );

    let closure = clone!(
        #[strong]
        v,
        move |_x| false
    );
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = clone!(
        #[strong]
        v,
        move || false
    );
}

#[test]
fn test_clone_macro_double_simple() {
    let v = Rc::new(1);
    let w = Rc::new(2);

    let closure = clone!(
        #[weak]
        v,
        #[weak]
        w,
        #[upgrade_or_panic]
        move |_x| {
            println!("v: {v}, w: {w}");
        }
    );
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = clone!(
        #[weak]
        v,
        #[weak]
        w,
        #[upgrade_or_panic]
        move || println!("v: {v}, w: {w}")
    );

    let closure = clone!(
        #[strong]
        v,
        #[strong]
        w,
        move |_x| {
            println!("v: {v}, w: {w}");
        }
    );
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = clone!(
        #[strong]
        v,
        #[strong]
        w,
        move || println!("v: {v}, w: {w}")
    );

    let closure = clone!(
        #[weak]
        v,
        #[weak]
        w,
        move |_x| {
            println!("v: {v}, w: {w}");
        }
    );
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = clone!(
        #[weak]
        v,
        #[weak]
        w,
        move || println!("v: {v}, w: {w}")
    );

    let closure = clone!(
        #[strong]
        v,
        #[strong]
        w,
        move |_x| {
            println!("v: {v}, w: {w}");
        }
    );
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = clone!(
        #[strong]
        v,
        #[strong]
        w,
        move || println!("v: {v}, w: {w}")
    );

    let closure = clone!(
        #[weak]
        v,
        #[weak]
        w,
        #[upgrade_or]
        true,
        move |_x| false
    );
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = clone!(
        #[weak]
        v,
        #[weak]
        w,
        #[upgrade_or_else]
        || true,
        move || false
    );

    let closure = clone!(
        #[strong]
        v,
        #[strong]
        w,
        move |_x| false
    );
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = clone!(
        #[strong]
        v,
        #[strong]
        w,
        move || false
    );
}

#[test]
fn test_clone_macro_double_rename() {
    let v = Rc::new(1);
    let w = Rc::new(2);
    let done = Rc::new(RefCell::new(0));

    let closure = clone!(
        #[weak(rename_to = x)]
        v,
        #[weak]
        w,
        #[upgrade_or_panic]
        move |z| z + *x + *w
    );
    assert_eq!(closure(1i8), 4i8);
    let closure = clone!(
        #[weak(rename_to = x)]
        v,
        #[weak]
        w,
        #[upgrade_or_panic]
        move || 1
    );
    assert_eq!(closure(), 1);

    let closure = clone!(
        #[weak]
        v,
        #[weak(rename_to = x)]
        w,
        #[upgrade_or_panic]
        move |z| z + *v + *x
    );
    assert_eq!(closure(10i8), 13i8);
    let closure = clone!(
        #[weak]
        v,
        #[weak(rename_to = x)]
        w,
        #[upgrade_or_panic]
        move || 2 + *x
    );
    assert_eq!(closure(), 4);

    let closure = clone!(
        #[strong(rename_to = x)]
        v,
        #[strong]
        w,
        move |z| z + *x + *w
    );
    assert_eq!(closure(3i8), 6i8);
    let closure = clone!(
        #[strong(rename_to = x)]
        v,
        #[strong]
        w,
        move || 4 + *w
    );
    assert_eq!(closure(), 6);

    let closure = clone!(
        #[strong]
        v,
        #[strong(rename_to = x)]
        w,
        move |z| z + *v + *x
    );
    assert_eq!(closure(0i8), 3i8);
    let closure = clone!(
        #[strong]
        v,
        #[strong(rename_to = x)]
        w,
        move || 5
    );
    assert_eq!(closure(), 5);

    let t_done = done.clone();
    let closure = clone!(
        #[weak(rename_to = x)]
        v,
        #[weak]
        w,
        move |z| {
            *t_done.borrow_mut() = z + *x + *w;
        }
    );
    closure(4i8);
    assert_eq!(*done.borrow(), 7);
    let t_done = done.clone();
    let closure = clone!(
        #[weak(rename_to = x)]
        v,
        #[weak]
        w,
        move || *t_done.borrow_mut() = *x + *w
    );
    closure();
    assert_eq!(*done.borrow(), 3);

    let t_done = done.clone();
    let closure = clone!(
        #[weak]
        v,
        #[weak(rename_to = x)]
        w,
        move |z| {
            *t_done.borrow_mut() = z + *v + *x;
        }
    );
    closure(8i8);
    assert_eq!(*done.borrow(), 11i8);
    let t_done = done.clone();
    let closure = clone!(
        #[weak]
        v,
        #[weak(rename_to = x)]
        w,
        move || *t_done.borrow_mut() = *v * *x
    );
    closure();
    assert_eq!(*done.borrow(), 2);

    let t_done = done.clone();
    let closure = clone!(
        #[strong(rename_to = x)]
        v,
        #[strong]
        w,
        move |z| {
            *t_done.borrow_mut() = z + *x + *w;
        }
    );
    closure(9i8);
    assert_eq!(*done.borrow(), 12i8);
    let t_done = done.clone();
    let closure = clone!(
        #[strong(rename_to = x)]
        v,
        #[strong]
        w,
        move || *t_done.borrow_mut() = *x - *w
    );
    closure();
    assert_eq!(*done.borrow(), -1);

    let t_done = done.clone();
    let closure = clone!(
        #[strong]
        v,
        #[strong(rename_to = x)]
        w,
        move |z| {
            *t_done.borrow_mut() = *v + *x * z;
        }
    );
    closure(2i8);
    assert_eq!(*done.borrow(), 5);
    let t_done = done.clone();
    let closure = clone!(
        #[strong]
        v,
        #[strong(rename_to = x)]
        w,
        move || *t_done.borrow_mut() = *x - *v
    );
    closure();
    assert_eq!(*done.borrow(), 1);

    let closure = clone!(
        #[weak(rename_to = x)]
        v,
        #[weak]
        w,
        #[upgrade_or]
        true,
        move |_| false
    );
    assert!(!closure(0u8));
    let closure = clone!(
        #[weak(rename_to = x)]
        v,
        #[weak]
        w,
        #[upgrade_or_else]
        || true,
        move || false
    );
    assert!(!closure());

    let closure = clone!(
        #[weak]
        v,
        #[weak(rename_to = x)]
        w,
        #[upgrade_or]
        true,
        move |_| false
    );
    assert!(!closure("a"));
    let closure = clone!(
        #[weak]
        v,
        #[weak(rename_to = x)]
        w,
        #[upgrade_or_else]
        || true,
        move || false
    );
    assert!(!closure());
    let closure = clone!(
        #[weak]
        v,
        #[weak(rename_to = x)]
        w,
        #[upgrade_or_default]
        move || true
    );
    assert!(closure());

    let closure = clone!(
        #[strong(rename_to = x)]
        v,
        #[strong]
        w,
        move |_| false
    );
    assert!(!closure('a'));
    let closure = clone!(
        #[strong(rename_to = x)]
        v,
        #[strong]
        w,
        move || false
    );
    assert!(!closure());

    let closure = clone!(
        #[strong]
        v,
        #[strong(rename_to = x)]
        w,
        move |_| false
    );
    assert!(!closure(12.));
    let closure = clone!(
        #[strong]
        v,
        #[strong(rename_to = x)]
        w,
        move || false
    );
    assert!(!closure());
}

#[test]
fn test_clone_macro_typed_args() {
    macro_rules! test_closure {
        ($kind:tt, panic) => {{
            // We need Arc and Mutex to use them below in the thread.
            let check = Arc::new(Mutex::new(0));
            let v = Arc::new(Mutex::new(1));
            let w = Arc::new(Mutex::new(1));

            let closure = clone!(
                #[$kind(rename_to = x)]
                v,
                #[$kind]
                w,
                #[weak]
                check,
                #[upgrade_or_panic]
                move |arg: i8| {
                    *x.lock().unwrap() += arg;
                    *w.lock().unwrap() += arg;
                    *check.lock().unwrap() += 1;
                }
            );
            closure(1);
            assert_eq!(2, *v.lock().unwrap());
            assert_eq!(2, *w.lock().unwrap());
            assert_eq!(1, *check.lock().unwrap());

            let closure2 = clone!(
                #[$kind]
                v,
                #[$kind(rename_to = x)]
                w,
                #[weak]
                check,
                #[upgrade_or_panic]
                move |arg: i8| {
                    *v.lock().unwrap() += arg;
                    *x.lock().unwrap() += arg;
                    *check.lock().unwrap() += 1;
                }
            );
            closure2(1);
            assert_eq!(3, *v.lock().unwrap());
            assert_eq!(3, *w.lock().unwrap());
            assert_eq!(2, *check.lock().unwrap());

            #[allow(unused_macro_rules)]
            macro_rules! inner {
                (strong) => {{}};
                (weak) => {{
                    std::mem::drop(v);
                    std::mem::drop(w);

                    // We use the threads to ensure that the closure panics as expected.
                    assert!(thread::spawn(move || {
                        closure(1);
                    })
                    .join()
                    .is_err());
                    assert_eq!(2, *check.lock().unwrap());
                    assert!(thread::spawn(move || {
                        closure2(1);
                    })
                    .join()
                    .is_err());
                    assert_eq!(2, *check.lock().unwrap());
                }};
            }

            inner!($kind);
        }};
        ($kind:tt) => {{
            let check = Rc::new(RefCell::new(0));
            let v = Rc::new(RefCell::new(1));
            let w = Rc::new(RefCell::new(1));

            let closure = clone!(
                #[$kind(rename_to = x)]
                v,
                #[$kind]
                w,
                #[weak]
                check,
                move |arg: i8| {
                    *x.borrow_mut() += arg;
                    *w.borrow_mut() += arg;
                    *check.borrow_mut() += 1;
                }
            );
            closure(1);
            assert_eq!(2, *v.borrow());
            assert_eq!(2, *w.borrow());
            assert_eq!(1, *check.borrow());

            let closure2 = clone!(
                #[$kind]
                v,
                #[$kind(rename_to = x)]
                w,
                #[weak]
                check,
                move |arg: i8| {
                    *v.borrow_mut() += arg;
                    *x.borrow_mut() += arg;
                    *check.borrow_mut() += 1;
                }
            );
            closure2(1);
            assert_eq!(3, *v.borrow());
            assert_eq!(3, *w.borrow());
            assert_eq!(2, *check.borrow());

            #[allow(unused_macro_rules)]
            macro_rules! inner {
                (strong) => {{}};
                (weak) => {{
                    std::mem::drop(v);
                    std::mem::drop(w);

                    closure(1);
                    assert_eq!(2, *check.borrow());
                    closure2(1);
                    assert_eq!(2, *check.borrow());
                }};
            }

            inner!($kind);
        }};
    }

    test_closure!(weak, panic);
    test_closure!(strong, panic);
    test_closure!(weak);
    test_closure!(strong);

    let check = Rc::new(RefCell::new(0));
    let v = Rc::new(RefCell::new(1));
    let w = Rc::new(RefCell::new(1));
    let closure = clone!(
        #[weak]
        v,
        #[weak(rename_to = x)]
        w,
        #[weak]
        check,
        move |arg: i8, arg2| {
            *v.borrow_mut() = arg;
            *x.borrow_mut() = arg2;
            *check.borrow_mut() += 1;
        }
    );
    closure(0, 9);
    assert_eq!(0, *v.borrow());
    assert_eq!(9, *w.borrow());
    assert_eq!(1, *check.borrow());

    std::mem::drop(v);
    std::mem::drop(w);
    assert_eq!(1, *check.borrow());
}

#[test]
#[allow(clippy::bool_assert_comparison)]
#[allow(clippy::nonminimal_bool)]
fn test_clone_macro_upgrade_failure() {
    macro_rules! test_default {
        ($ret:expr, $($closure_body:tt)*) => {{
            let v = Rc::new(1);
            let tmp = clone!(#[weak] v, #[upgrade_or_else] || $ret, move || $($closure_body)*);
            assert_eq!(tmp(), $($closure_body)*, "shouldn't use default-return value!");
            ::std::mem::drop(v);
            assert_eq!(tmp(), $ret, "should use default-return value!");
        }}
    }

    #[derive(PartialEq, Debug)]
    struct Foo(i32);

    test_default!(Foo(0), Foo(1));

    #[derive(PartialEq, Debug)]
    struct Bar {
        x: i32,
    }

    test_default!(Bar { x: 0 }, Bar { x: 1 });

    #[derive(PartialEq, Debug)]
    enum Enum {
        A,
        B(i32),
        C { x: i32 },
    }
    test_default!(Enum::A, Enum::B(0));
    test_default!(Enum::B(0), Enum::A);
    test_default!(Enum::C { x: 0 }, Enum::A);
    test_default!(
        {
            let x = 12;
            x + 2
        },
        19
    );
    // This one is simply to check that we wait for the comma for the default-return value.
    test_default!(Enum::A == Enum::B(0) || false, true);
}

#[test]
fn test_clone_macro_body() {
    let v = Arc::new(Mutex::new(0));

    let closure = clone!(
        #[weak]
        v,
        move || {
            std::thread::spawn(move || {
                let mut lock = v.lock().expect("failed to lock");
                for _ in 1..=10 {
                    *lock += 1;
                }
            })
            .join()
            .expect("thread::spawn failed");
        }
    );
    closure();
    assert_eq!(10, *v.lock().expect("failed to lock"));
}

#[test]
fn test_clone_macro_async_kinds() {
    let v = Rc::new(RefCell::new(1));

    block_on(clone!(
        #[weak]
        v,
        async move {
            *v.borrow_mut() += 1;
        }
    ));
    assert_eq!(*v.borrow(), 2);
}

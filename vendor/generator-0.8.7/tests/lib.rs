#![allow(deprecated)]
#![allow(unused_assignments)]

extern crate generator;

use generator::*;

#[test]
fn test_return() {
    let mut g = Gn::new_scoped(|_s| 42u32);
    assert_eq!(g.next(), Some(42));
    assert!(g.is_done());
}

#[test]
fn generator_is_done() {
    let mut g = Gn::<()>::new(|| {
        yield_with(());
    });

    g.next();
    assert!(!g.is_done());
    g.next();
    assert!(g.is_done());
}

#[test]
fn generator_is_done1() {
    let mut g = Gn::new_scoped(|mut s| {
        s.yield_(2);
        done!();
    });

    assert_eq!(g.next(), Some(2));
    assert!(!g.is_done());
    assert_eq!(g.next(), None);
    assert!(g.is_done());
}

#[test]
fn generator_is_done_with_drop() {
    let mut g = Gn::new_scoped(|mut s| {
        s.yield_(String::from("string"));
        done!();
    });

    assert_eq!(g.next(), Some(String::from("string")));
    assert!(!g.is_done());
    assert_eq!(g.next(), None);
    assert!(g.is_done());
}

#[test]
fn test_yield_a() {
    let mut g = Gn::<i32>::new(|| {
        let r: i32 = yield_(10).unwrap();
        r * 2
    });

    // first start the generator
    let i = g.raw_send(None).unwrap();
    assert_eq!(i, 10);
    let i = g.send(3);
    assert_eq!(i, 6);
    assert!(g.is_done());
}

#[test]
fn test_yield_with() {
    let mut g = Gn::new(|| {
        yield_with(10);
        20
    });

    // the para type could be deduced here
    let i = g.send(());
    assert!(i == 10);

    let j = g.next();
    assert!(j.unwrap() == 20);
}

#[test]
#[should_panic]
fn test_yield_with_type_error() {
    let mut g = Gn::<()>::new(|| {
        // yield_with::<i32>(10);
        yield_with(10u32);
        20i32
    });

    g.next();
}

#[test]
#[should_panic]
fn test_get_yield_type_error() {
    let mut g = Gn::<u32>::new(|| {
        get_yield::<i32>();
    });

    g.send(10);
}

#[test]
#[should_panic]
fn test_deep_yield_with_type_error() {
    let mut g = Gn::<()>::new(|| {
        let mut g = Gn::<()>::new(|| {
            yield_with(0);
        });
        g.next();
    });

    g.next();
}

#[test]
fn test_scoped() {
    use std::cell::RefCell;
    use std::rc::Rc;

    let x = Rc::new(RefCell::new(10));

    let x1 = x.clone();
    let mut g = Gn::<()>::new_scoped_local(move |mut s| {
        *x1.borrow_mut() = 20;
        s.yield_with(());
        *x1.borrow_mut() = 5;
    });

    g.next();
    assert!(*x.borrow() == 20);

    g.next();
    assert!(*x.borrow() == 5);

    assert!(g.is_done());
}

#[test]
fn test_scoped_1() {
    let mut x = 10;
    {
        let mut g = Gn::<()>::new(|| {
            x = 5;
        });
        g.next();
    }

    assert!(x == 5);
}

#[test]
fn test_scoped_yield() {
    let mut g = Gn::new_scoped(|mut s| {
        let mut i = 0;
        loop {
            let v = s.yield_(i);
            i += 1;
            match v {
                Some(x) => {
                    // dbg!(x, i);
                    assert_eq!(x, i);
                }
                None => {
                    // for elegant exit
                    break;
                }
            }
        }
        20usize
    });

    // start g
    g.raw_send(None);

    for i in 1..100 {
        let data: usize = g.send(i);
        assert_eq!(data, i);
    }

    // quit g
    g.raw_send(None);
}

#[test]
fn test_inner_ref() {
    let mut g = Gn::<()>::new_scoped(|mut s| {
        // setup something
        let mut x: u32 = 10;

        // return internal ref not compiled because the
        // lifetime of internal ref is smaller than the generator
        // but the generator interface require the return type's
        // lifetime bigger than the generator

        // the x memory remains on heap even returned!
        // the life time of x is associated with the generator
        // however modify this internal value is really unsafe
        // but this is useful pattern for setup and teardown
        // which can be put in the same place
        unsafe {
            let mut_ref: &mut u32 = std::mem::transmute(&mut x);
            s.yield_unsafe(mut_ref);
        };

        // this was modified by the invoker
        assert!(x == 5);
        // teardown happened when the generator get dropped
        done!()
    });

    // use the resource setup from generator
    let a = g.next().unwrap();
    assert!(*a == 10);
    *a = 5;
    // a keeps valid until the generator dropped
}

#[test]
fn test_drop() {
    let mut x = 10;
    {
        let mut g = Gn::<()>::new(|| {
            x = 1;
            yield_with(());
            x = 5;
        });
        g.send(());
    }

    assert!(x == 1);
}

#[test]
fn test_ill_drop() {
    let mut x = 10u32;
    {
        Gn::<u32>::new(|| {
            x = 5;
            // here we got None from drop
            x = get_yield().unwrap_or(0);
        });
        // not started the gen, change nothing
    }

    assert!(x == 10);
}

#[test]
fn test_loop_drop() {
    let mut x = 10u32;
    {
        let mut g = Gn::<()>::new(|| {
            x = 5;
            loop {
                yield_with(());
            }
        });
        g.send(());
        // here the generator drop will cancel the loop
    }

    assert!(x == 5);
}

#[test]
fn test_panic_inside() {
    use std::panic::{catch_unwind, AssertUnwindSafe};
    let mut x = 10;
    {
        let mut wrapper = AssertUnwindSafe(&mut x);
        if let Err(panic) = catch_unwind(move || {
            let mut g = Gn::<()>::new(|| {
                **wrapper = 5;
                panic!("panic inside!");
            });
            g.resume();
        }) {
            match panic.downcast_ref::<&str>() {
                // why can't get the message here?? is it lost?
                Some(msg) => println!("get panic: {msg:?}"),
                None => println!("can't get panic message"),
            }
        }
        // wrapper dropped here
    }

    assert!(x == 5);
}

#[test]
#[allow(unreachable_code)]
fn test_cancel() {
    let mut g = Gn::<()>::new(|| {
        let mut i = 0;
        loop {
            yield_with(i);
            i += 1;
        }
        i
    });

    loop {
        let i = g.next().unwrap();
        if i > 10 {
            g.cancel();
            break;
        }
    }

    assert!(g.is_done());
}

#[test]
#[should_panic]
fn test_yield_from_functor_context() {
    // this is not run from generator
    yield_::<(), _>(0);
}

#[test]
#[should_panic]
fn test_yield_with_from_functor_context() {
    // this is not run from generator
    yield_with(0);
}

#[test]
fn test_yield_from_generator_context() {
    let mut g = Gn::<()>::new(|| {
        let mut g1 = Gn::<()>::new(|| {
            yield_with(5);
            10
        });

        let i = g1.send(());
        yield_with(i);
        0
    });

    let n = g.send(());
    assert!(n == 5);

    let n = g.send(());
    assert!(n == 0);
}

#[test]
fn test_yield_from() {
    let mut g = Gn::<()>::new(|| {
        let g1 = Gn::<()>::new(|| {
            yield_with(5);
            10
        });

        yield_from(g1);
        0
    });

    let n = g.send(());
    assert!(n == 5);
    let n = g.send(());
    assert!(n == 10);
    let n = g.send(());
    assert!(n == 0);
    assert!(g.is_done());
}

#[test]
fn test_yield_from_send() {
    let mut g = Gn::<u32>::new(|| {
        let g1 = Gn::<u32>::new(|| {
            let mut i: u32 = yield_(1u32).unwrap();
            i = yield_(i * 2).unwrap();
            i * 2
        });

        let i = yield_from(g1).unwrap();
        assert_eq!(i, 10);

        // here we need a unused return to indicate this function's return type
        0u32
    });

    // first start the generator
    let n = g.raw_send(None).unwrap();
    assert!(n == 1);

    let n = g.send(3);
    assert!(n == 6);
    let n = g.send(4);
    assert!(n == 8);
    let n = g.send(10);
    assert!(n == 0);
    assert!(g.is_done());
}

#[test]
#[should_panic]
fn test_yield_from_send_type_miss_match() {
    let mut g = Gn::<u32>::new(|| {
        let g1 = Gn::<u32>::new(|| {
            let mut i: u32 = yield_(1u32).unwrap();
            i = yield_(i * 2).unwrap();
            i * 2
        });

        yield_from(g1);
        // here the return type should be 0u32
        0
    });

    let n = g.send(3);
    assert!(n == 1);
    let n = g.send(4);
    assert!(n == 6);
    let n = g.send(10);
    assert!(n == 8);
    // the last send has no meaning for the return
    let n = g.send(0);
    assert!(n == 0);
    assert!(g.is_done());
}

// windows has it's own check, this test would make the app abort
// #[test]
// #[should_panic]
// fn test_stack_overflow() {
//     // here the stack size is not big enough
//     // and will panic when get detected in drop
//     let clo = || {
//         let big_data = [0usize; 0x400];
//         println!("this would overflow the stack, {}", big_data[100]);
//     };
//     Gn::<()>::new_opt(clo, 10);
// }

#[test]
fn test_scope_gen() {
    // now we can even deduce the input para type
    let mut g = Gn::new_scoped(|mut s| {
        let i = s.yield_(0).unwrap();
        // below would have a compile error, nice!
        // s.yield_(Box::new(0));
        i * 2
    });

    assert_eq!(g.raw_send(None), Some(0));
    assert_eq!(g.raw_send(Some(3)), Some(6));
    assert_eq!(g.raw_send(None), None);
}

#[test]
fn test_scope_yield_from_send() {
    let mut g = Gn::new_scoped(|mut s| {
        let g1 = Gn::new_scoped(|mut s| {
            let mut i: u32 = s.yield_(1u32).unwrap();
            i = s.yield_(i * 2).unwrap();
            i * 2
        });

        let i = s.yield_from(g1).unwrap();
        // here the return type should be 0u32
        i * 2
    });

    let n = g.send(3);
    assert_eq!(n, 1);
    let n = g.send(4);
    assert_eq!(n, 8);
    let n = g.send(10);
    assert_eq!(n, 20);
    // the last send has no meaning for the return
    let n = g.send(7);
    assert!(n == 14);
    assert!(g.is_done());
}

#[test]
fn test_re_init() {
    let clo = || {
        |mut s: Scope<'_, 'static, (), _>| {
            s.yield_(0);
            s.yield_(3);
            5
        }
    };

    let mut g = Gn::new_opt(0x800, || 0);
    g.scoped_init(clo());

    assert_eq!(g.next(), Some(0));
    assert_eq!(g.next(), Some(3));
    assert_eq!(g.next(), Some(5));
    assert!(g.is_done());

    // re-init generator
    g.scoped_init(clo());

    assert_eq!(g.next(), Some(0));
    assert_eq!(g.next(), Some(3));
    assert_eq!(g.next(), Some(5));
    assert!(g.is_done());
}

#[test]
#[should_panic]
fn done_in_normal() {
    done!();
}

#[test]
#[should_panic]
fn invalid_yield_in_scope() {
    let g = Gn::new_scoped(|_| {
        // invalid use raw yield API with scope
        yield_::<String, _>(());
    });

    for () in g {}
}

#[test]
fn test_yield_float() {
    let mut g = Gn::<f64>::new(|| {
        let r: f64 = yield_(10.0).unwrap();
        let x = r * 2.0; // 6
        let y = x * 9.0; // 54
        let z = y / 3.0; // 18
        let r: f64 = yield_(6.0).unwrap();
        x * r * y * z
    });

    // first start the generator
    let i = g.raw_send(None).unwrap();
    let x = i * 10.0;
    assert_eq!(i, 10.0);
    let i = g.send(3.0);
    assert_eq!(i, 6.0);
    let i = g.send(x / 25.0);
    assert_eq!(i, 23328.0);
    assert!(g.is_done());
}

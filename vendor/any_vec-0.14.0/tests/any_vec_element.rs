use std::any::TypeId;
use itertools::{assert_equal};
use any_vec::any_value::{AnyValue, AnyValueCloneable, LazyClone};
use any_vec::AnyVec;
use any_vec::element::ElementReference;
use any_vec::mem::{MemBuilder, Stack};
use any_vec::traits::{Cloneable, Trait};

#[test]
fn lazy_clone_test(){
    let mut any_vec: AnyVec<dyn Cloneable> = AnyVec::new::<String>();
    {
        let mut vec = any_vec.downcast_mut::<String>().unwrap();
        vec.push(String::from("0"));
        vec.push(String::from("1"));
        vec.push(String::from("2"));
    }

    let mut any_vec_other: AnyVec<dyn Cloneable> = AnyVec::new::<String>();
    {
        let e1 = any_vec.swap_remove(1);
        let lz1 = LazyClone::new(&e1);
        let lz2 = LazyClone::new(&lz1).clone();

        any_vec_other.push(LazyClone::new(&e1));
        any_vec_other.push(LazyClone::new(&lz2));
    }

    assert_equal(
        any_vec.downcast_ref::<String>().unwrap().as_slice(),
        &[String::from("0"), String::from("2")]
    );
    assert_equal(
        any_vec_other.downcast_ref::<String>().unwrap().as_slice(),
        &[String::from("1"), String::from("1")]
    );
}

#[test]
fn any_vec_get_test(){
    let mut any_vec: AnyVec<dyn Cloneable> = AnyVec::new::<String>();
    assert_eq!(any_vec.element_typeid(), TypeId::of::<String>());
    {
        let mut vec = any_vec.downcast_mut::<String>().unwrap();
        vec.push(String::from("0"));
        vec.push(String::from("1"));
        vec.push(String::from("2"));
    }

    let e1_ref = any_vec.get(1).unwrap();
    assert_eq!(e1_ref.downcast_ref::<String>().unwrap(), &String::from("1"));
    assert_eq!(e1_ref.value_typeid(), TypeId::of::<String>());

    {
        let e1 = e1_ref.lazy_clone();
        let e2 = e1.lazy_clone();
        let e3 = e2.lazy_clone();

        // Consume in reverse order, since lazy clone source must outlive it.
        assert_eq!(e3.downcast::<String>().unwrap(), String::from("1"));
        assert_eq!(e2.downcast::<String>().unwrap(), String::from("1"));
        assert_eq!(e1.downcast::<String>().unwrap(), String::from("1"));
    }

    // Mutability test
    {
        let e = any_vec.downcast_mut::<String>().unwrap().as_mut_slice().get_mut(1).unwrap();
        *e += "A";

        let str = any_vec.get_mut(1).unwrap().downcast_mut::<String>().unwrap();
        *str += "B";

        assert_eq!(any_vec.get(1).unwrap().downcast_ref::<String>().unwrap(), &String::from("1AB"));
    }
}

#[test]
fn any_vec_iter_test(){
    let mut any_vec: AnyVec<dyn Cloneable> = AnyVec::new::<String>();
    {
        let mut vec = any_vec.downcast_mut::<String>().unwrap();
        vec.push(String::from("0"));
        vec.push(String::from("1"));
        vec.push(String::from("2"));
    }

    assert_equal(
        any_vec.iter().map(|e|e.downcast_ref::<String>().unwrap()),
        any_vec.downcast_ref::<String>().unwrap().as_slice()
    );

    let mut any_vec2 = any_vec.clone_empty();
    for e in any_vec.iter(){
        any_vec2.push(e.lazy_clone());
    }

    assert_equal(
        any_vec.downcast_ref::<String>().unwrap().as_slice(),
        any_vec2.downcast_ref::<String>().unwrap().as_slice()
    );

    for mut e in any_vec2.iter_mut(){
        *e.downcast_mut::<String>().unwrap() = String::from("100");
    }
    assert_equal(
        any_vec2.downcast_ref::<String>().unwrap().as_slice(),
        &[String::from("100"), String::from("100"), String::from("100")]
    );

    // test ElementRef cloneability
    {
        let r1 = any_vec.get(1).unwrap();
        let r2 = r1.clone();
        assert_eq!(r1.downcast_ref::<String>().unwrap(), &String::from("1"));
        assert_eq!(r2.downcast_ref::<String>().unwrap(), &String::from("1"));
    }

    // This should not compile
    /*{
        let iter = any_vec.iter();
        let mut iter_mut = any_vec.iter_mut();
        iter_mut.next().unwrap().downcast_mut::<String>();
        iter.next().unwrap().downcast::<String>();
    }*/
}

#[test]
fn any_vec_iter_clone_test(){
    let mut any_vec: AnyVec = AnyVec::new::<usize>();
    {
        let mut vec = any_vec.downcast_mut::<usize>().unwrap();
        vec.push(1);
        vec.push(10);
        vec.push(100);
    }

    fn into_usize<'a, T: ?Sized + Trait, M: MemBuilder + 'a>(e: impl ElementReference<'a, T, M>) -> &'a usize
    {
        e.downcast_ref::<usize>().unwrap()
    }
    assert_eq!(any_vec.iter().clone().map(into_usize).sum::<usize>(), 111);
    assert_eq!(any_vec.iter_mut().clone().map(into_usize).sum::<usize>(), 111);
}

#[test]
fn any_vec_push_to_self_test(){
    let mut any_vec: AnyVec<dyn Cloneable> = AnyVec::new::<String>();
    {
        let mut vec = any_vec.downcast_mut::<String>().unwrap();
        vec.push(String::from("0"));
        vec.push(String::from("1"));
        vec.push(String::from("2"));
    }

    let mut intermediate = any_vec.clone_empty_in(Stack::<512>);
    intermediate.push(any_vec.get(1).unwrap().lazy_clone());
    let e = intermediate.pop().unwrap();

    any_vec.push(e.lazy_clone());
    any_vec.push(e);

    assert!(intermediate.is_empty());
    assert_equal(
        any_vec.downcast_ref::<String>().unwrap().as_slice(),
        &[
            String::from("0"),
            String::from("1"),
            String::from("2"),
            String::from("1"),
            String::from("1")
        ]
    );
}

#[test]
fn any_vec_double_ended_iterator_test(){
    let mut any_vec: AnyVec<dyn Cloneable> = AnyVec::new::<String>();
    {
        let mut vec = any_vec.downcast_mut::<String>().unwrap();
        vec.push(String::from("0"));
        vec.push(String::from("1"));
        vec.push(String::from("2"));
    }

    assert_equal(
        any_vec.iter().rev().map(|e|e.downcast_ref::<String>().unwrap()),
        [
            String::from("2"),
            String::from("1"),
            String::from("0"),
        ].iter()
    );
}
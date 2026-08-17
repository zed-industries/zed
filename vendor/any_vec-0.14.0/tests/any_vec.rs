use std::any::{TypeId};
use std::mem::{size_of};
use std::mem::forget;
use std::ptr::NonNull;
use itertools::assert_equal;
use any_vec::AnyVec;
use any_vec::any_value::{AnyValueMut, AnyValueRaw, AnyValueWrapper};
use any_vec::mem::Stack;

#[allow(dead_code)]
unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    std::slice::from_raw_parts(
        (p as *const T) as *const u8,
        std::mem::size_of::<T>(),
    )
}

struct S{i:usize}
impl Drop for S{
    fn drop(&mut self) {
        println!("Drop {}",self.i);
    }
}

#[test]
fn drop_test() {
    let mut any_vec: AnyVec = AnyVec::new::<S>();
    let mut vec = any_vec.downcast_mut::<S>().unwrap();
    vec.push(S{i:1});
    vec.push(S{i:2});
    vec.push(S{i:3});

    assert_equal(vec.as_slice().iter().map(|s|s.i), [1, 2, 3]);
}

#[test]
fn any_value_raw_test() {
    let mut any_vec: AnyVec = AnyVec::new::<String>();

    unsafe{
        let str1 = "Hello".to_string();
        any_vec.push(AnyValueRaw::new(
            NonNull::from(&str1).cast::<u8>(), size_of::<String>(),
            TypeId::of::<String>()
        ));
        forget(str1);

        let str2 = " to ".to_string();
        any_vec.push(AnyValueRaw::new(
            NonNull::from(&str2).cast::<u8>(), size_of::<String>(),
            TypeId::of::<String>()
        ));
        forget(str2);

        let str3 = "world".to_string();
        any_vec.push(AnyValueRaw::new(
            NonNull::from(&str3).cast::<u8>(), size_of::<String>(),
            TypeId::of::<String>()
        ));
        forget(str3);
    }

    assert_equal(
        any_vec.downcast_ref::<String>().unwrap().as_slice(),
        ["Hello", " to ", "world"]
    );
}

#[test]
pub fn push_with_capacity_test(){
    const SIZE: usize = 10000;
    let mut any_vec: AnyVec = AnyVec::with_capacity::<usize>(SIZE);
    let mut vec = any_vec.downcast_mut::<usize>().unwrap();
    for i in 0..SIZE{
        vec.push(i);
    }

    assert_equal(vec.as_slice().iter().copied(), 0..SIZE);
}

#[test]
fn zero_size_type_test() {
    struct Empty{}
    let mut any_vec: AnyVec = AnyVec::new::<Empty>();
    let mut vec = any_vec.downcast_mut::<Empty>().unwrap();
    vec.push(Empty{});
    vec.push(Empty{});
    vec.push(Empty{});

    let mut i = 0;
    for _ in vec.as_mut_slice(){
        i += 1;
    }
    assert_eq!(i, 3);
}

#[test]
fn remove_test() {
    let mut any_vec: AnyVec = AnyVec::new::<String>();
    {
        let mut vec = any_vec.downcast_mut::<String>().unwrap();
        vec.push(String::from("0"));
        vec.push(String::from("1"));
        vec.push(String::from("2"));
        vec.push(String::from("3"));
        vec.push(String::from("4"));
    }

    // type erased remove
    {
        let mut removed = any_vec.remove(2);
        // test temp_mut_access
        let str = removed.downcast_mut::<String>().unwrap();
        str.push_str("xxx");
        assert_eq!(str, "2xxx");
    }
    assert_equal(any_vec.downcast_ref::<String>().unwrap().as_slice(), &[
        String::from("0"),
        String::from("1"),
        String::from("3"),
        String::from("4"),
    ]);

    // remove last
    any_vec.remove(3);
    assert_equal(any_vec.downcast_ref::<String>().unwrap().as_slice(), &[
        String::from("0"),
        String::from("1"),
        String::from("3"),
    ]);

    // remove first
    any_vec.remove(0);
    assert_equal(any_vec.downcast_ref::<String>().unwrap().as_slice(), &[
        String::from("1"),
        String::from("3"),
    ]);
}

#[test]
fn swap_remove_test() {
    let mut any_vec: AnyVec = AnyVec::new::<String>();
    {
        let mut vec = any_vec.downcast_mut::<String>().unwrap();
        vec.push(String::from("0"));
        vec.push(String::from("1"));
        vec.push(String::from("2"));
        vec.push(String::from("3"));
        vec.push(String::from("4"));

        let e: String = vec.swap_remove(1);
        assert_eq!(e, String::from("1"));
        assert_equal(vec.as_slice(), &[
            String::from("0"),
            String::from("4"),
            String::from("2"),
            String::from("3"),
        ]);
    }

    any_vec.swap_remove(2);
    assert_equal(any_vec.downcast_ref::<String>().unwrap().as_slice(), &[
        String::from("0"),
        String::from("4"),
        String::from("3"),
    ]);

    any_vec.swap_remove(2);
    assert_equal(any_vec.downcast_ref::<String>().unwrap().as_slice(), &[
        String::from("0"),
        String::from("4"),
    ]);
}

#[test]
fn any_vec_swap_remove_push_test() {
    let mut any_vec: AnyVec = AnyVec::new::<String>();
    any_vec.push(AnyValueWrapper::new(String::from("0")));
    any_vec.push(AnyValueWrapper::new(String::from("1")));
    any_vec.push(AnyValueWrapper::new(String::from("3")));
    any_vec.push(AnyValueWrapper::new(String::from("4")));

    let mut any_vec_other: AnyVec = AnyVec::new::<String>();
    let mut element = any_vec.swap_remove(1);
    *element.downcast_mut::<String>().unwrap() += "+";
    any_vec_other.push(element);

    assert_equal(any_vec.downcast_ref::<String>().unwrap().as_slice(), &[
        String::from("0"),
        String::from("4"),
        String::from("3"),
    ]);
    assert_equal(any_vec_other.downcast_ref::<String>().unwrap().as_slice(), &[
        String::from("1+"),
    ]);
}

#[test]
fn any_vec_drain_all_test() {
    let mut any_vec: AnyVec = AnyVec::new::<String>();
    any_vec.push(AnyValueWrapper::new(String::from("0")));
    any_vec.push(AnyValueWrapper::new(String::from("1")));
    any_vec.push(AnyValueWrapper::new(String::from("2")));
    any_vec.push(AnyValueWrapper::new(String::from("3")));

    let mut any_vec2: AnyVec = AnyVec::new::<String>();
    for e in any_vec.drain(..){
        any_vec2.push(e);
    }
    assert_eq!(any_vec.len(), 0);
    assert_equal(any_vec2.downcast_ref::<String>().unwrap().as_slice(), &[
        String::from("0"),
        String::from("1"),
        String::from("2"),
        String::from("3"),
    ]);
}

#[test]
fn any_vec_drain_in_the_middle_test() {
    let mut any_vec: AnyVec = AnyVec::new::<String>();
    any_vec.push(AnyValueWrapper::new(String::from("0")));
    any_vec.push(AnyValueWrapper::new(String::from("1")));
    any_vec.push(AnyValueWrapper::new(String::from("2")));
    any_vec.push(AnyValueWrapper::new(String::from("3")));
    any_vec.push(AnyValueWrapper::new(String::from("4")));

    let mut any_vec2: AnyVec = AnyVec::new::<String>();
    for e in any_vec.drain(1..3){
        any_vec2.push(e);
    }
    assert_equal(any_vec2.downcast_ref::<String>().unwrap().as_slice(), &[
        String::from("1"),
        String::from("2"),
    ]);
    assert_equal(any_vec.downcast_ref::<String>().unwrap().as_slice(), &[
        String::from("0"),
        String::from("3"),
        String::from("4"),
    ]);
}

#[test]
fn any_vec_splice_test() {
    let mut any_vec: AnyVec = AnyVec::new::<String>();
    any_vec.push(AnyValueWrapper::new(String::from("0")));
    any_vec.push(AnyValueWrapper::new(String::from("1")));
    any_vec.push(AnyValueWrapper::new(String::from("2")));
    any_vec.push(AnyValueWrapper::new(String::from("3")));
    any_vec.push(AnyValueWrapper::new(String::from("4")));

    let mut any_vec2: AnyVec = AnyVec::new::<String>();
    let drained = any_vec.splice(1..4, [
        AnyValueWrapper::new(String::from("100")),
        AnyValueWrapper::new(String::from("200"))
    ]);
    assert_eq!(drained.len(), 3);   // Test ExactSizeIterator
    for e in drained{
        any_vec2.push(e);
    }
    assert_equal(any_vec2.downcast_ref::<String>().unwrap().as_slice(), &[
        String::from("1"),
        String::from("2"),
        String::from("3"),
    ]);
    assert_equal(any_vec.downcast_ref::<String>().unwrap().as_slice(), &[
        String::from("0"),
        String::from("100"),
        String::from("200"),
        String::from("4"),
    ]);
}

#[test]
fn any_vec_insert_front(){
    let mut any_vec: AnyVec = AnyVec::new::<usize>();
    let mut vec = any_vec.downcast_mut::<usize>().unwrap();
    for i in 0..100{
        vec.insert(0, i);
    }
    assert_equal(vec.as_slice().iter().copied(), (0..100).rev());
}

#[test]
fn any_vec_insert_back(){
    let mut any_vec: AnyVec = AnyVec::new::<usize>();
    let mut vec = any_vec.downcast_mut::<usize>().unwrap();
    for i in 0..100{
        vec.insert(i, i);
    }
    assert_equal(vec.as_slice().iter().copied(), 0..100);
}

#[test]
fn reserve_test(){
    let mut any_vec: AnyVec = AnyVec::new::<String>();
    assert_eq!(any_vec.capacity(), 0);

    any_vec.reserve(4);
    assert_eq!(any_vec.capacity(), 4);

    any_vec.reserve(6);
    assert_eq!(any_vec.capacity(), 8);
}

#[test]
fn reserve_exact_test(){
    let mut any_vec: AnyVec = AnyVec::new::<String>();
    assert_eq!(any_vec.capacity(), 0);

    any_vec.reserve_exact(4);
    assert_eq!(any_vec.capacity(), 4);

    any_vec.reserve_exact(6);
    assert_eq!(any_vec.capacity(), 6);
}

#[test]
fn shrink_to_fit_test(){
    let mut any_vec: AnyVec = AnyVec::new::<String>();
    assert_eq!(any_vec.capacity(), 0);

    any_vec.reserve_exact(10);
    assert_eq!(any_vec.capacity(), 10);

    any_vec.shrink_to_fit();
    assert_eq!(any_vec.capacity(), 0);
}

#[test]
fn shrink_to_test(){
    let mut any_vec: AnyVec = AnyVec::new::<String>();
    assert_eq!(any_vec.capacity(), 0);

    any_vec.reserve_exact(10);
    assert_eq!(any_vec.capacity(), 10);

    any_vec.shrink_to(5);
    assert_eq!(any_vec.capacity(), 5);
}

#[test]
fn mem_stack_test(){
    use any_vec::traits::None;
    type FixedAnyVec<Traits = dyn None> = AnyVec<Traits, Stack<513>>;

    let mut any_vec: FixedAnyVec = AnyVec::new::<String>();
    {
        let mut vec = any_vec.downcast_mut::<String>().unwrap();
        vec.push(String::from("0"));
        vec.push(String::from("1"));
        vec.push(String::from("2"));
        vec.push(String::from("3"));
    }

    // Should fail to compile.
    //any_vec.reserve(1);

    assert_eq!(any_vec.capacity(), 513/size_of::<String>());
    assert_eq!(any_vec.len(), 4);
    assert_equal(any_vec.downcast_ref::<String>().unwrap(), &[
        String::from("0"),
        String::from("1"),
        String::from("2"),
        String::from("3")
    ]);
}

#[test]
fn any_vec_into_iter_test() {
    let mut any_vec: AnyVec = AnyVec::new::<usize>();
    {
        let mut vec = any_vec.downcast_mut::<usize>().unwrap();
        vec.push(1);
        vec.push(10);
        vec.push(100);
    }

    let mut sum = 0;
    for e in &any_vec{
        sum += e.downcast_ref::<usize>().unwrap();
    }
    assert_eq!(sum, 111);

    let mut sum = 0;
    for mut e in &mut any_vec{
        let value = e.downcast_mut::<usize>().unwrap();
        *value *= 10;
        sum += *value;
    }
    assert_eq!(sum, 1110);
}

#[test]
fn any_vec_debug() {
    let any_vec: AnyVec = AnyVec::new::<usize>();
    let typeid = TypeId::of::<usize>();
    assert_eq!(format!("{any_vec:?}"), format!("AnyVec {{ typeid: {typeid:?}, len: 0 }}"));
}
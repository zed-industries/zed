extern crate arrayvec;
#[macro_use] extern crate matches;

use arrayvec::ArrayVec;
use arrayvec::ArrayString;
use std::mem;
use arrayvec::CapacityError;

use std::collections::HashMap;


#[test]
fn test_simple() {
    use std::ops::Add;

    let mut vec: ArrayVec<Vec<i32>,  3> = ArrayVec::new();

    vec.push(vec![1, 2, 3, 4]);
    vec.push(vec![10]);
    vec.push(vec![-1, 13, -2]);

    for elt in &vec {
        assert_eq!(elt.iter().fold(0, Add::add), 10);
    }

    let sum_len = vec.into_iter().map(|x| x.len()).fold(0, Add::add);
    assert_eq!(sum_len, 8);
}

#[test]
fn test_capacity_left() {
    let mut vec: ArrayVec<usize,  4> = ArrayVec::new();
    assert_eq!(vec.remaining_capacity(), 4);
    vec.push(1);
    assert_eq!(vec.remaining_capacity(), 3);
    vec.push(2);
    assert_eq!(vec.remaining_capacity(), 2);
    vec.push(3);
    assert_eq!(vec.remaining_capacity(), 1);
    vec.push(4);
    assert_eq!(vec.remaining_capacity(), 0);
}

#[test]
fn test_extend_from_slice() {
    let mut vec: ArrayVec<usize,  10> = ArrayVec::new();

    vec.try_extend_from_slice(&[1, 2, 3]).unwrap();
    assert_eq!(vec.len(), 3);
    assert_eq!(&vec[..], &[1, 2, 3]);
    assert_eq!(vec.pop(), Some(3));
    assert_eq!(&vec[..], &[1, 2]);
}

#[test]
fn test_extend_from_slice_error() {
    let mut vec: ArrayVec<usize,  10> = ArrayVec::new();

    vec.try_extend_from_slice(&[1, 2, 3]).unwrap();
    let res = vec.try_extend_from_slice(&[0; 8]);
    assert_matches!(res, Err(_));

    let mut vec: ArrayVec<usize,  0> = ArrayVec::new();
    let res = vec.try_extend_from_slice(&[0; 1]);
    assert_matches!(res, Err(_));
}

#[test]
fn test_try_from_slice_error() {
    use arrayvec::ArrayVec;
    use std::convert::TryInto as _;

    let res: Result<ArrayVec<_,  2>, _> = (&[1, 2, 3] as &[_]).try_into();
    assert_matches!(res, Err(_));
}

#[test]
fn test_u16_index() {
    const N: usize = 4096;
    let mut vec: ArrayVec<_,  N> = ArrayVec::new();
    for _ in 0..N {
        assert!(vec.try_push(1u8).is_ok());
    }
    assert!(vec.try_push(0).is_err());
    assert_eq!(vec.len(), N);
}

#[test]
fn test_iter() {
    let mut iter = ArrayVec::from([1, 2, 3]).into_iter();
    assert_eq!(iter.size_hint(), (3, Some(3)));
    assert_eq!(iter.next_back(), Some(3));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next_back(), Some(2));
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.next_back(), None);
}

#[test]
fn test_drop() {
    use std::cell::Cell;

    let flag = &Cell::new(0);

    #[derive(Clone)]
    struct Bump<'a>(&'a Cell<i32>);

    impl<'a> Drop for Bump<'a> {
        fn drop(&mut self) {
            let n = self.0.get();
            self.0.set(n + 1);
        }
    }

    {
        let mut array = ArrayVec::<Bump,  128>::new();
        array.push(Bump(flag));
        array.push(Bump(flag));
    }
    assert_eq!(flag.get(), 2);

    // test something with the nullable pointer optimization
    flag.set(0);

    {
        let mut array = ArrayVec::<_,  3>::new();
        array.push(vec![Bump(flag)]);
        array.push(vec![Bump(flag), Bump(flag)]);
        array.push(vec![]);
        let push4 = array.try_push(vec![Bump(flag)]);
        assert_eq!(flag.get(), 0);
        drop(push4);
        assert_eq!(flag.get(), 1);
        drop(array.pop());
        assert_eq!(flag.get(), 1);
        drop(array.pop());
        assert_eq!(flag.get(), 3);
    }

    assert_eq!(flag.get(), 4);

    // test into_inner
    flag.set(0);
    {
        let mut array = ArrayVec::<_,  3>::new();
        array.push(Bump(flag));
        array.push(Bump(flag));
        array.push(Bump(flag));
        let inner = array.into_inner();
        assert!(inner.is_ok());
        assert_eq!(flag.get(), 0);
        drop(inner);
        assert_eq!(flag.get(), 3);
    }

    // test take
    flag.set(0);
    {
        let mut array1 = ArrayVec::<_,  3>::new();
        array1.push(Bump(flag));
        array1.push(Bump(flag));
        array1.push(Bump(flag));
        let array2 = array1.take();
        assert_eq!(flag.get(), 0);
        drop(array1);
        assert_eq!(flag.get(), 0);
        drop(array2);
        assert_eq!(flag.get(), 3);
    }

    // test cloning into_iter
    flag.set(0);
    {
        let mut array = ArrayVec::<_,  3>::new();
        array.push(Bump(flag));
        array.push(Bump(flag));
        array.push(Bump(flag));
        let mut iter = array.into_iter();
        assert_eq!(flag.get(), 0);
        iter.next();
        assert_eq!(flag.get(), 1);
        let clone = iter.clone();
        assert_eq!(flag.get(), 1);
        drop(clone);
        assert_eq!(flag.get(), 3);
        drop(iter);
        assert_eq!(flag.get(), 5);
    }
}

#[test]
fn test_drop_panics() {
    use std::cell::Cell;
    use std::panic::catch_unwind;
    use std::panic::AssertUnwindSafe;

    let flag = &Cell::new(0);

    struct Bump<'a>(&'a Cell<i32>);

    // Panic in the first drop
    impl<'a> Drop for Bump<'a> {
        fn drop(&mut self) {
            let n = self.0.get();
            self.0.set(n + 1);
            if n == 0 {
                panic!("Panic in Bump's drop");
            }
        }
    }
    // check if rust is new enough
    flag.set(0);
    {
        let array = vec![Bump(flag), Bump(flag)];
        let res = catch_unwind(AssertUnwindSafe(|| {
            drop(array);
        }));
        assert!(res.is_err());
    }

    if flag.get() != 2 {
        println!("test_drop_panics: skip, this version of Rust doesn't continue in drop_in_place");
        return;
    }

    flag.set(0);
    {
        let mut array = ArrayVec::<Bump,  128>::new();
        array.push(Bump(flag));
        array.push(Bump(flag));
        array.push(Bump(flag));

        let res = catch_unwind(AssertUnwindSafe(|| {
            drop(array);
        }));
        assert!(res.is_err());
    }
    // Check that all the elements drop, even if the first drop panics.
    assert_eq!(flag.get(), 3);


    flag.set(0);
    {
        let mut array = ArrayVec::<Bump,  16>::new();
        array.push(Bump(flag));
        array.push(Bump(flag));
        array.push(Bump(flag));
        array.push(Bump(flag));
        array.push(Bump(flag));

        let i = 2;
        let tail_len = array.len() - i;

        let res = catch_unwind(AssertUnwindSafe(|| {
            array.truncate(i);
        }));
        assert!(res.is_err());
        // Check that all the tail elements drop, even if the first drop panics.
        assert_eq!(flag.get(), tail_len as i32);
    }


}

#[test]
fn test_extend() {
    let mut range = 0..10;

    let mut array: ArrayVec<_,  5> = range.by_ref().take(5).collect();
    assert_eq!(&array[..], &[0, 1, 2, 3, 4]);
    assert_eq!(range.next(), Some(5));

    array.extend(range.by_ref().take(0));
    assert_eq!(range.next(), Some(6));

    let mut array: ArrayVec<_,  10> = (0..3).collect();
    assert_eq!(&array[..], &[0, 1, 2]);
    array.extend(3..5);
    assert_eq!(&array[..], &[0, 1, 2, 3, 4]);
}

#[should_panic]
#[test]
fn test_extend_capacity_panic_1() {
    let mut range = 0..10;

    let _: ArrayVec<_,  5> = range.by_ref().collect();
}

#[should_panic]
#[test]
fn test_extend_capacity_panic_2() {
    let mut range = 0..10;

    let mut array: ArrayVec<_,  5> = range.by_ref().take(5).collect();
    assert_eq!(&array[..], &[0, 1, 2, 3, 4]);
    assert_eq!(range.next(), Some(5));
    array.extend(range.by_ref().take(1));
}

#[test]
fn test_is_send_sync() {
    let data = ArrayVec::<Vec<i32>,  5>::new();
    &data as &dyn Send;
    &data as &dyn Sync;
}

#[test]
fn test_compact_size() {
    // 4 bytes + padding + length
    type ByteArray = ArrayVec<u8,  4>;
    println!("{}", mem::size_of::<ByteArray>());
    assert!(mem::size_of::<ByteArray>() <= 2 * mem::size_of::<u32>());

    // just length
    type EmptyArray = ArrayVec<u8,  0>;
    println!("{}", mem::size_of::<EmptyArray>());
    assert!(mem::size_of::<EmptyArray>() <= mem::size_of::<u32>());

    // 3 elements + padding + length
    type QuadArray = ArrayVec<u32,  3>;
    println!("{}", mem::size_of::<QuadArray>());
    assert!(mem::size_of::<QuadArray>() <= 4 * 4 + mem::size_of::<u32>());
}

#[test]
fn test_still_works_with_option_arrayvec() {
    type RefArray = ArrayVec<&'static i32,  2>;
    let array = Some(RefArray::new());
    assert!(array.is_some());
    println!("{:?}", array);
}

#[test]
fn test_drain() {
    let mut v = ArrayVec::from([0; 8]);
    v.pop();
    v.drain(0..7);
    assert_eq!(&v[..], &[]);

    v.extend(0..8);
    v.drain(1..4);
    assert_eq!(&v[..], &[0, 4, 5, 6, 7]);
    let u: ArrayVec<_,  3> = v.drain(1..4).rev().collect();
    assert_eq!(&u[..], &[6, 5, 4]);
    assert_eq!(&v[..], &[0, 7]);
    v.drain(..);
    assert_eq!(&v[..], &[]);
}

#[test]
fn test_drain_range_inclusive() {
    let mut v = ArrayVec::from([0; 8]);
    v.drain(0..=7);
    assert_eq!(&v[..], &[]);

    v.extend(0..8);
    v.drain(1..=4);
    assert_eq!(&v[..], &[0, 5, 6, 7]);
    let u: ArrayVec<_,  3> = v.drain(1..=2).rev().collect();
    assert_eq!(&u[..], &[6, 5]);
    assert_eq!(&v[..], &[0, 7]);
    v.drain(..);
    assert_eq!(&v[..], &[]);
}

#[test]
#[should_panic]
fn test_drain_range_inclusive_oob() {
    let mut v = ArrayVec::from([0; 0]);
    v.drain(0..=0);
}

#[test]
fn test_retain() {
    let mut v = ArrayVec::from([0; 8]);
    for (i, elt) in v.iter_mut().enumerate() {
        *elt = i;
    }
    v.retain(|_| true);
    assert_eq!(&v[..], &[0, 1, 2, 3, 4, 5, 6, 7]);
    v.retain(|elt| {
        *elt /= 2;
        *elt % 2 == 0
    });
    assert_eq!(&v[..], &[0, 0, 2, 2]);
    v.retain(|_| false);
    assert_eq!(&v[..], &[]);
}

#[test]
#[should_panic]
fn test_drain_oob() {
    let mut v = ArrayVec::from([0; 8]);
    v.pop();
    v.drain(0..8);
}

#[test]
#[should_panic]
fn test_drop_panic() {
    struct DropPanic;

    impl Drop for DropPanic {
        fn drop(&mut self) {
            panic!("drop");
        }
    }

    let mut array = ArrayVec::<DropPanic,  1>::new();
    array.push(DropPanic);
}

#[test]
#[should_panic]
fn test_drop_panic_into_iter() {
    struct DropPanic;

    impl Drop for DropPanic {
        fn drop(&mut self) {
            panic!("drop");
        }
    }

    let mut array = ArrayVec::<DropPanic,  1>::new();
    array.push(DropPanic);
    array.into_iter();
}

#[test]
fn test_insert() {
    let mut v = ArrayVec::from([]);
    assert_matches!(v.try_push(1), Err(_));

    let mut v = ArrayVec::<_,  3>::new();
    v.insert(0, 0);
    v.insert(1, 1);
    //let ret1 = v.try_insert(3, 3);
    //assert_matches!(ret1, Err(InsertError::OutOfBounds(_)));
    assert_eq!(&v[..], &[0, 1]);
    v.insert(2, 2);
    assert_eq!(&v[..], &[0, 1, 2]);

    let ret2 = v.try_insert(1, 9);
    assert_eq!(&v[..], &[0, 1, 2]);
    assert_matches!(ret2, Err(_));

    let mut v = ArrayVec::from([2]);
    assert_matches!(v.try_insert(0, 1), Err(CapacityError { .. }));
    assert_matches!(v.try_insert(1, 1), Err(CapacityError { .. }));
    //assert_matches!(v.try_insert(2, 1), Err(CapacityError { .. }));
}

#[test]
fn test_into_inner_1() {
    let mut v = ArrayVec::from([1, 2]);
    v.pop();
    let u = v.clone();
    assert_eq!(v.into_inner(), Err(u));
}

#[test]
fn test_into_inner_2() {
    let mut v = ArrayVec::<String,  4>::new();
    v.push("a".into());
    v.push("b".into());
    v.push("c".into());
    v.push("d".into());
    assert_eq!(v.into_inner().unwrap(), ["a", "b", "c", "d"]);
}

#[test]
fn test_into_inner_3() {
    let mut v = ArrayVec::<i32,  4>::new();
    v.extend(1..=4);
    assert_eq!(v.into_inner().unwrap(), [1, 2, 3, 4]);
}

#[test]
fn test_take() {
    let mut v1 = ArrayVec::<i32,  4>::new();
    v1.extend(1..=4);
    let v2 = v1.take();
    assert!(v1.into_inner().is_err());
    assert_eq!(v2.into_inner().unwrap(), [1, 2, 3, 4]);
}

#[cfg(feature="std")]
#[test]
fn test_write() {
    use std::io::Write;
    let mut v = ArrayVec::<_,  8>::new();
    write!(&mut v, "\x01\x02\x03").unwrap();
    assert_eq!(&v[..], &[1, 2, 3]);
    let r = v.write(&[9; 16]).unwrap();
    assert_eq!(r, 5);
    assert_eq!(&v[..], &[1, 2, 3, 9, 9, 9, 9, 9]);
}

#[test]
fn array_clone_from() {
    let mut v = ArrayVec::<_,  4>::new();
    v.push(vec![1, 2]);
    v.push(vec![3, 4, 5]);
    v.push(vec![6]);
    let reference = v.to_vec();
    let mut u = ArrayVec::<_,  4>::new();
    u.clone_from(&v);
    assert_eq!(&u, &reference[..]);

    let mut t = ArrayVec::<_,  4>::new();
    t.push(vec![97]);
    t.push(vec![]);
    t.push(vec![5, 6, 2]);
    t.push(vec![2]);
    t.clone_from(&v);
    assert_eq!(&t, &reference[..]);
    t.clear();
    t.clone_from(&v);
    assert_eq!(&t, &reference[..]);
}

#[cfg(feature="std")]
#[test]
fn test_string() {
    use std::error::Error;

    let text = "hello world";
    let mut s = ArrayString::<16>::new();
    s.try_push_str(text).unwrap();
    assert_eq!(&s, text);
    assert_eq!(text, &s);

    // Make sure Hash / Eq / Borrow match up so we can use HashMap
    let mut map = HashMap::new();
    map.insert(s, 1);
    assert_eq!(map[text], 1);

    let mut t = ArrayString::<2>::new();
    assert!(t.try_push_str(text).is_err());
    assert_eq!(&t, "");

    t.push_str("ab");
    // DerefMut
    let tmut: &mut str = &mut t;
    assert_eq!(tmut, "ab");

    // Test Error trait / try
    let t = || -> Result<(), Box<dyn Error>> {
        let mut t = ArrayString::<2>::new();
        t.try_push_str(text)?;
        Ok(())
    }();
    assert!(t.is_err());
}

#[test]
fn test_string_from() {
    let text = "hello world";
	// Test `from` constructor
    let u = ArrayString::<11>::from(text).unwrap();
    assert_eq!(&u, text);
    assert_eq!(u.len(), text.len());
}

#[test]
fn test_string_parse_from_str() {
    let text = "hello world";
    let u: ArrayString<11> = text.parse().unwrap();
    assert_eq!(&u, text);
    assert_eq!(u.len(), text.len());
}

#[test]
fn test_string_from_bytes() {
    let text = "hello world";
    let u = ArrayString::from_byte_string(b"hello world").unwrap();
    assert_eq!(&u, text);
    assert_eq!(u.len(), text.len());
}

#[test]
fn test_string_clone() {
    let text = "hi";
    let mut s = ArrayString::<4>::new();
    s.push_str("abcd");
    let t = ArrayString::<4>::from(text).unwrap();
    s.clone_from(&t);
    assert_eq!(&t, &s);
}

#[test]
fn test_string_push() {
    let text = "abcαβγ";
    let mut s = ArrayString::<8>::new();
    for c in text.chars() {
        if let Err(_) = s.try_push(c) {
            break;
        }
    }
    assert_eq!("abcαβ", &s[..]);
    s.push('x');
    assert_eq!("abcαβx", &s[..]);
    assert!(s.try_push('x').is_err());
}


#[test]
fn test_insert_at_length() {
    let mut v = ArrayVec::<_,  8>::new();
    let result1 = v.try_insert(0, "a");
    let result2 = v.try_insert(1, "b");
    assert!(result1.is_ok() && result2.is_ok());
    assert_eq!(&v[..], &["a", "b"]);
}

#[should_panic]
#[test]
fn test_insert_out_of_bounds() {
    let mut v = ArrayVec::<_,  8>::new();
    let _ = v.try_insert(1, "test");
}

/*
 * insert that pushes out the last
    let mut u = ArrayVec::from([1, 2, 3, 4]);
    let ret = u.try_insert(3, 99);
    assert_eq!(&u[..], &[1, 2, 3, 99]);
    assert_matches!(ret, Err(_));
    let ret = u.try_insert(4, 77);
    assert_eq!(&u[..], &[1, 2, 3, 99]);
    assert_matches!(ret, Err(_));
*/

#[test]
fn test_drop_in_insert() {
    use std::cell::Cell;

    let flag = &Cell::new(0);

    struct Bump<'a>(&'a Cell<i32>);

    impl<'a> Drop for Bump<'a> {
        fn drop(&mut self) {
            let n = self.0.get();
            self.0.set(n + 1);
        }
    }

    flag.set(0);

    {
        let mut array = ArrayVec::<_,  2>::new();
        array.push(Bump(flag));
        array.insert(0, Bump(flag));
        assert_eq!(flag.get(), 0);
        let ret = array.try_insert(1, Bump(flag));
        assert_eq!(flag.get(), 0);
        assert_matches!(ret, Err(_));
        drop(ret);
        assert_eq!(flag.get(), 1);
    }
    assert_eq!(flag.get(), 3);
}

#[test]
fn test_pop_at() {
    let mut v = ArrayVec::<String,  4>::new();
    let s = String::from;
    v.push(s("a"));
    v.push(s("b"));
    v.push(s("c"));
    v.push(s("d"));

    assert_eq!(v.pop_at(4), None);
    assert_eq!(v.pop_at(1), Some(s("b")));
    assert_eq!(v.pop_at(1), Some(s("c")));
    assert_eq!(v.pop_at(2), None);
    assert_eq!(&v[..], &["a", "d"]);
}

#[test]
fn test_sizes() {
    let v = ArrayVec::from([0u8; 1 << 16]);
    assert_eq!(vec![0u8; v.len()], &v[..]);
}

#[test]
fn test_default() {
    use std::net;
    let s: ArrayString<4> = Default::default();
    // Something without `Default` implementation.
    let v: ArrayVec<net::TcpStream,  4> = Default::default();
    assert_eq!(s.len(), 0);
    assert_eq!(v.len(), 0);
}

#[test]
fn test_extend_zst() {
    let mut range = 0..10;
    #[derive(Copy, Clone, PartialEq, Debug)]
    struct Z; // Zero sized type

    let mut array: ArrayVec<_,  5> = range.by_ref().take(5).map(|_| Z).collect();
    assert_eq!(&array[..], &[Z; 5]);
    assert_eq!(range.next(), Some(5));

    array.extend(range.by_ref().take(0).map(|_| Z));
    assert_eq!(range.next(), Some(6));

    let mut array: ArrayVec<_,  10> = (0..3).map(|_| Z).collect();
    assert_eq!(&array[..], &[Z; 3]);
    array.extend((3..5).map(|_| Z));
    assert_eq!(&array[..], &[Z; 5]);
    assert_eq!(array.len(), 5);
}

#[test]
fn test_try_from_argument() {
    use core::convert::TryFrom;
    let v = ArrayString::<16>::try_from(format_args!("Hello {}", 123)).unwrap();
    assert_eq!(&v, "Hello 123");
}

#[test]
fn allow_max_capacity_arrayvec_type() {
    // this type is allowed to be used (but can't be constructed)
    let _v: ArrayVec<(), {usize::MAX}>;
}

#[should_panic(expected="largest supported capacity")]
#[test]
fn deny_max_capacity_arrayvec_value() {
    if mem::size_of::<usize>() <= mem::size_of::<u32>() {
        panic!("This test does not work on this platform. 'largest supported capacity'");
    }
    // this type is allowed to be used (but can't be constructed)
    let _v: ArrayVec<(), {usize::MAX}> = ArrayVec::new();
}

#[should_panic(expected="index out of bounds")]
#[test]
fn deny_max_capacity_arrayvec_value_const() {
    if mem::size_of::<usize>() <= mem::size_of::<u32>() {
        panic!("This test does not work on this platform. 'index out of bounds'");
    }
    // this type is allowed to be used (but can't be constructed)
    let _v: ArrayVec<(), {usize::MAX}> = ArrayVec::new_const();
}

#[test]
fn test_arrayvec_const_constructible() {
    const OF_U8: ArrayVec<Vec<u8>, 10> = ArrayVec::new_const();

    let mut var = OF_U8;
    assert!(var.is_empty());
    assert_eq!(var, ArrayVec::new());
    var.push(vec![3, 5, 8]);
    assert_eq!(var[..], [vec![3, 5, 8]]);
}

#[test]
fn test_arraystring_const_constructible() {
    const AS: ArrayString<10> = ArrayString::new_const();

    let mut var = AS;
    assert!(var.is_empty());
    assert_eq!(var, ArrayString::new());
    var.push_str("hello");
    assert_eq!(var, *"hello");
}


#[test]
fn test_arraystring_zero_filled_has_some_sanity_checks() {
    let string = ArrayString::<4>::zero_filled();
    assert_eq!(string.as_str(), "\0\0\0\0");
    assert_eq!(string.len(), 4);
}

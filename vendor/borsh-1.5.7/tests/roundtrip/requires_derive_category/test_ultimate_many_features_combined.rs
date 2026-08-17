use core::{ops, result::Result};

use alloc::{
    borrow,
    boxed::Box,
    collections::{BTreeMap, BTreeSet, LinkedList, VecDeque},
    string::{String, ToString},
    vec,
    vec::Vec,
};

use bytes::{Bytes, BytesMut};

use borsh::{from_slice, to_vec, BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
struct A<'a> {
    x: u64,
    b: B,
    y: f32,
    z: String,
    t: (String, u64),
    btree_map_string: BTreeMap<String, String>,
    btree_set_u64: BTreeSet<u64>,
    linked_list_string: LinkedList<String>,
    vec_deque_u64: VecDeque<u64>,
    bytes: Bytes,
    bytes_mut: BytesMut,
    v: Vec<String>,
    w: Box<[u8]>,
    box_str: Box<str>,
    i: [u8; 32],
    u: Result<String, String>,
    lazy: Option<u64>,
    c: borrow::Cow<'a, str>,
    cow_arr: borrow::Cow<'a, [borrow::Cow<'a, str>]>,
    range_u32: ops::Range<u32>,
    #[borsh(skip)]
    skipped: Option<u64>,
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
struct B {
    x: u64,
    y: i32,
    c: C,
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
enum C {
    C1,
    C2(u64),
    C3(u64, u64),
    C4 { x: u64, y: u64 },
    C5(D),
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
struct D {
    x: u64,
}

#[derive(BorshSerialize)]
struct E<'a, 'b> {
    a: &'a A<'b>,
}

#[derive(BorshSerialize)]
struct F1<'a, 'b> {
    aa: &'a [&'a A<'b>],
}

#[derive(BorshDeserialize)]
struct F2<'b> {
    aa: Vec<A<'b>>,
}

#[test]
fn test_ultimate_combined_all_features() {
    let mut map: BTreeMap<String, String> = BTreeMap::new();
    map.insert("test".into(), "test".into());
    let mut set: BTreeSet<u64> = BTreeSet::new();
    set.insert(u64::MAX);
    let cow_arr = [
        borrow::Cow::Borrowed("Hello1"),
        borrow::Cow::Owned("Hello2".to_string()),
    ];
    let a = A {
        x: 1,
        b: B {
            x: 2,
            y: 3,
            c: C::C5(D { x: 1 }),
        },
        y: 4.0,
        z: "123".to_string(),
        t: ("Hello".to_string(), 10),
        btree_map_string: map.clone(),
        btree_set_u64: set.clone(),
        linked_list_string: vec!["a".to_string(), "b".to_string()].into_iter().collect(),
        vec_deque_u64: vec![1, 2, 3].into_iter().collect(),
        bytes: vec![5, 4, 3, 2, 1].into(),
        bytes_mut: BytesMut::from(&[1, 2, 3, 4, 5][..]),
        v: vec!["qwe".to_string(), "zxc".to_string()],
        w: vec![0].into_boxed_slice(),
        box_str: Box::from("asd"),
        i: [4u8; 32],
        u: Ok("Hello".to_string()),
        lazy: Some(5),
        c: borrow::Cow::Borrowed("Hello"),
        cow_arr: borrow::Cow::Borrowed(&cow_arr),
        range_u32: 12..71,
        skipped: Some(6),
    };
    let encoded_a = to_vec(&a).unwrap();
    let e = E { a: &a };
    let encoded_ref_a = to_vec(&e).unwrap();
    assert_eq!(encoded_ref_a, encoded_a);
    #[cfg(feature = "std")]
    insta::assert_debug_snapshot!(encoded_a);

    let decoded_a = from_slice::<A>(&encoded_a).unwrap();
    let expected_a = A {
        x: 1,
        b: B {
            x: 2,
            y: 3,
            c: C::C5(D { x: 1 }),
        },
        y: 4.0,
        z: a.z.clone(),
        t: ("Hello".to_string(), 10),
        btree_map_string: map,
        btree_set_u64: set,
        linked_list_string: vec!["a".to_string(), "b".to_string()].into_iter().collect(),
        vec_deque_u64: vec![1, 2, 3].into_iter().collect(),
        bytes: vec![5, 4, 3, 2, 1].into(),
        bytes_mut: BytesMut::from(&[1, 2, 3, 4, 5][..]),
        v: a.v.clone(),
        w: a.w.clone(),
        box_str: Box::from("asd"),
        i: a.i,
        u: Ok("Hello".to_string()),
        lazy: Some(5),
        c: borrow::Cow::Owned("Hello".to_string()),
        cow_arr: borrow::Cow::Owned(vec![
            borrow::Cow::Owned("Hello1".to_string()),
            borrow::Cow::Owned("Hello2".to_string()),
        ]),
        range_u32: 12..71,
        skipped: None,
    };

    assert_eq!(expected_a, decoded_a);

    let f1 = F1 { aa: &[&a, &a] };
    let encoded_f1 = to_vec(&f1).unwrap();
    #[cfg(feature = "std")]
    insta::assert_debug_snapshot!(encoded_f1);
    let decoded_f2 = from_slice::<F2>(&encoded_f1).unwrap();
    assert_eq!(decoded_f2.aa.len(), 2);
    assert!(decoded_f2.aa.iter().all(|f2_a| f2_a == &expected_a));
}

#[test]
fn test_object_length() {
    let mut map: BTreeMap<String, String> = BTreeMap::new();
    map.insert("test".into(), "test".into());
    let mut set: BTreeSet<u64> = BTreeSet::new();
    set.insert(u64::MAX);
    set.insert(100);
    set.insert(103);
    set.insert(109);
    let cow_arr = [
        borrow::Cow::Borrowed("Hello1"),
        borrow::Cow::Owned("Hello2".to_string()),
    ];
    let a = A {
        x: 1,
        b: B {
            x: 2,
            y: 3,
            c: C::C5(D { x: 1 }),
        },
        y: 4.0,
        z: "123".to_string(),
        t: ("Hello".to_string(), 10),
        btree_map_string: map.clone(),
        btree_set_u64: set.clone(),
        linked_list_string: vec!["a".to_string(), "b".to_string()].into_iter().collect(),
        vec_deque_u64: vec![1, 2, 3].into_iter().collect(),
        bytes: vec![5, 4, 3, 2, 1].into(),
        bytes_mut: BytesMut::from(&[1, 2, 3, 4, 5][..]),
        v: vec!["qwe".to_string(), "zxc".to_string()],
        w: vec![0].into_boxed_slice(),
        box_str: Box::from("asd"),
        i: [4u8; 32],
        u: Ok("Hello".to_string()),
        lazy: Some(5),
        c: borrow::Cow::Borrowed("Hello"),
        cow_arr: borrow::Cow::Borrowed(&cow_arr),
        range_u32: 12..71,
        skipped: Some(6),
    };
    let encoded_a_len = to_vec(&a).unwrap().len();

    let len_helper_result = borsh::object_length(&a).unwrap();

    assert_eq!(encoded_a_len, len_helper_result);
}

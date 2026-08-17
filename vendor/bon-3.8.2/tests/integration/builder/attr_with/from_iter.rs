#![cfg(feature = "std")]
#![allow(clippy::linkedlist)]

use crate::prelude::*;
use core::fmt;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

type RenamedVec<T> = Vec<T>;

#[test]
fn test_struct() {
    #[derive(Debug, Builder)]
    #[builder(derive(Clone))]
    struct Sut {
        #[builder(with = <_>::from_iter)]
        _vec: Vec<i32>,

        #[builder(with = FromIterator::from_iter)]
        _optional_vec: Option<::alloc::vec::Vec<i32>>,

        #[builder(with = <_>::from_iter, default)]
        _default_vec: Vec<i32>,

        #[builder(with = FromIterator::from_iter)]
        _renamed_vec: RenamedVec<i32>,

        #[builder(with = <_>::from_iter)]
        _hash_set: HashSet<i32>,

        #[builder(with = FromIterator::from_iter)]
        _btree_set: BTreeSet<i32>,

        #[builder(with = <_>::from_iter)]
        _vec_deque: VecDeque<i32>,

        #[builder(with = FromIterator::from_iter)]
        _binary_heap: BinaryHeap<i32>,

        #[builder(with = <_>::from_iter)]
        _linked_list: std::collections::LinkedList<i32>,

        #[builder(with = FromIterator::from_iter)]
        _hash_map: HashMap<i32, i32>,

        #[builder(with = <_>::from_iter)]
        _btree_map: std::collections::BTreeMap<i32, i32>,
    }

    let builder = Sut::builder();

    let _ignore = builder.clone().optional_vec([1, 2, 3]);
    let builder = builder.maybe_optional_vec(Some([4, 5, 6]));

    let _ignore = builder.clone().default_vec([7, 8, 9]);
    let builder = builder.maybe_default_vec(Some([10, 11, 12]));

    // `Hash*`` collections have random order of iteration, so their debug
    // output is unstable. To work around this instability, we just specify
    // a single element for `Hash*` collections.
    let sut = builder
        .vec([13, 14, 15])
        .renamed_vec([16, 17, 18])
        .hash_set(std::iter::once(19))
        .btree_set([20, 21, 22, 22, 21])
        .vec_deque([23, 24, 25])
        .binary_heap([26, 27, 28])
        .linked_list([29, 30, 31])
        .hash_map(std::iter::once((32, 33)))
        .btree_map([(34, 35), (34, 36), (37, 38)])
        .build();

    assert_debug_eq(
        &sut,
        expect![[r#"
        Sut {
            _vec: [
                13,
                14,
                15,
            ],
            _optional_vec: Some(
                [
                    4,
                    5,
                    6,
                ],
            ),
            _default_vec: [
                10,
                11,
                12,
            ],
            _renamed_vec: [
                16,
                17,
                18,
            ],
            _hash_set: {
                19,
            },
            _btree_set: {
                20,
                21,
                22,
            },
            _vec_deque: [
                23,
                24,
                25,
            ],
            _binary_heap: [
                28,
                27,
                26,
            ],
            _linked_list: [
                29,
                30,
                31,
            ],
            _hash_map: {
                32: 33,
            },
            _btree_map: {
                34: 36,
                37: 38,
            },
        }"#]],
    );
}

#[test]
fn test_function() {
    #[builder(derive(Clone))]
    fn sut(
        #[builder(with = <_>::from_iter)] vec: Vec<i32>,
        #[builder(with = FromIterator::from_iter)] optional_vec: Option<::alloc::vec::Vec<i32>>,
        #[builder(with = <_>::from_iter, default)] default_vec: Vec<i32>,
        #[builder(with = FromIterator::from_iter)] renamed_vec: RenamedVec<i32>,
        #[builder(with = <_>::from_iter)] hash_set: HashSet<i32>,
        #[builder(with = FromIterator::from_iter)] btree_set: BTreeSet<i32>,
        #[builder(with = <_>::from_iter)] vec_deque: VecDeque<i32>,
        #[builder(with = FromIterator::from_iter)] binary_heap: BinaryHeap<i32>,
        #[builder(with = <_>::from_iter)] linked_list: std::collections::LinkedList<i32>,
        #[builder(with = FromIterator::from_iter)] hash_map: HashMap<i32, i32>,
        #[builder(with = <_>::from_iter)] btree_map: std::collections::BTreeMap<i32, i32>,
    ) -> impl fmt::Debug {
        (
            vec,
            optional_vec,
            default_vec,
            renamed_vec,
            hash_set,
            btree_set,
            vec_deque,
            binary_heap,
            linked_list,
            hash_map,
            btree_map,
        )
    }

    let builder = sut();

    let _ignore = builder.clone().optional_vec([1, 2, 3]);
    let builder = builder.maybe_optional_vec(Some([4, 5, 6]));

    let _ignore = builder.clone().default_vec([7, 8, 9]);
    let builder = builder.maybe_default_vec(Some([10, 11, 12]));

    // `Hash*`` collections have random order of iteration, so their debug
    // output is unstable. To work around this instability, we just specify
    // a single element for `Hash*` collections.
    let sut = builder
        .vec([13, 14, 15])
        .renamed_vec([16, 17, 18])
        .hash_set(std::iter::once(19))
        .btree_set([20, 21, 22, 22, 21])
        .vec_deque([23, 24, 25])
        .binary_heap([26, 27, 28])
        .linked_list([29, 30, 31])
        .hash_map(std::iter::once((32, 33)))
        .btree_map([(34, 35), (34, 36), (37, 38)])
        .call();

    assert_debug_eq(
        &sut,
        expect![[r#"
        (
            [
                13,
                14,
                15,
            ],
            Some(
                [
                    4,
                    5,
                    6,
                ],
            ),
            [
                10,
                11,
                12,
            ],
            [
                16,
                17,
                18,
            ],
            {
                19,
            },
            {
                20,
                21,
                22,
            },
            [
                23,
                24,
                25,
            ],
            [
                28,
                27,
                26,
            ],
            [
                29,
                30,
                31,
            ],
            {
                32: 33,
            },
            {
                34: 36,
                37: 38,
            },
        )"#]],
    );
}

#[test]
fn test_method() {
    struct Sut;

    #[bon]
    impl Sut {
        #[builder(derive(Clone))]
        fn sut(
            #[builder(with = <_>::from_iter)] vec: Vec<i32>,
            #[builder(with = FromIterator::from_iter)] optional_vec: Option<::alloc::vec::Vec<i32>>,
            #[builder(with = <_>::from_iter, default)] default_vec: Vec<i32>,
            #[builder(with = FromIterator::from_iter)] renamed_vec: RenamedVec<i32>,
            #[builder(with = <_>::from_iter)] hash_set: HashSet<i32>,
            #[builder(with = FromIterator::from_iter)] btree_set: BTreeSet<i32>,
            #[builder(with = <_>::from_iter)] vec_deque: VecDeque<i32>,
            #[builder(with = FromIterator::from_iter)] binary_heap: BinaryHeap<i32>,
            #[builder(with = <_>::from_iter)] linked_list: std::collections::LinkedList<i32>,
            #[builder(with = FromIterator::from_iter)] hash_map: HashMap<i32, i32>,
            #[builder(with = <_>::from_iter)] btree_map: std::collections::BTreeMap<i32, i32>,
        ) -> impl fmt::Debug {
            (
                vec,
                optional_vec,
                default_vec,
                renamed_vec,
                hash_set,
                btree_set,
                vec_deque,
                binary_heap,
                linked_list,
                hash_map,
                btree_map,
            )
        }

        #[builder(derive(Clone))]
        fn with_self(
            &self,
            #[builder(with = <_>::from_iter)] vec: Vec<i32>,
            #[builder(with = FromIterator::from_iter)] optional_vec: Option<::alloc::vec::Vec<i32>>,
            #[builder(with = <_>::from_iter, default)] default_vec: Vec<i32>,
            #[builder(with = FromIterator::from_iter)] renamed_vec: RenamedVec<i32>,
            #[builder(with = <_>::from_iter)] hash_set: HashSet<i32>,
            #[builder(with = FromIterator::from_iter)] btree_set: BTreeSet<i32>,
            #[builder(with = <_>::from_iter)] vec_deque: VecDeque<i32>,
            #[builder(with = FromIterator::from_iter)] binary_heap: BinaryHeap<i32>,
            #[builder(with = <_>::from_iter)] linked_list: std::collections::LinkedList<i32>,
            #[builder(with = FromIterator::from_iter)] hash_map: HashMap<i32, i32>,
            #[builder(with = <_>::from_iter)] btree_map: std::collections::BTreeMap<i32, i32>,
        ) -> impl fmt::Debug {
            let _ = self;
            (
                vec,
                optional_vec,
                default_vec,
                renamed_vec,
                hash_set,
                btree_set,
                vec_deque,
                binary_heap,
                linked_list,
                hash_map,
                btree_map,
            )
        }
    }

    let builder = Sut::sut();

    let _ignore = builder.clone().optional_vec([1, 2, 3]);
    let builder = builder.maybe_optional_vec(Some([4, 5, 6]));

    let _ignore = builder.clone().default_vec([7, 8, 9]);
    let builder = builder.maybe_default_vec(Some([10, 11, 12]));

    // `Hash*`` collections have random order of iteration, so their debug
    // output is unstable. To work around this instability, we just specify
    // a single element for `Hash*` collections.
    let sut = builder
        .vec([13, 14, 15])
        .renamed_vec([16, 17, 18])
        .hash_set(std::iter::once(19))
        .btree_set([20, 21, 22, 22, 21])
        .vec_deque([23, 24, 25])
        .binary_heap([26, 27, 28])
        .linked_list([29, 30, 31])
        .hash_map(std::iter::once((32, 33)))
        .btree_map([(34, 35), (34, 36), (37, 38)])
        .call();

    assert_debug_eq(
        &sut,
        expect![[r#"
        (
            [
                13,
                14,
                15,
            ],
            Some(
                [
                    4,
                    5,
                    6,
                ],
            ),
            [
                10,
                11,
                12,
            ],
            [
                16,
                17,
                18,
            ],
            {
                19,
            },
            {
                20,
                21,
                22,
            },
            [
                23,
                24,
                25,
            ],
            [
                28,
                27,
                26,
            ],
            [
                29,
                30,
                31,
            ],
            {
                32: 33,
            },
            {
                34: 36,
                37: 38,
            },
        )"#]],
    );

    let builder = Sut.with_self();

    let _ignore = builder.clone().optional_vec([1, 2, 3]);
    let builder = builder.maybe_optional_vec(Some([4, 5, 6]));

    let _ignore = builder.clone().default_vec([7, 8, 9]);
    let builder = builder.maybe_default_vec(Some([10, 11, 12]));

    // `Hash*`` collections have random order of iteration, so their debug
    // output is unstable. To work around this instability, we just specify
    // a single element for `Hash*` collections.
    let sut = builder
        .vec([13, 14, 15])
        .renamed_vec([16, 17, 18])
        .hash_set(std::iter::once(19))
        .btree_set([20, 21, 22, 22, 21])
        .vec_deque([23, 24, 25])
        .binary_heap([26, 27, 28])
        .linked_list([29, 30, 31])
        .hash_map(std::iter::once((32, 33)))
        .btree_map([(34, 35), (34, 36), (37, 38)])
        .call();

    assert_debug_eq(
        &sut,
        expect![[r#"
        (
            [
                13,
                14,
                15,
            ],
            Some(
                [
                    4,
                    5,
                    6,
                ],
            ),
            [
                10,
                11,
                12,
            ],
            [
                16,
                17,
                18,
            ],
            {
                19,
            },
            {
                20,
                21,
                22,
            },
            [
                23,
                24,
                25,
            ],
            [
                28,
                27,
                26,
            ],
            [
                29,
                30,
                31,
            ],
            {
                32: 33,
            },
            {
                34: 36,
                37: 38,
            },
        )"#]],
    );
}

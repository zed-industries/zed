//! Let it torture the implementation with some randomized operations.

use std::mem;
use std::sync::Arc;

use arc_swap::{ArcSwapAny, DefaultStrategy, IndependentStrategy};
use once_cell::sync::Lazy;
use proptest::prelude::*;

#[derive(Copy, Clone, Debug)]
enum OpsInstruction {
    Store(usize),
    Swap(usize),
    LoadFull,
    Load,
}

impl OpsInstruction {
    fn random() -> impl Strategy<Value = Self> {
        prop_oneof![
            any::<usize>().prop_map(Self::Store),
            any::<usize>().prop_map(Self::Swap),
            Just(Self::LoadFull),
            Just(Self::Load),
        ]
    }
}

proptest! {}

const LIMIT: usize = 5;
#[cfg(not(miri))]
const SIZE: usize = 100;
#[cfg(miri)]
const SIZE: usize = 10;
static ARCS: Lazy<Vec<Arc<usize>>> = Lazy::new(|| (0..LIMIT).map(Arc::new).collect());

#[derive(Copy, Clone, Debug)]
enum SelInstruction {
    Swap(usize),
    Cas(usize, usize),
}

impl SelInstruction {
    fn random() -> impl Strategy<Value = Self> {
        prop_oneof![
            (0..LIMIT).prop_map(Self::Swap),
            (0..LIMIT, 0..LIMIT).prop_map(|(cur, new)| Self::Cas(cur, new)),
        ]
    }
}

// Generate the same tests for bunch of strategies (one module for one strategy)
macro_rules! t {
    (@full => $name: ident, $strategy: ty) => {
        t!(@compose => $name, $strategy,
            #[test]
            fn selection(
                instructions in proptest::collection::vec(SelInstruction::random(), 1..SIZE),
            ) {
                let mut bare = Arc::clone(&ARCS[0]);
                #[allow(deprecated)] // We use "deprecated" testing strategies in here.
                let a = ArcSwapAny::<_, $strategy>::from(Arc::clone(&ARCS[0]));
                for ins in instructions {
                    match ins {
                        SelInstruction::Swap(idx) => {
                            let expected = mem::replace(&mut bare, Arc::clone(&ARCS[idx]));
                            let actual = a.swap(Arc::clone(&ARCS[idx]));
                            assert!(Arc::ptr_eq(&expected, &actual));
                        }
                        SelInstruction::Cas(cur, new) => {
                            let expected = Arc::clone(&bare);
                            if bare == ARCS[cur] {
                                bare = Arc::clone(&ARCS[new]);
                            }
                            let actual = a.compare_and_swap(&ARCS[cur], Arc::clone(&ARCS[new]));
                            assert!(Arc::ptr_eq(&expected, &actual));
                        }
                    }
                }
            }
        );
    };
    (@nocas => $name: ident, $strategy: ty) => {
        t!(@compose => $name, $strategy, );
    };
    (@compose => $name: ident, $strategy: ty, $($extra: tt)*) => {
        mod $name {
            use super::*;

            proptest! {
                $($extra)*

                #[test]
                fn ops(
                    instructions in proptest::collection::vec(OpsInstruction::random(), 1..SIZE),
                ) {
                    use crate::OpsInstruction::*;
                    let mut m = 0;
                    #[allow(deprecated)] // We use "deprecated" testing strategies in here.
                    let a = ArcSwapAny::<_, $strategy>::new(Arc::new(0usize));
                    for ins in instructions {
                        match ins {
                            Store(v) => {
                                m = v;
                                a.store(Arc::new(v));
                            }
                            Swap(v) => {
                                let old = mem::replace(&mut m, v);
                                assert_eq!(old, *a.swap(Arc::new(v)));
                            }
                            Load => assert_eq!(m, **a.load()),
                            LoadFull => assert_eq!(m, *a.load_full()),
                        }
                    }
                }
            }
        }
    };
}

t!(@full => default, DefaultStrategy);
t!(@full => independent, IndependentStrategy);
#[cfg(feature = "internal-test-strategies")]
t!(@full => full_slots, arc_swap::strategy::test_strategies::FillFastSlots);

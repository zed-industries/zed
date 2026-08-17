use core::convert::TryInto;

use proptest::collection::*;
use proptest::prelude::*;

pub fn array_strategy<const N: usize>() -> impl Strategy<Value = [String; N]> {
    vec(any::<String>(), N).prop_map(|v| v.try_into().unwrap())
}

pub fn vec_strategy(n: usize) -> impl Strategy<Value = Vec<String>> {
    vec(any::<String>(), n)
}

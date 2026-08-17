#![allow(unused)]
use crate::common_macro::schema_imports::*;

use core::fmt::{Debug, Display};

/// test: Sausage wasn't populated with param Sausage<W>
#[derive(borsh::BorshSchema, Debug)]
enum AWithSkip<C, W> {
    Bacon,
    Eggs,
    Salad(u32, C, u32),
    Sausage {
        #[borsh(skip)]
        wrapper: W,
        filling: u32,
    },
}

/// test: inner structs in BorshSchema derive don't need any bounds, unrelated to BorshSchema
// #[derive(borsh::BorshSchema)]
// struct SideLeft<A>(
//     A,
// )
// where
//     A: Display + Debug,
//     B: Display + Debug;
#[derive(borsh::BorshSchema)]
enum Side<A, B>
where
    A: Display + Debug,
    B: Display + Debug,
{
    Left(A),
    Right(B),
}

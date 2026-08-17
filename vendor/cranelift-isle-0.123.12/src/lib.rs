#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

macro_rules! declare_id {
    (
        $(#[$attr:meta])*
            $name:ident
    ) => {
        $(#[$attr])*
            #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(pub usize);
        impl $name {
            /// Get the index of this id.
            pub fn index(self) -> usize {
                self.0
            }
        }
    };
}

pub mod ast;
pub mod codegen;
pub mod compile;
pub mod disjointsets;
pub mod error;
pub mod files;
pub mod lexer;
mod log;
pub mod overlap;
pub mod parser;
pub mod printer;
pub mod sema;
pub mod serialize;
pub mod stablemapset;
pub mod trie_again;

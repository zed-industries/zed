// Copyright 2017 Brian Langenberger
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Traits and implementations for reading or writing Huffman codes
//! from or to a stream.

#![warn(missing_docs)]

/// A trait for building a final value from individual bits
///
/// Though similar to the [`crate::read::FromBitStream`] trait,
/// this is intended to parse short symbols from a stream of bits
/// while `FromBitStream` is meant for parsing larger structs from
/// a whole reader.
/// For example, one might have several [`FromBits`] implementations
/// in a single program that all generate `i32` symbols from bits,
/// but implementing `FromBitStream` multiple times for `i32`
/// isn't possible (or practical).
pub trait FromBits {
    /// Our returned symbol type
    type Symbol;

    /// Given a fallable bit generator, return our output type
    ///
    /// # Errors
    ///
    /// Passes along any error from the bit generator
    fn from_bits<F, E>(next: F) -> Result<Self::Symbol, E>
    where
        F: FnMut() -> Result<bool, E>;
}

/// For building individual bits from a final value
///
/// Though similar to the [`crate::write::ToBitStream`] trait,
/// this is intended to generate a stream of bits from short symbols
/// while `ToBitStream` is meant for writing larger structs to
/// a whole writer.
/// For example, one might have several [`ToBits`] implementations
/// in a single program that all write `i32` symbols to bits,
/// but implementing `ToBitStream` multiple times for `i32`
/// isn't possible (or practical).
pub trait ToBits {
    /// The type we accept to output
    type Symbol;

    /// Given a value to generate, write out bits as needed.
    ///
    /// Outputs nothing if the symbol isn't defined.
    ///
    /// # Errors
    ///
    /// Passes along any error from the bit generator
    fn to_bits<F, E>(value: Self::Symbol, write: F) -> Result<(), E>
    where
        F: FnMut(bool) -> Result<(), E>;
}

/// Defines a new Huffman tree for reading and writing
///
/// Its syntax is: `define_huffman_tree!(name : type = nodes)`
/// where `name` is some identifier to identify the tree in the
/// macro's current scope, `type` is the tree's output
/// type (which should implement `Copy` and `Eq`), and `nodes` is either a
/// final leaf value or a `[bit_0, bit_1]` pair where `bit_0` is
/// the tree visited on a `0` bit, and `bit_1` is the tree visited
/// on a `1` bit.
///
/// # Example
///
/// ```
/// use bitstream_io::{define_huffman_tree, huffman::FromBits};
/// define_huffman_tree!(TreeName : &'static str = ["bit 0", ["bit 1->0", "bit 1->1"]]);
/// let mut bits = [true, false].iter().copied();
/// assert_eq!(TreeName::from_bits(|| bits.next().ok_or(())).unwrap(), "bit 1->0");
/// ```
#[macro_export]
macro_rules! define_huffman_tree {
    ($name:ident : $type:ty = $nodes:tt) => {
        #[derive(Copy, Clone, Debug)]
        struct $name;

        impl $crate::huffman::FromBits for $name {
            type Symbol = $type;

            fn from_bits<F, E>(mut next: F) -> Result<Self::Symbol, E>
            where
                F: FnMut() -> Result<bool, E>,
            {
                $crate::compile_read_tree_nodes!(next, $nodes)
            }
        }

        impl $crate::huffman::ToBits for $name {
            type Symbol = $type;

            fn to_bits<F, E>(value: Self::Symbol, mut write: F) -> Result<(), E>
            where
                F: FnMut(bool) -> Result<(), E>
            {
                $crate::compile_write_tree_nodes!(value ; write ; $nodes ; );
                Ok(())
            }
        }
    };
}

/// A helper macro for compiling individual Huffman tree nodes
#[macro_export]
macro_rules! compile_read_tree_nodes {
    ($next:ident , [$bit_0:tt, $bit_1:tt]) => {
        if $next()? {
            $crate::compile_read_tree_nodes!($next, $bit_1)
        } else {
            $crate::compile_read_tree_nodes!($next, $bit_0)
        }
    };
    ($next:ident , $final:tt) => {
        Ok($final)
    };
}

/// A helper macro for compiling individual Huffman tree nodes
#[macro_export]
macro_rules! compile_write_tree_nodes {
    ($value:ident ; $write:ident ; [$bit_0:tt, $bit_1:tt] ; ) => {
        $crate::compile_write_tree_nodes!($value ; $write ; $bit_0 ; false);
        $crate::compile_write_tree_nodes!($value ; $write ; $bit_1 ; true);
    };
    ($value:ident ; $write:ident ; [$bit_0:tt, $bit_1:tt] ; $($bits:tt),*) => {
        $crate::compile_write_tree_nodes!($value ; $write ; $bit_0 ; $($bits),* , false);
        $crate::compile_write_tree_nodes!($value ; $write ; $bit_1 ; $($bits),* , true);
    };
    ($value:ident ; $write:ident ; $final:tt ; $( $bits:tt),* ) => {
        if $value == $final {
            $( $write($bits)?; )*
            return Ok(());
        }
    };
}

/// A limited unary reader which stops at the given maximum.
///
/// Counts non-`STOP_BIT` values (which must be 0 or 1)
/// until `STOP_BIT`, or until `MAXIMUM` is reached.
/// Returns the number of non-`STOP_BIT` bits, or `None` if
/// maximum is reached beforehand.
///
/// # Examples
/// ```
/// use bitstream_io::{BitReader, BitRead, BigEndian, huffman::LimitedUnary};
///
/// let data: &[u8] = &[0b001_00000, 0b1111_1111];
/// let mut r = BitReader::endian(data, BigEndian);
/// // get 2 bits until the next 1 bit
/// assert_eq!(r.read_huffman::<LimitedUnary<1, 5>>().unwrap(), Some(2));
/// // but 5 bits in a row is our maximum
/// assert_eq!(r.read_huffman::<LimitedUnary<1, 5>>().unwrap(), None);
/// // the remaining 8 bits are ok to be read
/// assert_eq!(r.read::<8, u8>().unwrap(), 0b1111_1111);
/// ```
///
/// ```
/// use bitstream_io::{BitWriter, BitWrite, BigEndian, huffman::LimitedUnary};
///
/// let mut w = BitWriter::endian(vec![], BigEndian);
/// // writes 2 as a regular unary value which stops at the 1 bit
/// w.write_huffman::<LimitedUnary<1, 5>>(Some(2)).unwrap();
/// // writing values beyond the maximum does nothing
/// w.write_huffman::<LimitedUnary<1, 5>>(Some(10)).unwrap();
/// // writes 5, 0 bits (which is our maximum)
/// w.write_huffman::<LimitedUnary<1, 5>>(None).unwrap();
/// // write some 1 bits to pad out the stream
/// w.write::<8, u8>(0b1111_1111);
///
/// assert_eq!(w.into_writer(), &[0b001_00000, 0b1111_1111]);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct LimitedUnary<const STOP_BIT: u8, const MAXIMUM: u32>;

impl<const STOP_BIT: u8, const MAXIMUM: u32> FromBits for LimitedUnary<STOP_BIT, MAXIMUM> {
    type Symbol = Option<u32>;

    fn from_bits<F, E>(mut next: F) -> Result<Self::Symbol, E>
    where
        F: FnMut() -> Result<bool, E>,
    {
        const {
            assert!(matches!(STOP_BIT, 0 | 1), "stop bit must be 0 or 1");
        }

        let mut bits = 0;
        while bits < MAXIMUM {
            if next()?
                != match STOP_BIT {
                    0 => false,
                    1 => true,
                    _ => unreachable!(),
                }
            {
                bits += 1;
            } else {
                return Ok(Some(bits));
            }
        }
        Ok(None)
    }
}

impl<const STOP_BIT: u8, const MAXIMUM: u32> ToBits for LimitedUnary<STOP_BIT, MAXIMUM> {
    type Symbol = Option<u32>;

    fn to_bits<F, E>(value: Option<u32>, mut write: F) -> Result<(), E>
    where
        F: FnMut(bool) -> Result<(), E>,
    {
        const {
            assert!(matches!(STOP_BIT, 0 | 1), "stop bit must be 0 or 1");
        }

        match value {
            Some(bits) if bits < MAXIMUM => {
                (0..bits).try_for_each(|_| {
                    write(match STOP_BIT {
                        0 => true,
                        1 => false,
                        _ => unreachable!(),
                    })
                })?;
                write(match STOP_BIT {
                    0 => false,
                    1 => true,
                    _ => unreachable!(),
                })
            }
            Some(_) => {
                /*more bits than MAXIMUM, so output nothing*/
                Ok(())
            }
            None => (0..MAXIMUM).try_for_each(|_| {
                write(match STOP_BIT {
                    0 => true,
                    1 => false,
                    _ => unreachable!(),
                })
            }),
        }
    }
}

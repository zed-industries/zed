//! A set of scalar types, represented as a bitset.

use crate::ir::Scalar;
use crate::proc::overloads::one_bits_iter::OneBitsIter;

macro_rules! define_scalar_set {
    { $( $scalar:ident, )* } => {
        /// An enum used to assign distinct bit numbers to [`ScalarSet`] elements.
        #[expect(non_camel_case_types, clippy::upper_case_acronyms)]
        #[repr(u32)]
        enum ScalarSetBits {
            $( $scalar, )*
            Count,
        }

        /// A table mapping bit numbers to the [`Scalar`] values they represent.
        static SCALARS_FOR_BITS: [Scalar; ScalarSetBits::Count as usize] = [
            $(
                Scalar::$scalar,
            )*
        ];

        bitflags::bitflags! {
            /// A set of scalar types.
            ///
            /// This represents a set of [`Scalar`] types.
            ///
            /// The Naga IR conversion rules arrange scalar types into a
            /// lattice. The scalar types' bit values are chosen such that, if
            /// A is convertible to B, then A's bit value is less than B's.
            #[derive(Copy, Clone, Debug)]
            pub(crate) struct ScalarSet: u16 {
                $(
                    const $scalar = 1 << (ScalarSetBits::$scalar as u32);
                )*
            }
        }

        impl ScalarSet {
            /// Return the set of scalars containing only `scalar`.
            #[expect(dead_code)]
            pub const fn singleton(scalar: Scalar) -> Self {
                match scalar {
                    $(
                        Scalar::$scalar => Self::$scalar,
                    )*
                    _ => Self::empty(),
                }
            }
        }
    }
}

define_scalar_set! {
    // Scalar types must be listed here in an order such that, if A is
    // convertible to B, then A appears before B.
    //
    // In the concrete types, the 32-bit types *must* appear before
    // other sizes, since that is how we represent conversion rank.
    ABSTRACT_INT, ABSTRACT_FLOAT,
    I32, I64,
    U32, U64,
    F32, F16, F64,
    BOOL,
}

impl ScalarSet {
    /// Return the set of scalars to which `scalar` can be automatically
    /// converted.
    pub fn convertible_from(scalar: Scalar) -> Self {
        use Scalar as Sc;
        match scalar {
            Sc::I32 => Self::I32,
            Sc::I64 => Self::I64,
            Sc::U32 => Self::U32,
            Sc::U64 => Self::U64,
            Sc::F16 => Self::F16,
            Sc::F32 => Self::F32,
            Sc::F64 => Self::F64,
            Sc::BOOL => Self::BOOL,
            Sc::ABSTRACT_INT => Self::INTEGER | Self::FLOAT,
            Sc::ABSTRACT_FLOAT => Self::FLOAT,
            _ => Self::empty(),
        }
    }

    /// Return the lowest-ranked member of `self` as a [`Scalar`].
    ///
    /// # Panics
    ///
    /// Panics if `self` is empty.
    pub fn most_general_scalar(self) -> Scalar {
        // If the set is empty, this returns the full bit-length of
        // `self.bits()`, an index which is out of bounds for
        // `SCALARS_FOR_BITS`.
        let lowest = self.bits().trailing_zeros();
        *SCALARS_FOR_BITS.get(lowest as usize).unwrap()
    }

    /// Return an iterator over this set's members.
    ///
    /// Members are produced as singleton, in order from most general to least.
    pub fn members(self) -> impl Iterator<Item = ScalarSet> {
        OneBitsIter::new(self.bits() as u64).map(|bit| Self::from_bits(bit as u16).unwrap())
    }

    pub const FLOAT: Self = Self::ABSTRACT_FLOAT
        .union(Self::F16)
        .union(Self::F32)
        .union(Self::F64);

    pub const INTEGER: Self = Self::ABSTRACT_INT
        .union(Self::I32)
        .union(Self::I64)
        .union(Self::U32)
        .union(Self::U64);

    pub const NUMERIC: Self = Self::FLOAT.union(Self::INTEGER);
    pub const ABSTRACT: Self = Self::ABSTRACT_INT.union(Self::ABSTRACT_FLOAT);
    pub const CONCRETE: Self = Self::all().difference(Self::ABSTRACT);
    pub const CONCRETE_INTEGER: Self = Self::INTEGER.intersection(Self::CONCRETE);
    pub const CONCRETE_FLOAT: Self = Self::FLOAT.intersection(Self::CONCRETE);

    /// Floating-point scalars, with the abstract floats omitted for
    /// #7405.
    pub const FLOAT_ABSTRACT_UNIMPLEMENTED: Self = Self::CONCRETE_FLOAT;
}

macro_rules! scalar_set {
    ( $( $scalar:ident )|* ) => {
        {
            use $crate::proc::overloads::scalar_set::ScalarSet;
            ScalarSet::empty()
                $(
                    .union(ScalarSet::$scalar)
                )*
        }
    }
}

pub(in crate::proc::overloads) use scalar_set;

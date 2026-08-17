//! A set of type constructors, represented as a bitset.

use crate::ir;
use crate::proc::overloads::one_bits_iter::OneBitsIter;

bitflags::bitflags! {
    /// A set of type constructors.
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub(crate) struct ConstructorSet: u16 {
        const SCALAR = 1 << 0;
        const VEC2 = 1 << 1;
        const VEC3 = 1 << 2;
        const VEC4 = 1 << 3;
        const MAT2X2 = 1 << 4;
        const MAT2X3 = 1 << 5;
        const MAT2X4 = 1 << 6;
        const MAT3X2 = 1 << 7;
        const MAT3X3 = 1 << 8;
        const MAT3X4 = 1 << 9;
        const MAT4X2 = 1 << 10;
        const MAT4X3 = 1 << 11;
        const MAT4X4 = 1 << 12;

        const VECN = Self::VEC2.bits()
            | Self::VEC3.bits()
            | Self::VEC4.bits();
    }
}

impl ConstructorSet {
    /// Return the single-member set containing `inner`'s constructor.
    pub const fn singleton(inner: &ir::TypeInner) -> ConstructorSet {
        use ir::TypeInner as Ti;
        use ir::VectorSize as Vs;
        match *inner {
            Ti::Scalar(_) => Self::SCALAR,
            Ti::Vector { size, scalar: _ } => match size {
                Vs::Bi => Self::VEC2,
                Vs::Tri => Self::VEC3,
                Vs::Quad => Self::VEC4,
            },
            Ti::Matrix {
                columns,
                rows,
                scalar: _,
            } => match (columns, rows) {
                (Vs::Bi, Vs::Bi) => Self::MAT2X2,
                (Vs::Bi, Vs::Tri) => Self::MAT2X3,
                (Vs::Bi, Vs::Quad) => Self::MAT2X4,
                (Vs::Tri, Vs::Bi) => Self::MAT3X2,
                (Vs::Tri, Vs::Tri) => Self::MAT3X3,
                (Vs::Tri, Vs::Quad) => Self::MAT3X4,
                (Vs::Quad, Vs::Bi) => Self::MAT4X2,
                (Vs::Quad, Vs::Tri) => Self::MAT4X3,
                (Vs::Quad, Vs::Quad) => Self::MAT4X4,
            },
            _ => Self::empty(),
        }
    }

    pub const fn is_singleton(self) -> bool {
        self.bits().is_power_of_two()
    }

    /// Return an iterator over this set's members.
    ///
    /// Members are produced as singleton, in order from most general to least.
    pub fn members(self) -> impl Iterator<Item = ConstructorSet> {
        OneBitsIter::new(self.bits() as u64).map(|bit| Self::from_bits(bit as u16).unwrap())
    }

    /// Return the size of the sole element of `self`.
    ///
    /// # Panics
    ///
    /// Panic if `self` is not a singleton.
    pub fn size(self) -> ConstructorSize {
        use ir::VectorSize as Vs;
        use ConstructorSize as Cs;

        match self {
            ConstructorSet::SCALAR => Cs::Scalar,
            ConstructorSet::VEC2 => Cs::Vector(Vs::Bi),
            ConstructorSet::VEC3 => Cs::Vector(Vs::Tri),
            ConstructorSet::VEC4 => Cs::Vector(Vs::Quad),
            ConstructorSet::MAT2X2 => Cs::Matrix {
                columns: Vs::Bi,
                rows: Vs::Bi,
            },
            ConstructorSet::MAT2X3 => Cs::Matrix {
                columns: Vs::Bi,
                rows: Vs::Tri,
            },
            ConstructorSet::MAT2X4 => Cs::Matrix {
                columns: Vs::Bi,
                rows: Vs::Quad,
            },
            ConstructorSet::MAT3X2 => Cs::Matrix {
                columns: Vs::Tri,
                rows: Vs::Bi,
            },
            ConstructorSet::MAT3X3 => Cs::Matrix {
                columns: Vs::Tri,
                rows: Vs::Tri,
            },
            ConstructorSet::MAT3X4 => Cs::Matrix {
                columns: Vs::Tri,
                rows: Vs::Quad,
            },
            ConstructorSet::MAT4X2 => Cs::Matrix {
                columns: Vs::Quad,
                rows: Vs::Bi,
            },
            ConstructorSet::MAT4X3 => Cs::Matrix {
                columns: Vs::Quad,
                rows: Vs::Tri,
            },
            ConstructorSet::MAT4X4 => Cs::Matrix {
                columns: Vs::Quad,
                rows: Vs::Quad,
            },
            _ => unreachable!("ConstructorSet was not a singleton"),
        }
    }
}

/// The sizes a member of [`ConstructorSet`] might have.
#[derive(Clone, Copy)]
pub enum ConstructorSize {
    /// The constructor is [`SCALAR`].
    ///
    /// [`SCALAR`]: ConstructorSet::SCALAR
    Scalar,

    /// The constructor is `VECN` for some `N`.
    Vector(ir::VectorSize),

    /// The constructor is `MATCXR` for some `C` and `R`.
    Matrix {
        columns: ir::VectorSize,
        rows: ir::VectorSize,
    },
}

impl ConstructorSize {
    /// Construct a [`TypeInner`] for a type with this size and the given `scalar`.
    ///
    /// [`TypeInner`]: ir::TypeInner
    pub const fn to_inner(self, scalar: ir::Scalar) -> ir::TypeInner {
        match self {
            Self::Scalar => ir::TypeInner::Scalar(scalar),
            Self::Vector(size) => ir::TypeInner::Vector { size, scalar },
            Self::Matrix { columns, rows } => ir::TypeInner::Matrix {
                columns,
                rows,
                scalar,
            },
        }
    }
}

macro_rules! constructor_set {
    ( $( $constr:ident )|* ) => {
        {
            use $crate::proc::overloads::constructor_set::ConstructorSet;
            ConstructorSet::empty()
                $(
                    .union(ConstructorSet::$constr)
                )*
        }
    }
}

pub(in crate::proc::overloads) use constructor_set;

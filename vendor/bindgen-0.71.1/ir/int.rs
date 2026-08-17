//! Intermediate representation for integral types.

/// Which integral type are we dealing with?
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum IntKind {
    /// A `bool`.
    Bool,

    /// A `signed char`.
    SChar,

    /// An `unsigned char`.
    UChar,

    /// A `wchar_t`.
    WChar,

    /// A platform-dependent `char` type, with the signedness support.
    Char {
        /// Whether the char is signed for the target platform.
        is_signed: bool,
    },

    /// A `short`.
    Short,

    /// An `unsigned short`.
    UShort,

    /// An `int`.
    Int,

    /// An `unsigned int`.
    UInt,

    /// A `long`.
    Long,

    /// An `unsigned long`.
    ULong,

    /// A `long long`.
    LongLong,

    /// An `unsigned long long`.
    ULongLong,

    /// A 8-bit signed integer.
    I8,

    /// A 8-bit unsigned integer.
    U8,

    /// A 16-bit signed integer.
    I16,

    /// Either a `char16_t` or a `wchar_t`.
    U16,

    /// A 32-bit signed integer.
    I32,

    /// A 32-bit unsigned integer.
    U32,

    /// A 64-bit signed integer.
    I64,

    /// A 64-bit unsigned integer.
    U64,

    /// An `int128_t`
    I128,

    /// A `uint128_t`.
    U128,

    /// A custom integer type, used to allow custom macro types depending on
    /// range.
    Custom {
        /// The name of the type, which would be used without modification.
        name: &'static str,
        /// Whether the type is signed or not.
        is_signed: bool,
    },
}

impl IntKind {
    /// Is this integral type signed?
    pub(crate) fn is_signed(&self) -> bool {
        use self::IntKind::*;
        match *self {
            // TODO(emilio): wchar_t can in theory be signed, but we have no way
            // to know whether it is or not right now (unlike char, there's no
            // WChar_S / WChar_U).
            Bool | UChar | UShort | UInt | ULong | ULongLong | U8 | U16 |
            WChar | U32 | U64 | U128 => false,

            SChar | Short | Int | Long | LongLong | I8 | I16 | I32 | I64 |
            I128 => true,

            Char { is_signed } => is_signed,

            Custom { is_signed, .. } => is_signed,
        }
    }

    /// If this type has a known size, return it (in bytes). This is to
    /// alleviate libclang sometimes not giving us a layout (like in the case
    /// when an enum is defined inside a class with template parameters).
    pub(crate) fn known_size(&self) -> Option<usize> {
        use self::IntKind::*;
        Some(match *self {
            Bool | UChar | SChar | U8 | I8 | Char { .. } => 1,
            U16 | I16 => 2,
            U32 | I32 => 4,
            U64 | I64 => 8,
            I128 | U128 => 16,
            _ => return None,
        })
    }

    /// Whether this type's signedness matches the value.
    pub(crate) fn signedness_matches(&self, val: i64) -> bool {
        val >= 0 || self.is_signed()
    }
}

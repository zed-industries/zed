//! Pulley opcodes (without operands).

macro_rules! define_opcode {
    (
        $(
            $( #[$attr:meta] )*
            $snake_name:ident = $name:ident $( {
                $(
                    $( #[$field_attr:meta] )*
                    $field:ident : $field_ty:ty
                ),*
            } )? ;
        )*
    ) => {
        /// An opcode without its immediates and operands.
        #[repr(u8)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum Opcode {
            $(
                $( #[$attr] )*
                $name,
            )*
            /// The extended-op opcode. An `ExtendedOpcode` follows this opcode.
            ExtendedOp,
        }

        impl Opcode {
            /// The value of the maximum defined opcode.
            pub const MAX: u8 = Opcode::ExtendedOp as u8;
        }
    }
}
for_each_op!(define_opcode);

impl Opcode {
    /// Create a new `Opcode` from the given byte.
    ///
    /// Returns `None` if `byte` is not a valid opcode.
    pub fn new(byte: u8) -> Option<Self> {
        if byte <= Self::MAX {
            Some(unsafe { Self::unchecked_new(byte) })
        } else {
            None
        }
    }

    /// Like `new` but does not check whether `byte` is a valid opcode.
    ///
    /// # Safety
    ///
    /// It is unsafe to pass a `byte` that is not a valid opcode.
    pub unsafe fn unchecked_new(byte: u8) -> Self {
        debug_assert!(byte <= Self::MAX);
        unsafe { core::mem::transmute(byte) }
    }
}

macro_rules! define_extended_opcode {
    (
        $(
            $( #[$attr:meta] )*
            $snake_name:ident = $name:ident $( { $( $field:ident : $field_ty:ty ),* } )? ;
        )*
    ) => {
        /// An extended opcode.
        #[repr(u16)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum ExtendedOpcode {
            $(
                $( #[$attr] )*
                    $name,
            )*
        }

        impl ExtendedOpcode {
            /// The value of the maximum defined extended opcode.
            pub const MAX: u16 = $(
                if true { 1 } else { ExtendedOpcode::$name as u16 } +
            )* 0;
        }
    };
}
for_each_extended_op!(define_extended_opcode);

impl ExtendedOpcode {
    /// Create a new `ExtendedOpcode` from the given bytes.
    ///
    /// Returns `None` if `bytes` is not a valid extended opcode.
    pub fn new(bytes: u16) -> Option<Self> {
        if bytes <= Self::MAX {
            Some(unsafe { Self::unchecked_new(bytes) })
        } else {
            None
        }
    }

    /// Like `new` but does not check whether `bytes` is a valid opcode.
    ///
    /// # Safety
    ///
    /// It is unsafe to pass `bytes` that is not a valid opcode.
    pub unsafe fn unchecked_new(byte: u16) -> Self {
        debug_assert!(byte <= Self::MAX);
        unsafe { core::mem::transmute(byte) }
    }
}

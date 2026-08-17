//! Encoding support for pulley bytecode.

use crate::imms::*;
use crate::opcode::{ExtendedOpcode, Opcode};
use crate::regs::*;

/// Helper trait to encode instructions into a "sink".
pub trait Encode {
    /// The encoded width of this instruction.
    const WIDTH: u8;

    /// Encodes this operand or instruction into the provided `sink`.
    fn encode<E>(&self, sink: &mut E)
    where
        E: Extend<u8>;
}

impl Encode for u8 {
    const WIDTH: u8 = 1;

    fn encode<E>(&self, sink: &mut E)
    where
        E: Extend<u8>,
    {
        sink.extend(core::iter::once(*self));
    }
}

impl Encode for u16 {
    const WIDTH: u8 = 2;

    fn encode<E>(&self, sink: &mut E)
    where
        E: Extend<u8>,
    {
        sink.extend(self.to_le_bytes());
    }
}

impl Encode for u32 {
    const WIDTH: u8 = 4;

    fn encode<E>(&self, sink: &mut E)
    where
        E: Extend<u8>,
    {
        sink.extend(self.to_le_bytes());
    }
}

impl Encode for u64 {
    const WIDTH: u8 = 8;

    fn encode<E>(&self, sink: &mut E)
    where
        E: Extend<u8>,
    {
        sink.extend(self.to_le_bytes());
    }
}

impl Encode for u128 {
    const WIDTH: u8 = 16;

    fn encode<E>(&self, sink: &mut E)
    where
        E: Extend<u8>,
    {
        sink.extend(self.to_le_bytes());
    }
}

impl Encode for i8 {
    const WIDTH: u8 = 1;

    fn encode<E>(&self, sink: &mut E)
    where
        E: Extend<u8>,
    {
        sink.extend(core::iter::once(*self as u8));
    }
}

impl Encode for i16 {
    const WIDTH: u8 = 2;

    fn encode<E>(&self, sink: &mut E)
    where
        E: Extend<u8>,
    {
        sink.extend(self.to_le_bytes());
    }
}

impl Encode for i32 {
    const WIDTH: u8 = 4;

    fn encode<E>(&self, sink: &mut E)
    where
        E: Extend<u8>,
    {
        sink.extend(self.to_le_bytes());
    }
}

impl Encode for i64 {
    const WIDTH: u8 = 8;

    fn encode<E>(&self, sink: &mut E)
    where
        E: Extend<u8>,
    {
        sink.extend(self.to_le_bytes());
    }
}

impl Encode for i128 {
    const WIDTH: u8 = 16;

    fn encode<E>(&self, sink: &mut E)
    where
        E: Extend<u8>,
    {
        sink.extend(self.to_le_bytes());
    }
}

impl Encode for XReg {
    const WIDTH: u8 = 1;

    fn encode<E>(&self, sink: &mut E)
    where
        E: Extend<u8>,
    {
        sink.extend(core::iter::once(self.to_u8()));
    }
}

impl Encode for FReg {
    const WIDTH: u8 = 1;

    fn encode<E>(&self, sink: &mut E)
    where
        E: Extend<u8>,
    {
        sink.extend(core::iter::once(self.to_u8()));
    }
}

impl Encode for VReg {
    const WIDTH: u8 = 1;

    fn encode<E>(&self, sink: &mut E)
    where
        E: Extend<u8>,
    {
        sink.extend(core::iter::once(self.to_u8()));
    }
}

impl Encode for PcRelOffset {
    const WIDTH: u8 = 4;

    fn encode<E>(&self, sink: &mut E)
    where
        E: Extend<u8>,
    {
        i32::from(*self).encode(sink);
    }
}

impl<D: Reg, S1: Reg, S2: Reg> Encode for BinaryOperands<D, S1, S2> {
    const WIDTH: u8 = 2;

    fn encode<E>(&self, sink: &mut E)
    where
        E: Extend<u8>,
    {
        self.to_bits().encode(sink);
    }
}

impl<D: Reg, S1: Reg> Encode for BinaryOperands<D, S1, U6> {
    const WIDTH: u8 = 2;

    fn encode<E>(&self, sink: &mut E)
    where
        E: Extend<u8>,
    {
        self.to_bits().encode(sink);
    }
}

impl<R: Reg + Encode> Encode for UpperRegSet<R> {
    const WIDTH: u8 = 2;

    fn encode<E>(&self, sink: &mut E)
    where
        E: Extend<u8>,
    {
        self.to_bitset().0.encode(sink);
    }
}

impl Encode for AddrO32 {
    const WIDTH: u8 = 5;

    fn encode<E>(&self, sink: &mut E)
    where
        E: Extend<u8>,
    {
        self.addr.encode(sink);
        self.offset.encode(sink);
    }
}

impl Encode for AddrZ {
    const WIDTH: u8 = 5;

    fn encode<E>(&self, sink: &mut E)
    where
        E: Extend<u8>,
    {
        self.addr.encode(sink);
        self.offset.encode(sink);
    }
}

impl Encode for AddrG32 {
    const WIDTH: u8 = 4;

    fn encode<E>(&self, sink: &mut E)
    where
        E: Extend<u8>,
    {
        self.to_bits().encode(sink);
    }
}

impl Encode for AddrG32Bne {
    const WIDTH: u8 = 4;

    fn encode<E>(&self, sink: &mut E)
    where
        E: Extend<u8>,
    {
        self.to_bits().encode(sink);
    }
}

macro_rules! impl_encoders {
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
        $(
            $( #[$attr] )*
            pub fn $snake_name<E>(into: &mut E $( $( , $field : impl Into<$field_ty> )* )? )
            where
                E: Extend<u8>,
            {
                into.extend(core::iter::once(Opcode::$name as u8));
                $(
                    $(
                        $field.into().encode(into);
                    )*
                )?
            }

            impl Encode for crate::op::$name {
                const WIDTH: u8 = 1 $($( + <$field_ty as Encode>::WIDTH)*)?;

                fn encode<E>(&self, sink: &mut E)
                where
                    E: Extend<u8>,
                {
                    let Self { $(  $( $field ),* )? } = *self;
                    $snake_name(sink $( $(, $field)* )?)
                }
            }
        )*
    };
}
for_each_op!(impl_encoders);

macro_rules! impl_extended_encoders {
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
        $(
            $( #[$attr] )*
            pub fn $snake_name<E>(into: &mut E $( $( , $field : impl Into<$field_ty> )* )? )
            where
                E: Extend<u8>,
            {
                into.extend(core::iter::once(Opcode::ExtendedOp as u8));
                into.extend((ExtendedOpcode::$name as u16).to_le_bytes());
                $(
                    $(
                        $field.into().encode(into);
                    )*
                )?
            }

            impl Encode for crate::op::$name {
                const WIDTH: u8 = 3 $($( + <$field_ty as Encode>::WIDTH)*)?;

                fn encode<E>(&self, sink: &mut E)
                where
                    E: Extend<u8>,
                {
                    let Self { $(  $( $field ),* )? } = *self;
                    $snake_name(sink $( $(, $field)* )?)
                }
            }
        )*
    };
}
for_each_extended_op!(impl_extended_encoders);

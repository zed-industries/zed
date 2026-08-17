//! Pulley bytecode operations with their operands.

#[cfg(feature = "encode")]
use crate::encode::Encode;
use crate::imms::*;
use crate::regs::*;

macro_rules! define_op {
    (
        $(
            $( #[$attr:meta] )*
            $snake_name:ident = $name:ident $( { $( $field:ident : $field_ty:ty ),* } )? ;
        )*
    ) => {
        /// A complete, materialized operation/instruction.
        ///
        /// This type is useful for debugging, writing tests, etc... but is not
        /// actually ever used by the interpreter, encoder, or decoder, all of
        /// which avoid materializing ops.
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
        pub enum Op {
            $(
                $( #[$attr] )*
                $name($name),
            )*
            /// An extended operation/instruction.
            ExtendedOp(ExtendedOp),
        }

        $(
            $( #[$attr] )*
            #[derive(Clone, Copy, Debug, PartialEq, Eq)]
            #[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
            pub struct $name { $(
                $(
                    // TODO: add doc comments to all fields and update all
                    // the macros to match them.
                    #[expect(missing_docs, reason = "macro-generated code")]
                    pub $field : $field_ty,
                )*
            )? }

            impl From<$name> for Op {
                #[inline]
                fn from(op: $name) -> Self {
                    Self::$name(op)
                }
            }
        )*
    };
}
for_each_op!(define_op);

impl From<ExtendedOp> for Op {
    #[inline]
    fn from(op: ExtendedOp) -> Self {
        Op::ExtendedOp(op)
    }
}

macro_rules! define_extended_op {
    (
        $(
            $( #[$attr:meta] )*
            $snake_name:ident = $name:ident $( { $( $field:ident : $field_ty:ty ),* } )? ;
        )*
    ) => {
        /// An extended operation/instruction.
        ///
        /// These tend to be colder than `Op`s.
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
        pub enum ExtendedOp {
            $(
                $( #[$attr] )*
                $name($name),
            )*
        }

        $(
            $( #[$attr] )*
            #[derive(Clone, Copy, Debug, PartialEq, Eq)]
            #[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
            pub struct $name { $(
                $(
                    // TODO: add doc comments to all fields and update all
                    // the macros to match them.
                    #[expect(missing_docs, reason = "macro-generated code")]
                    pub $field : $field_ty,
                )*
            )? }

            #[doc(hidden)]
            #[expect(non_camel_case_types, reason = "used in macros as an alternative to camel case")]
            pub type $snake_name = $name;

            impl From<$name> for Op {
                #[inline]
                fn from(op: $name) -> Self {
                    Self::ExtendedOp(ExtendedOp::$name(op))
                }
            }

            impl From<$name> for ExtendedOp {
                #[inline]
                fn from(op: $name) -> Self {
                    Self::$name(op)
                }
            }
        )*
    };
}
for_each_extended_op!(define_extended_op);

macro_rules! define_op_encode {
    (
        $(
            $( #[$attr:meta] )*
            $snake_name:ident = $name:ident $( { $( $field:ident : $field_ty:ty ),* } )? ;
        )*
    ) => {
        impl Op {
            /// Encode this op into the given sink.
            #[cfg(feature = "encode")]
            pub fn encode<E>(&self, into: &mut E)
            where
                E: Extend<u8>,
            {
                match self {
                    $(
                        Self::$name(op) => op.encode(into),
                    )*
                    Self::ExtendedOp(op) => op.encode(into),
                }
            }

            /// Returns the encoded size of this op.
            #[cfg(feature = "encode")]
            pub fn width(&self) -> u8 {
                match self {
                    $(
                        Self::$name(_) => <$name as Encode>::WIDTH,
                    )*
                    Self::ExtendedOp(op) => op.width(),
                }
            }
        }

        $(
            impl $name {
                /// Encode this
                #[doc = concat!("`", stringify!($name), "`")]
                /// op into the given sink.
                #[cfg(feature = "encode")]
                pub fn encode<E>(&self, into: &mut E)
                where
                    E: Extend<u8>,
                {
                    crate::encode::$snake_name(into $( $( , self.$field )* )?);
                }
            }
        )*
    };
}
for_each_op!(define_op_encode);

macro_rules! define_extended_op_encode {
    (
        $(
            $( #[$attr:meta] )*
            $snake_name:ident = $name:ident $( { $( $field:ident : $field_ty:ty ),* } )? ;
        )*
    ) => {
        impl ExtendedOp {
            /// Encode this extended op into the given sink.
            #[cfg(feature = "encode")]
            pub fn encode<E>(&self, into: &mut E)
            where
                E: Extend<u8>,
            {
                match self {
                    $(
                        Self::$name(op) => op.encode(into),
                    )*
                }
            }

            /// Returns the encoded size of this op.
            #[cfg(feature = "encode")]
            pub fn width(&self) -> u8 {
                match self {
                    $(
                        Self::$name(_) => <$name as Encode>::WIDTH,
                    )*
                }
            }
        }

        $(
            impl $name {
                /// Encode this
                #[doc = concat!("`", stringify!($name), "`")]
                /// op into the given sink.
                #[cfg(feature = "encode")]
                pub fn encode<E>(&self, into: &mut E)
                where
                    E: Extend<u8>,
                {
                    crate::encode::$snake_name(into $( $( , self.$field )* )?);
                }
            }
        )*
    };
}
for_each_extended_op!(define_extended_op_encode);

/// A visitor that materializes whole `Op`s as it decodes the bytecode stream.
#[cfg(feature = "decode")]
#[derive(Default)]
pub struct MaterializeOpsVisitor<B> {
    bytecode: B,
}

#[cfg(feature = "decode")]
impl<B> MaterializeOpsVisitor<B> {
    /// Create a new op-materializing visitor for the given bytecode.
    pub fn new(bytecode: B) -> Self {
        Self { bytecode }
    }
}

macro_rules! define_materialize_op_visitor {
    (
        $(
            $( #[$attr:meta] )*
            $snake_name:ident = $name:ident $( { $( $field:ident : $field_ty:ty ),* } )? ;
        )*
    ) => {
        #[cfg(feature = "decode")]
        impl<B: crate::decode::BytecodeStream> crate::decode::OpVisitor for MaterializeOpsVisitor<B> {
            type BytecodeStream = B;

            fn bytecode(&mut self) -> &mut Self::BytecodeStream {
                &mut self.bytecode
            }

            type Return = crate::op::Op;

            $(
                $( #[$attr] )*
                fn $snake_name(&mut self $( $( , $field : $field_ty )* )? ) -> Self::Return {
                    crate::op::Op::$name(crate::op::$name { $( $(
                        $field,
                    )* )? })
                }
            )*
        }
    };
}
for_each_op!(define_materialize_op_visitor);

macro_rules! define_materialize_extended_op_visitor {
    (
        $(
            $( #[$attr:meta] )*
            $snake_name:ident = $name:ident $( { $( $field:ident : $field_ty:ty ),* } )? ;
        )*
    ) => {
        #[cfg(feature = "decode")]
        impl<B: crate::decode::BytecodeStream> crate::decode::ExtendedOpVisitor for MaterializeOpsVisitor<B> {
            $(
                $( #[$attr] )*
                fn $snake_name(&mut self $( $( , $field : $field_ty )* )? ) -> Self::Return {
                    crate::op::ExtendedOp::$name(crate::op::$name { $( $(
                        $field,
                    )* )? }).into()
                }
            )*
        }
    };
}
for_each_extended_op!(define_materialize_extended_op_visitor);

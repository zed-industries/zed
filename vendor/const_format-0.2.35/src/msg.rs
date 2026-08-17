#![allow(non_camel_case_types)]

use crate::fmt::{Error, StrWriter};

use core::marker::PhantomData;

macro_rules! type_level_error {
    (
        arguments($($args:tt)*)

        $($error:ident => $error_ty:ident<$($error_param:ident),*> ,)*
    ) => {
        type_level_error!{
            @inner
            arguments($($args)*)

            Result::Ok(()), Ok => Ok<>,
            $(
                Result::Err(Error::$error), $error => $error_ty<$($error_param),*>,
            )*
        }
    };
    (@inner
        arguments($cap:ident)

        $($matched:pat , $error:ident => $error_ty:ident<$($error_param:ident),*> ,)*
    ) => {

        enum ErrorKind {
            $($error,)*
        }

        impl ErrorTuple {
            pub const EMPTY: ErrorTuple = ErrorTuple{
                error_variant: ErrorKind::Ok as usize,
                capacity: 0,
            };

            pub const fn new(opt: Result<(), Error>, writer: &StrWriter) -> Self{
                let variant = match opt {
                    $($matched => ErrorKind::$error as usize,)*
                };

                Self{
                    error_variant: variant,
                    capacity: writer.capacity(),
                }
            }

        }


        $(
            pub struct $error_ty<$($error_param,)*>(PhantomData<($($error_param,)*)>);

            impl<$($error_param,)*> $error_ty<$($error_param,)*> {
                pub const NEW: Self = Self(PhantomData);
            }

            impl<$cap> ErrorAsType for ErrorPicker<[(); ErrorKind::$error as usize], $cap> {
                type Type = $error_ty<$($error_param,)*>;
            }
        )*
    }
}

pub struct ErrorTupleAndStrWriter<A> {
    pub error: ErrorTuple,
    pub writer: StrWriter<A>,
}

pub struct ErrorPicker<E, Cap>(PhantomData<fn() -> (E, Cap)>);

pub struct ErrorTuple {
    pub error_variant: usize,
    pub capacity: usize,
}

pub trait ErrorAsType {
    type Type;
}

type_level_error! {
    arguments(cap)

    NotEnoughSpace => not_enough_space_to_write_text_in_StrWriter_with_this_capacity<cap>,

    NotAscii => input_text_was_not_ascii<>,

    NotOnCharBoundary => NotOnCharBoundary<>,
}

// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

macro_rules! mint_vec {
    ($name:ident [ $($field:ident),* ] = $std_name:ident) => {
        #[cfg(feature = "mint")]
        impl<T, U> From<mint::$std_name<T>> for $name<T, U> {
            fn from(v: mint::$std_name<T>) -> Self {
                $name {
                    $( $field: v.$field, )*
                    _unit: PhantomData,
                }
            }
        }
        #[cfg(feature = "mint")]
        impl<T, U> From<$name<T, U>> for mint::$std_name<T> {
            fn from(v: $name<T, U>) -> Self {
                mint::$std_name {
                    $( $field: v.$field, )*
                }
            }
        }
    }
}

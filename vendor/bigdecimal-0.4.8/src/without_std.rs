#[allow(unused_imports)]
#[macro_use]
extern crate alloc;

#[cfg(test)]
extern crate siphasher;

// Without this import we get the following error:
// error[E0599]: no method naemed `powi` found for type `f64` in the current scope
#[allow(unused_imports)]
use num_traits::float::FloatCore;

// Wrap core:: modules in namespace
#[allow(unused_imports)]
mod stdlib {

    pub use core::{
        cmp,
        convert,
        default,
        fmt,
        hash,
        mem,
        num,
        ops,
        iter,
        slice,
        str,
        i8,
        f32,
        f64,
    };

    #[cfg(test)]
    pub use siphasher::sip::SipHasher as DefaultHasher;

    pub use alloc::borrow;
    pub use alloc::string;
    pub use alloc::vec::Vec;
}

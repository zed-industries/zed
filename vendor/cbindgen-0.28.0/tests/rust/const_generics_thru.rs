// Propagating const arguments through generics that use generics.

#[repr(C)]
pub struct Inner<const N: usize> {
    pub bytes: [u8; N],
}

#[repr(C)]
pub struct Outer<const N: usize> {
    pub inner: Inner<N>, // don't declare two different structs named `Inner_N`
}

#[no_mangle]
pub extern "C" fn one() -> Outer<1> {
    Outer { inner: Inner { bytes: [0] } }
}

#[no_mangle]
pub extern "C" fn two() -> Outer<2> {
    Outer { inner: Inner { bytes: [0, 0] } }
}


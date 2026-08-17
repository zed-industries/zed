// The output of `bindgen!` should be compatible with `no_std` by default, so
// test that here with a no_std crate.

#![no_std]

extern crate std;

macro_rules! gentest {
    ($id:ident $name:tt $path:tt) => {
        mod $id {
            wasmtime::component::bindgen!(in $path);
        }
    };
}

component_macro_test_helpers::foreach!(gentest);

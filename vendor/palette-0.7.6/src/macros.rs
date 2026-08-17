// From https://stackoverflow.com/questions/60187436/rust-macro-repetition-with-plus
macro_rules! strip_plus {
    (+ $($rest: tt)*) => {
        $($rest)*
    }
}

#[macro_use]
mod arithmetics;
#[macro_use]
mod casting;
#[macro_use]
mod mix;
#[macro_use]
mod lighten_saturate;
#[macro_use]
mod equality;
#[macro_use]
mod blend;
#[macro_use]
mod lazy_select;
#[macro_use]
mod simd;
#[macro_use]
mod clamp;
#[macro_use]
mod convert;
#[macro_use]
mod color_difference;
#[macro_use]
mod struct_of_arrays;
#[macro_use]
mod reference_component;
#[macro_use]
mod copy_clone;
#[macro_use]
mod hue;
#[macro_use]
mod random;
#[macro_use]
mod color_theory;

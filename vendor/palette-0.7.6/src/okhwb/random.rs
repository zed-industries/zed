use crate::{
    okhsv::{Okhsv, UniformOkhsv},
    Okhwb,
};

impl_rand_traits_hwb_cone!(
    UniformOkhwb,
    Okhwb,
    UniformOkhsv,
    Okhsv {
        height: value,
        radius: saturation
    }
);

//! Defmt implementations for heapless types

use crate::{
    len_type::LenType,
    string::{StringInner, StringStorage},
    vec::{VecInner, VecStorage},
    CapacityError,
};
use defmt::Formatter;

impl<T, LenT: LenType, S: VecStorage<T> + ?Sized> defmt::Format for VecInner<T, LenT, S>
where
    T: defmt::Format,
{
    fn format(&self, fmt: Formatter<'_>) {
        defmt::write!(fmt, "{=[?]}", self.as_slice());
    }
}

impl<LenT: LenType, S: StringStorage + ?Sized> defmt::Format for StringInner<LenT, S> {
    fn format(&self, fmt: Formatter<'_>) {
        defmt::write!(fmt, "{=str}", self.as_str());
    }
}

impl defmt::Format for CapacityError {
    fn format(&self, fmt: Formatter<'_>) {
        defmt::write!(fmt, "CapacityError");
    }
}

#[repr(C)]
pub struct FixedPoint<const FRACTION_BITS: u16> {
    value: u16,
}

pub const FONT_WEIGHT_FRACTION_BITS: u16 = 6;

pub type FontWeightFixedPoint = FixedPoint<FONT_WEIGHT_FRACTION_BITS>;

#[repr(C)]
pub struct FontWeight(FontWeightFixedPoint);

impl FontWeight {
    pub const NORMAL: FontWeight = FontWeight(FontWeightFixedPoint { value: 400 << FONT_WEIGHT_FRACTION_BITS });
}

#[no_mangle]
pub extern "C" fn root(w: FontWeight) {}

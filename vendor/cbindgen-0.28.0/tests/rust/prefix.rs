pub const LEN: i32 = 22;

pub type NamedLenArray = [i32; LEN];
pub type ValuedLenArray = [i32; 22];

#[repr(u8)]
pub enum AbsoluteFontWeight {
    Weight(f32),
    Normal,
    Bold,
}

#[no_mangle]
pub extern "C" fn root(x: NamedLenArray, y: ValuedLenArray, z: AbsoluteFontWeight) {}

#[no_mangle]
pub const X: i64 = 22 << 22;

#[no_mangle]
pub const Y: i64 = X + X;

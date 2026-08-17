pub struct Overflow;

/// Negate `u64` checking for overflow.
pub fn neg(value: u64) -> Result<i64, Overflow> {
    if value <= 0x7fff_ffff_ffff_ffff {
        Ok(-(value as i64))
    } else if value == 0x8000_0000_0000_0000 {
        Ok(-0x8000_0000_0000_0000)
    } else {
        Err(Overflow)
    }
}

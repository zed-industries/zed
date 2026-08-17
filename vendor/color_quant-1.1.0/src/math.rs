#[inline]
pub(crate) fn clamp(a: i32) -> i32 {
    if a < 0 {
        0
    } else if a > 255 {
        255
    } else {
        a
    }
}

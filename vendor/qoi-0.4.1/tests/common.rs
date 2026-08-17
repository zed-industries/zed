#[allow(unused)]
pub fn hash<const N: usize>(px: [u8; N]) -> u8 {
    let r = px[0];
    let g = px[1];
    let b = px[2];
    let a = if N >= 4 { px[3] } else { 0xff };
    let rm = r.wrapping_mul(3);
    let gm = g.wrapping_mul(5);
    let bm = b.wrapping_mul(7);
    let am = a.wrapping_mul(11);
    rm.wrapping_add(gm).wrapping_add(bm).wrapping_add(am) % 64
}

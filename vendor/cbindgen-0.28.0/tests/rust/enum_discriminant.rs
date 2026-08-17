pub const FOUR: i8 = 4;

#[repr(i8)]
enum E {
    A = 1,
    B = -1,
    C = 1 + 2,
    D = FOUR,
    F = (5),
    G = b'6' as i8,
    H = false as i8,
}

#[no_mangle]
pub extern "C" fn root(_: &E) {}

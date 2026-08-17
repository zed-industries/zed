type Usize = usize;
type Isize = isize;

#[repr(usize)]
enum UE {
    UV,
}

#[repr(isize)]
enum IE {
    IV,
}

#[no_mangle]
pub extern "C" fn root(_: Usize, _: Isize, _: UE, _: IE) {}

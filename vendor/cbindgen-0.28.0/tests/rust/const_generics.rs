#[repr(transparent)]
pub struct CArrayString<const CAP: usize> {
    pub chars: [i8; CAP],
}

pub const TITLE_SIZE: usize = 80;

#[repr(C)]
pub struct Book {
    pub title: CArrayString<TITLE_SIZE>,
    pub author: CArrayString<40>,
}

#[no_mangle]
pub extern "C" fn root(a: *mut Book) {}

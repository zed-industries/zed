// Name mangling can cope with char-like byte literals.

#[repr(C)]
pub struct Parser<const OPEN: u8, const CLOSE: u8> {
    pub buf: *mut u8,
    pub len: usize,
}

#[no_mangle]
pub unsafe extern "C" fn init_parens_parser(p: *mut Parser<b'(', b')'>, buf: *mut u8, len: usize) {
    unsafe {
        *p = Parser { buf, len };
    }
}

// The same type as above, because `b'(' == 40 && b')' == 41`. And it happens
// to mangle to the same C identifier. It doesn't always work out that way!
#[no_mangle]
pub unsafe extern "C" fn destroy_parens_parser(p: *mut Parser<40, 41>) {
    // nothing to do
}


#[no_mangle]
pub unsafe extern "C" fn init_braces_parser(p: *mut Parser<b'{', b'}'>, buf: *mut u8, len: usize) {
    unsafe {
        *p = Parser { buf, len };
    }
}

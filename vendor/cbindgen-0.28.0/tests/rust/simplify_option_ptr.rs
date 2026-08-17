
struct Opaque();

#[repr(C)]
struct Foo {
    x: Option<&Opaque>,
    y: Option<&mut Opaque>,
    z: Option<fn () -> ()>,
    zz: *mut Option<fn () -> ()>,
}

#[repr(C)]
union Bar {
    x: Option<&Opaque>,
    y: Option<&mut Opaque>,
    z: Option<fn () -> ()>,
    zz: *mut Option<fn () -> ()>,
}

#[no_mangle]
pub extern "C" fn root(
	a: Option<&Opaque>,
    b: Option<&mut Opaque>,
    c: Foo,
    d: Bar,
    e: *mut Option<*mut Opaque>,
    f: extern "C" fn(Option<&Opaque>),
) { }

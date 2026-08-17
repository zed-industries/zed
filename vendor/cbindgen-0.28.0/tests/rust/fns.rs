#[repr(C)]
pub struct Fns {
  noArgs: fn(),
  anonymousArg: fn(i32),
  returnsNumber: fn() -> i32,
  namedArgs: fn(first: i32, snd: i16) -> i8,
  namedArgsWildcards: fn(_: i32, named: i16, _: i64) -> i8,
}

#[no_mangle]
pub extern "C" fn root(_fns: Fns) {}

#[no_mangle]
pub extern "C" fn no_return() -> ! {
    loop {}
}

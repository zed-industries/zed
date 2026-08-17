#[repr(C)]
struct List<T> {
     members: *mut T,
     count: usize
}

struct A;

struct B;

#[no_mangle]
pub extern "C" fn foo(a: List<A>) { }

#[no_mangle]
pub extern "C" fn bar(b: List<B>) { }

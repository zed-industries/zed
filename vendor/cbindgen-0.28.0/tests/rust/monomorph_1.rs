#[repr(C)]
struct Foo<T> {
    data: *const T
}

struct Bar<T> {
    data: *const T
}

#[repr(C)]
struct Tuple<T, E> {
    a: *const T,
    b: *const E,
}

type Indirection<T> = Tuple<T, f32>;

#[no_mangle]
pub extern "C" fn root(
    a: Foo<i32>,
    b: Foo<f32>,
    c: Bar<f32>,
    d: Foo<Bar<f32>>,
    e: Bar<Foo<f32>>,
    f: Bar<Bar<f32>>,
    g: Tuple<Foo<f32>, f32>,
    h: Indirection<f32>
) { }

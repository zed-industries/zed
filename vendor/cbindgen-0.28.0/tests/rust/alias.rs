#[repr(C)]
struct Dep {
    a: i32,
    b: f32,
}

#[repr(C)]
struct Foo<X> {
    a: X,
    b: X,
    c: Dep,
}

#[repr(u32)]
enum Status {
    Ok,
    Err,
}

type IntFoo = Foo<i32>;
type DoubleFoo = Foo<f64>;

type Unit = i32;
type SpecialStatus = Status;

#[no_mangle]
pub extern "C" fn root(
    x: IntFoo,
    y: DoubleFoo,
    z: Unit,
    w: SpecialStatus
) { }

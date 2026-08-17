#[repr(C)]
struct ABC {
    pub a: f32,
    pub b: u32,
    pub c: u32,
}

#[repr(C)]
struct BAC {
    pub b: u32,
    pub a: f32,
    pub c: i32,
}

impl ABC {
    pub const abc: ABC = ABC { a: 1.0, b: 2, c: 3 };
    pub const bac: ABC = ABC { b: 2, a: 1.0, c: 3 };
    pub const cba: ABC = ABC { c: 3, b: 2, a: 1.0 };
}

impl BAC {
    pub const abc: BAC = BAC { a: 2.0, b: 1, c: 3 };
    pub const bac: BAC = BAC { b: 1, a: 2.0, c: 3 };
    pub const cba: BAC = BAC { c: 3, b: 1, a: 2.0 };
}

#[no_mangle]
pub extern "C" fn root(a1: ABC, a2: BAC) {}

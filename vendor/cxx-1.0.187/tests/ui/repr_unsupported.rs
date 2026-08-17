#[cxx::bridge]
mod ffi {
    #[repr(align(2))]
    enum EnumAlign {
        A,
    }

    #[repr(i64)]
    struct StructInt {
        i: i32,
    }

    #[repr(align(1 << 10))]
    struct StructExpr {
        i: i32,
    }
}

fn main() {}

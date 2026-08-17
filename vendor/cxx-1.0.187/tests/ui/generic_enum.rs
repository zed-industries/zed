#[cxx::bridge]
mod ffi {
    enum A<T> {
        Field,
    }

    enum B<T> where T: Copy {
        Field,
    }

    enum C where void: Copy {
        Field,
    }
}

fn main() {}

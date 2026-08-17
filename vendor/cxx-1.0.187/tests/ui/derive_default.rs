#[cxx::bridge]
mod ffi {
    #[derive(Default)]
    enum NoDefault {
        Two,
        Three,
        Five,
        Seven,
    }

    #[derive(Default)]
    enum MultipleDefault {
        #[default]
        Two,
        Three,
        Five,
        #[default]
        Seven,
    }
}

#[cxx::bridge]
mod ffi2 {
    #[derive(Default)]
    enum BadDefault {
        #[default(repr)]
        Two,
        #[default = 3]
        Three,
    }
}

fn main() {}

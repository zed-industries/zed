include!(concat!(env!("OUT_DIR"), "/_.default.rs"));
pub mod bar {
    include!(concat!(env!("OUT_DIR"), "/bar.rs"));
}
pub mod foo {
    include!(concat!(env!("OUT_DIR"), "/foo.rs"));
    pub mod bar {
        include!(concat!(env!("OUT_DIR"), "/foo.bar.rs"));
        pub mod a {
            pub mod b {
                pub mod c {
                    include!(concat!(env!("OUT_DIR"), "/foo.bar.a.b.c.rs"));
                }
            }
        }
        pub mod baz {
            include!(concat!(env!("OUT_DIR"), "/foo.bar.baz.rs"));
        }
        pub mod qux {
            include!(concat!(env!("OUT_DIR"), "/foo.bar.qux.rs"));
        }
    }
}

#![allow(unused_macros, reason = "testing codegen, not functionality")]
#![allow(
    dead_code,
    unused_variables,
    reason = "testing codegen, not functionality"
)]

#[allow(unused, reason = "testing codegen, not functionality")]
mod multiple_paths {
    wit_bindgen::generate!({
        inline: r#"
        package test:paths;

        world test {
            import paths:path1/test;
            export paths:path2/test;
        }
        "#,
        path: ["tests/wit/path1", "tests/wit/path2"],
        generate_all,
    });
}

#[allow(unused, reason = "testing codegen, not functionality")]
mod inline_and_path {
    wit_bindgen::generate!({
        inline: r#"
        package test:paths;

        world test {
            import test:inline-and-path/bar;
        }
        "#,
        path: ["tests/wit/path3"],
        generate_all,
    });
}

#[allow(unused, reason = "testing codegen, not functionality")]
mod newtyped_list {
    use std::ops::Deref;

    wit_bindgen::generate!({
        inline: r#"
        package test:newtyped-list;

        interface byte {
            type typed-list-of-byte = list<u8>;
            type newtyped-list-of-byte = list<u8>;

            record rec-of-lists {
                l: list<u8>,
                tl: typed-list-of-byte,
                nl: newtyped-list-of-byte,
            }

            use-list-of-byte: func(l: list<u8>) -> list<u8>;
            use-typed-list-of-byte: func(tl: typed-list-of-byte) -> typed-list-of-byte;
            use-newtyped-list-of-byte: func(nl: newtyped-list-of-byte) -> newtyped-list-of-byte;
            use-rec-of-lists: func(t: rec-of-lists) -> rec-of-lists;
        }

        interface noncopy-byte {
            // this will be new-typed by a non-copy struct
            type noncopy-byte = u8;

            type newtyped-list-of-noncopy-byte = list<noncopy-byte>;
            type typed-list-of-noncopy-byte = list<noncopy-byte>;

            record rec-of-lists-of-noncopy-byte {
                ntl: newtyped-list-of-noncopy-byte,
                tl: typed-list-of-noncopy-byte,
                l: list<noncopy-byte>,
            }

            use-newtyped-list-of-noncopy-byte: func(nl: newtyped-list-of-noncopy-byte) -> newtyped-list-of-noncopy-byte;
            use-typed-list-of-noncopy-byte: func(tl: typed-list-of-noncopy-byte) -> typed-list-of-noncopy-byte;
            use-list-of-noncopy-byte: func(l: list<noncopy-byte>) -> list<noncopy-byte>;
            use-rec-of-lists-of-noncopy-byte: func(t: rec-of-lists-of-noncopy-byte) -> rec-of-lists-of-noncopy-byte;
        }

        world test {
            import byte;
            export byte;
            import noncopy-byte;
            export noncopy-byte;
        }
        "#,
        with: {
            "test:newtyped-list/byte/newtyped-list-of-byte": crate::newtyped_list::NewtypedListOfByte,
            "test:newtyped-list/noncopy-byte/noncopy-byte": crate::newtyped_list::NoncopyByte,
            "test:newtyped-list/noncopy-byte/newtyped-list-of-noncopy-byte": crate::newtyped_list::NewtypedListofNoncopyByte,
        }
    });

    #[derive(Debug, Clone)]
    pub struct NewtypedListOfByte(Vec<u8>);

    impl From<Vec<u8>> for NewtypedListOfByte {
        fn from(value: Vec<u8>) -> Self {
            NewtypedListOfByte(value)
        }
    }

    impl From<NewtypedListOfByte> for Vec<u8> {
        fn from(value: NewtypedListOfByte) -> Self {
            value.0
        }
    }

    impl Deref for NewtypedListOfByte {
        type Target = Vec<u8>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    #[derive(Debug, Clone)]
    pub struct NoncopyByte(u8);

    #[derive(Debug, Clone)]
    pub struct NewtypedListofNoncopyByte(Vec<NoncopyByte>);

    impl From<Vec<NoncopyByte>> for NewtypedListofNoncopyByte {
        fn from(value: Vec<NoncopyByte>) -> Self {
            NewtypedListofNoncopyByte(value)
        }
    }

    impl From<NewtypedListofNoncopyByte> for Vec<NoncopyByte> {
        fn from(value: NewtypedListofNoncopyByte) -> Self {
            value.0
        }
    }

    impl Deref for NewtypedListofNoncopyByte {
        type Target = Vec<NoncopyByte>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
}
#[allow(unused, reason = "testing codegen, not functionality")]
mod retyped_list {
    use std::ops::Deref;

    wit_bindgen::generate!({
        inline: r#"
        package test:retyped-list;

        interface bytes {
            // this will be the `Bytes` type from the bytes crate
            type retyped-list-as-bytes = list<u8>;

            record rec-bytes {
                rl: retyped-list-as-bytes,
            }

            use-retyped-list-as-bytes: func(ri: retyped-list-as-bytes) -> retyped-list-as-bytes;
            use-rec-of-retyped-list-as-bytes: func(rl: retyped-list-as-bytes) -> retyped-list-as-bytes;
        }

        world test {
            import bytes;
            export bytes;
        }
        "#,
        with: {
            "test:retyped-list/bytes/retyped-list-as-bytes": bytes::Bytes,
        }
    });
}

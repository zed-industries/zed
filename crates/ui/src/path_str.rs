#[cfg(test)]
mod tests {
    use strum::EnumString;
    use ui_macros::{path_str, DerivePathStr};

    #[test]
    fn test_derive_path_str_with_prefix() {
        #[derive(Debug, EnumString, DerivePathStr)]
        #[strum(serialize_all = "snake_case")]
        #[path_str(prefix = "test_prefix")]
        enum MyEnum {
            FooBar,
            Baz,
        }

        assert_eq!(MyEnum::FooBar.path(), "test_prefix/foo_bar");
        assert_eq!(MyEnum::Baz.path(), "test_prefix/baz");
    }

    #[test]
    fn test_derive_path_str_with_prefix_and_suffix() {
        #[derive(Debug, EnumString, DerivePathStr)]
        #[strum(serialize_all = "snake_case")]
        #[path_str(prefix = "test_prefix", suffix = ".txt")]
        enum MyEnum {
            FooBar,
            Baz,
        }

        assert_eq!(MyEnum::FooBar.path(), "test_prefix/foo_bar.txt");
        assert_eq!(MyEnum::Baz.path(), "test_prefix/baz.txt");
    }
}

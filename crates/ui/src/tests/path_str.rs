// We need to test [ui_macros::DerivePathStr] here as we can't invoke it
// in the `ui_macros` crate.
#[cfg(test)]
mod tests {
    use strum::EnumString;
    use ui_macros::{DerivePathStr, path_str};

    #[test]
    fn test_derive_path_str_with_prefix() {
        #[derive(Debug, EnumString, DerivePathStr)]
        #[strum(serialize_all = "snake_case")]
        #[path_str(prefix = "test_prefix")]
        enum SomeAsset {
            FooBar,
            Baz,
        }

        assert_eq!(SomeAsset::FooBar.path(), "test_prefix/foo_bar");
        assert_eq!(SomeAsset::Baz.path(), "test_prefix/baz");
    }

    #[test]
    fn test_derive_path_str_with_prefix_and_suffix() {
        #[derive(Debug, EnumString, DerivePathStr)]
        #[strum(serialize_all = "snake_case")]
        #[path_str(prefix = "test_prefix", suffix = ".svg")]
        enum SomeAsset {
            FooBar,
            Baz,
        }

        assert_eq!(SomeAsset::FooBar.path(), "test_prefix/foo_bar.svg");
        assert_eq!(SomeAsset::Baz.path(), "test_prefix/baz.svg");
    }
}

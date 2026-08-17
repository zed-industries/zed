use crate::descriptor::{Package, TypePath};

#[derive(Debug)]
pub struct Resolver<'a> {
    extern_types: &'a [(String, String)],
    retain_enum_prefix: bool,
    package: &'a Package,
}

impl<'a> Resolver<'a> {
    /// Creates a new resolver for `package`
    pub fn new(
        extern_types: &'a [(String, String)],
        package: &'a Package,
        retain_enum_prefix: bool,
    ) -> Self {
        Resolver {
            extern_types,
            package,
            retain_enum_prefix,
        }
    }

    /// Lookup an extern type, returns the rust path followed by the number of
    /// leading path segments to skip (they are already included in the rust path)
    fn resolve_extern(&self, path: &TypePath) -> Option<(String, usize)> {
        let mut match_len = 0_usize;
        let mut match_idx = None;

        for (idx, (proto_path, _)) in self.extern_types.iter().enumerate() {
            match path.prefix_match(proto_path) {
                Some(m) if m >= match_len => {
                    match_idx = Some(idx);
                    match_len = m;
                }
                _ => {}
            }
        }

        let (_, rust_path) = &self.extern_types[match_idx?];

        if match_len == path.len() {
            Some((rust_path.clone(), match_len))
        } else {
            Some((format!("{}::", rust_path), match_len))
        }
    }

    /// Returns the rust type for `path`
    pub fn rust_type(&self, path: &TypePath) -> String {
        let (mut ret, skip) = match self.resolve_extern(path) {
            Some(x) => x,
            None => {
                // Compute the amount the resolver and path have in common
                let shared_prefix = self
                    .package
                    .path()
                    .iter()
                    .zip(path.path())
                    .take_while(|(a, b)| a == b)
                    .count();

                let super_count = self.package.path().len() - shared_prefix;

                let mut ret = String::new();
                for _ in 0..super_count {
                    ret.push_str("super::")
                }

                (ret, shared_prefix)
            }
        };

        let mut iter = path.path().skip(skip).peekable();
        while let Some(i) = iter.next() {
            match iter.peek() {
                Some(_) => {
                    ret.push_str(i.to_snake_case_ident().as_str());
                    ret.push_str("::");
                }
                None => {
                    ret.push_str(i.to_upper_camel_case().as_str());
                }
            }
        }
        ret
    }

    pub fn rust_variant(&self, enumeration: &TypePath, variant: &str) -> String {
        use heck::ToUpperCamelCase;
        let variant = variant.to_upper_camel_case();
        match self.retain_enum_prefix {
            true => variant,
            false => {
                let prefix = enumeration.path().last().unwrap().to_upper_camel_case();
                let stripped = variant.strip_prefix(&prefix).unwrap_or(&variant);

                // "Foo" should not be stripped from "Foobar".
                match stripped.chars().next().map(char::is_uppercase) {
                    Some(true) => stripped.to_string(),
                    _ => variant,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::descriptor::TypeName;

    #[test]
    fn test_resolver() {
        let resolver_package = Package::new("test.syntax3");
        let extern_types = &[
            (".test.external".to_string(), "foo::common".to_string()),
            (".test.external.Boo".to_string(), "foo::common".to_string()),
            (
                ".test.external.Fiz.Buz".to_string(),
                "foo::bar::Buz".to_string(),
            ),
        ];
        let resolver = Resolver::new(extern_types, &resolver_package, false);

        // A type in the same package
        let same_type = TypePath::new(resolver_package.clone()).child(TypeName::new("Foo"));
        assert_eq!(resolver.rust_type(&same_type), "Foo");

        let nested_type = TypePath::new(resolver_package.clone())
            .child(TypeName::new("Foo"))
            .child(TypeName::new("Bar"));

        assert_eq!(resolver.rust_type(&nested_type), "foo::Bar");

        let other_package = Package::new("test.common");

        // A type in another package
        let other_type = TypePath::new(other_package.clone()).child(TypeName::new("Bar"));
        assert_eq!(resolver.rust_type(&other_type), "super::common::Bar");

        let other_nested_type = TypePath::new(other_package)
            .child(TypeName::new("Foo"))
            .child(TypeName::new("Bar"));

        assert_eq!(
            resolver.rust_type(&other_nested_type),
            "super::common::foo::Bar"
        );

        let external_package = Package::new("test.external");

        // An external type
        let external_type = TypePath::new(external_package.clone()).child(TypeName::new("Fiz"));
        assert_eq!(resolver.rust_type(&external_type), "foo::common::Fiz");

        // A nested external type
        let nested_external_type = external_type.child(TypeName::new("Bar"));

        assert_eq!(
            resolver.rust_type(&nested_external_type),
            "foo::common::fiz::Bar"
        );

        // An external type within an external type
        let external_nested_type = external_type.child(TypeName::new("Buz"));
        assert_eq!(resolver.rust_type(&external_nested_type), "foo::bar::Buz");

        // An external type with a different rust path length
        let external_nested_type = TypePath::new(external_package)
            .child(TypeName::new("Boo"))
            .child(TypeName::new("Bar"));
        assert_eq!(
            resolver.rust_type(&external_nested_type),
            "foo::common::Bar"
        );
    }

    #[test]
    // https://github.com/influxdata/pbjson/issues/48
    fn test_resolver_shared_prefix_false_match() {
        assert_eq!(
            Resolver::new(&[], &Package::new("test.api.v1"), false).rust_type(
                &TypePath::new(Package::new("test.domain.v1"))
                    .child(TypeName::new("Foo"))
                    .child(TypeName::new("Bar"))
            ),
            "super::super::domain::v1::foo::Bar"
        );
    }

    #[test]
    fn test_variant() {
        let package = Package::new("test.syntax3");
        let resolver = Resolver::new(&[], &package, false);

        let tests = [
            ("MyEnum", "MyEnumFoo", "Foo"),
            ("MyEnum", "MyEnumfoo", "MyEnumfoo"),
            ("MyEnum", "MY_ENUM_foo", "Foo"),
        ];

        for (enumeration, variant, expected) in tests {
            assert_eq!(
                resolver.rust_variant(
                    &TypePath::new(Package::new("test.syntax3")).child(TypeName::new(enumeration)),
                    variant,
                ),
                expected,
                "{}::{}",
                enumeration,
                variant,
            );
        }
    }
}

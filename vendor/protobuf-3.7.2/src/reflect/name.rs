pub(crate) fn concat_paths(a: &str, b: &str) -> String {
    if a.is_empty() {
        b.to_owned()
    } else if b.is_empty() {
        b.to_owned()
    } else {
        format!("{}.{}", a, b)
    }
}

pub(crate) fn protobuf_name_starts_with_package<'a>(
    name: &'a str,
    package: &str,
) -> Option<&'a str> {
    assert!(
        !package.starts_with("."),
        "package must not start with dot: {}",
        package
    );

    assert!(
        name.starts_with("."),
        "full name must start with dot: {}",
        name
    );
    let name = &name[1..];
    // assert!(!name.starts_with("."), "full name must not start with dot: {}", name);

    if package.is_empty() {
        Some(name)
    } else {
        if name.starts_with(package) {
            let rem = &name[package.len()..];
            if rem.starts_with(".") {
                Some(&rem[1..])
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[test]
fn test_protobuf_name_starts_with_package() {
    assert_eq!(
        Some("bar"),
        protobuf_name_starts_with_package(".foo.bar", "foo")
    );
    assert_eq!(None, protobuf_name_starts_with_package(".foo", "foo"));
    assert_eq!(Some("foo"), protobuf_name_starts_with_package(".foo", ""));
}

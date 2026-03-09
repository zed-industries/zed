use project::yarn::*;
use std::path::Path;

#[test]
fn test_resolve_virtual() {
    let test_cases = vec![
        (
            "/path/to/some/folder/__virtual__/a0b1c2d3/0/subpath/to/file.dat",
            Some(Path::new("/path/to/some/folder/subpath/to/file.dat")),
        ),
        (
            "/path/to/some/folder/__virtual__/e4f5a0b1/0/subpath/to/file.dat",
            Some(Path::new("/path/to/some/folder/subpath/to/file.dat")),
        ),
        (
            "/path/to/some/folder/__virtual__/a0b1c2d3/1/subpath/to/file.dat",
            Some(Path::new("/path/to/some/subpath/to/file.dat")),
        ),
        (
            "/path/to/some/folder/__virtual__/a0b1c2d3/3/subpath/to/file.dat",
            Some(Path::new("/path/subpath/to/file.dat")),
        ),
        ("/path/to/nonvirtual/", None),
        ("/path/to/malformed/__virtual__", None),
        ("/path/to/malformed/__virtual__/a0b1c2d3", None),
        (
            "/path/to/malformed/__virtual__/a0b1c2d3/this-should-be-a-number",
            None,
        ),
    ];

    for (input, expected) in test_cases {
        let input_path = Path::new(input);
        let resolved_path = resolve_virtual(input_path);
        assert_eq!(resolved_path.as_deref(), expected);
    }
}

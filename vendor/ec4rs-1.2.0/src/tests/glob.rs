pub fn test<'a, 'b>(
    pattern: &str,
    valid: impl IntoIterator<Item = &'a str>,
    invalid: impl IntoIterator<Item = &'b str>,
) {
    use crate::glob::Glob;
    let glob = Glob::new(pattern);
    for path in valid {
        assert!(
            glob.matches(path.as_ref()),
            "`{}` didn't match pattern `{}`; chain: {:?}",
            path,
            pattern,
            glob
        )
    }
    for path in invalid {
        assert!(
            !glob.matches(path.as_ref()),
            "`{}` wrongly matched pattern `{}`; chain {:?}",
            path,
            pattern,
            glob
        )
    }
}

#[test]
fn basic() {
    test(
        "foo",
        ["foo", "/foo", "./foo", "/bar/foo"],
        ["/foobar", "/barfoo"],
    );
    test("foo,bar", ["/foo,bar"], ["/foo", "/bar"]);
}

#[test]
fn path() {
    test(
        "bar/foo",
        ["/bar/foo", "bar/foo", "/bar//foo"],
        ["/bar/foo/baz", "/baz/bar/foo"],
    );
}

#[test]
fn root_star() {
    test(
        "/*",
        ["/foo.txt", "/bar.xml", "/baz.json"],
        ["/bar/foo/baz.txt", "/baz/bar/foo.xml", "/bar/foo.txt"],
    );
}

#[test]
fn root_double_star() {
    test(
        "/**",
        [
            "/foo.txt",
            "/bar.xml",
            "/baz.json",
            "/bar/foo/baz.txt",
            "/baz/bar/foo.xml",
            "/bar/foo.txt",
        ],
        [],
    );
}

#[test]
fn star() {
    test("*", ["/*", "/a"], []);
    test(
        "*.foo",
        ["/a.foo", "/b.foo", "/ab.foo", "/bar/abc.foo", "/.foo"],
        ["/foo"],
    );
    test(
        "bar*.foo",
        ["/bar.foo", "/barab.foo", "/baz/bara.foo", "/bar.foo"],
        ["/bar/.foo"],
    );
}

#[test]
fn doublestar() {
    test("**.foo", ["/a.foo", "/a/a.foo", "/a/b.foo", "/.foo"], []);
    test(
        "a**d",
        ["/a/d", "/a/bd", "/a/bcd", "/a/b/c/d"],
        ["/bd", "/b/d", "/bcd"],
    );
}

#[test]
fn charclass_basic() {
    test("[a]", ["/a"], ["/aa", "/b"]);
    test("[a][b]", ["/ab"], ["/aa", "/ba", "/cab"]);
    test("[ab]", ["/a", "/b"], ["/ab"]);
    test("[!ab]", ["/c"], ["/a", "/b", "/ab", "/ac"])
}

#[test]
fn charclass_slash() {
    // See the brackets_slash_inside tests.
    test("a[b/]c", ["/a[b/]c"], ["/abc", "/a/c"]);
}

#[test]
fn charclass_range() {
    test("[a-c]", ["/a", "/b", "/c"], ["/d"]);
    test("[-]", ["/-"], ["/"]);
    test("[-a]", ["/-", "/a"], []);
    test("[a-]", ["/-", "/a"], []);
}

#[test]
fn charclass_escape() {
    test("[\\]a]", ["/]", "/a"], []);
    test("[a\\-c]", ["/a", "/-", "/c"], ["/b"]);
    test("[[-\\]^]", ["/[", "/]", "/^"], []);
}

#[test]
fn numrange() {
    test("{1..3}", ["2"], ["1..3", "{1..3}"]);
    test("{8..11}", ["/8", "/9", "/10", "/11"], ["/12", "/1", "/01"]);
    test("{-3..-1}", ["/-3", "/-2", "/-1"], ["/0", "/1"]);
    test("{2..-1}", ["/2", "/1", "/0", "/-1"], ["/-2"]);
}

#[test]
fn alt_basic() {
    test("{}", ["/{}"], ["/"]);
    test("{foo}", ["/{foo}"], ["/foo"]);
    test("{foo}.bar", ["/{foo}.bar"], ["/foo", "/foo.bar"]);
    test(
        "{foo,bar}",
        ["/foo", "/bar"],
        ["/foo,bar", "/foobar", "/{foo,bar}"],
    );
}

#[test]
fn alt_star() {
    test("{*}", ["/{}", "/{a}", "/{ab}"], []);
    test("{a,*}", ["/a", "/b"], []);
}

#[test]
fn alt_unmatched() {
    test("{.foo", ["/{.foo"], ["/.foo", "/{.foo}"]);
    test("{},foo}", ["/{},foo}"], ["/.foo", "/.foo}"]);
    test("{,a,{b}", ["/{,a,{b}"], []);
}

#[test]
fn alt_nested() {
    test("{a{bc,cd},e}", ["/abc", "/acd", "/e"], ["/cd"]);
}

#[test]
fn alt_empty() {
    test("a{b,,c}", ["/a", "/ab", "/ac"], []);
}

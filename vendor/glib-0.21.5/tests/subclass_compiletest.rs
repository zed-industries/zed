#[test]
pub fn test() {
    let t = trybuild2::TestCases::new();

    t.pass("tests/subclass_compiletest/01-auto-send-sync.rs");
    t.compile_fail("tests/subclass_compiletest/02-no-auto-send-sync.rs");
    t.compile_fail("tests/subclass_compiletest/03-object-no-auto-send-sync.rs");
    t.pass("tests/subclass_compiletest/04-auto-send-sync-with-send-sync-parent.rs");
    t.compile_fail("tests/subclass_compiletest/05-no-auto-send-sync-with-non-send-sync-parent.rs");
    t.compile_fail(
        "tests/subclass_compiletest/06-no-auto-send-sync-with-non-send-sync-ffi-parent.rs",
    );
}

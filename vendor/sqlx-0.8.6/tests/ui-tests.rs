use std::path::Path;

#[test]
#[ignore]
fn ui_tests() {
    let t = trybuild::TestCases::new();

    if cfg!(feature = "postgres") {
        t.compile_fail("tests/ui/postgres/*.rs");

        // UI tests for column types that require gated features
        if cfg!(not(feature = "chrono")) && cfg!(not(feature = "time")) {
            t.compile_fail("tests/ui/postgres/gated/chrono.rs");
        }

        if cfg!(not(feature = "uuid")) {
            t.compile_fail("tests/ui/postgres/gated/uuid.rs");
        }

        if cfg!(not(feature = "ipnet")) && cfg!(not(feature = "ipnetwork")) {
            t.compile_fail("tests/ui/postgres/gated/ipnetwork.rs");
        }
    }

    if cfg!(feature = "mysql") {
        t.compile_fail("tests/ui/mysql/*.rs");

        // UI tests for column types that require gated features
        if cfg!(not(feature = "chrono")) && cfg!(not(feature = "time")) {
            t.compile_fail("tests/ui/mysql/gated/chrono.rs");
        }
    }

    if cfg!(feature = "_sqlite") {
        if dotenvy::var("DATABASE_URL").map_or(true, |v| {
            Path::is_relative(v.trim_start_matches("sqlite://").as_ref())
        }) {
            // this isn't `Trybuild`'s fault: https://github.com/dtolnay/trybuild/issues/69#issuecomment-620329526
            panic!("DATABASE_URL must contain an absolute path for SQLite UI tests")
        }

        t.compile_fail("tests/ui/sqlite/*.rs");
    }

    t.compile_fail("tests/ui/*.rs");
}

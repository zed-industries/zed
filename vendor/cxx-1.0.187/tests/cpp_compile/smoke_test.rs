use crate::cpp_compile;
use indoc::indoc;
use quote::quote;

#[test]
fn test_success() {
    let test = cpp_compile::Test::new(quote! {
        #[cxx::bridge]
        mod ffi {
            unsafe extern "C++" {
                include!("include.h");
                pub fn do_cpp_thing();
            }
        }
    });
    test.write_file(
        "include.h",
        indoc! {"
            void do_cpp_thing();
        "},
    );
    test.compile().assert_success();
}

#[test]
fn test_failure() {
    let test = cpp_compile::Test::new(quote! {
        #[cxx::bridge]
        mod ffi {
            unsafe extern "C++" {
                include!("include.h");
            }
        }
    });
    test.write_file(
        "include.h",
        indoc! {r#"
            static_assert(false, "This is a failure smoke test");
        "#},
    );
    let err_msg = test.compile().expect_single_error();
    assert!(err_msg.contains("This is a failure smoke test"));
}

#[test]
#[should_panic = "Unexpectedly more than 1 error line was present"]
fn test_unexpected_extra_error() {
    let test = cpp_compile::Test::new(quote! {
        #[cxx::bridge]
        mod ffi {
            unsafe extern "C++" {
                include!("include.h");
            }
        }
    });
    test.write_file(
        "include.h",
        indoc! {r#"
            static_assert(false, "First error line");
            static_assert(false, "Second error line");
        "#},
    );

    // We `should_panic` inside `expect_single_error` below:
    let _ = test.compile().expect_single_error();
}

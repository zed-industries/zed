mod cpp_compile;

use indoc::indoc;
use quote::quote;

/// This is a regression test for `static_assert(::rust::is_complete...)`
/// which we started to emit in <https://github.com/dtolnay/cxx/commit/534627667>
#[test]
fn test_unique_ptr_of_incomplete_foward_declared_pointee() {
    let test = cpp_compile::Test::new(quote! {
        #[cxx::bridge]
        mod ffi {
            unsafe extern "C++" {
                include!("include.h");
                type ForwardDeclaredType;
            }
            impl UniquePtr<ForwardDeclaredType> {}
        }
    });
    test.write_file(
        "include.h",
        indoc! {"
            class ForwardDeclaredType;
        "},
    );
    let err_msg = test.compile().expect_single_error();
    assert!(err_msg.contains("definition of `::ForwardDeclaredType` is required"));
}

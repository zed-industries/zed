use crate::expand;
use crate::syntax::file::Module;
use proc_macro2::TokenStream;
use quote::quote;
use syn::File;

fn bridge(cxx_bridge: TokenStream) -> String {
    let module = syn::parse2::<Module>(cxx_bridge).unwrap();
    let tokens = expand::bridge(module).unwrap();
    let file = syn::parse2::<File>(tokens).unwrap();
    let pretty = prettyplease::unparse(&file);
    eprintln!("{0:/<80}\n{pretty}{0:/<80}", "");
    pretty
}

#[test]
fn test_unique_ptr_with_elided_lifetime_implicit_impl() {
    let rs = bridge(quote! {
        mod ffi {
            unsafe extern "C++" {
                type Borrowed<'a>;
                fn borrowed(arg: &i32) -> UniquePtr<Borrowed>;
            }
        }
    });

    // It is okay that the return type elides Borrowed's lifetime parameter.
    assert!(rs.contains("pub fn borrowed(arg: &i32) -> ::cxx::UniquePtr<Borrowed>"));

    // But in impl blocks, the lifetime parameter needs to be present.
    assert!(rs.contains("unsafe impl<'a> ::cxx::ExternType for Borrowed<'a> {"));
    assert!(rs.contains("unsafe impl<'a> ::cxx::memory::UniquePtrTarget for Borrowed<'a> {"));

    // Wrong.
    assert!(!rs.contains("unsafe impl ::cxx::ExternType for Borrowed {"));
    assert!(!rs.contains("unsafe impl ::cxx::memory::UniquePtrTarget for Borrowed {"));

    // Potentially okay, but not what we currently do.
    assert!(!rs.contains("unsafe impl ::cxx::ExternType for Borrowed<'_> {"));
    assert!(!rs.contains("unsafe impl ::cxx::memory::UniquePtrTarget for Borrowed<'_> {"));
}

#[test]
fn test_unique_ptr_lifetimes_from_explicit_impl() {
    let rs = bridge(quote! {
        mod ffi {
            unsafe extern "C++" {
                type Borrowed<'a>;
            }
            impl<'b> UniquePtr<Borrowed<'c>> {}
        }
    });

    // Lifetimes use the name from the extern type.
    assert!(rs.contains("unsafe impl<'a> ::cxx::ExternType for Borrowed<'a>"));

    // Lifetimes use the names written in the explicit impl if one is present.
    assert!(rs.contains("unsafe impl<'b> ::cxx::memory::UniquePtrTarget for Borrowed<'c>"));
}

#[test]
fn test_vec_string() {
    let rs = bridge(quote! {
        mod ffi {
            extern "Rust" {
                fn foo() -> Vec<String>;
            }
        }
    });

    // No substitution of String <=> ::cxx::private::RustString.
    assert!(rs.contains("__return: *mut ::cxx::private::RustVec<::cxx::alloc::string::String>"));
    assert!(rs.contains("fn __foo() -> ::cxx::alloc::vec::Vec<::cxx::alloc::string::String>"));

    let rs = bridge(quote! {
        mod ffi {
            extern "Rust" {
                fn foo(v: &Vec<String>);
            }
        }
    });

    // No substitution of String <=> ::cxx::private::RustString.
    assert!(rs.contains("v: &::cxx::private::RustVec<::cxx::alloc::string::String>"));
    assert!(rs.contains("fn __foo(v: &::cxx::alloc::vec::Vec<::cxx::alloc::string::String>)"));
}

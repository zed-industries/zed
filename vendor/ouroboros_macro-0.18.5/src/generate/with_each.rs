use crate::info_structures::{FieldType, Options, StructInfo};
use proc_macro2::TokenStream;
use proc_macro2_diagnostics::Diagnostic;
use quote::{format_ident, quote};
use syn::Error;

pub enum ProcessingError {
    Syntax(Error),
    Covariance(Vec<Diagnostic>),
}

pub fn make_with_functions(info: &StructInfo, options: Options) -> (Vec<TokenStream>, Vec<Diagnostic>) {
    let mut users = Vec::new();
    let mut errors = Vec::new();
    for field in &info.fields {
        let visibility = &field.vis;
        let field_name = &field.name;
        let field_type = &field.typ;
        // If the field is not a tail, we need to serve up the same kind of reference that other
        // fields in the struct may have borrowed to ensure safety.
        if field.field_type == FieldType::Tail {
            let user_name = format_ident!("with_{}", &field.name);
            let documentation = format!(
                concat!(
                    "Provides an immutable reference to `{0}`. This method was generated because ",
                    "`{0}` is a [tail field](https://docs.rs/ouroboros/latest/ouroboros/attr.self_referencing.html#definitions)."
                ),
                field.name.to_string()
            );
            let documentation = if !options.do_no_doc {
                quote! {
                    #[doc=#documentation]
                }
            } else {
                quote! { #[doc(hidden)] }
            };
            users.push(quote! {
                #documentation
                #[inline(always)]
                #visibility fn #user_name <'outer_borrow, ReturnType>(
                    &'outer_borrow self,
                    user: impl for<'this> ::core::ops::FnOnce(&'outer_borrow #field_type) -> ReturnType,
                ) -> ReturnType {
                    let field = &unsafe { self.actual_data.assume_init_ref() }.#field_name;
                    user(field)
                }
            });
            if field.covariant == Some(true) {
                let borrower_name = format_ident!("borrow_{}", &field.name);
                users.push(quote! {
                    #documentation
                    #[inline(always)]
                    #visibility fn #borrower_name<'this>(
                        &'this self,
                    ) -> &'this #field_type {
                        &unsafe { self.actual_data.assume_init_ref() }.#field_name
                    }
                });
            } else if field.covariant.is_none() {
                errors.push(field.covariance_error());
            }
            // If it is not borrowed at all it's safe to allow mutably borrowing it.
            let user_name = format_ident!("with_{}_mut", &field.name);
            let documentation = format!(
                concat!(
                    "Provides a mutable reference to `{0}`. This method was generated because ",
                    "`{0}` is a [tail field](https://docs.rs/ouroboros/latest/ouroboros/attr.self_referencing.html#definitions). ",
                    "No `borrow_{0}_mut` function was generated because Rust's borrow checker is ",
                    "currently unable to guarantee that such a method would be used safely."
                ),
                field.name.to_string()
            );
            let documentation = if !options.do_no_doc {
                quote! {
                    #[doc=#documentation]
                }
            } else {
                quote! { #[doc(hidden)] }
            };
            users.push(quote! {
                #documentation
                #[inline(always)]
                #visibility fn #user_name <'outer_borrow, ReturnType>(
                    &'outer_borrow mut self,
                    user: impl for<'this> ::core::ops::FnOnce(&'outer_borrow mut #field_type) -> ReturnType,
                ) -> ReturnType {
                    let field = &mut unsafe { self.actual_data.assume_init_mut() }.#field_name;
                    user(field)
                }
            });
        } else if field.field_type == FieldType::Borrowed {
            let user_name = format_ident!("with_{}", &field.name);
            let documentation = format!(
                concat!(
                    "Provides limited immutable access to `{0}`. This method was generated ",
                    "because the contents of `{0}` are immutably borrowed by other fields."
                ),
                field.name.to_string()
            );
            let documentation = if !options.do_no_doc {
                quote! {
                    #[doc=#documentation]
                }
            } else {
                quote! { #[doc(hidden)] }
            };
            users.push(quote! {
                #documentation
                #[inline(always)]
                #visibility fn #user_name <'outer_borrow, ReturnType>(
                    &'outer_borrow self,
                    user: impl for<'this> ::core::ops::FnOnce(&'outer_borrow #field_type) -> ReturnType,
                ) -> ReturnType {
                    let field = &unsafe { self.actual_data.assume_init_ref() }.#field_name;
                    user(field)
                }
            });
            if field.self_referencing {
                if field.covariant == Some(false) {
                    // Skip the other functions, they will cause compiler errors.
                    continue;
                } else if field.covariant.is_none() {
                    errors.push(field.covariance_error());
                }
            }
            let borrower_name = format_ident!("borrow_{}", &field.name);
            users.push(quote! {
                #documentation
                #[inline(always)]
                #visibility fn #borrower_name<'this>(
                    &'this self,
                ) -> &'this #field_type {
                    &unsafe { self.actual_data.assume_init_ref() }.#field_name
                }
            });
        } else if field.field_type == FieldType::BorrowedMut {
            // Do not generate anything because if it is borrowed mutably once, we should not be able
            // to get any other kinds of references to it.
        }
    }
    (users, errors)
}

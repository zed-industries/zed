use super::assert_snapshot;
use crate::util::prelude::*;

#[test]
fn setters_docs_and_vis() {
    let actual_tokens = crate::builder::generate_from_derive(quote! {
        #[builder(on(u8, setters(doc(default(skip)))))]
        struct Sut {
            /// Docs on the required field itself
            #[builder(setters(
                vis = "pub(in overridden)",
                doc {
                    /// Docs on the required field setters.
                    /// Multiline.
                }
            ))]
            required_field: u32,

            /// Docs on the optional field itself
            #[builder(setters(
                vis = "pub(in overridden)",
                doc {
                    /// Docs on the optional field setters.
                    /// Multiline.
                }
            ))]
            optional_field: Option<u32>,

            /// Docs on the default field itself
            #[builder(
                setters(
                    vis = "pub(in overridden)",
                    doc {
                        /// Docs on the default field setters.
                        /// Multiline.
                    }
                ),
                default = 2 + 2 * 3
            )]
            default_field: u32,

            /// Docs on the field itself
            #[builder(
                setters(
                    some_fn(
                        vis = "pub(in some_fn_overridden)",
                        doc {
                            /// Docs on some_fn
                            /// Multiline.
                        }
                    ),
                    option_fn(
                        vis = "pub(in option_fn_overridden)",
                        doc {
                            /// Docs on option_fn
                            /// Multiline.
                        }
                    )
                )
            )]
            optional_field_with_specific_overrides: Option<u32>,

            #[builder(
                setters(
                    some_fn(
                        vis = "pub(in some_fn_overridden)",
                        doc {
                            /// Docs on some_fn
                            /// Multiline.
                        }
                    ),
                    option_fn(
                        vis = "pub(in option_fn_overridden)",
                        doc {
                            /// Docs on option_fn
                            /// Multiline.
                        }
                    )
                ),
                default = 2 + 2 * 3
            )]
            default_field_with_specific_overrides: u32,

            #[builder(setters(
                doc {
                    /// Common docs
                    /// Multiline.
                },
                vis = "pub(in overridden)",
                option_fn(
                    vis = "pub(in option_fn_overridden)",
                    doc {
                        /// Docs on option_fn
                        /// Multiline.
                    }
                )
            ))]
            optional_field_with_inherited_overrides: Option<u32>,


            #[builder(
                setters(
                    doc {
                        /// Common docs
                        /// Multiline.
                    },
                    vis = "pub(in overridden)",
                    option_fn(
                        vis = "pub(in option_fn_overridden)",
                        doc {
                            /// Docs on option_fn
                            /// Multiline.
                        }
                    )
                ),
                default = 2 + 2 * 3
            )]
            default_field_with_inherited_overrides: u32,

            #[builder(
                setters(doc(default(skip))),
                default = 2 + 2 * 3
            )]
            setters_doc_default_skip: u32,

            #[builder(
                setters(
                    doc(default(skip)),
                    doc {
                        /// Custom docs
                        /// Multiline.
                    }
                ),
                default = 2 + 2 * 3
            )]
            setters_doc_default_skip_and_custom_docs_block: u32,

            #[builder(default = 2 + 2 * 3)]
            setters_doc_default_skip_from_top_level_on: u8,
        }
    });

    let mut actual: syn::File = syn::parse2(actual_tokens.clone()).unwrap();

    // Sanitize the output. Keep only setters and remove their bodies.
    let builder_impl = actual
        .items
        .iter_mut()
        .find_map(|item| match item {
            syn::Item::Impl(impl_item) => {
                (impl_item.self_ty == syn::parse_quote!(SutBuilder<S>)).then(|| impl_item)
            }
            _ => None,
        })
        .unwrap_or_else(|| {
            panic!("No builder impl block found. Generated toekens:\n{actual_tokens}")
        });

    builder_impl.items.retain_mut(|item| match item {
        syn::ImplItem::Fn(fn_item) => {
            if fn_item.sig.ident == "build" {
                return false;
            }

            // Remove noise attributes
            fn_item.attrs.retain(|attr| {
                ["allow", "inline"]
                    .iter()
                    .all(|ident| !attr.path().is_ident(ident))
            });

            fn_item.block = syn::parse_quote!({});

            true
        }
        _ => true,
    });

    assert_snapshot("setters_docs_and_vis", builder_impl);
}

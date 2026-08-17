use proc_macro2::TokenStream;
use quote::quote;

use crate::options::ForwardAttrs;
use crate::util::PathList;

/// Infrastructure for generating an attribute extractor.
pub trait ExtractAttribute {
    /// A set of mutable declarations for all members of the implementing type.
    fn local_declarations(&self) -> TokenStream;

    /// Gets the list of attribute names that should be parsed by the extractor.
    fn attr_names(&self) -> &PathList;

    fn forwarded_attrs(&self) -> Option<&ForwardAttrs>;

    /// Gets the name used by the generated impl to return to the `syn` item passed as input.
    fn param_name(&self) -> TokenStream;

    /// Get the tokens to access a borrowed list of attributes where extraction will take place.
    ///
    /// By default, this will be `&#input.attrs` where `#input` is `self.param_name()`.
    fn attrs_accessor(&self) -> TokenStream {
        let input = self.param_name();
        quote!(&#input.attrs)
    }

    /// Gets the core from-meta-item loop that should be used on matching attributes.
    fn core_loop(&self) -> TokenStream;

    /// Generates the main extraction loop.
    fn extractor(&self) -> TokenStream {
        let declarations = self.local_declarations();

        let will_parse_any = !self.attr_names().is_empty();
        let will_fwd_any = self
            .forwarded_attrs()
            .map(|fa| !fa.is_empty())
            .unwrap_or_default();

        if !(will_parse_any || will_fwd_any) {
            return quote! {
                #declarations
            };
        }

        let attrs_accessor = self.attrs_accessor();

        // The block for parsing attributes whose names have been claimed by the target
        // struct. If no attributes were claimed, this is a pass-through.
        let parse_handled = if will_parse_any {
            let attr_names = self.attr_names().to_strings();
            let core_loop = self.core_loop();
            quote!(
                #(#attr_names)|* => {
                    match ::darling::util::parse_attribute_to_meta_list(__attr) {
                        ::darling::export::Ok(__data) => {
                            if __data.nested.is_empty() {
                                continue;
                            }

                            let __items = &__data.nested;

                            #core_loop
                        }
                        // darling was asked to handle this attribute name, but the actual attribute
                        // isn't one that darling can work with. This either indicates a typing error
                        // or some misunderstanding of the meta attribute syntax; in either case, the
                        // caller should get a useful error.
                        ::darling::export::Err(__err) => {
                            __errors.push(__err);
                        }
                    }
                }
            )
        } else {
            quote!()
        };

        // Specifies the behavior for unhandled attributes. They will either be silently ignored or
        // forwarded to the inner struct for later analysis.
        let forward_unhandled = if will_fwd_any {
            forwards_to_local(self.forwarded_attrs().unwrap())
        } else {
            quote!(_ => continue)
        };

        quote!(
            #declarations
            use ::darling::ToTokens;
            let mut __fwd_attrs: ::darling::export::Vec<::darling::export::syn::Attribute> = vec![];

            for __attr in #attrs_accessor {
                // Filter attributes based on name
                match  ::darling::export::ToString::to_string(&__attr.path.clone().into_token_stream()).as_str() {
                    #parse_handled
                    #forward_unhandled
                }
            }
        )
    }
}

fn forwards_to_local(behavior: &ForwardAttrs) -> TokenStream {
    let push_command = quote!(__fwd_attrs.push(__attr.clone()));
    match *behavior {
        ForwardAttrs::All => quote!(_ => #push_command),
        ForwardAttrs::Only(ref idents) => {
            let names = idents.to_strings();
            quote!(
                #(#names)|* => #push_command,
                _ => continue,
            )
        }
    }
}

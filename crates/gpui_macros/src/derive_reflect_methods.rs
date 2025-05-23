use proc_macro::TokenStream;
use quote::quote;
use syn::{FnArg, Ident, Item, ItemTrait, ReturnType, TraitItem, Type, parse_macro_input};

pub fn reflect_methods(_args: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as Item);

    match item {
        Item::Trait(trait_item) => generate_reflected_trait(trait_item),
        _ => syn::Error::new_spanned(
            quote!(#item),
            "#[reflect_methods] can only be applied to traits",
        )
        .to_compile_error()
        .into(),
    }
}

fn generate_reflected_trait(trait_item: ItemTrait) -> TokenStream {
    let trait_name = &trait_item.ident;
    let vis = &trait_item.vis;

    // Collect method information for methods of form fn name(self) -> Self or fn name(mut self) -> Self
    let mut method_infos = Vec::new();

    for item in &trait_item.items {
        if let TraitItem::Fn(method) = item {
            let method_name = &method.sig.ident;

            // Check if method has self or mut self receiver
            let has_valid_self_receiver = method
                .sig
                .inputs
                .iter()
                .any(|arg| matches!(arg, FnArg::Receiver(r) if r.reference.is_none()));

            // Check if method returns Self
            let returns_self = match &method.sig.output {
                ReturnType::Type(_, ty) => {
                    matches!(**ty, Type::Path(ref path) if path.path.is_ident("Self"))
                }
                ReturnType::Default => false,
            };

            // Check if method has exactly one parameter (self or mut self)
            let param_count = method.sig.inputs.len();

            // Include methods of form fn name(self) -> Self or fn name(mut self) -> Self
            // This includes methods with default implementations
            if has_valid_self_receiver && returns_self && param_count == 1 {
                method_infos.push(method_name.clone());
            }
        }
    }

    // Generate the reflection module name
    let reflection_mod_name = Ident::new(
        &format!("{}_reflection", trait_name.to_string().to_lowercase()),
        trait_name.span(),
    );

    // Generate wrapper functions for each method
    // These wrappers use type erasure to allow runtime invocation
    let wrapper_functions = method_infos.iter().map(|method_name| {
        let wrapper_name = Ident::new(
            &format!("__wrapper_{}", method_name),
            method_name.span(),
        );
        quote! {
            fn #wrapper_name<T: #trait_name + 'static>(value: Box<dyn std::any::Any>) -> Box<dyn std::any::Any> {
                if let Ok(concrete) = value.downcast::<T>() {
                    Box::new(concrete.#method_name())
                } else {
                    panic!("Type mismatch in reflection wrapper");
                }
            }
        }
    });

    // Generate method info entries
    let method_info_entries = method_infos.iter().map(|method_name| {
        let method_name_str = method_name.to_string();
        let wrapper_name = Ident::new(&format!("__wrapper_{}", method_name), method_name.span());
        quote! {
            MethodInfo {
                name: #method_name_str,
                invoke: #wrapper_name::<T>,
            }
        }
    });

    let method_count = method_infos.len();

    // Generate the complete output
    let output = quote! {
        #trait_item

        /// Implements function reflection
        #vis mod #reflection_mod_name {
            use super::*;
            use std::any::Any;

            /// Type alias for the function pointer that invokes a method
            pub type InvokeFn = fn(Box<dyn Any>) -> Box<dyn Any>;

            /// Information about a reflectable method
            #[derive(Clone, Copy)]
            pub struct MethodInfo {
                /// The name of the method
                pub name: &'static str,
                /// Function pointer to invoke the method
                pub invoke: InvokeFn,
            }

            #(#wrapper_functions)*

            /// Get all reflectable methods for a concrete type implementing the trait
            pub fn methods<T: #trait_name + 'static>() -> [MethodInfo; #method_count] {
                [
                    #(#method_info_entries),*
                ]
            }

            /// Find a method by name for a concrete type implementing the trait
            pub fn find_method<T: #trait_name + 'static>(name: &str) -> Option<MethodInfo> {
                methods::<T>().into_iter().find(|m| m.name == name)
            }

            /// Invoke a method by name on a value
            ///
            /// Returns `Some(result)` if the method exists and was successfully invoked,
            /// or `None` if the method was not found.
            ///
            /// # Panics
            ///
            /// Panics if the type erasure fails (this should not happen with correct usage).
            pub fn invoke_method<T: #trait_name + 'static>(name: &str, value: T) -> Option<T> {
                if let Some(method) = find_method::<T>(name) {
                    let boxed = Box::new(value) as Box<dyn Any>;
                    let result = (method.invoke)(boxed);
                    result.downcast::<T>().ok().map(|b| *b)
                } else {
                    None
                }
            }
        }
    };

    TokenStream::from(output)
}

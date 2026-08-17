use super::*;

impl Config<'_> {
    pub fn write_core(&self) -> TokenStream {
        if self.sys {
            if self.package || !self.no_deps {
                quote! { windows_sys::core:: }
            } else if self.flat {
                quote! {}
            } else {
                let mut tokens = TokenStream::new();

                for _ in 0..self.namespace.split('.').count() {
                    tokens.push_str("super::");
                }

                tokens
            }
        } else {
            quote! { windows_core:: }
        }
    }

    pub fn write_generic_phantoms(&self, generics: &[Type]) -> TokenStream {
        if generics.is_empty() {
            quote! {}
        } else {
            let generics = generics.iter().map(|ty| ty.write_name(self));
            quote! { #(core::marker::PhantomData::<#generics>),* }
        }
    }

    pub fn write_generic_named_phantoms(&self, generics: &[Type]) -> TokenStream {
        if generics.is_empty() {
            quote! {}
        } else {
            let generics = generics.iter().map(|ty| ty.write_name(self));
            quote! { #(#generics: core::marker::PhantomData::<#generics>),* }
        }
    }

    pub fn write_generic_constraints(&self, generics: &[Type]) -> TokenStream {
        if generics.is_empty() {
            quote! {}
        } else {
            let generics = generics.iter().map(|ty| ty.write_name(self));
            quote! { #(#generics: windows_core::RuntimeType + 'static,)* }
        }
    }

    pub fn write_namespace(&self, type_name: TypeName) -> TokenStream {
        let mut tokens = TokenStream::new();

        if type_name.namespace().is_empty() {
            return tokens;
        }

        if let Some(reference) = {
            if self.types.contains_key(&type_name) {
                None
            } else {
                self.references.contains(type_name)
            }
        } {
            tokens.push_str(&reference.name);
            tokens.push_str("::");

            if reference.style == ReferenceStyle::Flat {
                return tokens;
            }

            let mut namespace = type_name.namespace().split('.').peekable();

            if reference.style == ReferenceStyle::SkipRoot {
                namespace.next();
            }

            for namespace in namespace {
                tokens.push_str(namespace);
                tokens.push_str("::");
            }

            tokens
        } else {
            if self.flat || type_name.namespace() == self.namespace {
                return tokens;
            }

            let mut relative = self.namespace.split('.').peekable();
            let mut namespace = type_name.namespace().split('.').peekable();

            while relative.peek() == namespace.peek() {
                if relative.next().is_none() {
                    break;
                }

                namespace.next();
            }

            for _ in 0..relative.count() {
                tokens.push_str("super::");
            }

            for namespace in namespace {
                tokens.push_str(namespace);
                tokens.push_str("::");
            }

            tokens
        }
    }
}

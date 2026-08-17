use crate::syntax::Receiver;
use proc_macro2::TokenStream;
use quote::{quote_spanned, ToTokens};
use syn::Token;

pub(crate) struct ReceiverType<'a>(&'a Receiver);
pub(crate) struct ReceiverTypeSelf<'a>(&'a Receiver);

impl Receiver {
    // &TheType
    pub(crate) fn ty(&self) -> ReceiverType {
        ReceiverType(self)
    }

    // &Self
    pub(crate) fn ty_self(&self) -> ReceiverTypeSelf {
        ReceiverTypeSelf(self)
    }
}

impl ToTokens for ReceiverType<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Receiver {
            pinned: _,
            ampersand,
            lifetime,
            mutable: _,
            var: _,
            colon_token: _,
            ty,
            shorthand: _,
            pin_tokens,
            mutability,
        } = &self.0;
        if let Some((pin, langle, _rangle)) = pin_tokens {
            tokens.extend(quote_spanned!(pin.span=> ::cxx::core::pin::Pin));
            langle.to_tokens(tokens);
        }
        ampersand.to_tokens(tokens);
        lifetime.to_tokens(tokens);
        mutability.to_tokens(tokens);
        ty.to_tokens(tokens);
        if let Some((_pin, _langle, rangle)) = pin_tokens {
            rangle.to_tokens(tokens);
        }
    }
}

impl ToTokens for ReceiverTypeSelf<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Receiver {
            pinned: _,
            ampersand,
            lifetime,
            mutable: _,
            var: _,
            colon_token: _,
            ty,
            shorthand: _,
            pin_tokens,
            mutability,
        } = &self.0;
        if let Some((pin, langle, _rangle)) = pin_tokens {
            tokens.extend(quote_spanned!(pin.span=> ::cxx::core::pin::Pin));
            langle.to_tokens(tokens);
        }
        ampersand.to_tokens(tokens);
        lifetime.to_tokens(tokens);
        mutability.to_tokens(tokens);
        Token![Self](ty.rust.span()).to_tokens(tokens);
        if let Some((_pin, _langle, rangle)) = pin_tokens {
            rangle.to_tokens(tokens);
        }
    }
}

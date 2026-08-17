use std::borrow::Cow;
use std::rc::Rc;

use super::{Literal, TokenStream};

/// Types that can be interpolated inside a `quote!` invocation.
///
/// [`quote!`]: macro.quote.html
pub trait ToTokens {
    /// Write `self` to the given `TokenStream`.
    fn to_tokens(&self, tokens: &mut TokenStream);

    /// Convert `self` directly into a `TokenStream` object.
    ///
    /// This method is implicitly implemented using `to_tokens`, and acts as a
    /// convenience method for consumers of the `ToTokens` trait.
    fn to_token_stream(&self) -> TokenStream {
        let mut tokens = TokenStream::new();
        self.to_tokens(&mut tokens);
        tokens
    }

    /// Convert `self` directly into a `TokenStream` object.
    ///
    /// This method is implicitly implemented using `to_tokens`, and acts as a
    /// convenience method for consumers of the `ToTokens` trait.
    fn into_token_stream(self) -> TokenStream
    where
        Self: Sized,
    {
        self.to_token_stream()
    }
}

impl<T: ?Sized + ToTokens> ToTokens for &T {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        (**self).to_tokens(tokens);
    }
}

impl<T: ?Sized + ToTokens> ToTokens for &mut T {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        (**self).to_tokens(tokens);
    }
}

impl<T: ?Sized + ToOwned + ToTokens> ToTokens for Cow<'_, T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        (**self).to_tokens(tokens);
    }
}

impl<T: ?Sized + ToTokens> ToTokens for Box<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        (**self).to_tokens(tokens);
    }
}

impl<T: ?Sized + ToTokens> ToTokens for Rc<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        (**self).to_tokens(tokens);
    }
}

impl<T: ToTokens> ToTokens for Option<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if let Some(ref t) = *self {
            t.to_tokens(tokens);
        }
    }
}

impl ToTokens for str {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.push_str(" \"");
        tokens.push_str(self);
        tokens.push('"');
    }
}

impl ToTokens for String {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.as_str().to_tokens(tokens);
    }
}

macro_rules! primitive {
    ($($t:ident => $name:ident)*) => ($(
        impl ToTokens for $t {
            fn to_tokens(&self, tokens: &mut TokenStream) {
                tokens.push_space();
                tokens.push_str(&self.to_string());
                tokens.push_str(stringify!($t));
            }
        }
    )*)
}

primitive! {
    i8 => i8_suffixed
    i16 => i16_suffixed
    i32 => i32_suffixed
    i64 => i64_suffixed
    i128 => i128_suffixed
    isize => isize_suffixed

    u8 => u8_suffixed
    u16 => u16_suffixed
    u32 => u32_suffixed
    u64 => u64_suffixed
    u128 => u128_suffixed
    usize => usize_suffixed

    f32 => f32_suffixed
    f64 => f64_suffixed
}

impl ToTokens for char {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.push_space();
        tokens.push('\'');
        tokens.push(*self);
        tokens.push('\'');
    }
}

impl ToTokens for bool {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let word = if *self { "true" } else { "false" };
        tokens.push_space();
        tokens.push_str(word);
    }
}

impl ToTokens for Literal {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.push_str(self.as_str());
    }
}

impl ToTokens for TokenStream {
    fn to_tokens(&self, dst: &mut TokenStream) {
        dst.combine(self);
    }

    fn into_token_stream(self) -> TokenStream {
        self
    }
}

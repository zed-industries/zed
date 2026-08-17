use proc_macro2::TokenStream;
use quote::ToTokens;
use std::fmt::{self, Display};

pub(crate) struct Message(String);

impl Message {
    pub fn new() -> Self {
        Message(String::new())
    }

    pub fn write_fmt(&mut self, args: fmt::Arguments) {
        fmt::Write::write_fmt(&mut self.0, args).unwrap();
    }
}

impl Display for Message {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl ToTokens for Message {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);
    }
}

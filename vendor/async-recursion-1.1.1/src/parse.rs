use proc_macro2::Span;
use syn::{
    parse::{Error, Parse, ParseStream, Result},
    token::Question,
    ItemFn, Token,
};

pub struct AsyncItem(pub ItemFn);

impl Parse for AsyncItem {
    fn parse(input: ParseStream) -> Result<Self> {
        let item: ItemFn = input.parse()?;

        // Check that this is an async function
        if item.sig.asyncness.is_none() {
            return Err(Error::new(Span::call_site(), "expected an async function"));
        }

        Ok(AsyncItem(item))
    }
}

pub struct RecursionArgs {
    pub send_bound: bool,
    pub sync_bound: bool,
}

/// Custom keywords for parser
mod kw {
    syn::custom_keyword!(Send);
    syn::custom_keyword!(Sync);
}

#[derive(Debug, PartialEq, Eq)]
enum Arg {
    NotSend,
    Sync,
}

impl std::fmt::Display for Arg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotSend => write!(f, "?Send"),
            Self::Sync => write!(f, "Sync"),
        }
    }
}

impl Parse for Arg {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![?]) {
            input.parse::<Question>()?;
            input.parse::<kw::Send>()?;
            Ok(Arg::NotSend)
        } else {
            input.parse::<kw::Sync>()?;
            Ok(Arg::Sync)
        }
    }
}

impl Parse for RecursionArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut send_bound: bool = true;
        let mut sync_bound: bool = false;

        let args_parsed: Vec<Arg> =
            syn::punctuated::Punctuated::<Arg, syn::Token![,]>::parse_terminated(input)
                .map_err(|e| input.error(format!("failed to parse macro arguments: {e}")))?
                .into_iter()
                .collect();

        // Avoid sloppy input
        if args_parsed.len() > 2 {
            return Err(Error::new(Span::call_site(), "received too many arguments"));
        } else if args_parsed.len() == 2 && args_parsed[0] == args_parsed[1] {
            return Err(Error::new(
                Span::call_site(),
                format!("received duplicate argument: `{}`", args_parsed[0]),
            ));
        }

        for arg in args_parsed {
            match arg {
                Arg::NotSend => send_bound = false,
                Arg::Sync => sync_bound = true,
            }
        }

        Ok(Self {
            send_bound,
            sync_bound,
        })
    }
}

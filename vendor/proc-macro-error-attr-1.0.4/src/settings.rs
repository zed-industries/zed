use crate::{Error, Result};
use proc_macro2::{Ident, Span, TokenStream, TokenTree};

macro_rules! decl_settings {
    ($($val:expr => $variant:ident),+ $(,)*) => {
        #[derive(PartialEq)]
        pub(crate) enum Setting {
            $($variant),*
        }

        fn ident_to_setting(ident: Ident) -> Result<Setting> {
            match &*ident.to_string() {
                $($val => Ok(Setting::$variant),)*
                _ => {
                    let possible_vals = [$($val),*]
                        .iter()
                        .map(|v| format!("`{}`", v))
                        .collect::<Vec<_>>()
                        .join(", ");

                    Err(Error::new(
                        ident.span(),
                        format!("unknown setting `{}`, expected one of {}", ident, possible_vals)))
                }
            }
        }
    };
}

decl_settings! {
    "assert_unwind_safe" => AssertUnwindSafe,
    "allow_not_macro"    => AllowNotMacro,
    "proc_macro_hack"    => ProcMacroHack,
}

pub(crate) fn parse_settings(input: TokenStream) -> Result<Settings> {
    let mut input = input.into_iter();
    let mut res = Settings(Vec::new());
    loop {
        match input.next() {
            Some(TokenTree::Ident(ident)) => {
                res.0.push(ident_to_setting(ident)?);
            }
            None => return Ok(res),
            other => {
                let span = other.map_or(Span::call_site(), |tt| tt.span());
                return Err(Error::new(span, "expected identifier".to_string()));
            }
        }

        match input.next() {
            Some(TokenTree::Punct(ref punct)) if punct.as_char() == ',' => {}
            None => return Ok(res),
            other => {
                let span = other.map_or(Span::call_site(), |tt| tt.span());
                return Err(Error::new(span, "expected `,`".to_string()));
            }
        }
    }
}

pub(crate) struct Settings(Vec<Setting>);

impl Settings {
    pub(crate) fn is_set(&self, setting: Setting) -> bool {
        self.0.iter().any(|s| *s == setting)
    }

    pub(crate) fn set(&mut self, setting: Setting) {
        self.0.push(setting)
    }
}

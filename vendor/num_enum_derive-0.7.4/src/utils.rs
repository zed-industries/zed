macro_rules! die {
    ($spanned:expr=>
        $msg:expr
    ) => {
        return Err(::syn::Error::new_spanned($spanned, $msg))
    };

    (
        $msg:expr
    ) => {
        return Err(::syn::Error::new(::proc_macro2::Span::call_site(), $msg))
    };
}

pub(crate) use die;

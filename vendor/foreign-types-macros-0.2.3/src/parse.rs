use syn::parse::{self, Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token;
use syn::{braced, Attribute, Expr, Generics, Ident, Path, Token, Type, Visibility};

mod kw {
    syn::custom_keyword!(Sync);
    syn::custom_keyword!(Send);
    syn::custom_keyword!(PhantomData);
    syn::custom_keyword!(CType);
    syn::custom_keyword!(drop);
    syn::custom_keyword!(clone);
}

pub struct Input {
    pub crate_: Path,
    pub types: Vec<ForeignType>,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> parse::Result<Input> {
        let crate_ = input.parse()?;
        let mut types = vec![];
        while !input.is_empty() {
            types.push(input.parse()?);
        }

        Ok(Input { crate_, types })
    }
}

pub struct ForeignType {
    pub attrs: Vec<Attribute>,
    pub visibility: Visibility,
    pub name: Ident,
    pub generics: Generics,
    pub oibits: Punctuated<Ident, Token![+]>,
    pub phantom_data: Option<Type>,
    pub ctype: Type,
    pub drop: Expr,
    pub clone: Option<Expr>,
}

impl Parse for ForeignType {
    fn parse(input: ParseStream) -> parse::Result<ForeignType> {
        let attrs = input.call(Attribute::parse_outer)?;
        let visibility = input.parse()?;
        input.parse::<Token![unsafe]>()?;
        input.parse::<Token![type]>()?;
        let name = input.parse()?;
        let generics = input.parse()?;
        let oibits = input.call(parse_oibits)?;
        let inner;
        braced!(inner in input);
        let ctype = inner.call(parse_type::<kw::CType>)?;
        let phantom_data = inner.call(parse_phantom_data)?;
        let drop = inner.call(parse_fn::<kw::drop>)?;
        let clone = inner.call(parse_clone)?;

        Ok(ForeignType {
            attrs,
            visibility,
            name,
            generics,
            oibits,
            ctype,
            phantom_data,
            drop,
            clone,
        })
    }
}

fn parse_oibit(input: ParseStream) -> parse::Result<Ident> {
    let lookahead = input.lookahead1();
    if lookahead.peek(kw::Sync) || lookahead.peek(kw::Send) {
        input.parse()
    } else {
        Err(lookahead.error())
    }
}

fn parse_oibits(input: ParseStream) -> parse::Result<Punctuated<Ident, Token![+]>> {
    let mut out = Punctuated::new();

    if input.parse::<Option<Token![:]>>()?.is_some() {
        loop {
            out.push_value(input.call(parse_oibit)?);
            if input.peek(token::Brace) {
                break;
            }
            out.push_punct(input.parse()?);
            if input.peek(token::Brace) {
                break;
            }
        }
    }

    Ok(out)
}

fn parse_type<T>(input: ParseStream) -> parse::Result<Type>
where
    T: Parse,
{
    input.parse::<Token![type]>()?;
    input.parse::<T>()?;
    input.parse::<Token![=]>()?;
    let type_ = input.parse()?;
    input.parse::<Token![;]>()?;
    Ok(type_)
}

fn parse_phantom_data(input: ParseStream) -> parse::Result<Option<Type>> {
    if input.peek(Token![type]) && input.peek2(kw::PhantomData) {
        input.call(parse_type::<kw::PhantomData>).map(Some)
    } else {
        Ok(None)
    }
}

fn parse_fn<T>(input: ParseStream) -> parse::Result<Expr>
where
    T: Parse,
{
    input.parse::<Token![fn]>()?;
    input.parse::<T>()?;
    input.parse::<Token![=]>()?;
    let path = input.parse()?;
    input.parse::<Token![;]>()?;
    Ok(path)
}

fn parse_clone(input: ParseStream) -> parse::Result<Option<Expr>> {
    if input.peek(Token![fn]) && input.peek2(kw::clone) {
        input.call(parse_fn::<kw::clone>).map(Some)
    } else {
        Ok(None)
    }
}

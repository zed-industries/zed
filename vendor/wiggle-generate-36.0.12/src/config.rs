use {
    proc_macro2::{Span, TokenStream},
    std::{collections::HashMap, path::PathBuf},
    syn::{
        Error, Ident, LitStr, Result, Token, braced, bracketed,
        parse::{Parse, ParseStream},
        punctuated::Punctuated,
    },
};

#[derive(Debug, Clone)]
pub struct Config {
    pub witx: WitxConf,
    pub errors: ErrorConf,
    pub async_: AsyncConf,
    pub wasmtime: bool,
    pub tracing: TracingConf,
    pub mutable: bool,
}

mod kw {
    syn::custom_keyword!(witx);
    syn::custom_keyword!(witx_literal);
    syn::custom_keyword!(block_on);
    syn::custom_keyword!(errors);
    syn::custom_keyword!(target);
    syn::custom_keyword!(wasmtime);
    syn::custom_keyword!(mutable);
    syn::custom_keyword!(tracing);
    syn::custom_keyword!(disable_for);
    syn::custom_keyword!(trappable);
}

#[derive(Debug, Clone)]
pub enum ConfigField {
    Witx(WitxConf),
    Error(ErrorConf),
    Async(AsyncConf),
    Wasmtime(bool),
    Tracing(TracingConf),
    Mutable(bool),
}

impl Parse for ConfigField {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::witx) {
            input.parse::<kw::witx>()?;
            input.parse::<Token![:]>()?;
            Ok(ConfigField::Witx(WitxConf::Paths(input.parse()?)))
        } else if lookahead.peek(kw::witx_literal) {
            input.parse::<kw::witx_literal>()?;
            input.parse::<Token![:]>()?;
            Ok(ConfigField::Witx(WitxConf::Literal(input.parse()?)))
        } else if lookahead.peek(kw::errors) {
            input.parse::<kw::errors>()?;
            input.parse::<Token![:]>()?;
            Ok(ConfigField::Error(input.parse()?))
        } else if lookahead.peek(Token![async]) {
            input.parse::<Token![async]>()?;
            input.parse::<Token![:]>()?;
            Ok(ConfigField::Async(AsyncConf {
                block_with: None,
                functions: input.parse()?,
            }))
        } else if lookahead.peek(kw::block_on) {
            input.parse::<kw::block_on>()?;
            let block_with = if input.peek(syn::token::Bracket) {
                let content;
                let _ = bracketed!(content in input);
                content.parse()?
            } else {
                quote::quote!(wiggle::run_in_dummy_executor)
            };
            input.parse::<Token![:]>()?;
            Ok(ConfigField::Async(AsyncConf {
                block_with: Some(block_with),
                functions: input.parse()?,
            }))
        } else if lookahead.peek(kw::wasmtime) {
            input.parse::<kw::wasmtime>()?;
            input.parse::<Token![:]>()?;
            Ok(ConfigField::Wasmtime(input.parse::<syn::LitBool>()?.value))
        } else if lookahead.peek(kw::tracing) {
            input.parse::<kw::tracing>()?;
            input.parse::<Token![:]>()?;
            Ok(ConfigField::Tracing(input.parse()?))
        } else if lookahead.peek(kw::mutable) {
            input.parse::<kw::mutable>()?;
            input.parse::<Token![:]>()?;
            Ok(ConfigField::Mutable(input.parse::<syn::LitBool>()?.value))
        } else {
            Err(lookahead.error())
        }
    }
}

impl Config {
    pub fn build(fields: impl Iterator<Item = ConfigField>, err_loc: Span) -> Result<Self> {
        let mut witx = None;
        let mut errors = None;
        let mut async_ = None;
        let mut wasmtime = None;
        let mut tracing = None;
        let mut mutable = None;
        for f in fields {
            match f {
                ConfigField::Witx(c) => {
                    if witx.is_some() {
                        return Err(Error::new(err_loc, "duplicate `witx` field"));
                    }
                    witx = Some(c);
                }
                ConfigField::Error(c) => {
                    if errors.is_some() {
                        return Err(Error::new(err_loc, "duplicate `errors` field"));
                    }
                    errors = Some(c);
                }
                ConfigField::Async(c) => {
                    if async_.is_some() {
                        return Err(Error::new(err_loc, "duplicate `async` field"));
                    }
                    async_ = Some(c);
                }
                ConfigField::Wasmtime(c) => {
                    if wasmtime.is_some() {
                        return Err(Error::new(err_loc, "duplicate `wasmtime` field"));
                    }
                    wasmtime = Some(c);
                }
                ConfigField::Tracing(c) => {
                    if tracing.is_some() {
                        return Err(Error::new(err_loc, "duplicate `tracing` field"));
                    }
                    tracing = Some(c);
                }
                ConfigField::Mutable(c) => {
                    if mutable.is_some() {
                        return Err(Error::new(err_loc, "duplicate `mutable` field"));
                    }
                    mutable = Some(c);
                }
            }
        }
        Ok(Config {
            witx: witx
                .take()
                .ok_or_else(|| Error::new(err_loc, "`witx` field required"))?,
            errors: errors.take().unwrap_or_default(),
            async_: async_.take().unwrap_or_default(),
            wasmtime: wasmtime.unwrap_or(true),
            tracing: tracing.unwrap_or_default(),
            mutable: mutable.unwrap_or(true),
        })
    }

    /// Load the `witx` document for the configuration.
    ///
    /// # Panics
    ///
    /// This method will panic if the paths given in the `witx` field were not valid documents.
    pub fn load_document(&self) -> witx::Document {
        self.witx.load_document()
    }
}

impl Parse for Config {
    fn parse(input: ParseStream) -> Result<Self> {
        let contents;
        let _lbrace = braced!(contents in input);
        let fields: Punctuated<ConfigField, Token![,]> =
            contents.parse_terminated(ConfigField::parse, Token![,])?;
        Ok(Config::build(fields.into_iter(), input.span())?)
    }
}

/// The witx document(s) that will be loaded from a [`Config`](struct.Config.html).
///
/// A witx interface definition can be provided either as a collection of relative paths to
/// documents, or as a single inlined string literal. Note that `(use ...)` directives are not
/// permitted when providing a string literal.
#[derive(Debug, Clone)]
pub enum WitxConf {
    /// A collection of paths pointing to witx files.
    Paths(Paths),
    /// A single witx document, provided as a string literal.
    Literal(Literal),
}

impl WitxConf {
    /// Load the `witx` document.
    ///
    /// # Panics
    ///
    /// This method will panic if the paths given in the `witx` field were not valid documents, or
    /// if any of the given documents were not syntactically valid.
    pub fn load_document(&self) -> witx::Document {
        match self {
            Self::Paths(paths) => witx::load(paths.as_ref()).expect("loading witx"),
            Self::Literal(doc) => witx::parse(doc.as_ref()).expect("parsing witx"),
        }
    }
}

/// A collection of paths, pointing to witx documents.
#[derive(Debug, Clone)]
pub struct Paths(Vec<PathBuf>);

impl Paths {
    /// Create a new, empty collection of paths.
    pub fn new() -> Self {
        Default::default()
    }
}

impl Default for Paths {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl AsRef<[PathBuf]> for Paths {
    fn as_ref(&self) -> &[PathBuf] {
        self.0.as_ref()
    }
}

impl AsMut<[PathBuf]> for Paths {
    fn as_mut(&mut self) -> &mut [PathBuf] {
        self.0.as_mut()
    }
}

impl FromIterator<PathBuf> for Paths {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = PathBuf>,
    {
        Self(iter.into_iter().collect())
    }
}

impl Parse for Paths {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let _ = bracketed!(content in input);
        let path_lits: Punctuated<LitStr, Token![,]> =
            content.parse_terminated(Parse::parse, Token![,])?;

        let expanded_paths = path_lits
            .iter()
            .map(|lit| {
                PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join(lit.value())
            })
            .collect::<Vec<PathBuf>>();

        Ok(Paths(expanded_paths))
    }
}

/// A single witx document, provided as a string literal.
#[derive(Debug, Clone)]
pub struct Literal(String);

impl AsRef<str> for Literal {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Parse for Literal {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self(input.parse::<syn::LitStr>()?.value()))
    }
}

#[derive(Clone, Default, Debug)]
/// Map from abi error type to rich error type
pub struct ErrorConf(HashMap<Ident, ErrorConfField>);

impl ErrorConf {
    pub fn iter(&self) -> impl Iterator<Item = (&Ident, &ErrorConfField)> {
        self.0.iter()
    }
}

impl Parse for ErrorConf {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let _ = braced!(content in input);
        let items: Punctuated<ErrorConfField, Token![,]> =
            content.parse_terminated(Parse::parse, Token![,])?;
        let mut m = HashMap::new();
        for i in items {
            match m.insert(i.abi_error().clone(), i.clone()) {
                None => {}
                Some(prev_def) => {
                    return Err(Error::new(
                        *i.err_loc(),
                        format!(
                            "duplicate definition of rich error type for {:?}: previously defined at {:?}",
                            i.abi_error(),
                            prev_def.err_loc(),
                        ),
                    ));
                }
            }
        }
        Ok(ErrorConf(m))
    }
}

#[derive(Debug, Clone)]
pub enum ErrorConfField {
    Trappable(TrappableErrorConfField),
    User(UserErrorConfField),
}
impl ErrorConfField {
    pub fn abi_error(&self) -> &Ident {
        match self {
            Self::Trappable(t) => &t.abi_error,
            Self::User(u) => &u.abi_error,
        }
    }
    pub fn err_loc(&self) -> &Span {
        match self {
            Self::Trappable(t) => &t.err_loc,
            Self::User(u) => &u.err_loc,
        }
    }
}

impl Parse for ErrorConfField {
    fn parse(input: ParseStream) -> Result<Self> {
        let err_loc = input.span();
        let abi_error = input.parse::<Ident>()?;
        let _arrow: Token![=>] = input.parse()?;

        let lookahead = input.lookahead1();
        if lookahead.peek(kw::trappable) {
            let _ = input.parse::<kw::trappable>()?;
            let rich_error = input.parse()?;
            Ok(ErrorConfField::Trappable(TrappableErrorConfField {
                abi_error,
                rich_error,
                err_loc,
            }))
        } else {
            let rich_error = input.parse::<syn::Path>()?;
            Ok(ErrorConfField::User(UserErrorConfField {
                abi_error,
                rich_error,
                err_loc,
            }))
        }
    }
}

#[derive(Clone, Debug)]
pub struct TrappableErrorConfField {
    pub abi_error: Ident,
    pub rich_error: Ident,
    pub err_loc: Span,
}

#[derive(Clone)]
pub struct UserErrorConfField {
    pub abi_error: Ident,
    pub rich_error: syn::Path,
    pub err_loc: Span,
}

impl std::fmt::Debug for UserErrorConfField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ErrorConfField")
            .field("abi_error", &self.abi_error)
            .field("rich_error", &"(...)")
            .field("err_loc", &self.err_loc)
            .finish()
    }
}

#[derive(Clone, Default, Debug)]
/// Modules and funcs that have async signatures
pub struct AsyncConf {
    block_with: Option<TokenStream>,
    functions: AsyncFunctions,
}

#[derive(Clone, Debug)]
pub enum Asyncness {
    /// Wiggle function is synchronous, wasmtime Func is synchronous
    Sync,
    /// Wiggle function is asynchronous, but wasmtime Func is synchronous
    Blocking { block_with: TokenStream },
    /// Wiggle function and wasmtime Func are asynchronous.
    Async,
}

impl Asyncness {
    pub fn is_async(&self) -> bool {
        match self {
            Self::Async => true,
            _ => false,
        }
    }
    pub fn blocking(&self) -> Option<&TokenStream> {
        match self {
            Self::Blocking { block_with } => Some(block_with),
            _ => None,
        }
    }
    pub fn is_sync(&self) -> bool {
        match self {
            Self::Sync => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug)]
pub enum AsyncFunctions {
    Some(HashMap<String, Vec<String>>),
    All,
}
impl Default for AsyncFunctions {
    fn default() -> Self {
        AsyncFunctions::Some(HashMap::default())
    }
}

impl AsyncConf {
    pub fn get(&self, module: &str, function: &str) -> Asyncness {
        let a = match &self.block_with {
            Some(block_with) => Asyncness::Blocking {
                block_with: block_with.clone(),
            },
            None => Asyncness::Async,
        };
        match &self.functions {
            AsyncFunctions::Some(fs) => {
                if fs
                    .get(module)
                    .and_then(|fs| fs.iter().find(|f| *f == function))
                    .is_some()
                {
                    a
                } else {
                    Asyncness::Sync
                }
            }
            AsyncFunctions::All => a,
        }
    }

    pub fn contains_async(&self, module: &witx::Module) -> bool {
        for f in module.funcs() {
            if self.get(module.name.as_str(), f.name.as_str()).is_async() {
                return true;
            }
        }
        false
    }
}

impl Parse for AsyncFunctions {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let lookahead = input.lookahead1();
        if lookahead.peek(syn::token::Brace) {
            let _ = braced!(content in input);
            let items: Punctuated<FunctionField, Token![,]> =
                content.parse_terminated(Parse::parse, Token![,])?;
            let mut functions: HashMap<String, Vec<String>> = HashMap::new();
            use std::collections::hash_map::Entry;
            for i in items {
                let function_names = i
                    .function_names
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>();
                match functions.entry(i.module_name.to_string()) {
                    Entry::Occupied(o) => o.into_mut().extend(function_names),
                    Entry::Vacant(v) => {
                        v.insert(function_names);
                    }
                }
            }
            Ok(AsyncFunctions::Some(functions))
        } else if lookahead.peek(Token![*]) {
            let _: Token![*] = input.parse().unwrap();
            Ok(AsyncFunctions::All)
        } else {
            Err(lookahead.error())
        }
    }
}

#[derive(Clone)]
pub struct FunctionField {
    pub module_name: Ident,
    pub function_names: Vec<Ident>,
    pub err_loc: Span,
}

impl Parse for FunctionField {
    fn parse(input: ParseStream) -> Result<Self> {
        let err_loc = input.span();
        let module_name = input.parse::<Ident>()?;
        let _doublecolon: Token![::] = input.parse()?;
        let lookahead = input.lookahead1();
        if lookahead.peek(syn::token::Brace) {
            let content;
            let _ = braced!(content in input);
            let function_names: Punctuated<Ident, Token![,]> =
                content.parse_terminated(Parse::parse, Token![,])?;
            Ok(FunctionField {
                module_name,
                function_names: function_names.iter().cloned().collect(),
                err_loc,
            })
        } else if lookahead.peek(Ident) {
            let name = input.parse()?;
            Ok(FunctionField {
                module_name,
                function_names: vec![name],
                err_loc,
            })
        } else {
            Err(lookahead.error())
        }
    }
}

#[derive(Clone)]
pub struct WasmtimeConfig {
    pub c: Config,
    pub target: syn::Path,
}

#[derive(Clone)]
pub enum WasmtimeConfigField {
    Core(ConfigField),
    Target(syn::Path),
}
impl WasmtimeConfig {
    pub fn build(fields: impl Iterator<Item = WasmtimeConfigField>, err_loc: Span) -> Result<Self> {
        let mut target = None;
        let mut cs = Vec::new();
        for f in fields {
            match f {
                WasmtimeConfigField::Target(c) => {
                    if target.is_some() {
                        return Err(Error::new(err_loc, "duplicate `target` field"));
                    }
                    target = Some(c);
                }
                WasmtimeConfigField::Core(c) => cs.push(c),
            }
        }
        let c = Config::build(cs.into_iter(), err_loc)?;
        Ok(WasmtimeConfig {
            c,
            target: target
                .take()
                .ok_or_else(|| Error::new(err_loc, "`target` field required"))?,
        })
    }
}

impl Parse for WasmtimeConfig {
    fn parse(input: ParseStream) -> Result<Self> {
        let contents;
        let _lbrace = braced!(contents in input);
        let fields: Punctuated<WasmtimeConfigField, Token![,]> =
            contents.parse_terminated(WasmtimeConfigField::parse, Token![,])?;
        Ok(WasmtimeConfig::build(fields.into_iter(), input.span())?)
    }
}

impl Parse for WasmtimeConfigField {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(kw::target) {
            input.parse::<kw::target>()?;
            input.parse::<Token![:]>()?;
            Ok(WasmtimeConfigField::Target(input.parse()?))
        } else {
            Ok(WasmtimeConfigField::Core(input.parse()?))
        }
    }
}

#[derive(Clone, Debug)]
pub struct TracingConf {
    enabled: bool,
    excluded_functions: HashMap<String, Vec<String>>,
}

impl TracingConf {
    pub fn enabled_for(&self, module: &str, function: &str) -> bool {
        if !self.enabled {
            return false;
        }
        self.excluded_functions
            .get(module)
            .and_then(|fs| fs.iter().find(|f| *f == function))
            .is_none()
    }
}

impl Default for TracingConf {
    fn default() -> Self {
        Self {
            enabled: true,
            excluded_functions: HashMap::new(),
        }
    }
}

impl Parse for TracingConf {
    fn parse(input: ParseStream) -> Result<Self> {
        let enabled = input.parse::<syn::LitBool>()?.value;

        let lookahead = input.lookahead1();
        if lookahead.peek(kw::disable_for) {
            input.parse::<kw::disable_for>()?;
            let content;
            let _ = braced!(content in input);
            let items: Punctuated<FunctionField, Token![,]> =
                content.parse_terminated(Parse::parse, Token![,])?;
            let mut functions: HashMap<String, Vec<String>> = HashMap::new();
            use std::collections::hash_map::Entry;
            for i in items {
                let function_names = i
                    .function_names
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>();
                match functions.entry(i.module_name.to_string()) {
                    Entry::Occupied(o) => o.into_mut().extend(function_names),
                    Entry::Vacant(v) => {
                        v.insert(function_names);
                    }
                }
            }

            Ok(TracingConf {
                enabled,
                excluded_functions: functions,
            })
        } else {
            Ok(TracingConf {
                enabled,
                excluded_functions: HashMap::new(),
            })
        }
    }
}

// Functionality that is shared between the cxxbridge macro and the cmd.

pub(crate) mod atom;
pub(crate) mod attrs;
pub(crate) mod cfg;
pub(crate) mod check;
pub(crate) mod derive;
pub(crate) mod discriminant;
mod doc;
pub(crate) mod error;
pub(crate) mod file;
pub(crate) mod ident;
mod impls;
mod improper;
pub(crate) mod instantiate;
pub(crate) mod mangle;
pub(crate) mod map;
pub(crate) mod message;
mod names;
pub(crate) mod namespace;
mod parse;
mod pod;
pub(crate) mod primitive;
pub(crate) mod qualified;
pub(crate) mod query;
pub(crate) mod report;
pub(crate) mod repr;
pub(crate) mod resolve;
pub(crate) mod set;
mod signature;
pub(crate) mod symbol;
mod tokens;
mod toposort;
pub(crate) mod trivial;
pub(crate) mod types;
pub(crate) mod unpin;
mod visit;

use self::attrs::OtherAttrs;
use self::cfg::CfgExpr;
use self::namespace::Namespace;
use self::parse::kw;
use self::symbol::Symbol;
use proc_macro2::{Ident, Span};
use syn::punctuated::Punctuated;
use syn::token::{Brace, Bracket, Paren};
use syn::{Expr, Generics, Lifetime, LitInt, Token, Type as RustType};

pub(crate) use self::atom::Atom;
pub(crate) use self::derive::{Derive, Trait};
pub(crate) use self::discriminant::Discriminant;
pub(crate) use self::doc::Doc;
pub(crate) use self::names::ForeignName;
pub(crate) use self::parse::parse_items;
pub(crate) use self::types::Types;

pub(crate) enum Api {
    #[cfg_attr(proc_macro, expect(dead_code))]
    Include(Include),
    Struct(Struct),
    Enum(Enum),
    CxxType(ExternType),
    CxxFunction(ExternFn),
    RustType(ExternType),
    RustFunction(ExternFn),
    TypeAlias(TypeAlias),
    Impl(Impl),
}

pub(crate) struct Include {
    pub cfg: CfgExpr,
    pub path: String,
    pub kind: IncludeKind,
    #[cfg_attr(proc_macro, expect(dead_code))]
    pub begin_span: Span,
    #[cfg_attr(proc_macro, expect(dead_code))]
    pub end_span: Span,
}

/// Whether to emit `#include "path"` or `#include <path>`.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum IncludeKind {
    /// `#include "quoted/path/to"`
    Quoted,
    /// `#include <bracketed/path/to>`
    Bracketed,
}

pub(crate) struct ExternType {
    #[cfg_attr(proc_macro, expect(dead_code))]
    pub cfg: CfgExpr,
    pub lang: Lang,
    pub doc: Doc,
    pub derives: Vec<Derive>,
    pub attrs: OtherAttrs,
    #[cfg_attr(not(proc_macro), expect(dead_code))]
    pub visibility: Token![pub],
    pub type_token: Token![type],
    pub name: Pair,
    pub generics: Lifetimes,
    #[expect(dead_code)]
    pub colon_token: Option<Token![:]>,
    pub bounds: Vec<Derive>,
    #[cfg_attr(not(proc_macro), expect(dead_code))]
    pub semi_token: Token![;],
    pub trusted: bool,
}

pub(crate) struct Struct {
    pub cfg: CfgExpr,
    pub doc: Doc,
    pub derives: Vec<Derive>,
    pub align: Option<LitInt>,
    pub attrs: OtherAttrs,
    #[cfg_attr(not(proc_macro), expect(dead_code))]
    pub visibility: Token![pub],
    pub struct_token: Token![struct],
    pub name: Pair,
    pub generics: Lifetimes,
    pub brace_token: Brace,
    pub fields: Vec<Var>,
}

pub(crate) struct Enum {
    pub cfg: CfgExpr,
    pub doc: Doc,
    pub derives: Vec<Derive>,
    pub attrs: OtherAttrs,
    #[cfg_attr(not(proc_macro), expect(dead_code))]
    pub visibility: Token![pub],
    pub enum_token: Token![enum],
    pub name: Pair,
    pub generics: Lifetimes,
    pub brace_token: Brace,
    pub variants: Vec<Variant>,
    pub repr: EnumRepr,
    pub explicit_repr: bool,
}

pub(crate) struct EnumRepr {
    pub atom: Atom,
    pub repr_type: Type,
}

pub(crate) struct ExternFn {
    pub cfg: CfgExpr,
    pub lang: Lang,
    pub doc: Doc,
    #[cfg_attr(not(proc_macro), expect(dead_code))]
    pub attrs: OtherAttrs,
    #[cfg_attr(not(proc_macro), expect(dead_code))]
    pub visibility: Token![pub],
    pub name: Pair,
    pub sig: Signature,
    pub semi_token: Token![;],
    pub trusted: bool,
}

pub(crate) struct TypeAlias {
    #[cfg_attr(proc_macro, expect(dead_code))]
    pub cfg: CfgExpr,
    #[cfg_attr(not(proc_macro), expect(dead_code))]
    pub doc: Doc,
    pub derives: Vec<Derive>,
    pub attrs: OtherAttrs,
    #[cfg_attr(not(proc_macro), expect(dead_code))]
    pub visibility: Token![pub],
    pub type_token: Token![type],
    pub name: Pair,
    pub generics: Lifetimes,
    #[cfg_attr(not(proc_macro), expect(dead_code))]
    pub eq_token: Token![=],
    #[cfg_attr(not(proc_macro), expect(dead_code))]
    pub ty: RustType,
    #[cfg_attr(not(proc_macro), expect(dead_code))]
    pub semi_token: Token![;],
}

pub(crate) struct Impl {
    pub cfg: CfgExpr,
    #[expect(dead_code)]
    pub attrs: OtherAttrs,
    pub impl_token: Token![impl],
    pub impl_generics: Lifetimes,
    #[expect(dead_code)]
    pub negative: bool,
    pub ty: Type,
    #[cfg_attr(not(proc_macro), expect(dead_code))]
    pub ty_generics: Lifetimes,
    pub brace_token: Brace,
    pub negative_token: Option<Token![!]>,
}

#[derive(Clone, Default)]
pub(crate) struct Lifetimes {
    pub lt_token: Option<Token![<]>,
    pub lifetimes: Punctuated<Lifetime, Token![,]>,
    pub gt_token: Option<Token![>]>,
}

pub(crate) struct Signature {
    pub asyncness: Option<Token![async]>,
    pub unsafety: Option<Token![unsafe]>,
    pub fn_token: Token![fn],
    pub generics: Generics,
    pub kind: FnKind,
    pub args: Punctuated<Var, Token![,]>,
    pub ret: Option<Type>,
    pub throws: bool,
    pub paren_token: Paren,
    pub throws_tokens: Option<(kw::Result, Token![<], Token![>])>,
}

#[derive(PartialEq, Hash)]
pub(crate) enum FnKind {
    /// Rust method or C++ non-static member function.
    Method(Receiver),
    /// Rust associated function or C++ static member function.
    Assoc(Ident),
    /// Non-member function.
    Free,
}

pub(crate) struct Var {
    pub cfg: CfgExpr,
    pub doc: Doc,
    #[cfg_attr(not(proc_macro), expect(dead_code))]
    pub attrs: OtherAttrs,
    #[cfg_attr(not(proc_macro), expect(dead_code))]
    pub visibility: Token![pub],
    pub name: Pair,
    #[cfg_attr(not(proc_macro), expect(dead_code))]
    pub colon_token: Token![:],
    pub ty: Type,
}

pub(crate) struct Receiver {
    pub pinned: bool,
    pub ampersand: Token![&],
    pub lifetime: Option<Lifetime>,
    pub mutable: bool,
    pub var: Token![self],
    pub ty: NamedType,
    #[cfg_attr(not(proc_macro), expect(dead_code))]
    pub colon_token: Token![:],
    pub shorthand: bool,
    #[cfg_attr(not(proc_macro), expect(dead_code))]
    pub pin_tokens: Option<(kw::Pin, Token![<], Token![>])>,
    pub mutability: Option<Token![mut]>,
}

pub(crate) struct Variant {
    #[cfg_attr(proc_macro, expect(dead_code))]
    pub cfg: CfgExpr,
    pub doc: Doc,
    pub default: bool,
    #[cfg_attr(not(proc_macro), expect(dead_code))]
    pub attrs: OtherAttrs,
    pub name: Pair,
    pub discriminant: Discriminant,
    #[expect(dead_code)]
    pub expr: Option<Expr>,
}

pub(crate) enum Type {
    Ident(NamedType),
    RustBox(Box<Ty1>),
    RustVec(Box<Ty1>),
    UniquePtr(Box<Ty1>),
    SharedPtr(Box<Ty1>),
    WeakPtr(Box<Ty1>),
    Ref(Box<Ref>),
    Ptr(Box<Ptr>),
    Str(Box<Ref>),
    CxxVector(Box<Ty1>),
    Fn(Box<Signature>),
    Void(Span),
    SliceRef(Box<SliceRef>),
    Array(Box<Array>),
}

pub(crate) struct Ty1 {
    pub name: Ident,
    pub langle: Token![<],
    pub inner: Type,
    pub rangle: Token![>],
}

pub(crate) struct Ref {
    pub pinned: bool,
    pub ampersand: Token![&],
    pub lifetime: Option<Lifetime>,
    pub mutable: bool,
    pub inner: Type,
    pub pin_tokens: Option<(kw::Pin, Token![<], Token![>])>,
    pub mutability: Option<Token![mut]>,
}

pub(crate) struct Ptr {
    pub star: Token![*],
    pub mutable: bool,
    pub inner: Type,
    pub mutability: Option<Token![mut]>,
    pub constness: Option<Token![const]>,
}

pub(crate) struct SliceRef {
    pub ampersand: Token![&],
    pub lifetime: Option<Lifetime>,
    pub mutable: bool,
    pub bracket: Bracket,
    pub inner: Type,
    pub mutability: Option<Token![mut]>,
}

pub(crate) struct Array {
    pub bracket: Bracket,
    pub inner: Type,
    pub semi_token: Token![;],
    pub len: usize,
    pub len_token: LitInt,
}

#[derive(Copy, Clone, PartialEq)]
pub(crate) enum Lang {
    Cxx,
    CxxUnwind,
    Rust,
}

// An association of a defined Rust name with a fully resolved, namespace
// qualified C++ name.
#[derive(Clone)]
pub(crate) struct Pair {
    pub namespace: Namespace,
    pub cxx: ForeignName,
    pub rust: Ident,
}

// Wrapper for a type which needs to be resolved before it can be printed in
// C++.
#[derive(PartialEq, Eq, Hash)]
pub(crate) struct NamedType {
    pub rust: Ident,
    pub generics: Lifetimes,
}

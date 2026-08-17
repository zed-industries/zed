//! Intermediate representation for C/C++ functions and methods.

use super::comp::MethodKind;
use super::context::{BindgenContext, TypeId};
use super::dot::DotAttributes;
use super::item::Item;
use super::traversal::{EdgeKind, Trace, Tracer};
use super::ty::TypeKind;
use crate::callbacks::{ItemInfo, ItemKind};
use crate::clang::{self, ABIKind, Attribute};
use crate::parse::{ClangSubItemParser, ParseError, ParseResult};
use clang_sys::CXCallingConv;

use quote::TokenStreamExt;
use std::io;
use std::str::FromStr;

const RUST_DERIVE_FUNPTR_LIMIT: usize = 12;

/// What kind of a function are we looking at?
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum FunctionKind {
    /// A plain, free function.
    Function,
    /// A method of some kind.
    Method(MethodKind),
}

impl FunctionKind {
    /// Given a clang cursor, return the kind of function it represents, or
    /// `None` otherwise.
    pub(crate) fn from_cursor(cursor: &clang::Cursor) -> Option<FunctionKind> {
        // FIXME(emilio): Deduplicate logic with `ir::comp`.
        Some(match cursor.kind() {
            clang_sys::CXCursor_FunctionDecl => FunctionKind::Function,
            clang_sys::CXCursor_Constructor => {
                FunctionKind::Method(MethodKind::Constructor)
            }
            clang_sys::CXCursor_Destructor => {
                FunctionKind::Method(if cursor.method_is_virtual() {
                    MethodKind::VirtualDestructor {
                        pure_virtual: cursor.method_is_pure_virtual(),
                    }
                } else {
                    MethodKind::Destructor
                })
            }
            clang_sys::CXCursor_CXXMethod => {
                if cursor.method_is_virtual() {
                    FunctionKind::Method(MethodKind::Virtual {
                        pure_virtual: cursor.method_is_pure_virtual(),
                    })
                } else if cursor.method_is_static() {
                    FunctionKind::Method(MethodKind::Static)
                } else {
                    FunctionKind::Method(MethodKind::Normal)
                }
            }
            _ => return None,
        })
    }
}

/// The style of linkage
#[derive(Debug, Clone, Copy)]
pub(crate) enum Linkage {
    /// Externally visible and can be linked against
    External,
    /// Not exposed externally. 'static inline' functions will have this kind of linkage
    Internal,
}

/// A function declaration, with a signature, arguments, and argument names.
///
/// The argument names vector must be the same length as the ones in the
/// signature.
#[derive(Debug)]
pub(crate) struct Function {
    /// The name of this function.
    name: String,

    /// The mangled name, that is, the symbol.
    mangled_name: Option<String>,

    /// The link name. If specified, overwrite `mangled_name`.
    link_name: Option<String>,

    /// The ID pointing to the current function signature.
    signature: TypeId,

    /// The kind of function this is.
    kind: FunctionKind,

    /// The linkage of the function.
    linkage: Linkage,
}

impl Function {
    /// Construct a new function.
    pub(crate) fn new(
        name: String,
        mangled_name: Option<String>,
        link_name: Option<String>,
        signature: TypeId,
        kind: FunctionKind,
        linkage: Linkage,
    ) -> Self {
        Function {
            name,
            mangled_name,
            link_name,
            signature,
            kind,
            linkage,
        }
    }

    /// Get this function's name.
    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    /// Get this function's name.
    pub(crate) fn mangled_name(&self) -> Option<&str> {
        self.mangled_name.as_deref()
    }

    /// Get this function's link name.
    pub fn link_name(&self) -> Option<&str> {
        self.link_name.as_deref()
    }

    /// Get this function's signature type.
    pub(crate) fn signature(&self) -> TypeId {
        self.signature
    }

    /// Get this function's kind.
    pub(crate) fn kind(&self) -> FunctionKind {
        self.kind
    }

    /// Get this function's linkage.
    pub(crate) fn linkage(&self) -> Linkage {
        self.linkage
    }
}

impl DotAttributes for Function {
    fn dot_attributes<W>(
        &self,
        _ctx: &BindgenContext,
        out: &mut W,
    ) -> io::Result<()>
    where
        W: io::Write,
    {
        if let Some(ref mangled) = self.mangled_name {
            let mangled: String =
                mangled.chars().flat_map(|c| c.escape_default()).collect();
            writeln!(out, "<tr><td>mangled name</td><td>{mangled}</td></tr>")?;
        }

        Ok(())
    }
}

/// A valid rust ABI.
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum Abi {
    /// The default C ABI.
    C,
    /// The "stdcall" ABI.
    Stdcall,
    /// The "efiapi" ABI.
    EfiApi,
    /// The "fastcall" ABI.
    Fastcall,
    /// The "thiscall" ABI.
    ThisCall,
    /// The "vectorcall" ABI.
    Vectorcall,
    /// The "aapcs" ABI.
    Aapcs,
    /// The "win64" ABI.
    Win64,
    /// The "C-unwind" ABI.
    CUnwind,
    /// The "system" ABI.
    System,
}

impl FromStr for Abi {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "C" => Ok(Self::C),
            "stdcall" => Ok(Self::Stdcall),
            "efiapi" => Ok(Self::EfiApi),
            "fastcall" => Ok(Self::Fastcall),
            "thiscall" => Ok(Self::ThisCall),
            "vectorcall" => Ok(Self::Vectorcall),
            "aapcs" => Ok(Self::Aapcs),
            "win64" => Ok(Self::Win64),
            "C-unwind" => Ok(Self::CUnwind),
            "system" => Ok(Self::System),
            _ => Err(format!("Invalid or unknown ABI {s:?}")),
        }
    }
}

impl std::fmt::Display for Abi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            Self::C => "C",
            Self::Stdcall => "stdcall",
            Self::EfiApi => "efiapi",
            Self::Fastcall => "fastcall",
            Self::ThisCall => "thiscall",
            Self::Vectorcall => "vectorcall",
            Self::Aapcs => "aapcs",
            Self::Win64 => "win64",
            Self::CUnwind => "C-unwind",
            Abi::System => "system",
        };

        s.fmt(f)
    }
}

impl quote::ToTokens for Abi {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let abi = self.to_string();
        tokens.append_all(quote! { #abi });
    }
}

/// An ABI extracted from a clang cursor.
#[derive(Debug, Copy, Clone)]
pub(crate) enum ClangAbi {
    /// An ABI known by Rust.
    Known(Abi),
    /// An unknown or invalid ABI.
    Unknown(CXCallingConv),
}

impl ClangAbi {
    /// Returns whether this Abi is known or not.
    fn is_unknown(&self) -> bool {
        matches!(*self, ClangAbi::Unknown(..))
    }
}

impl quote::ToTokens for ClangAbi {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match *self {
            Self::Known(abi) => abi.to_tokens(tokens),
            Self::Unknown(cc) => panic!(
                "Cannot turn unknown calling convention to tokens: {cc:?}"
            ),
        }
    }
}

/// A function signature.
#[derive(Debug)]
pub(crate) struct FunctionSig {
    /// The name of this function signature.
    name: String,

    /// The return type of the function.
    return_type: TypeId,

    /// The type of the arguments, optionally with the name of the argument when
    /// declared.
    argument_types: Vec<(Option<String>, TypeId)>,

    /// Whether this function is variadic.
    is_variadic: bool,
    is_divergent: bool,

    /// Whether this function's return value must be used.
    must_use: bool,

    /// The ABI of this function.
    abi: ClangAbi,
}

fn get_abi(cc: CXCallingConv) -> ClangAbi {
    use clang_sys::*;
    match cc {
        CXCallingConv_Default => ClangAbi::Known(Abi::C),
        CXCallingConv_C => ClangAbi::Known(Abi::C),
        CXCallingConv_X86StdCall => ClangAbi::Known(Abi::Stdcall),
        CXCallingConv_X86FastCall => ClangAbi::Known(Abi::Fastcall),
        CXCallingConv_X86ThisCall => ClangAbi::Known(Abi::ThisCall),
        CXCallingConv_X86VectorCall => ClangAbi::Known(Abi::Vectorcall),
        CXCallingConv_AAPCS => ClangAbi::Known(Abi::Aapcs),
        CXCallingConv_X86_64Win64 => ClangAbi::Known(Abi::Win64),
        CXCallingConv_AArch64VectorCall => ClangAbi::Known(Abi::Vectorcall),
        other => ClangAbi::Unknown(other),
    }
}

/// Get the mangled name for the cursor's referent.
pub(crate) fn cursor_mangling(
    ctx: &BindgenContext,
    cursor: &clang::Cursor,
) -> Option<String> {
    if !ctx.options().enable_mangling {
        return None;
    }

    // We early return here because libclang may crash in some case
    // if we pass in a variable inside a partial specialized template.
    // See rust-lang/rust-bindgen#67, and rust-lang/rust-bindgen#462.
    if cursor.is_in_non_fully_specialized_template() {
        return None;
    }

    let is_itanium_abi = ctx.abi_kind() == ABIKind::GenericItanium;
    let is_destructor = cursor.kind() == clang_sys::CXCursor_Destructor;
    if let Ok(mut manglings) = cursor.cxx_manglings() {
        while let Some(m) = manglings.pop() {
            // Only generate the destructor group 1, see below.
            if is_itanium_abi && is_destructor && !m.ends_with("D1Ev") {
                continue;
            }

            return Some(m);
        }
    }

    let mut mangling = cursor.mangling();
    if mangling.is_empty() {
        return None;
    }

    if is_itanium_abi && is_destructor {
        // With old (3.8-) libclang versions, and the Itanium ABI, clang returns
        // the "destructor group 0" symbol, which means that it'll try to free
        // memory, which definitely isn't what we want.
        //
        // Explicitly force the destructor group 1 symbol.
        //
        // See http://refspecs.linuxbase.org/cxxabi-1.83.html#mangling-special
        // for the reference, and http://stackoverflow.com/a/6614369/1091587 for
        // a more friendly explanation.
        //
        // We don't need to do this for constructors since clang seems to always
        // have returned the C1 constructor.
        //
        // FIXME(emilio): Can a legit symbol in other ABIs end with this string?
        // I don't think so, but if it can this would become a linker error
        // anyway, not an invalid free at runtime.
        //
        // TODO(emilio, #611): Use cpp_demangle if this becomes nastier with
        // time.
        if mangling.ends_with("D0Ev") {
            let new_len = mangling.len() - 4;
            mangling.truncate(new_len);
            mangling.push_str("D1Ev");
        }
    }

    Some(mangling)
}

fn args_from_ty_and_cursor(
    ty: &clang::Type,
    cursor: &clang::Cursor,
    ctx: &mut BindgenContext,
) -> Vec<(Option<String>, TypeId)> {
    let cursor_args = cursor.args().unwrap_or_default().into_iter();
    let type_args = ty.args().unwrap_or_default().into_iter();

    // Argument types can be found in either the cursor or the type, but argument names may only be
    // found on the cursor. We often have access to both a type and a cursor for each argument, but
    // in some cases we may only have one.
    //
    // Prefer using the type as the source of truth for the argument's type, but fall back to
    // inspecting the cursor (this happens for Objective C interfaces).
    //
    // Prefer using the cursor for the argument's type, but fall back to using the parent's cursor
    // (this happens for function pointer return types).
    cursor_args
        .map(Some)
        .chain(std::iter::repeat(None))
        .zip(type_args.map(Some).chain(std::iter::repeat(None)))
        .take_while(|(cur, ty)| cur.is_some() || ty.is_some())
        .map(|(arg_cur, arg_ty)| {
            let name = arg_cur.map(|a| a.spelling()).and_then(|name| {
                if name.is_empty() {
                    None
                } else {
                    Some(name)
                }
            });

            let cursor = arg_cur.unwrap_or(*cursor);
            let ty = arg_ty.unwrap_or_else(|| cursor.cur_type());
            (name, Item::from_ty_or_ref(ty, cursor, None, ctx))
        })
        .collect()
}

impl FunctionSig {
    /// Get the function name.
    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    /// Construct a new function signature from the given Clang type.
    pub(crate) fn from_ty(
        ty: &clang::Type,
        cursor: &clang::Cursor,
        ctx: &mut BindgenContext,
    ) -> Result<Self, ParseError> {
        use clang_sys::*;
        debug!("FunctionSig::from_ty {ty:?} {cursor:?}");

        // Skip function templates
        let kind = cursor.kind();
        if kind == CXCursor_FunctionTemplate {
            return Err(ParseError::Continue);
        }

        let spelling = cursor.spelling();

        // Don't parse operatorxx functions in C++
        let is_operator = |spelling: &str| {
            spelling.starts_with("operator") &&
                !clang::is_valid_identifier(spelling)
        };
        if is_operator(&spelling) {
            return Err(ParseError::Continue);
        }

        // Constructors of non-type template parameter classes for some reason
        // include the template parameter in their name. Just skip them, since
        // we don't handle well non-type template parameters anyway.
        if (kind == CXCursor_Constructor || kind == CXCursor_Destructor) &&
            spelling.contains('<')
        {
            return Err(ParseError::Continue);
        }

        let cursor = if cursor.is_valid() {
            *cursor
        } else {
            ty.declaration()
        };

        let mut args = match kind {
            CXCursor_FunctionDecl |
            CXCursor_Constructor |
            CXCursor_CXXMethod |
            CXCursor_ObjCInstanceMethodDecl |
            CXCursor_ObjCClassMethodDecl => {
                args_from_ty_and_cursor(ty, &cursor, ctx)
            }
            _ => {
                // For non-CXCursor_FunctionDecl, visiting the cursor's children
                // is the only reliable way to get parameter names.
                let mut args = vec![];
                cursor.visit(|c| {
                    if c.kind() == CXCursor_ParmDecl {
                        let ty =
                            Item::from_ty_or_ref(c.cur_type(), c, None, ctx);
                        let name = c.spelling();
                        let name =
                            if name.is_empty() { None } else { Some(name) };
                        args.push((name, ty));
                    }
                    CXChildVisit_Continue
                });

                if args.is_empty() {
                    // FIXME(emilio): Sometimes libclang doesn't expose the
                    // right AST for functions tagged as stdcall and such...
                    //
                    // https://bugs.llvm.org/show_bug.cgi?id=45919
                    args_from_ty_and_cursor(ty, &cursor, ctx)
                } else {
                    args
                }
            }
        };

        let (must_use, mut is_divergent) =
            if ctx.options().enable_function_attribute_detection {
                let [must_use, no_return, no_return_cpp] = cursor.has_attrs(&[
                    Attribute::MUST_USE,
                    Attribute::NO_RETURN,
                    Attribute::NO_RETURN_CPP,
                ]);
                (must_use, no_return || no_return_cpp)
            } else {
                Default::default()
            };

        // Check if the type contains __attribute__((noreturn)) outside of parentheses. This is
        // somewhat fragile, but it seems to be the only way to get at this information as of
        // libclang 9.
        let ty_spelling = ty.spelling();
        let has_attribute_noreturn = ty_spelling
            .match_indices("__attribute__((noreturn))")
            .any(|(i, _)| {
                let depth = ty_spelling[..i]
                    .bytes()
                    .filter_map(|ch| match ch {
                        b'(' => Some(1),
                        b')' => Some(-1),
                        _ => None,
                    })
                    .sum::<isize>();
                depth == 0
            });
        is_divergent = is_divergent || has_attribute_noreturn;

        let is_method = kind == CXCursor_CXXMethod;
        let is_constructor = kind == CXCursor_Constructor;
        let is_destructor = kind == CXCursor_Destructor;
        if (is_constructor || is_destructor || is_method) &&
            cursor.lexical_parent() != cursor.semantic_parent()
        {
            // Only parse constructors once.
            return Err(ParseError::Continue);
        }

        if is_method || is_constructor || is_destructor {
            let is_const = is_method && cursor.method_is_const();
            let is_virtual = is_method && cursor.method_is_virtual();
            let is_static = is_method && cursor.method_is_static();
            if !is_static && !is_virtual {
                let parent = cursor.semantic_parent();
                let class = Item::parse(parent, None, ctx)
                    .expect("Expected to parse the class");
                // The `class` most likely is not finished parsing yet, so use
                // the unchecked variant.
                let class = class.as_type_id_unchecked();

                let class = if is_const {
                    let const_class_id = ctx.next_item_id();
                    ctx.build_const_wrapper(
                        const_class_id,
                        class,
                        None,
                        &parent.cur_type(),
                    )
                } else {
                    class
                };

                let ptr =
                    Item::builtin_type(TypeKind::Pointer(class), false, ctx);
                args.insert(0, (Some("this".into()), ptr));
            } else if is_virtual {
                let void = Item::builtin_type(TypeKind::Void, false, ctx);
                let ptr =
                    Item::builtin_type(TypeKind::Pointer(void), false, ctx);
                args.insert(0, (Some("this".into()), ptr));
            }
        }

        let ty_ret_type = if kind == CXCursor_ObjCInstanceMethodDecl ||
            kind == CXCursor_ObjCClassMethodDecl
        {
            ty.ret_type()
                .or_else(|| cursor.ret_type())
                .ok_or(ParseError::Continue)?
        } else {
            ty.ret_type().ok_or(ParseError::Continue)?
        };

        let ret = if is_constructor && ctx.is_target_wasm32() {
            // Constructors in Clang wasm32 target return a pointer to the object
            // being constructed.
            let void = Item::builtin_type(TypeKind::Void, false, ctx);
            Item::builtin_type(TypeKind::Pointer(void), false, ctx)
        } else {
            Item::from_ty_or_ref(ty_ret_type, cursor, None, ctx)
        };

        // Clang plays with us at "find the calling convention", see #549 and
        // co. This seems to be a better fix than that commit.
        let mut call_conv = ty.call_conv();
        if let Some(ty) = cursor.cur_type().canonical_type().pointee_type() {
            let cursor_call_conv = ty.call_conv();
            if cursor_call_conv != CXCallingConv_Invalid {
                call_conv = cursor_call_conv;
            }
        }

        let abi = get_abi(call_conv);

        if abi.is_unknown() {
            warn!("Unknown calling convention: {call_conv:?}");
        }

        Ok(Self {
            name: spelling,
            return_type: ret,
            argument_types: args,
            is_variadic: ty.is_variadic(),
            is_divergent,
            must_use,
            abi,
        })
    }

    /// Get this function signature's return type.
    pub(crate) fn return_type(&self) -> TypeId {
        self.return_type
    }

    /// Get this function signature's argument (name, type) pairs.
    pub(crate) fn argument_types(&self) -> &[(Option<String>, TypeId)] {
        &self.argument_types
    }

    /// Get this function signature's ABI.
    pub(crate) fn abi(
        &self,
        ctx: &BindgenContext,
        name: Option<&str>,
    ) -> crate::codegen::error::Result<ClangAbi> {
        // FIXME (pvdrz): Try to do this check lazily instead. Maybe store the ABI inside `ctx`
        // instead?.
        let abi = if let Some(name) = name {
            if let Some((abi, _)) = ctx
                .options()
                .abi_overrides
                .iter()
                .find(|(_, regex_set)| regex_set.matches(name))
            {
                ClangAbi::Known(*abi)
            } else {
                self.abi
            }
        } else if let Some((abi, _)) = ctx
            .options()
            .abi_overrides
            .iter()
            .find(|(_, regex_set)| regex_set.matches(&self.name))
        {
            ClangAbi::Known(*abi)
        } else {
            self.abi
        };

        match abi {
            ClangAbi::Known(Abi::ThisCall)
                if !ctx.options().rust_features().thiscall_abi =>
            {
                Err(crate::codegen::error::Error::UnsupportedAbi("thiscall"))
            }
            ClangAbi::Known(Abi::Vectorcall)
                if !ctx.options().rust_features().vectorcall_abi =>
            {
                Err(crate::codegen::error::Error::UnsupportedAbi("vectorcall"))
            }
            ClangAbi::Known(Abi::CUnwind)
                if !ctx.options().rust_features().c_unwind_abi =>
            {
                Err(crate::codegen::error::Error::UnsupportedAbi("C-unwind"))
            }
            ClangAbi::Known(Abi::EfiApi)
                if !ctx.options().rust_features().abi_efiapi =>
            {
                Err(crate::codegen::error::Error::UnsupportedAbi("efiapi"))
            }
            ClangAbi::Known(Abi::Win64) if self.is_variadic() => {
                Err(crate::codegen::error::Error::UnsupportedAbi("Win64"))
            }
            abi => Ok(abi),
        }
    }

    /// Is this function signature variadic?
    pub(crate) fn is_variadic(&self) -> bool {
        // Clang reports some functions as variadic when they *might* be
        // variadic. We do the argument check because rust doesn't codegen well
        // variadic functions without an initial argument.
        self.is_variadic && !self.argument_types.is_empty()
    }

    /// Must this function's return value be used?
    pub(crate) fn must_use(&self) -> bool {
        self.must_use
    }

    /// Are function pointers with this signature able to derive Rust traits?
    /// Rust only supports deriving traits for function pointers with a limited
    /// number of parameters and a couple ABIs.
    ///
    /// For more details, see:
    ///
    /// * <https://github.com/rust-lang/rust-bindgen/issues/547>,
    /// * <https://github.com/rust-lang/rust/issues/38848>,
    /// * and <https://github.com/rust-lang/rust/issues/40158>
    pub(crate) fn function_pointers_can_derive(&self) -> bool {
        if self.argument_types.len() > RUST_DERIVE_FUNPTR_LIMIT {
            return false;
        }

        matches!(self.abi, ClangAbi::Known(Abi::C) | ClangAbi::Unknown(..))
    }

    /// Whether this function has attributes marking it as divergent.
    pub(crate) fn is_divergent(&self) -> bool {
        self.is_divergent
    }
}

impl ClangSubItemParser for Function {
    fn parse(
        cursor: clang::Cursor,
        context: &mut BindgenContext,
    ) -> Result<ParseResult<Self>, ParseError> {
        use clang_sys::*;

        let kind = match FunctionKind::from_cursor(&cursor) {
            None => return Err(ParseError::Continue),
            Some(k) => k,
        };

        debug!("Function::parse({cursor:?}, {:?})", cursor.cur_type());
        let visibility = cursor.visibility();
        if visibility != CXVisibility_Default {
            return Err(ParseError::Continue);
        }

        if cursor.access_specifier() == CX_CXXPrivate {
            return Err(ParseError::Continue);
        }

        let linkage = cursor.linkage();
        let linkage = match linkage {
            CXLinkage_External | CXLinkage_UniqueExternal => Linkage::External,
            CXLinkage_Internal => Linkage::Internal,
            _ => return Err(ParseError::Continue),
        };

        if cursor.is_inlined_function() ||
            cursor.definition().is_some_and(|x| x.is_inlined_function())
        {
            if !context.options().generate_inline_functions &&
                !context.options().wrap_static_fns
            {
                return Err(ParseError::Continue);
            }

            if cursor.is_deleted_function() {
                return Err(ParseError::Continue);
            }

            // We cannot handle `inline` functions that are not `static`.
            if context.options().wrap_static_fns &&
                cursor.is_inlined_function() &&
                matches!(linkage, Linkage::External)
            {
                return Err(ParseError::Continue);
            }
        }

        // Grab the signature using Item::from_ty.
        let sig = Item::from_ty(&cursor.cur_type(), cursor, None, context)?;

        let mut name = cursor.spelling();
        assert!(!name.is_empty(), "Empty function name?");

        if cursor.kind() == CXCursor_Destructor {
            // Remove the leading `~`. The alternative to this is special-casing
            // code-generation for destructor functions, which seems less than
            // ideal.
            if name.starts_with('~') {
                name.remove(0);
            }

            // Add a suffix to avoid colliding with constructors. This would be
            // technically fine (since we handle duplicated functions/methods),
            // but seems easy enough to handle it here.
            name.push_str("_destructor");
        }
        if let Some(nm) = context.options().last_callback(|callbacks| {
            callbacks.generated_name_override(ItemInfo {
                name: name.as_str(),
                kind: ItemKind::Function,
            })
        }) {
            name = nm;
        }
        assert!(!name.is_empty(), "Empty function name.");

        let mangled_name = cursor_mangling(context, &cursor);

        let link_name = context.options().last_callback(|callbacks| {
            callbacks.generated_link_name_override(ItemInfo {
                name: name.as_str(),
                kind: ItemKind::Function,
            })
        });

        let function = Self::new(
            name.clone(),
            mangled_name,
            link_name,
            sig,
            kind,
            linkage,
        );

        Ok(ParseResult::New(function, Some(cursor)))
    }
}

impl Trace for FunctionSig {
    type Extra = ();

    fn trace<T>(&self, _: &BindgenContext, tracer: &mut T, _: &())
    where
        T: Tracer,
    {
        tracer.visit_kind(self.return_type().into(), EdgeKind::FunctionReturn);

        for &(_, ty) in self.argument_types() {
            tracer.visit_kind(ty.into(), EdgeKind::FunctionParameter);
        }
    }
}

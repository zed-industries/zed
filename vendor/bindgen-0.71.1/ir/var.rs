//! Intermediate representation of variables.

use super::super::codegen::MacroTypeVariation;
use super::context::{BindgenContext, TypeId};
use super::dot::DotAttributes;
use super::function::cursor_mangling;
use super::int::IntKind;
use super::item::Item;
use super::ty::{FloatKind, TypeKind};
use crate::callbacks::{ItemInfo, ItemKind, MacroParsingBehavior};
use crate::clang;
use crate::clang::ClangToken;
use crate::parse::{ClangSubItemParser, ParseError, ParseResult};

use std::io;
use std::num::Wrapping;

/// The type for a constant variable.
#[derive(Debug)]
pub(crate) enum VarType {
    /// A boolean.
    Bool(bool),
    /// An integer.
    Int(i64),
    /// A floating point number.
    Float(f64),
    /// A character.
    Char(u8),
    /// A string, not necessarily well-formed utf-8.
    String(Vec<u8>),
}

/// A `Var` is our intermediate representation of a variable.
#[derive(Debug)]
pub(crate) struct Var {
    /// The name of the variable.
    name: String,
    /// The mangled name of the variable.
    mangled_name: Option<String>,
    /// The link name of the variable.
    link_name: Option<String>,
    /// The type of the variable.
    ty: TypeId,
    /// The value of the variable, that needs to be suitable for `ty`.
    val: Option<VarType>,
    /// Whether this variable is const.
    is_const: bool,
}

impl Var {
    /// Construct a new `Var`.
    pub(crate) fn new(
        name: String,
        mangled_name: Option<String>,
        link_name: Option<String>,
        ty: TypeId,
        val: Option<VarType>,
        is_const: bool,
    ) -> Var {
        assert!(!name.is_empty());
        Var {
            name,
            mangled_name,
            link_name,
            ty,
            val,
            is_const,
        }
    }

    /// Is this variable `const` qualified?
    pub(crate) fn is_const(&self) -> bool {
        self.is_const
    }

    /// The value of this constant variable, if any.
    pub(crate) fn val(&self) -> Option<&VarType> {
        self.val.as_ref()
    }

    /// Get this variable's type.
    pub(crate) fn ty(&self) -> TypeId {
        self.ty
    }

    /// Get this variable's name.
    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    /// Get this variable's mangled name.
    pub(crate) fn mangled_name(&self) -> Option<&str> {
        self.mangled_name.as_deref()
    }

    /// Get this variable's link name.
    pub fn link_name(&self) -> Option<&str> {
        self.link_name.as_deref()
    }
}

impl DotAttributes for Var {
    fn dot_attributes<W>(
        &self,
        _ctx: &BindgenContext,
        out: &mut W,
    ) -> io::Result<()>
    where
        W: io::Write,
    {
        if self.is_const {
            writeln!(out, "<tr><td>const</td><td>true</td></tr>")?;
        }

        if let Some(ref mangled) = self.mangled_name {
            writeln!(out, "<tr><td>mangled name</td><td>{mangled}</td></tr>")?;
        }

        Ok(())
    }
}

fn default_macro_constant_type(ctx: &BindgenContext, value: i64) -> IntKind {
    if value < 0 ||
        ctx.options().default_macro_constant_type ==
            MacroTypeVariation::Signed
    {
        if value < i64::from(i32::MIN) || value > i64::from(i32::MAX) {
            IntKind::I64
        } else if !ctx.options().fit_macro_constants ||
            value < i64::from(i16::MIN) ||
            value > i64::from(i16::MAX)
        {
            IntKind::I32
        } else if value < i64::from(i8::MIN) || value > i64::from(i8::MAX) {
            IntKind::I16
        } else {
            IntKind::I8
        }
    } else if value > i64::from(u32::MAX) {
        IntKind::U64
    } else if !ctx.options().fit_macro_constants || value > i64::from(u16::MAX)
    {
        IntKind::U32
    } else if value > i64::from(u8::MAX) {
        IntKind::U16
    } else {
        IntKind::U8
    }
}

/// Parses tokens from a `CXCursor_MacroDefinition` pointing into a function-like
/// macro, and calls the `func_macro` callback.
fn handle_function_macro(
    cursor: &clang::Cursor,
    callbacks: &dyn crate::callbacks::ParseCallbacks,
) {
    let is_closing_paren = |t: &ClangToken| {
        // Test cheap token kind before comparing exact spellings.
        t.kind == clang_sys::CXToken_Punctuation && t.spelling() == b")"
    };
    let tokens: Vec<_> = cursor.tokens().iter().collect();
    if let Some(boundary) = tokens.iter().position(is_closing_paren) {
        let mut spelled = tokens.iter().map(ClangToken::spelling);
        // Add 1, to convert index to length.
        let left = spelled.by_ref().take(boundary + 1);
        let left = left.collect::<Vec<_>>().concat();
        if let Ok(left) = String::from_utf8(left) {
            let right: Vec<_> = spelled.collect();
            callbacks.func_macro(&left, &right);
        }
    }
}

impl ClangSubItemParser for Var {
    fn parse(
        cursor: clang::Cursor,
        ctx: &mut BindgenContext,
    ) -> Result<ParseResult<Self>, ParseError> {
        use cexpr::expr::EvalResult;
        use cexpr::literal::CChar;
        use clang_sys::*;
        match cursor.kind() {
            CXCursor_MacroDefinition => {
                for callbacks in &ctx.options().parse_callbacks {
                    match callbacks.will_parse_macro(&cursor.spelling()) {
                        MacroParsingBehavior::Ignore => {
                            return Err(ParseError::Continue);
                        }
                        MacroParsingBehavior::Default => {}
                    }

                    if cursor.is_macro_function_like() {
                        handle_function_macro(&cursor, callbacks.as_ref());
                        // We handled the macro, skip macro processing below.
                        return Err(ParseError::Continue);
                    }
                }

                let value = parse_macro(ctx, &cursor);

                let Some((id, value)) = value else {
                    return Err(ParseError::Continue);
                };

                assert!(!id.is_empty(), "Empty macro name?");

                let previously_defined = ctx.parsed_macro(&id);

                // NB: It's important to "note" the macro even if the result is
                // not an integer, otherwise we might loose other kind of
                // derived macros.
                ctx.note_parsed_macro(id.clone(), value.clone());

                if previously_defined {
                    let name = String::from_utf8(id).unwrap();
                    duplicated_macro_diagnostic(&name, cursor.location(), ctx);
                    return Err(ParseError::Continue);
                }

                // NOTE: Unwrapping, here and above, is safe, because the
                // identifier of a token comes straight from clang, and we
                // enforce utf8 there, so we should have already panicked at
                // this point.
                let name = String::from_utf8(id).unwrap();
                let (type_kind, val) = match value {
                    EvalResult::Invalid => return Err(ParseError::Continue),
                    EvalResult::Float(f) => {
                        (TypeKind::Float(FloatKind::Double), VarType::Float(f))
                    }
                    EvalResult::Char(c) => {
                        let c = match c {
                            CChar::Char(c) => {
                                assert_eq!(c.len_utf8(), 1);
                                c as u8
                            }
                            CChar::Raw(c) => {
                                assert!(c <= u64::from(u8::MAX));
                                c as u8
                            }
                        };

                        (TypeKind::Int(IntKind::U8), VarType::Char(c))
                    }
                    EvalResult::Str(val) => {
                        let char_ty = Item::builtin_type(
                            TypeKind::Int(IntKind::U8),
                            true,
                            ctx,
                        );
                        for callbacks in &ctx.options().parse_callbacks {
                            callbacks.str_macro(&name, &val);
                        }
                        (TypeKind::Pointer(char_ty), VarType::String(val))
                    }
                    EvalResult::Int(Wrapping(value)) => {
                        let kind = ctx
                            .options()
                            .last_callback(|c| c.int_macro(&name, value))
                            .unwrap_or_else(|| {
                                default_macro_constant_type(ctx, value)
                            });

                        (TypeKind::Int(kind), VarType::Int(value))
                    }
                };

                let ty = Item::builtin_type(type_kind, true, ctx);

                Ok(ParseResult::New(
                    Var::new(name, None, None, ty, Some(val), true),
                    Some(cursor),
                ))
            }
            CXCursor_VarDecl => {
                let mut name = cursor.spelling();
                if cursor.linkage() == CXLinkage_External {
                    if let Some(nm) = ctx.options().last_callback(|callbacks| {
                        callbacks.generated_name_override(ItemInfo {
                            name: name.as_str(),
                            kind: ItemKind::Var,
                        })
                    }) {
                        name = nm;
                    }
                }
                // No more changes to name
                let name = name;

                if name.is_empty() {
                    warn!("Empty constant name?");
                    return Err(ParseError::Continue);
                }

                let link_name = ctx.options().last_callback(|callbacks| {
                    callbacks.generated_link_name_override(ItemInfo {
                        name: name.as_str(),
                        kind: ItemKind::Var,
                    })
                });

                let ty = cursor.cur_type();

                // TODO(emilio): do we have to special-case constant arrays in
                // some other places?
                let is_const = ty.is_const() ||
                    ([CXType_ConstantArray, CXType_IncompleteArray]
                        .contains(&ty.kind()) &&
                        ty.elem_type()
                            .is_some_and(|element| element.is_const()));

                let ty = match Item::from_ty(&ty, cursor, None, ctx) {
                    Ok(ty) => ty,
                    Err(e) => {
                        assert!(
                            matches!(ty.kind(), CXType_Auto | CXType_Unexposed),
                            "Couldn't resolve constant type, and it \
                             wasn't an nondeductible auto type or unexposed \
                             type: {ty:?}"
                        );
                        return Err(e);
                    }
                };

                // Note: Ty might not be totally resolved yet, see
                // tests/headers/inner_const.hpp
                //
                // That's fine because in that case we know it's not a literal.
                let canonical_ty = ctx
                    .safe_resolve_type(ty)
                    .and_then(|t| t.safe_canonical_type(ctx));

                let is_integer = canonical_ty.is_some_and(|t| t.is_integer());
                let is_float = canonical_ty.is_some_and(|t| t.is_float());

                // TODO: We could handle `char` more gracefully.
                // TODO: Strings, though the lookup is a bit more hard (we need
                // to look at the canonical type of the pointee too, and check
                // is char, u8, or i8 I guess).
                let value = if is_integer {
                    let TypeKind::Int(kind) = *canonical_ty.unwrap().kind()
                    else {
                        unreachable!()
                    };

                    let mut val = cursor.evaluate().and_then(|v| v.as_int());
                    if val.is_none() || !kind.signedness_matches(val.unwrap()) {
                        val = get_integer_literal_from_cursor(&cursor);
                    }

                    val.map(|val| {
                        if kind == IntKind::Bool {
                            VarType::Bool(val != 0)
                        } else {
                            VarType::Int(val)
                        }
                    })
                } else if is_float {
                    cursor
                        .evaluate()
                        .and_then(|v| v.as_double())
                        .map(VarType::Float)
                } else {
                    cursor
                        .evaluate()
                        .and_then(|v| v.as_literal_string())
                        .map(VarType::String)
                };

                let mangling = cursor_mangling(ctx, &cursor);
                let var =
                    Var::new(name, mangling, link_name, ty, value, is_const);

                Ok(ParseResult::New(var, Some(cursor)))
            }
            _ => {
                /* TODO */
                Err(ParseError::Continue)
            }
        }
    }
}

/// This function uses a [`FallbackTranslationUnit`][clang::FallbackTranslationUnit] to parse each
/// macro that cannot be parsed by the normal bindgen process for `#define`s.
///
/// To construct the [`FallbackTranslationUnit`][clang::FallbackTranslationUnit], first precompiled
/// headers are generated for all input headers. An empty temporary `.c` file is generated to pass
/// to the translation unit. On the evaluation of each macro, a [`String`] is generated with the
/// new contents of the empty file and passed in for reparsing. The precompiled headers and
/// preservation of the [`FallbackTranslationUnit`][clang::FallbackTranslationUnit] across macro
/// evaluations are both optimizations that have significantly improved the performance.
fn parse_macro_clang_fallback(
    ctx: &mut BindgenContext,
    cursor: &clang::Cursor,
) -> Option<(Vec<u8>, cexpr::expr::EvalResult)> {
    if !ctx.options().clang_macro_fallback {
        return None;
    }

    let ftu = ctx.try_ensure_fallback_translation_unit()?;
    let contents = format!("int main() {{ {}; }}", cursor.spelling());
    ftu.reparse(&contents).ok()?;
    // Children of root node of AST
    let root_children = ftu.translation_unit().cursor().collect_children();
    // Last child in root is function declaration
    // Should be FunctionDecl
    let main_func = root_children.last()?;
    // Children should all be statements in function declaration
    let all_stmts = main_func.collect_children();
    // First child in all_stmts should be the statement containing the macro to evaluate
    // Should be CompoundStmt
    let macro_stmt = all_stmts.first()?;
    // Children should all be expressions from the compound statement
    let paren_exprs = macro_stmt.collect_children();
    // First child in all_exprs is the expression utilizing the given macro to be evaluated
    // Should  be ParenExpr
    let paren = paren_exprs.first()?;

    Some((
        cursor.spelling().into_bytes(),
        cexpr::expr::EvalResult::Int(Wrapping(paren.evaluate()?.as_int()?)),
    ))
}

/// Try and parse a macro using all the macros parsed until now.
fn parse_macro(
    ctx: &mut BindgenContext,
    cursor: &clang::Cursor,
) -> Option<(Vec<u8>, cexpr::expr::EvalResult)> {
    use cexpr::expr;

    let cexpr_tokens = cursor.cexpr_tokens();

    let parser = expr::IdentifierParser::new(ctx.parsed_macros());

    match parser.macro_definition(&cexpr_tokens) {
        Ok((_, (id, val))) => Some((id.into(), val)),
        _ => parse_macro_clang_fallback(ctx, cursor),
    }
}

fn parse_int_literal_tokens(cursor: &clang::Cursor) -> Option<i64> {
    use cexpr::expr;
    use cexpr::expr::EvalResult;

    let cexpr_tokens = cursor.cexpr_tokens();

    // TODO(emilio): We can try to parse other kinds of literals.
    match expr::expr(&cexpr_tokens) {
        Ok((_, EvalResult::Int(Wrapping(val)))) => Some(val),
        _ => None,
    }
}

fn get_integer_literal_from_cursor(cursor: &clang::Cursor) -> Option<i64> {
    use clang_sys::*;
    let mut value = None;
    cursor.visit(|c| {
        match c.kind() {
            CXCursor_IntegerLiteral | CXCursor_UnaryOperator => {
                value = parse_int_literal_tokens(&c);
            }
            CXCursor_UnexposedExpr => {
                value = get_integer_literal_from_cursor(&c);
            }
            _ => (),
        }
        if value.is_some() {
            CXChildVisit_Break
        } else {
            CXChildVisit_Continue
        }
    });
    value
}

fn duplicated_macro_diagnostic(
    macro_name: &str,
    _location: clang::SourceLocation,
    _ctx: &BindgenContext,
) {
    warn!("Duplicated macro definition: {macro_name}");

    #[cfg(feature = "experimental")]
    // FIXME (pvdrz & amanjeev): This diagnostic message shows way too often to be actually
    // useful. We have to change the logic where this function is called to be able to emit this
    // message only when the duplication is an actual issue.
    //
    // If I understood correctly, `bindgen` ignores all `#undef` directives. Meaning that this:
    // ```c
    // #define FOO 1
    // #undef FOO
    // #define FOO 2
    // ```
    //
    // Will trigger this message even though there's nothing wrong with it.
    #[allow(clippy::overly_complex_bool_expr)]
    if false && _ctx.options().emit_diagnostics {
        use crate::diagnostics::{get_line, Diagnostic, Level, Slice};
        use std::borrow::Cow;

        let mut slice = Slice::default();
        let mut source = Cow::from(macro_name);

        let (file, line, col, _) = _location.location();
        if let Some(filename) = file.name() {
            if let Ok(Some(code)) = get_line(&filename, line) {
                source = code.into();
            }
            slice.with_location(filename, line, col);
        }

        slice.with_source(source);

        Diagnostic::default()
            .with_title("Duplicated macro definition.", Level::Warning)
            .add_slice(slice)
            .add_annotation("This macro had a duplicate.", Level::Note)
            .display();
    }
}

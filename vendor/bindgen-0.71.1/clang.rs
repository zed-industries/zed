//! A higher level Clang API built on top of the generated bindings in the
//! `clang_sys` module.

#![allow(non_upper_case_globals, dead_code)]
#![deny(clippy::missing_docs_in_private_items)]

use crate::ir::context::BindgenContext;
use clang_sys::*;
use std::cmp;

use std::ffi::{CStr, CString};
use std::fmt;
use std::fs::OpenOptions;
use std::hash::Hash;
use std::hash::Hasher;
use std::os::raw::{c_char, c_int, c_longlong, c_uint, c_ulong, c_ulonglong};
use std::sync::OnceLock;
use std::{mem, ptr, slice};

/// Type representing a clang attribute.
///
/// Values of this type can be used to check for different attributes using the `has_attrs`
/// function.
pub(crate) struct Attribute {
    name: &'static [u8],
    kind: Option<CXCursorKind>,
    token_kind: CXTokenKind,
}

impl Attribute {
    /// A `warn_unused_result` attribute.
    pub(crate) const MUST_USE: Self = Self {
        name: b"warn_unused_result",
        // FIXME(emilio): clang-sys doesn't expose `CXCursor_WarnUnusedResultAttr` (from clang 9).
        kind: Some(440),
        token_kind: CXToken_Identifier,
    };

    /// A `_Noreturn` attribute.
    pub(crate) const NO_RETURN: Self = Self {
        name: b"_Noreturn",
        kind: None,
        token_kind: CXToken_Keyword,
    };

    /// A `[[noreturn]]` attribute.
    pub(crate) const NO_RETURN_CPP: Self = Self {
        name: b"noreturn",
        kind: None,
        token_kind: CXToken_Identifier,
    };
}

/// A cursor into the Clang AST, pointing to an AST node.
///
/// We call the AST node pointed to by the cursor the cursor's "referent".
#[derive(Copy, Clone)]
pub(crate) struct Cursor {
    x: CXCursor,
}

impl fmt::Debug for Cursor {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "Cursor({} kind: {}, loc: {}, usr: {:?})",
            self.spelling(),
            kind_to_str(self.kind()),
            self.location(),
            self.usr()
        )
    }
}

impl Cursor {
    /// Get the Unified Symbol Resolution for this cursor's referent, if
    /// available.
    ///
    /// The USR can be used to compare entities across translation units.
    pub(crate) fn usr(&self) -> Option<String> {
        let s = unsafe { cxstring_into_string(clang_getCursorUSR(self.x)) };
        if s.is_empty() {
            None
        } else {
            Some(s)
        }
    }

    /// Is this cursor's referent a declaration?
    pub(crate) fn is_declaration(&self) -> bool {
        unsafe { clang_isDeclaration(self.kind()) != 0 }
    }

    /// Is this cursor's referent an anonymous record or so?
    pub(crate) fn is_anonymous(&self) -> bool {
        unsafe { clang_Cursor_isAnonymous(self.x) != 0 }
    }

    /// Get this cursor's referent's spelling.
    pub(crate) fn spelling(&self) -> String {
        unsafe { cxstring_into_string(clang_getCursorSpelling(self.x)) }
    }

    /// Get this cursor's referent's display name.
    ///
    /// This is not necessarily a valid identifier. It includes extra
    /// information, such as parameters for a function, etc.
    pub(crate) fn display_name(&self) -> String {
        unsafe { cxstring_into_string(clang_getCursorDisplayName(self.x)) }
    }

    /// Get the mangled name of this cursor's referent.
    pub(crate) fn mangling(&self) -> String {
        unsafe { cxstring_into_string(clang_Cursor_getMangling(self.x)) }
    }

    /// Gets the C++ manglings for this cursor, or an error if the manglings
    /// are not available.
    pub(crate) fn cxx_manglings(&self) -> Result<Vec<String>, ()> {
        use clang_sys::*;
        unsafe {
            let manglings = clang_Cursor_getCXXManglings(self.x);
            if manglings.is_null() {
                return Err(());
            }
            let count = (*manglings).Count as usize;

            let mut result = Vec::with_capacity(count);
            for i in 0..count {
                let string_ptr = (*manglings).Strings.add(i);
                result.push(cxstring_to_string_leaky(*string_ptr));
            }
            clang_disposeStringSet(manglings);
            Ok(result)
        }
    }

    /// Returns whether the cursor refers to a built-in definition.
    pub(crate) fn is_builtin(&self) -> bool {
        let (file, _, _, _) = self.location().location();
        file.name().is_none()
    }

    /// Get the `Cursor` for this cursor's referent's lexical parent.
    ///
    /// The lexical parent is the parent of the definition. The semantic parent
    /// is the parent of the declaration. Generally, the lexical parent doesn't
    /// have any effect on semantics, while the semantic parent does.
    ///
    /// In the following snippet, the `Foo` class would be the semantic parent
    /// of the out-of-line `method` definition, while the lexical parent is the
    /// translation unit.
    ///
    /// ```c++
    /// class Foo {
    ///     void method();
    /// };
    ///
    /// void Foo::method() { /* ... */ }
    /// ```
    pub(crate) fn lexical_parent(&self) -> Cursor {
        unsafe {
            Cursor {
                x: clang_getCursorLexicalParent(self.x),
            }
        }
    }

    /// Get the referent's semantic parent, if one is available.
    ///
    /// See documentation for `lexical_parent` for details on semantic vs
    /// lexical parents.
    pub(crate) fn fallible_semantic_parent(&self) -> Option<Cursor> {
        let sp = unsafe {
            Cursor {
                x: clang_getCursorSemanticParent(self.x),
            }
        };
        if sp == *self || !sp.is_valid() {
            return None;
        }
        Some(sp)
    }

    /// Get the referent's semantic parent.
    ///
    /// See documentation for `lexical_parent` for details on semantic vs
    /// lexical parents.
    pub(crate) fn semantic_parent(&self) -> Cursor {
        self.fallible_semantic_parent().unwrap()
    }

    /// Return the number of template arguments used by this cursor's referent,
    /// if the referent is either a template instantiation. Returns `None`
    /// otherwise.
    ///
    /// NOTE: This may not return `Some` for partial template specializations,
    /// see #193 and #194.
    pub(crate) fn num_template_args(&self) -> Option<u32> {
        // XXX: `clang_Type_getNumTemplateArguments` is sort of reliable, while
        // `clang_Cursor_getNumTemplateArguments` is totally unreliable.
        // Therefore, try former first, and only fallback to the latter if we
        // have to.
        self.cur_type()
            .num_template_args()
            .or_else(|| {
                let n: c_int =
                    unsafe { clang_Cursor_getNumTemplateArguments(self.x) };

                if n >= 0 {
                    Some(n as u32)
                } else {
                    debug_assert_eq!(n, -1);
                    None
                }
            })
            .or_else(|| {
                let canonical = self.canonical();
                if canonical == *self {
                    None
                } else {
                    canonical.num_template_args()
                }
            })
    }

    /// Get a cursor pointing to this referent's containing translation unit.
    ///
    /// Note that we shouldn't create a `TranslationUnit` struct here, because
    /// bindgen assumes there will only be one of them alive at a time, and
    /// disposes it on drop. That can change if this would be required, but I
    /// think we can survive fine without it.
    pub(crate) fn translation_unit(&self) -> Cursor {
        assert!(self.is_valid());
        unsafe {
            let tu = clang_Cursor_getTranslationUnit(self.x);
            let cursor = Cursor {
                x: clang_getTranslationUnitCursor(tu),
            };
            assert!(cursor.is_valid());
            cursor
        }
    }

    /// Is the referent a top level construct?
    pub(crate) fn is_toplevel(&self) -> bool {
        let mut semantic_parent = self.fallible_semantic_parent();

        while semantic_parent.is_some() &&
            (semantic_parent.unwrap().kind() == CXCursor_Namespace ||
                semantic_parent.unwrap().kind() ==
                    CXCursor_NamespaceAlias ||
                semantic_parent.unwrap().kind() == CXCursor_NamespaceRef)
        {
            semantic_parent =
                semantic_parent.unwrap().fallible_semantic_parent();
        }

        let tu = self.translation_unit();
        // Yes, this can happen with, e.g., macro definitions.
        semantic_parent == tu.fallible_semantic_parent()
    }

    /// There are a few kinds of types that we need to treat specially, mainly
    /// not tracking the type declaration but the location of the cursor, given
    /// clang doesn't expose a proper declaration for these types.
    pub(crate) fn is_template_like(&self) -> bool {
        matches!(
            self.kind(),
            CXCursor_ClassTemplate |
                CXCursor_ClassTemplatePartialSpecialization |
                CXCursor_TypeAliasTemplateDecl
        )
    }

    /// Is this Cursor pointing to a function-like macro definition?
    pub(crate) fn is_macro_function_like(&self) -> bool {
        unsafe { clang_Cursor_isMacroFunctionLike(self.x) != 0 }
    }

    /// Get the kind of referent this cursor is pointing to.
    pub(crate) fn kind(&self) -> CXCursorKind {
        self.x.kind
    }

    /// Returns true if the cursor is a definition
    pub(crate) fn is_definition(&self) -> bool {
        unsafe { clang_isCursorDefinition(self.x) != 0 }
    }

    /// Is the referent a template specialization?
    pub(crate) fn is_template_specialization(&self) -> bool {
        self.specialized().is_some()
    }

    /// Is the referent a fully specialized template specialization without any
    /// remaining free template arguments?
    pub(crate) fn is_fully_specialized_template(&self) -> bool {
        self.is_template_specialization() &&
            self.kind() != CXCursor_ClassTemplatePartialSpecialization &&
            self.num_template_args().unwrap_or(0) > 0
    }

    /// Is the referent a template specialization that still has remaining free
    /// template arguments?
    pub(crate) fn is_in_non_fully_specialized_template(&self) -> bool {
        if self.is_toplevel() {
            return false;
        }

        let parent = self.semantic_parent();
        if parent.is_fully_specialized_template() {
            return false;
        }

        if !parent.is_template_like() {
            return parent.is_in_non_fully_specialized_template();
        }

        true
    }

    /// Is the referent any kind of template parameter?
    pub(crate) fn is_template_parameter(&self) -> bool {
        matches!(
            self.kind(),
            CXCursor_TemplateTemplateParameter |
                CXCursor_TemplateTypeParameter |
                CXCursor_NonTypeTemplateParameter
        )
    }

    /// Does the referent's type or value depend on a template parameter?
    pub(crate) fn is_dependent_on_template_parameter(&self) -> bool {
        fn visitor(
            found_template_parameter: &mut bool,
            cur: Cursor,
        ) -> CXChildVisitResult {
            // If we found a template parameter, it is dependent.
            if cur.is_template_parameter() {
                *found_template_parameter = true;
                return CXChildVisit_Break;
            }

            // Get the referent and traverse it as well.
            if let Some(referenced) = cur.referenced() {
                if referenced.is_template_parameter() {
                    *found_template_parameter = true;
                    return CXChildVisit_Break;
                }

                referenced
                    .visit(|next| visitor(found_template_parameter, next));
                if *found_template_parameter {
                    return CXChildVisit_Break;
                }
            }

            // Continue traversing the AST at the original cursor.
            CXChildVisit_Recurse
        }

        if self.is_template_parameter() {
            return true;
        }

        let mut found_template_parameter = false;
        self.visit(|next| visitor(&mut found_template_parameter, next));

        found_template_parameter
    }

    /// Is this cursor pointing a valid referent?
    pub(crate) fn is_valid(&self) -> bool {
        unsafe { clang_isInvalid(self.kind()) == 0 }
    }

    /// Get the source location for the referent.
    pub(crate) fn location(&self) -> SourceLocation {
        unsafe {
            SourceLocation {
                x: clang_getCursorLocation(self.x),
            }
        }
    }

    /// Get the source location range for the referent.
    pub(crate) fn extent(&self) -> CXSourceRange {
        unsafe { clang_getCursorExtent(self.x) }
    }

    /// Get the raw declaration comment for this referent, if one exists.
    pub(crate) fn raw_comment(&self) -> Option<String> {
        let s = unsafe {
            cxstring_into_string(clang_Cursor_getRawCommentText(self.x))
        };
        if s.is_empty() {
            None
        } else {
            Some(s)
        }
    }

    /// Get the referent's parsed comment.
    pub(crate) fn comment(&self) -> Comment {
        unsafe {
            Comment {
                x: clang_Cursor_getParsedComment(self.x),
            }
        }
    }

    /// Get the referent's type.
    pub(crate) fn cur_type(&self) -> Type {
        unsafe {
            Type {
                x: clang_getCursorType(self.x),
            }
        }
    }

    /// Given that this cursor's referent is a reference to another type, or is
    /// a declaration, get the cursor pointing to the referenced type or type of
    /// the declared thing.
    pub(crate) fn definition(&self) -> Option<Cursor> {
        unsafe {
            let ret = Cursor {
                x: clang_getCursorDefinition(self.x),
            };

            if ret.is_valid() && ret.kind() != CXCursor_NoDeclFound {
                Some(ret)
            } else {
                None
            }
        }
    }

    /// Given that this cursor's referent is reference type, get the cursor
    /// pointing to the referenced type.
    pub(crate) fn referenced(&self) -> Option<Cursor> {
        unsafe {
            let ret = Cursor {
                x: clang_getCursorReferenced(self.x),
            };

            if ret.is_valid() {
                Some(ret)
            } else {
                None
            }
        }
    }

    /// Get the canonical cursor for this referent.
    ///
    /// Many types can be declared multiple times before finally being properly
    /// defined. This method allows us to get the canonical cursor for the
    /// referent type.
    pub(crate) fn canonical(&self) -> Cursor {
        unsafe {
            Cursor {
                x: clang_getCanonicalCursor(self.x),
            }
        }
    }

    /// Given that this cursor points to either a template specialization or a
    /// template instantiation, get a cursor pointing to the template definition
    /// that is being specialized.
    pub(crate) fn specialized(&self) -> Option<Cursor> {
        unsafe {
            let ret = Cursor {
                x: clang_getSpecializedCursorTemplate(self.x),
            };
            if ret.is_valid() {
                Some(ret)
            } else {
                None
            }
        }
    }

    /// Assuming that this cursor's referent is a template declaration, get the
    /// kind of cursor that would be generated for its specializations.
    pub(crate) fn template_kind(&self) -> CXCursorKind {
        unsafe { clang_getTemplateCursorKind(self.x) }
    }

    /// Traverse this cursor's referent and its children.
    ///
    /// Call the given function on each AST node traversed.
    pub(crate) fn visit<Visitor>(&self, mut visitor: Visitor)
    where
        Visitor: FnMut(Cursor) -> CXChildVisitResult,
    {
        let data = &mut visitor as *mut Visitor;
        unsafe {
            clang_visitChildren(self.x, visit_children::<Visitor>, data.cast());
        }
    }

    /// Traverse all of this cursor's children, sorted by where they appear in source code.
    ///
    /// Call the given function on each AST node traversed.
    pub(crate) fn visit_sorted<Visitor>(
        &self,
        ctx: &mut BindgenContext,
        mut visitor: Visitor,
    ) where
        Visitor: FnMut(&mut BindgenContext, Cursor),
    {
        // FIXME(#2556): The current source order stuff doesn't account well for different levels
        // of includes, or includes that show up at the same byte offset because they are passed in
        // via CLI.
        const SOURCE_ORDER_ENABLED: bool = false;
        if !SOURCE_ORDER_ENABLED {
            return self.visit(|c| {
                visitor(ctx, c);
                CXChildVisit_Continue
            });
        }

        let mut children = self.collect_children();
        for child in &children {
            if child.kind() == CXCursor_InclusionDirective {
                if let Some(included_file) = child.get_included_file_name() {
                    let location = child.location();
                    let (source_file, _, _, offset) = location.location();

                    if let Some(source_file) = source_file.name() {
                        ctx.add_include(source_file, included_file, offset);
                    }
                }
            }
        }
        children
            .sort_by(|child1, child2| child1.cmp_by_source_order(child2, ctx));
        for child in children {
            visitor(ctx, child);
        }
    }

    /// Compare source order of two cursors, considering `#include` directives.
    ///
    /// Built-in items provided by the compiler (which don't have a source file),
    /// are sorted first. Remaining files are sorted by their position in the source file.
    /// If the items' source files differ, they are sorted by the position of the first
    /// `#include` for their source file. If no source files are included, `None` is returned.
    fn cmp_by_source_order(
        &self,
        other: &Self,
        ctx: &BindgenContext,
    ) -> cmp::Ordering {
        let (file, _, _, offset) = self.location().location();
        let (other_file, _, _, other_offset) = other.location().location();

        let (file, other_file) = match (file.name(), other_file.name()) {
            (Some(file), Some(other_file)) => (file, other_file),
            // Built-in definitions should come first.
            (Some(_), None) => return cmp::Ordering::Greater,
            (None, Some(_)) => return cmp::Ordering::Less,
            (None, None) => return cmp::Ordering::Equal,
        };

        if file == other_file {
            // Both items are in the same source file, compare by byte offset.
            return offset.cmp(&other_offset);
        }

        let include_location = ctx.included_file_location(&file);
        let other_include_location = ctx.included_file_location(&other_file);
        match (include_location, other_include_location) {
            (Some((file2, offset2)), _) if file2 == other_file => {
                offset2.cmp(&other_offset)
            }
            (Some(_), None) => cmp::Ordering::Greater,
            (_, Some((other_file2, other_offset2))) if file == other_file2 => {
                offset.cmp(&other_offset2)
            }
            (None, Some(_)) => cmp::Ordering::Less,
            (Some((file2, offset2)), Some((other_file2, other_offset2))) => {
                if file2 == other_file2 {
                    offset2.cmp(&other_offset2)
                } else {
                    cmp::Ordering::Equal
                }
            }
            (None, None) => cmp::Ordering::Equal,
        }
    }

    /// Collect all of this cursor's children into a vec and return them.
    pub(crate) fn collect_children(&self) -> Vec<Cursor> {
        let mut children = vec![];
        self.visit(|c| {
            children.push(c);
            CXChildVisit_Continue
        });
        children
    }

    /// Does this cursor have any children?
    pub(crate) fn has_children(&self) -> bool {
        let mut has_children = false;
        self.visit(|_| {
            has_children = true;
            CXChildVisit_Break
        });
        has_children
    }

    /// Does this cursor have at least `n` children?
    pub(crate) fn has_at_least_num_children(&self, n: usize) -> bool {
        assert!(n > 0);
        let mut num_left = n;
        self.visit(|_| {
            num_left -= 1;
            if num_left == 0 {
                CXChildVisit_Break
            } else {
                CXChildVisit_Continue
            }
        });
        num_left == 0
    }

    /// Returns whether the given location contains a cursor with the given
    /// kind in the first level of nesting underneath (doesn't look
    /// recursively).
    pub(crate) fn contains_cursor(&self, kind: CXCursorKind) -> bool {
        let mut found = false;

        self.visit(|c| {
            if c.kind() == kind {
                found = true;
                CXChildVisit_Break
            } else {
                CXChildVisit_Continue
            }
        });

        found
    }

    /// Is the referent an inlined function?
    pub(crate) fn is_inlined_function(&self) -> bool {
        unsafe { clang_Cursor_isFunctionInlined(self.x) != 0 }
    }

    /// Is the referent a defaulted function?
    pub(crate) fn is_defaulted_function(&self) -> bool {
        unsafe { clang_CXXMethod_isDefaulted(self.x) != 0 }
    }

    /// Is the referent a deleted function?
    pub(crate) fn is_deleted_function(&self) -> bool {
        // Unfortunately, libclang doesn't yet have an API for checking if a
        // member function is deleted, but the following should be a good
        // enough approximation.
        // Deleted functions are implicitly inline according to paragraph 4 of
        // [dcl.fct.def.delete] in the C++ standard. Normal inline functions
        // have a definition in the same translation unit, so if this is an
        // inline function without a definition, and it's not a defaulted
        // function, we can reasonably safely conclude that it's a deleted
        // function.
        self.is_inlined_function() &&
            self.definition().is_none() &&
            !self.is_defaulted_function()
    }

    /// Is the referent a bit field declaration?
    pub(crate) fn is_bit_field(&self) -> bool {
        unsafe { clang_Cursor_isBitField(self.x) != 0 }
    }

    /// Get a cursor to the bit field's width expression, or `None` if it's not
    /// a bit field.
    pub(crate) fn bit_width_expr(&self) -> Option<Cursor> {
        if !self.is_bit_field() {
            return None;
        }

        let mut result = None;
        self.visit(|cur| {
            // The first child may or may not be a TypeRef, depending on whether
            // the field's type is builtin. Skip it.
            if cur.kind() == CXCursor_TypeRef {
                return CXChildVisit_Continue;
            }

            // The next expression or literal is the bit width.
            result = Some(cur);

            CXChildVisit_Break
        });

        result
    }

    /// Get the width of this cursor's referent bit field, or `None` if the
    /// referent is not a bit field or if the width could not be evaluated.
    pub(crate) fn bit_width(&self) -> Option<u32> {
        // It is not safe to check the bit width without ensuring it doesn't
        // depend on a template parameter. See
        // https://github.com/rust-lang/rust-bindgen/issues/2239
        if self.bit_width_expr()?.is_dependent_on_template_parameter() {
            return None;
        }

        unsafe {
            let w = clang_getFieldDeclBitWidth(self.x);
            if w == -1 {
                None
            } else {
                Some(w as u32)
            }
        }
    }

    /// Get the integer representation type used to hold this cursor's referent
    /// enum type.
    pub(crate) fn enum_type(&self) -> Option<Type> {
        unsafe {
            let t = Type {
                x: clang_getEnumDeclIntegerType(self.x),
            };
            if t.is_valid() {
                Some(t)
            } else {
                None
            }
        }
    }

    /// Get the boolean constant value for this cursor's enum variant referent.
    ///
    /// Returns None if the cursor's referent is not an enum variant.
    pub(crate) fn enum_val_boolean(&self) -> Option<bool> {
        unsafe {
            if self.kind() == CXCursor_EnumConstantDecl {
                Some(clang_getEnumConstantDeclValue(self.x) != 0)
            } else {
                None
            }
        }
    }

    /// Get the signed constant value for this cursor's enum variant referent.
    ///
    /// Returns None if the cursor's referent is not an enum variant.
    pub(crate) fn enum_val_signed(&self) -> Option<i64> {
        unsafe {
            if self.kind() == CXCursor_EnumConstantDecl {
                #[allow(clippy::unnecessary_cast)]
                Some(clang_getEnumConstantDeclValue(self.x) as i64)
            } else {
                None
            }
        }
    }

    /// Get the unsigned constant value for this cursor's enum variant referent.
    ///
    /// Returns None if the cursor's referent is not an enum variant.
    pub(crate) fn enum_val_unsigned(&self) -> Option<u64> {
        unsafe {
            if self.kind() == CXCursor_EnumConstantDecl {
                #[allow(clippy::unnecessary_cast)]
                Some(clang_getEnumConstantDeclUnsignedValue(self.x) as u64)
            } else {
                None
            }
        }
    }

    /// Does this cursor have the given attributes?
    pub(crate) fn has_attrs<const N: usize>(
        &self,
        attrs: &[Attribute; N],
    ) -> [bool; N] {
        let mut found_attrs = [false; N];
        let mut found_count = 0;

        self.visit(|cur| {
            let kind = cur.kind();
            for (idx, attr) in attrs.iter().enumerate() {
                let found_attr = &mut found_attrs[idx];
                if !*found_attr {
                    // `attr.name` and` attr.token_kind` are checked against unexposed attributes only.
                    if attr.kind == Some(kind) ||
                        (kind == CXCursor_UnexposedAttr &&
                            cur.tokens().iter().any(|t| {
                                t.kind == attr.token_kind &&
                                    t.spelling() == attr.name
                            }))
                    {
                        *found_attr = true;
                        found_count += 1;

                        if found_count == N {
                            return CXChildVisit_Break;
                        }
                    }
                }
            }

            CXChildVisit_Continue
        });

        found_attrs
    }

    /// Given that this cursor's referent is a `typedef`, get the `Type` that is
    /// being aliased.
    pub(crate) fn typedef_type(&self) -> Option<Type> {
        let inner = Type {
            x: unsafe { clang_getTypedefDeclUnderlyingType(self.x) },
        };

        if inner.is_valid() {
            Some(inner)
        } else {
            None
        }
    }

    /// Get the linkage kind for this cursor's referent.
    ///
    /// This only applies to functions and variables.
    pub(crate) fn linkage(&self) -> CXLinkageKind {
        unsafe { clang_getCursorLinkage(self.x) }
    }

    /// Get the visibility of this cursor's referent.
    pub(crate) fn visibility(&self) -> CXVisibilityKind {
        unsafe { clang_getCursorVisibility(self.x) }
    }

    /// Given that this cursor's referent is a function, return cursors to its
    /// parameters.
    ///
    /// Returns None if the cursor's referent is not a function/method call or
    /// declaration.
    pub(crate) fn args(&self) -> Option<Vec<Cursor>> {
        // match self.kind() {
        // CXCursor_FunctionDecl |
        // CXCursor_CXXMethod => {
        self.num_args().ok().map(|num| {
            (0..num)
                .map(|i| Cursor {
                    x: unsafe { clang_Cursor_getArgument(self.x, i as c_uint) },
                })
                .collect()
        })
    }

    /// Given that this cursor's referent is a function/method call or
    /// declaration, return the number of arguments it takes.
    ///
    /// Returns Err if the cursor's referent is not a function/method call or
    /// declaration.
    pub(crate) fn num_args(&self) -> Result<u32, ()> {
        unsafe {
            let w = clang_Cursor_getNumArguments(self.x);
            if w == -1 {
                Err(())
            } else {
                Ok(w as u32)
            }
        }
    }

    /// Get the access specifier for this cursor's referent.
    pub(crate) fn access_specifier(&self) -> CX_CXXAccessSpecifier {
        unsafe { clang_getCXXAccessSpecifier(self.x) }
    }

    /// Is the cursor's referent publicly accessible in C++?
    ///
    /// Returns true if `self.access_specifier()` is `CX_CXXPublic` or
    /// `CX_CXXInvalidAccessSpecifier`.
    pub(crate) fn public_accessible(&self) -> bool {
        let access = self.access_specifier();
        access == CX_CXXPublic || access == CX_CXXInvalidAccessSpecifier
    }

    /// Is this cursor's referent a field declaration that is marked as
    /// `mutable`?
    pub(crate) fn is_mutable_field(&self) -> bool {
        unsafe { clang_CXXField_isMutable(self.x) != 0 }
    }

    /// Get the offset of the field represented by the Cursor.
    pub(crate) fn offset_of_field(&self) -> Result<usize, LayoutError> {
        let offset = unsafe { clang_Cursor_getOffsetOfField(self.x) };

        if offset < 0 {
            Err(LayoutError::from(offset as i32))
        } else {
            Ok(offset as usize)
        }
    }

    /// Is this cursor's referent a member function that is declared `static`?
    pub(crate) fn method_is_static(&self) -> bool {
        unsafe { clang_CXXMethod_isStatic(self.x) != 0 }
    }

    /// Is this cursor's referent a member function that is declared `const`?
    pub(crate) fn method_is_const(&self) -> bool {
        unsafe { clang_CXXMethod_isConst(self.x) != 0 }
    }

    /// Is this cursor's referent a member function that is virtual?
    pub(crate) fn method_is_virtual(&self) -> bool {
        unsafe { clang_CXXMethod_isVirtual(self.x) != 0 }
    }

    /// Is this cursor's referent a member function that is pure virtual?
    pub(crate) fn method_is_pure_virtual(&self) -> bool {
        unsafe { clang_CXXMethod_isPureVirtual(self.x) != 0 }
    }

    /// Is this cursor's referent a struct or class with virtual members?
    pub(crate) fn is_virtual_base(&self) -> bool {
        unsafe { clang_isVirtualBase(self.x) != 0 }
    }

    /// Try to evaluate this cursor.
    pub(crate) fn evaluate(&self) -> Option<EvalResult> {
        EvalResult::new(*self)
    }

    /// Return the result type for this cursor
    pub(crate) fn ret_type(&self) -> Option<Type> {
        let rt = Type {
            x: unsafe { clang_getCursorResultType(self.x) },
        };
        if rt.is_valid() {
            Some(rt)
        } else {
            None
        }
    }

    /// Gets the tokens that correspond to that cursor.
    pub(crate) fn tokens(&self) -> RawTokens {
        RawTokens::new(self)
    }

    /// Gets the tokens that correspond to that cursor as  `cexpr` tokens.
    pub(crate) fn cexpr_tokens(self) -> Vec<cexpr::token::Token> {
        self.tokens()
            .iter()
            .filter_map(|token| token.as_cexpr_token())
            .collect()
    }

    /// Obtain the real path name of a cursor of `InclusionDirective` kind.
    ///
    /// Returns None if the cursor does not include a file, otherwise the file's full name
    pub(crate) fn get_included_file_name(&self) -> Option<String> {
        let file = unsafe { clang_getIncludedFile(self.x) };
        if file.is_null() {
            None
        } else {
            Some(unsafe { cxstring_into_string(clang_getFileName(file)) })
        }
    }

    /// Is this cursor's referent a namespace that is inline?
    pub(crate) fn is_inline_namespace(&self) -> bool {
        unsafe { clang_Cursor_isInlineNamespace(self.x) != 0 }
    }
}

/// A struct that owns the tokenizer result from a given cursor.
pub(crate) struct RawTokens<'a> {
    cursor: &'a Cursor,
    tu: CXTranslationUnit,
    tokens: *mut CXToken,
    token_count: c_uint,
}

impl<'a> RawTokens<'a> {
    fn new(cursor: &'a Cursor) -> Self {
        let mut tokens = ptr::null_mut();
        let mut token_count = 0;
        let range = cursor.extent();
        let tu = unsafe { clang_Cursor_getTranslationUnit(cursor.x) };
        unsafe { clang_tokenize(tu, range, &mut tokens, &mut token_count) };
        Self {
            cursor,
            tu,
            tokens,
            token_count,
        }
    }

    fn as_slice(&self) -> &[CXToken] {
        if self.tokens.is_null() {
            return &[];
        }
        unsafe { slice::from_raw_parts(self.tokens, self.token_count as usize) }
    }

    /// Get an iterator over these tokens.
    pub(crate) fn iter(&self) -> ClangTokenIterator {
        ClangTokenIterator {
            tu: self.tu,
            raw: self.as_slice().iter(),
        }
    }
}

impl Drop for RawTokens<'_> {
    fn drop(&mut self) {
        if !self.tokens.is_null() {
            unsafe {
                clang_disposeTokens(
                    self.tu,
                    self.tokens,
                    self.token_count as c_uint,
                );
            }
        }
    }
}

/// A raw clang token, that exposes only kind, spelling, and extent. This is a
/// slightly more convenient version of `CXToken` which owns the spelling
/// string and extent.
#[derive(Debug)]
pub(crate) struct ClangToken {
    spelling: CXString,
    /// The extent of the token. This is the same as the relevant member from
    /// `CXToken`.
    pub(crate) extent: CXSourceRange,
    /// The kind of the token. This is the same as the relevant member from
    /// `CXToken`.
    pub(crate) kind: CXTokenKind,
}

impl ClangToken {
    /// Get the token spelling, without being converted to utf-8.
    pub(crate) fn spelling(&self) -> &[u8] {
        let c_str = unsafe {
            CStr::from_ptr(clang_getCString(self.spelling) as *const _)
        };
        c_str.to_bytes()
    }

    /// Converts a `ClangToken` to a `cexpr` token if possible.
    pub(crate) fn as_cexpr_token(&self) -> Option<cexpr::token::Token> {
        use cexpr::token;

        let kind = match self.kind {
            CXToken_Punctuation => token::Kind::Punctuation,
            CXToken_Literal => token::Kind::Literal,
            CXToken_Identifier => token::Kind::Identifier,
            CXToken_Keyword => token::Kind::Keyword,
            // NB: cexpr is not too happy about comments inside
            // expressions, so we strip them down here.
            CXToken_Comment => return None,
            _ => {
                warn!("Found unexpected token kind: {self:?}");
                return None;
            }
        };

        Some(token::Token {
            kind,
            raw: self.spelling().to_vec().into_boxed_slice(),
        })
    }
}

impl Drop for ClangToken {
    fn drop(&mut self) {
        unsafe { clang_disposeString(self.spelling) }
    }
}

/// An iterator over a set of Tokens.
pub(crate) struct ClangTokenIterator<'a> {
    tu: CXTranslationUnit,
    raw: slice::Iter<'a, CXToken>,
}

impl Iterator for ClangTokenIterator<'_> {
    type Item = ClangToken;

    fn next(&mut self) -> Option<Self::Item> {
        let raw = self.raw.next()?;
        unsafe {
            let kind = clang_getTokenKind(*raw);
            let spelling = clang_getTokenSpelling(self.tu, *raw);
            let extent = clang_getTokenExtent(self.tu, *raw);
            Some(ClangToken {
                kind,
                extent,
                spelling,
            })
        }
    }
}

/// Checks whether the name looks like an identifier, i.e. is alphanumeric
/// (including '_') and does not start with a digit.
pub(crate) fn is_valid_identifier(name: &str) -> bool {
    let mut chars = name.chars();
    let first_valid =
        chars.next().is_some_and(|c| c.is_alphabetic() || c == '_');

    first_valid && chars.all(|c| c.is_alphanumeric() || c == '_')
}

extern "C" fn visit_children<Visitor>(
    cur: CXCursor,
    _parent: CXCursor,
    data: CXClientData,
) -> CXChildVisitResult
where
    Visitor: FnMut(Cursor) -> CXChildVisitResult,
{
    let func: &mut Visitor = unsafe { &mut *(data as *mut Visitor) };
    let child = Cursor { x: cur };

    (*func)(child)
}

impl PartialEq for Cursor {
    fn eq(&self, other: &Cursor) -> bool {
        unsafe { clang_equalCursors(self.x, other.x) == 1 }
    }
}

impl Eq for Cursor {}

impl Hash for Cursor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe { clang_hashCursor(self.x) }.hash(state);
    }
}

/// The type of a node in clang's AST.
#[derive(Clone, Copy)]
pub(crate) struct Type {
    x: CXType,
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        unsafe { clang_equalTypes(self.x, other.x) != 0 }
    }
}

impl Eq for Type {}

impl fmt::Debug for Type {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "Type({}, kind: {}, cconv: {}, decl: {:?}, canon: {:?})",
            self.spelling(),
            type_to_str(self.kind()),
            self.call_conv(),
            self.declaration(),
            self.declaration().canonical()
        )
    }
}

/// An error about the layout of a struct, class, or type.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum LayoutError {
    /// Asked for the layout of an invalid type.
    Invalid,
    /// Asked for the layout of an incomplete type.
    Incomplete,
    /// Asked for the layout of a dependent type.
    Dependent,
    /// Asked for the layout of a type that does not have constant size.
    NotConstantSize,
    /// Asked for the layout of a field in a type that does not have such a
    /// field.
    InvalidFieldName,
    /// An unknown layout error.
    Unknown,
}

impl ::std::convert::From<i32> for LayoutError {
    fn from(val: i32) -> Self {
        use self::LayoutError::*;

        match val {
            CXTypeLayoutError_Invalid => Invalid,
            CXTypeLayoutError_Incomplete => Incomplete,
            CXTypeLayoutError_Dependent => Dependent,
            CXTypeLayoutError_NotConstantSize => NotConstantSize,
            CXTypeLayoutError_InvalidFieldName => InvalidFieldName,
            _ => Unknown,
        }
    }
}

impl Type {
    /// Get this type's kind.
    pub(crate) fn kind(&self) -> CXTypeKind {
        self.x.kind
    }

    /// Get a cursor pointing to this type's declaration.
    pub(crate) fn declaration(&self) -> Cursor {
        unsafe {
            Cursor {
                x: clang_getTypeDeclaration(self.x),
            }
        }
    }

    /// Get the canonical declaration of this type, if it is available.
    pub(crate) fn canonical_declaration(
        &self,
        location: Option<&Cursor>,
    ) -> Option<CanonicalTypeDeclaration> {
        let mut declaration = self.declaration();
        if !declaration.is_valid() {
            if let Some(location) = location {
                let mut location = *location;
                if let Some(referenced) = location.referenced() {
                    location = referenced;
                }
                if location.is_template_like() {
                    declaration = location;
                }
            }
        }

        let canonical = declaration.canonical();
        if canonical.is_valid() && canonical.kind() != CXCursor_NoDeclFound {
            Some(CanonicalTypeDeclaration(*self, canonical))
        } else {
            None
        }
    }

    /// Get a raw display name for this type.
    pub(crate) fn spelling(&self) -> String {
        let s = unsafe { cxstring_into_string(clang_getTypeSpelling(self.x)) };
        // Clang 5.0 introduced changes in the spelling API so it returned the
        // full qualified name. Let's undo that here.
        if s.split("::").all(is_valid_identifier) {
            if let Some(s) = s.split("::").last() {
                return s.to_owned();
            }
        }

        s
    }

    /// Is this type const qualified?
    pub(crate) fn is_const(&self) -> bool {
        unsafe { clang_isConstQualifiedType(self.x) != 0 }
    }

    #[inline]
    fn is_non_deductible_auto_type(&self) -> bool {
        debug_assert_eq!(self.kind(), CXType_Auto);
        self.canonical_type() == *self
    }

    #[inline]
    fn clang_size_of(&self, ctx: &BindgenContext) -> c_longlong {
        match self.kind() {
            // Work-around https://bugs.llvm.org/show_bug.cgi?id=40975
            CXType_RValueReference | CXType_LValueReference => {
                ctx.target_pointer_size() as c_longlong
            }
            // Work-around https://bugs.llvm.org/show_bug.cgi?id=40813
            CXType_Auto if self.is_non_deductible_auto_type() => -6,
            _ => unsafe { clang_Type_getSizeOf(self.x) },
        }
    }

    #[inline]
    fn clang_align_of(&self, ctx: &BindgenContext) -> c_longlong {
        match self.kind() {
            // Work-around https://bugs.llvm.org/show_bug.cgi?id=40975
            CXType_RValueReference | CXType_LValueReference => {
                ctx.target_pointer_size() as c_longlong
            }
            // Work-around https://bugs.llvm.org/show_bug.cgi?id=40813
            CXType_Auto if self.is_non_deductible_auto_type() => -6,
            _ => unsafe { clang_Type_getAlignOf(self.x) },
        }
    }

    /// What is the size of this type? Paper over invalid types by returning `0`
    /// for them.
    pub(crate) fn size(&self, ctx: &BindgenContext) -> usize {
        let val = self.clang_size_of(ctx);
        if val < 0 {
            0
        } else {
            val as usize
        }
    }

    /// What is the size of this type?
    pub(crate) fn fallible_size(
        &self,
        ctx: &BindgenContext,
    ) -> Result<usize, LayoutError> {
        let val = self.clang_size_of(ctx);
        if val < 0 {
            Err(LayoutError::from(val as i32))
        } else {
            Ok(val as usize)
        }
    }

    /// What is the alignment of this type? Paper over invalid types by
    /// returning `0`.
    pub(crate) fn align(&self, ctx: &BindgenContext) -> usize {
        let val = self.clang_align_of(ctx);
        if val < 0 {
            0
        } else {
            val as usize
        }
    }

    /// What is the alignment of this type?
    pub(crate) fn fallible_align(
        &self,
        ctx: &BindgenContext,
    ) -> Result<usize, LayoutError> {
        let val = self.clang_align_of(ctx);
        if val < 0 {
            Err(LayoutError::from(val as i32))
        } else {
            Ok(val as usize)
        }
    }

    /// Get the layout for this type, or an error describing why it does not
    /// have a valid layout.
    pub(crate) fn fallible_layout(
        &self,
        ctx: &BindgenContext,
    ) -> Result<crate::ir::layout::Layout, LayoutError> {
        use crate::ir::layout::Layout;
        let size = self.fallible_size(ctx)?;
        let align = self.fallible_align(ctx)?;
        Ok(Layout::new(size, align))
    }

    /// Get the number of template arguments this type has, or `None` if it is
    /// not some kind of template.
    pub(crate) fn num_template_args(&self) -> Option<u32> {
        let n = unsafe { clang_Type_getNumTemplateArguments(self.x) };
        if n >= 0 {
            Some(n as u32)
        } else {
            debug_assert_eq!(n, -1);
            None
        }
    }

    /// If this type is a class template specialization, return its
    /// template arguments. Otherwise, return None.
    pub(crate) fn template_args(&self) -> Option<TypeTemplateArgIterator> {
        self.num_template_args().map(|n| TypeTemplateArgIterator {
            x: self.x,
            length: n,
            index: 0,
        })
    }

    /// Given that this type is a function prototype, return the types of its parameters.
    ///
    /// Returns None if the type is not a function prototype.
    pub(crate) fn args(&self) -> Option<Vec<Type>> {
        self.num_args().ok().map(|num| {
            (0..num)
                .map(|i| Type {
                    x: unsafe { clang_getArgType(self.x, i as c_uint) },
                })
                .collect()
        })
    }

    /// Given that this type is a function prototype, return the number of arguments it takes.
    ///
    /// Returns Err if the type is not a function prototype.
    pub(crate) fn num_args(&self) -> Result<u32, ()> {
        unsafe {
            let w = clang_getNumArgTypes(self.x);
            if w == -1 {
                Err(())
            } else {
                Ok(w as u32)
            }
        }
    }

    /// Given that this type is a pointer type, return the type that it points
    /// to.
    pub(crate) fn pointee_type(&self) -> Option<Type> {
        match self.kind() {
            CXType_Pointer |
            CXType_RValueReference |
            CXType_LValueReference |
            CXType_MemberPointer |
            CXType_BlockPointer |
            CXType_ObjCObjectPointer => {
                let ret = Type {
                    x: unsafe { clang_getPointeeType(self.x) },
                };
                debug_assert!(ret.is_valid());
                Some(ret)
            }
            _ => None,
        }
    }

    /// Given that this type is an array, vector, or complex type, return the
    /// type of its elements.
    pub(crate) fn elem_type(&self) -> Option<Type> {
        let current_type = Type {
            x: unsafe { clang_getElementType(self.x) },
        };
        if current_type.is_valid() {
            Some(current_type)
        } else {
            None
        }
    }

    /// Given that this type is an array or vector type, return its number of
    /// elements.
    pub(crate) fn num_elements(&self) -> Option<usize> {
        let num_elements_returned = unsafe { clang_getNumElements(self.x) };
        if num_elements_returned == -1 {
            None
        } else {
            Some(num_elements_returned as usize)
        }
    }

    /// Get the canonical version of this type. This sees through `typedef`s and
    /// aliases to get the underlying, canonical type.
    pub(crate) fn canonical_type(&self) -> Type {
        unsafe {
            Type {
                x: clang_getCanonicalType(self.x),
            }
        }
    }

    /// Is this type a variadic function type?
    pub(crate) fn is_variadic(&self) -> bool {
        unsafe { clang_isFunctionTypeVariadic(self.x) != 0 }
    }

    /// Given that this type is a function type, get the type of its return
    /// value.
    pub(crate) fn ret_type(&self) -> Option<Type> {
        let rt = Type {
            x: unsafe { clang_getResultType(self.x) },
        };
        if rt.is_valid() {
            Some(rt)
        } else {
            None
        }
    }

    /// Given that this type is a function type, get its calling convention. If
    /// this is not a function type, `CXCallingConv_Invalid` is returned.
    pub(crate) fn call_conv(&self) -> CXCallingConv {
        unsafe { clang_getFunctionTypeCallingConv(self.x) }
    }

    /// For elaborated types (types which use `class`, `struct`, or `union` to
    /// disambiguate types from local bindings), get the underlying type.
    pub(crate) fn named(&self) -> Type {
        unsafe {
            Type {
                x: clang_Type_getNamedType(self.x),
            }
        }
    }

    /// For atomic types, get the underlying type.
    pub(crate) fn atomic_value_type(&self) -> Type {
        unsafe {
            Type {
                x: clang_Type_getValueType(self.x),
            }
        }
    }

    /// Is this a valid type?
    pub(crate) fn is_valid(&self) -> bool {
        self.kind() != CXType_Invalid
    }

    /// Is this a valid and exposed type?
    pub(crate) fn is_valid_and_exposed(&self) -> bool {
        self.is_valid() && self.kind() != CXType_Unexposed
    }

    /// Is this type a fully instantiated template?
    pub(crate) fn is_fully_instantiated_template(&self) -> bool {
        // Yep, the spelling of this containing type-parameter is extremely
        // nasty... But can happen in <type_traits>. Unfortunately I couldn't
        // reduce it enough :(
        self.template_args().is_some_and(|args| args.len() > 0) &&
            !matches!(
                self.declaration().kind(),
                CXCursor_ClassTemplatePartialSpecialization |
                    CXCursor_TypeAliasTemplateDecl |
                    CXCursor_TemplateTemplateParameter
            )
    }

    /// Is this type an associated template type? Eg `T::Associated` in
    /// this example:
    ///
    /// ```c++
    /// template <typename T>
    /// class Foo {
    ///     typename T::Associated member;
    /// };
    /// ```
    pub(crate) fn is_associated_type(&self) -> bool {
        // This is terrible :(
        fn hacky_parse_associated_type<S: AsRef<str>>(spelling: S) -> bool {
            static ASSOC_TYPE_RE: OnceLock<regex::Regex> = OnceLock::new();
            ASSOC_TYPE_RE
                .get_or_init(|| {
                    regex::Regex::new(r"typename type\-parameter\-\d+\-\d+::.+")
                        .unwrap()
                })
                .is_match(spelling.as_ref())
        }

        self.kind() == CXType_Unexposed &&
            (hacky_parse_associated_type(self.spelling()) ||
                hacky_parse_associated_type(
                    self.canonical_type().spelling(),
                ))
    }
}

/// The `CanonicalTypeDeclaration` type exists as proof-by-construction that its
/// cursor is the canonical declaration for its type. If you have a
/// `CanonicalTypeDeclaration` instance, you know for sure that the type and
/// cursor match up in a canonical declaration relationship, and it simply
/// cannot be otherwise.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CanonicalTypeDeclaration(Type, Cursor);

impl CanonicalTypeDeclaration {
    /// Get the type.
    pub(crate) fn ty(&self) -> &Type {
        &self.0
    }

    /// Get the type's canonical declaration cursor.
    pub(crate) fn cursor(&self) -> &Cursor {
        &self.1
    }
}

/// An iterator for a type's template arguments.
pub(crate) struct TypeTemplateArgIterator {
    x: CXType,
    length: u32,
    index: u32,
}

impl Iterator for TypeTemplateArgIterator {
    type Item = Type;
    fn next(&mut self) -> Option<Type> {
        if self.index < self.length {
            let idx = self.index as c_uint;
            self.index += 1;
            Some(Type {
                x: unsafe { clang_Type_getTemplateArgumentAsType(self.x, idx) },
            })
        } else {
            None
        }
    }
}

impl ExactSizeIterator for TypeTemplateArgIterator {
    fn len(&self) -> usize {
        assert!(self.index <= self.length);
        (self.length - self.index) as usize
    }
}

/// A `SourceLocation` is a file, line, column, and byte offset location for
/// some source text.
pub(crate) struct SourceLocation {
    x: CXSourceLocation,
}

impl SourceLocation {
    /// Get the (file, line, column, byte offset) tuple for this source
    /// location.
    pub(crate) fn location(&self) -> (File, usize, usize, usize) {
        unsafe {
            let mut file = mem::zeroed();
            let mut line = 0;
            let mut col = 0;
            let mut off = 0;
            clang_getFileLocation(
                self.x, &mut file, &mut line, &mut col, &mut off,
            );
            (File { x: file }, line as usize, col as usize, off as usize)
        }
    }
}

impl fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (file, line, col, _) = self.location();
        if let Some(name) = file.name() {
            write!(f, "{name}:{line}:{col}")
        } else {
            "builtin definitions".fmt(f)
        }
    }
}

impl fmt::Debug for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self}")
    }
}

/// A comment in the source text.
///
/// Comments are sort of parsed by Clang, and have a tree structure.
pub(crate) struct Comment {
    x: CXComment,
}

impl Comment {
    /// What kind of comment is this?
    pub(crate) fn kind(&self) -> CXCommentKind {
        unsafe { clang_Comment_getKind(self.x) }
    }

    /// Get this comment's children comment
    pub(crate) fn get_children(&self) -> CommentChildrenIterator {
        CommentChildrenIterator {
            parent: self.x,
            length: unsafe { clang_Comment_getNumChildren(self.x) },
            index: 0,
        }
    }

    /// Given that this comment is the start or end of an HTML tag, get its tag
    /// name.
    pub(crate) fn get_tag_name(&self) -> String {
        unsafe { cxstring_into_string(clang_HTMLTagComment_getTagName(self.x)) }
    }

    /// Given that this comment is an HTML start tag, get its attributes.
    pub(crate) fn get_tag_attrs(&self) -> CommentAttributesIterator {
        CommentAttributesIterator {
            x: self.x,
            length: unsafe { clang_HTMLStartTag_getNumAttrs(self.x) },
            index: 0,
        }
    }
}

/// An iterator for a comment's children
pub(crate) struct CommentChildrenIterator {
    parent: CXComment,
    length: c_uint,
    index: c_uint,
}

impl Iterator for CommentChildrenIterator {
    type Item = Comment;
    fn next(&mut self) -> Option<Comment> {
        if self.index < self.length {
            let idx = self.index;
            self.index += 1;
            Some(Comment {
                x: unsafe { clang_Comment_getChild(self.parent, idx) },
            })
        } else {
            None
        }
    }
}

/// An HTML start tag comment attribute
pub(crate) struct CommentAttribute {
    /// HTML start tag attribute name
    pub(crate) name: String,
    /// HTML start tag attribute value
    pub(crate) value: String,
}

/// An iterator for a comment's attributes
pub(crate) struct CommentAttributesIterator {
    x: CXComment,
    length: c_uint,
    index: c_uint,
}

impl Iterator for CommentAttributesIterator {
    type Item = CommentAttribute;
    fn next(&mut self) -> Option<CommentAttribute> {
        if self.index < self.length {
            let idx = self.index;
            self.index += 1;
            Some(CommentAttribute {
                name: unsafe {
                    cxstring_into_string(clang_HTMLStartTag_getAttrName(
                        self.x, idx,
                    ))
                },
                value: unsafe {
                    cxstring_into_string(clang_HTMLStartTag_getAttrValue(
                        self.x, idx,
                    ))
                },
            })
        } else {
            None
        }
    }
}

/// A source file.
pub(crate) struct File {
    x: CXFile,
}

impl File {
    /// Get the name of this source file.
    pub(crate) fn name(&self) -> Option<String> {
        if self.x.is_null() {
            return None;
        }
        Some(unsafe { cxstring_into_string(clang_getFileName(self.x)) })
    }
}

fn cxstring_to_string_leaky(s: CXString) -> String {
    if s.data.is_null() {
        return "".to_owned();
    }
    let c_str = unsafe { CStr::from_ptr(clang_getCString(s) as *const _) };
    c_str.to_string_lossy().into_owned()
}

fn cxstring_into_string(s: CXString) -> String {
    let ret = cxstring_to_string_leaky(s);
    unsafe { clang_disposeString(s) };
    ret
}

/// An `Index` is an environment for a set of translation units that will
/// typically end up linked together in one final binary.
pub(crate) struct Index {
    x: CXIndex,
}

impl Index {
    /// Construct a new `Index`.
    ///
    /// The `pch` parameter controls whether declarations in pre-compiled
    /// headers are included when enumerating a translation unit's "locals".
    ///
    /// The `diag` parameter controls whether debugging diagnostics are enabled.
    pub(crate) fn new(pch: bool, diag: bool) -> Index {
        unsafe {
            Index {
                x: clang_createIndex(c_int::from(pch), c_int::from(diag)),
            }
        }
    }
}

impl fmt::Debug for Index {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Index {{ }}")
    }
}

impl Drop for Index {
    fn drop(&mut self) {
        unsafe {
            clang_disposeIndex(self.x);
        }
    }
}

/// A translation unit (or "compilation unit").
pub(crate) struct TranslationUnit {
    x: CXTranslationUnit,
}

impl fmt::Debug for TranslationUnit {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "TranslationUnit {{ }}")
    }
}

impl TranslationUnit {
    /// Parse a source file into a translation unit.
    pub(crate) fn parse(
        ix: &Index,
        file: &str,
        cmd_args: &[Box<str>],
        unsaved: &[UnsavedFile],
        opts: CXTranslationUnit_Flags,
    ) -> Option<TranslationUnit> {
        let fname = CString::new(file).unwrap();
        let _c_args: Vec<CString> = cmd_args
            .iter()
            .map(|s| CString::new(s.as_bytes()).unwrap())
            .collect();
        let c_args: Vec<*const c_char> =
            _c_args.iter().map(|s| s.as_ptr()).collect();
        let mut c_unsaved: Vec<CXUnsavedFile> =
            unsaved.iter().map(|f| f.x).collect();
        let tu = unsafe {
            clang_parseTranslationUnit(
                ix.x,
                fname.as_ptr(),
                c_args.as_ptr(),
                c_args.len() as c_int,
                c_unsaved.as_mut_ptr(),
                c_unsaved.len() as c_uint,
                opts,
            )
        };
        if tu.is_null() {
            None
        } else {
            Some(TranslationUnit { x: tu })
        }
    }

    /// Get the Clang diagnostic information associated with this translation
    /// unit.
    pub(crate) fn diags(&self) -> Vec<Diagnostic> {
        unsafe {
            let num = clang_getNumDiagnostics(self.x) as usize;
            let mut diags = vec![];
            for i in 0..num {
                diags.push(Diagnostic {
                    x: clang_getDiagnostic(self.x, i as c_uint),
                });
            }
            diags
        }
    }

    /// Get a cursor pointing to the root of this translation unit's AST.
    pub(crate) fn cursor(&self) -> Cursor {
        unsafe {
            Cursor {
                x: clang_getTranslationUnitCursor(self.x),
            }
        }
    }

    /// Save a translation unit to the given file.
    pub(crate) fn save(&mut self, file: &str) -> Result<(), CXSaveError> {
        let Ok(file) = CString::new(file) else {
            return Err(CXSaveError_Unknown);
        };
        let ret = unsafe {
            clang_saveTranslationUnit(
                self.x,
                file.as_ptr(),
                clang_defaultSaveOptions(self.x),
            )
        };
        if ret != 0 {
            Err(ret)
        } else {
            Ok(())
        }
    }

    /// Is this the null translation unit?
    pub(crate) fn is_null(&self) -> bool {
        self.x.is_null()
    }
}

impl Drop for TranslationUnit {
    fn drop(&mut self) {
        unsafe {
            clang_disposeTranslationUnit(self.x);
        }
    }
}

/// Translation unit used for macro fallback parsing
pub(crate) struct FallbackTranslationUnit {
    file_path: String,
    header_path: String,
    pch_path: String,
    idx: Box<Index>,
    tu: TranslationUnit,
}

impl fmt::Debug for FallbackTranslationUnit {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "FallbackTranslationUnit {{ }}")
    }
}

impl FallbackTranslationUnit {
    /// Create a new fallback translation unit
    pub(crate) fn new(
        file: String,
        header_path: String,
        pch_path: String,
        c_args: &[Box<str>],
    ) -> Option<Self> {
        // Create empty file
        OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&file)
            .ok()?;

        let f_index = Box::new(Index::new(true, false));
        let f_translation_unit = TranslationUnit::parse(
            &f_index,
            &file,
            c_args,
            &[],
            CXTranslationUnit_None,
        )?;
        Some(FallbackTranslationUnit {
            file_path: file,
            header_path,
            pch_path,
            tu: f_translation_unit,
            idx: f_index,
        })
    }

    /// Get reference to underlying translation unit.
    pub(crate) fn translation_unit(&self) -> &TranslationUnit {
        &self.tu
    }

    /// Reparse a translation unit.
    pub(crate) fn reparse(
        &mut self,
        unsaved_contents: &str,
    ) -> Result<(), CXErrorCode> {
        let unsaved = &[UnsavedFile::new(&self.file_path, unsaved_contents)];
        let mut c_unsaved: Vec<CXUnsavedFile> =
            unsaved.iter().map(|f| f.x).collect();
        let ret = unsafe {
            clang_reparseTranslationUnit(
                self.tu.x,
                unsaved.len() as c_uint,
                c_unsaved.as_mut_ptr(),
                clang_defaultReparseOptions(self.tu.x),
            )
        };
        if ret != 0 {
            Err(ret)
        } else {
            Ok(())
        }
    }
}

impl Drop for FallbackTranslationUnit {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.file_path);
        let _ = std::fs::remove_file(&self.header_path);
        let _ = std::fs::remove_file(&self.pch_path);
    }
}

/// A diagnostic message generated while parsing a translation unit.
pub(crate) struct Diagnostic {
    x: CXDiagnostic,
}

impl Diagnostic {
    /// Format this diagnostic message as a string, using the given option bit
    /// flags.
    pub(crate) fn format(&self) -> String {
        unsafe {
            let opts = clang_defaultDiagnosticDisplayOptions();
            cxstring_into_string(clang_formatDiagnostic(self.x, opts))
        }
    }

    /// What is the severity of this diagnostic message?
    pub(crate) fn severity(&self) -> CXDiagnosticSeverity {
        unsafe { clang_getDiagnosticSeverity(self.x) }
    }
}

impl Drop for Diagnostic {
    /// Destroy this diagnostic message.
    fn drop(&mut self) {
        unsafe {
            clang_disposeDiagnostic(self.x);
        }
    }
}

/// A file which has not been saved to disk.
pub(crate) struct UnsavedFile {
    x: CXUnsavedFile,
    /// The name of the unsaved file. Kept here to avoid leaving dangling pointers in
    /// `CXUnsavedFile`.
    pub(crate) name: CString,
    contents: CString,
}

impl UnsavedFile {
    /// Construct a new unsaved file with the given `name` and `contents`.
    pub(crate) fn new(name: &str, contents: &str) -> UnsavedFile {
        let name = CString::new(name.as_bytes()).unwrap();
        let contents = CString::new(contents.as_bytes()).unwrap();
        let x = CXUnsavedFile {
            Filename: name.as_ptr(),
            Contents: contents.as_ptr(),
            Length: contents.as_bytes().len() as c_ulong,
        };
        UnsavedFile { x, name, contents }
    }
}

impl fmt::Debug for UnsavedFile {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "UnsavedFile(name: {:?}, contents: {:?})",
            self.name, self.contents
        )
    }
}

/// Convert a cursor kind into a static string.
pub(crate) fn kind_to_str(x: CXCursorKind) -> String {
    unsafe { cxstring_into_string(clang_getCursorKindSpelling(x)) }
}

/// Convert a type kind to a static string.
pub(crate) fn type_to_str(x: CXTypeKind) -> String {
    unsafe { cxstring_into_string(clang_getTypeKindSpelling(x)) }
}

/// Dump the Clang AST to stdout for debugging purposes.
pub(crate) fn ast_dump(c: &Cursor, depth: isize) -> CXChildVisitResult {
    fn print_indent<S: AsRef<str>>(depth: isize, s: S) {
        for _ in 0..depth {
            print!("    ");
        }
        println!("{}", s.as_ref());
    }

    fn print_cursor<S: AsRef<str>>(depth: isize, prefix: S, c: &Cursor) {
        let prefix = prefix.as_ref();
        print_indent(
            depth,
            format!(" {prefix}kind = {}", kind_to_str(c.kind())),
        );
        print_indent(
            depth,
            format!(" {prefix}spelling = \"{}\"", c.spelling()),
        );
        print_indent(depth, format!(" {prefix}location = {}", c.location()));
        print_indent(
            depth,
            format!(" {prefix}is-definition? {}", c.is_definition()),
        );
        print_indent(
            depth,
            format!(" {prefix}is-declaration? {}", c.is_declaration()),
        );
        print_indent(
            depth,
            format!(
                " {prefix}is-inlined-function? {}",
                c.is_inlined_function()
            ),
        );

        let templ_kind = c.template_kind();
        if templ_kind != CXCursor_NoDeclFound {
            print_indent(
                depth,
                format!(" {prefix}template-kind = {}", kind_to_str(templ_kind)),
            );
        }
        if let Some(usr) = c.usr() {
            print_indent(depth, format!(" {prefix}usr = \"{usr}\""));
        }
        if let Ok(num) = c.num_args() {
            print_indent(depth, format!(" {prefix}number-of-args = {num}"));
        }
        if let Some(num) = c.num_template_args() {
            print_indent(
                depth,
                format!(" {prefix}number-of-template-args = {num}"),
            );
        }

        if c.is_bit_field() {
            let width = match c.bit_width() {
                Some(w) => w.to_string(),
                None => "<unevaluable>".to_string(),
            };
            print_indent(depth, format!(" {prefix}bit-width = {width}"));
        }

        if let Some(ty) = c.enum_type() {
            print_indent(
                depth,
                format!(" {prefix}enum-type = {}", type_to_str(ty.kind())),
            );
        }
        if let Some(val) = c.enum_val_signed() {
            print_indent(depth, format!(" {prefix}enum-val = {val}"));
        }
        if let Some(ty) = c.typedef_type() {
            print_indent(
                depth,
                format!(" {prefix}typedef-type = {}", type_to_str(ty.kind())),
            );
        }
        if let Some(ty) = c.ret_type() {
            print_indent(
                depth,
                format!(" {prefix}ret-type = {}", type_to_str(ty.kind())),
            );
        }

        if let Some(refd) = c.referenced() {
            if refd != *c {
                println!();
                print_cursor(
                    depth,
                    String::from(prefix) + "referenced.",
                    &refd,
                );
            }
        }

        let canonical = c.canonical();
        if canonical != *c {
            println!();
            print_cursor(
                depth,
                String::from(prefix) + "canonical.",
                &canonical,
            );
        }

        if let Some(specialized) = c.specialized() {
            if specialized != *c {
                println!();
                print_cursor(
                    depth,
                    String::from(prefix) + "specialized.",
                    &specialized,
                );
            }
        }

        if let Some(parent) = c.fallible_semantic_parent() {
            println!();
            print_cursor(
                depth,
                String::from(prefix) + "semantic-parent.",
                &parent,
            );
        }
    }

    fn print_type<S: AsRef<str>>(depth: isize, prefix: S, ty: &Type) {
        let prefix = prefix.as_ref();

        let kind = ty.kind();
        print_indent(depth, format!(" {prefix}kind = {}", type_to_str(kind)));
        if kind == CXType_Invalid {
            return;
        }

        print_indent(depth, format!(" {prefix}cconv = {}", ty.call_conv()));

        print_indent(
            depth,
            format!(" {prefix}spelling = \"{}\"", ty.spelling()),
        );
        let num_template_args =
            unsafe { clang_Type_getNumTemplateArguments(ty.x) };
        if num_template_args >= 0 {
            print_indent(
                depth,
                format!(
                    " {prefix}number-of-template-args = {num_template_args}"
                ),
            );
        }
        if let Some(num) = ty.num_elements() {
            print_indent(depth, format!(" {prefix}number-of-elements = {num}"));
        }
        print_indent(
            depth,
            format!(" {prefix}is-variadic? {}", ty.is_variadic()),
        );

        let canonical = ty.canonical_type();
        if canonical != *ty {
            println!();
            print_type(depth, String::from(prefix) + "canonical.", &canonical);
        }

        if let Some(pointee) = ty.pointee_type() {
            if pointee != *ty {
                println!();
                print_type(depth, String::from(prefix) + "pointee.", &pointee);
            }
        }

        if let Some(elem) = ty.elem_type() {
            if elem != *ty {
                println!();
                print_type(depth, String::from(prefix) + "elements.", &elem);
            }
        }

        if let Some(ret) = ty.ret_type() {
            if ret != *ty {
                println!();
                print_type(depth, String::from(prefix) + "return.", &ret);
            }
        }

        let named = ty.named();
        if named != *ty && named.is_valid() {
            println!();
            print_type(depth, String::from(prefix) + "named.", &named);
        }
    }

    print_indent(depth, "(");
    print_cursor(depth, "", c);

    println!();
    let ty = c.cur_type();
    print_type(depth, "type.", &ty);

    let declaration = ty.declaration();
    if declaration != *c && declaration.kind() != CXCursor_NoDeclFound {
        println!();
        print_cursor(depth, "type.declaration.", &declaration);
    }

    // Recurse.
    let mut found_children = false;
    c.visit(|s| {
        if !found_children {
            println!();
            found_children = true;
        }
        ast_dump(&s, depth + 1)
    });

    print_indent(depth, ")");

    CXChildVisit_Continue
}

/// Try to extract the clang version to a string
pub(crate) fn extract_clang_version() -> String {
    unsafe { cxstring_into_string(clang_getClangVersion()) }
}

/// A wrapper for the result of evaluating an expression.
#[derive(Debug)]
pub(crate) struct EvalResult {
    x: CXEvalResult,
    ty: Type,
}

impl EvalResult {
    /// Evaluate `cursor` and return the result.
    pub(crate) fn new(cursor: Cursor) -> Option<Self> {
        // Work around https://bugs.llvm.org/show_bug.cgi?id=42532, see:
        //  * https://github.com/rust-lang/rust-bindgen/issues/283
        //  * https://github.com/rust-lang/rust-bindgen/issues/1590
        {
            let mut found_cant_eval = false;
            cursor.visit(|c| {
                if c.kind() == CXCursor_TypeRef &&
                    c.cur_type().canonical_type().kind() == CXType_Unexposed
                {
                    found_cant_eval = true;
                    return CXChildVisit_Break;
                }

                CXChildVisit_Recurse
            });

            if found_cant_eval {
                return None;
            }
        }
        Some(EvalResult {
            x: unsafe { clang_Cursor_Evaluate(cursor.x) },
            ty: cursor.cur_type().canonical_type(),
        })
    }

    fn kind(&self) -> CXEvalResultKind {
        unsafe { clang_EvalResult_getKind(self.x) }
    }

    /// Try to get back the result as a double.
    pub(crate) fn as_double(&self) -> Option<f64> {
        match self.kind() {
            CXEval_Float => {
                Some(unsafe { clang_EvalResult_getAsDouble(self.x) })
            }
            _ => None,
        }
    }

    /// Try to get back the result as an integer.
    pub(crate) fn as_int(&self) -> Option<i64> {
        if self.kind() != CXEval_Int {
            return None;
        }

        if unsafe { clang_EvalResult_isUnsignedInt(self.x) } != 0 {
            let value = unsafe { clang_EvalResult_getAsUnsigned(self.x) };
            if value > i64::MAX as c_ulonglong {
                return None;
            }

            return Some(value as i64);
        }

        let value = unsafe { clang_EvalResult_getAsLongLong(self.x) };
        if value > i64::MAX as c_longlong {
            return None;
        }
        if value < i64::MIN as c_longlong {
            return None;
        }
        #[allow(clippy::unnecessary_cast)]
        Some(value as i64)
    }

    /// Evaluates the expression as a literal string, that may or may not be
    /// valid utf-8.
    pub(crate) fn as_literal_string(&self) -> Option<Vec<u8>> {
        if self.kind() != CXEval_StrLiteral {
            return None;
        }

        let char_ty = self.ty.pointee_type().or_else(|| self.ty.elem_type())?;
        match char_ty.kind() {
            CXType_Char_S | CXType_SChar | CXType_Char_U | CXType_UChar => {
                let ret = unsafe {
                    CStr::from_ptr(clang_EvalResult_getAsStr(self.x))
                };
                Some(ret.to_bytes().to_vec())
            }
            // FIXME: Support generating these.
            CXType_Char16 => None,
            CXType_Char32 => None,
            CXType_WChar => None,
            _ => None,
        }
    }
}

impl Drop for EvalResult {
    fn drop(&mut self) {
        unsafe { clang_EvalResult_dispose(self.x) };
    }
}
/// ABI kinds as defined in
/// <https://github.com/llvm/llvm-project/blob/ddf1de20a3f7db3bca1ef6ba7e6cbb90aac5fd2d/clang/include/clang/Basic/TargetCXXABI.def>
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub(crate) enum ABIKind {
    /// All the regular targets like Linux, Mac, WASM, etc. implement the Itanium ABI
    GenericItanium,
    /// The ABI used when compiling for the MSVC target
    Microsoft,
}

/// Target information obtained from libclang.
#[derive(Debug)]
pub(crate) struct TargetInfo {
    /// The target triple.
    pub(crate) triple: String,
    /// The width of the pointer _in bits_.
    pub(crate) pointer_width: usize,
    /// The ABI of the target
    pub(crate) abi: ABIKind,
}

impl TargetInfo {
    /// Tries to obtain target information from libclang.
    pub(crate) fn new(tu: &TranslationUnit) -> Self {
        let triple;
        let pointer_width;
        unsafe {
            let ti = clang_getTranslationUnitTargetInfo(tu.x);
            triple = cxstring_into_string(clang_TargetInfo_getTriple(ti));
            pointer_width = clang_TargetInfo_getPointerWidth(ti);
            clang_TargetInfo_dispose(ti);
        }
        assert!(pointer_width > 0);
        assert_eq!(pointer_width % 8, 0);

        let abi = if triple.contains("msvc") {
            ABIKind::Microsoft
        } else {
            ABIKind::GenericItanium
        };

        TargetInfo {
            triple,
            pointer_width: pointer_width as usize,
            abi,
        }
    }
}

use crate::ast::{self, kw};
use crate::parser::{Cursor, Parse, Parser, Peek, Result};

/// A nested WebAssembly nested module to be created as part of a module.
#[derive(Debug)]
pub struct NestedModule<'a> {
    /// Where this `nested module` was defined.
    pub span: ast::Span,
    /// An identifier that this nested module is resolved with (optionally) for name
    /// resolution.
    pub id: Option<ast::Id<'a>>,
    /// An optional name for this module stored in the custom `name` section.
    pub name: Option<ast::NameAnnotation<'a>>,
    /// If present, inline export annotations which indicate names this
    /// definition should be exported under.
    pub exports: ast::InlineExport<'a>,
    /// What kind of nested module this is, be it an inline-defined or imported one.
    pub kind: NestedModuleKind<'a>,
}

/// Possible ways to define a nested module in the text format.
#[derive(Debug)]
pub enum NestedModuleKind<'a> {
    /// An nested module which is actually defined as an import, such as:
    Import {
        /// Where this nested module is imported from
        import: ast::InlineImport<'a>,
        /// The type that this nested module will have.
        ty: ast::TypeUse<'a, ast::ModuleType<'a>>,
    },

    /// Nested modules whose instantiation is defined inline.
    Inline {
        /// Fields in the nested module.
        fields: Vec<ast::ModuleField<'a>>,
    },
}

impl<'a> Parse<'a> for NestedModule<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        // This is sort of a fundamental limitation of the way this crate is
        // designed. Everything is done through recursive descent parsing which
        // means, well, that we're recursively going down the stack as we parse
        // nested data structures. While we can handle this for wasm expressions
        // since that's a pretty local decision, handling this for nested
        // modules which be far trickier. For now we just say that when the
        // parser goes too deep we return an error saying there's too many
        // nested modules. It would be great to not return an error here,
        // though!
        if parser.parens_depth() > 100 {
            return Err(parser.error("module nesting too deep"));
        }

        let span = parser.parse::<kw::module>()?.0;
        let id = parser.parse()?;
        let name = parser.parse()?;
        let exports = parser.parse()?;

        let kind = if let Some(import) = parser.parse()? {
            NestedModuleKind::Import {
                import,
                ty: parser.parse()?,
            }
        } else {
            let mut fields = Vec::new();
            while !parser.is_empty() {
                fields.push(parser.parens(|p| p.parse())?);
            }
            NestedModuleKind::Inline { fields }
        };

        Ok(NestedModule {
            span,
            id,
            name,
            exports,
            kind,
        })
    }
}

// Note that this is a custom implementation to get multi-token lookahead to
// figure out how to parse `(type ...` when it's in an inline module. If this is
// `(type $x)` or `(type 0)` then it's an inline type annotation, otherwise it's
// probably a typedef like `(type $x (func))` or something like that. We only
// want to parse the two-token variant right now.
struct InlineType;

impl Peek for InlineType {
    fn peek(cursor: Cursor<'_>) -> bool {
        let cursor = match cursor.lparen() {
            Some(cursor) => cursor,
            None => return false,
        };
        let cursor = match cursor.keyword() {
            Some(("type", cursor)) => cursor,
            _ => return false,
        };

        // optional identifier
        let cursor = match cursor.id() {
            Some((_, cursor)) => cursor,
            None => match cursor.integer() {
                Some((_, cursor)) => cursor,
                None => return false,
            },
        };

        cursor.rparen().is_some()
    }

    fn display() -> &'static str {
        "inline type"
    }
}

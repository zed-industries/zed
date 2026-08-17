use crate::ast::{self, kw};
use crate::parser::{Parse, Parser, Result};

/// A WebAssembly global in a module
#[derive(Debug)]
pub struct Global<'a> {
    /// Where this `global` was defined.
    pub span: ast::Span,
    /// An optional name to reference this global by
    pub id: Option<ast::Id<'a>>,
    /// If present, inline export annotations which indicate names this
    /// definition should be exported under.
    pub exports: ast::InlineExport<'a>,
    /// The type of this global, both its value type and whether it's mutable.
    pub ty: ast::GlobalType<'a>,
    /// What kind of global this defined as.
    pub kind: GlobalKind<'a>,
}

/// Different kinds of globals that can be defined in a module.
#[derive(Debug)]
pub enum GlobalKind<'a> {
    /// A global which is actually defined as an import, such as:
    ///
    /// ```text
    /// (global i32 (import "foo" "bar"))
    /// ```
    Import(ast::InlineImport<'a>),

    /// A global defined inline in the module itself
    Inline(ast::Expression<'a>),
}

impl<'a> Parse<'a> for Global<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let span = parser.parse::<kw::global>()?.0;
        let id = parser.parse()?;
        let exports = parser.parse()?;

        let (ty, kind) = if let Some(import) = parser.parse()? {
            (parser.parse()?, GlobalKind::Import(import))
        } else {
            (parser.parse()?, GlobalKind::Inline(parser.parse()?))
        };
        Ok(Global {
            span,
            id,
            exports,
            ty,
            kind,
        })
    }
}

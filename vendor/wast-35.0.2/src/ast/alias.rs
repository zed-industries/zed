use crate::ast::{self, kw};
use crate::parser::{Parse, Parser, Result};

/// An `alias` statement used to juggle indices with nested modules.
#[derive(Debug)]
pub struct Alias<'a> {
    /// Where this `alias` was defined.
    pub span: ast::Span,
    /// An identifier that this alias is resolved with (optionally) for name
    /// resolution.
    pub id: Option<ast::Id<'a>>,
    /// An optional name for this alias stored in the custom `name` section.
    pub name: Option<ast::NameAnnotation<'a>>,
    /// The source of this alias.
    pub source: AliasSource<'a>,
    /// The kind of item that's being aliased.
    pub kind: ast::ExportKind,
}

#[derive(Debug)]
#[allow(missing_docs)]
pub enum AliasSource<'a> {
    InstanceExport {
        instance: ast::ItemRef<'a, kw::instance>,
        export: &'a str,
    },
    Outer {
        /// The index of the module that this reference is referring to.
        module: ast::Index<'a>,
        /// The index of the item within `module` that this alias is referering
        /// to.
        index: ast::Index<'a>,
    },
}

impl<'a> Parse<'a> for Alias<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let span = parser.parse::<kw::alias>()?.0;
        let source = if parser.parse::<Option<kw::outer>>()?.is_some() {
            AliasSource::Outer {
                module: parser.parse()?,
                index: parser.parse()?,
            }
        } else {
            AliasSource::InstanceExport {
                instance: parser.parse::<ast::IndexOrRef<_>>()?.0,
                export: parser.parse()?,
            }
        };
        let (kind, id, name) = parser.parens(|p| Ok((p.parse()?, p.parse()?, p.parse()?)))?;

        Ok(Alias {
            span,
            id,
            name,
            kind,
            source,
        })
    }
}

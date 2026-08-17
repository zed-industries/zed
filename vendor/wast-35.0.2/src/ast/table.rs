use crate::ast::{self, kw};
use crate::parser::{Parse, Parser, Result};

/// A WebAssembly `table` directive in a module.
#[derive(Debug)]
pub struct Table<'a> {
    /// Where this table was defined.
    pub span: ast::Span,
    /// An optional name to refer to this table by.
    pub id: Option<ast::Id<'a>>,
    /// If present, inline export annotations which indicate names this
    /// definition should be exported under.
    pub exports: ast::InlineExport<'a>,
    /// How this table is textually defined in the module.
    pub kind: TableKind<'a>,
}

/// Different ways to textually define a table.
#[derive(Debug)]
pub enum TableKind<'a> {
    /// This table is actually an inlined import definition.
    #[allow(missing_docs)]
    Import {
        import: ast::InlineImport<'a>,
        ty: ast::TableType<'a>,
    },

    /// A typical memory definition which simply says the limits of the table
    Normal(ast::TableType<'a>),

    /// The elem segments of this table, starting from 0, explicitly listed
    Inline {
        /// The element type of this table.
        elem: ast::RefType<'a>,
        /// The element table entries to have, and the length of this list is
        /// the limits of the table as well.
        payload: ElemPayload<'a>,
    },
}

impl<'a> Parse<'a> for Table<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let span = parser.parse::<kw::table>()?.0;
        let id = parser.parse()?;
        let exports = parser.parse()?;

        // Afterwards figure out which style this is, either:
        //
        //  *   `elemtype (elem ...)`
        //  *   `(import "a" "b") limits`
        //  *   `limits`
        let mut l = parser.lookahead1();
        let kind = if l.peek::<ast::RefType>() {
            let elem = parser.parse()?;
            let payload = parser.parens(|p| {
                p.parse::<kw::elem>()?;
                let ty = if parser.peek::<ast::LParen>() {
                    Some(elem)
                } else {
                    None
                };
                ElemPayload::parse_tail(parser, ty)
            })?;
            TableKind::Inline { elem, payload }
        } else if l.peek::<u32>() {
            TableKind::Normal(parser.parse()?)
        } else if let Some(import) = parser.parse()? {
            TableKind::Import {
                import,
                ty: parser.parse()?,
            }
        } else {
            return Err(l.error());
        };
        Ok(Table {
            span,
            id,
            exports,
            kind,
        })
    }
}

/// An `elem` segment in a WebAssembly module.
#[derive(Debug)]
pub struct Elem<'a> {
    /// Where this `elem` was defined.
    pub span: ast::Span,
    /// An optional name by which to refer to this segment.
    pub id: Option<ast::Id<'a>>,
    /// The way this segment was defined in the module.
    pub kind: ElemKind<'a>,
    /// The payload of this element segment, typically a list of functions.
    pub payload: ElemPayload<'a>,
}

/// Different ways to define an element segment in an mdoule.
#[derive(Debug)]
pub enum ElemKind<'a> {
    /// A passive segment that isn't associated with a table and can be used in
    /// various bulk-memory instructions.
    Passive,

    /// A declared element segment that is purely used to declare function
    /// references.
    Declared,

    /// An active segment associated with a table.
    Active {
        /// The table this `elem` is initializing.
        table: ast::ItemRef<'a, kw::table>,
        /// The offset within `table` that we'll initialize at.
        offset: ast::Expression<'a>,
    },
}

/// Different ways to define the element segment payload in a module.
#[derive(Debug, Clone)]
pub enum ElemPayload<'a> {
    /// This element segment has a contiguous list of function indices
    Indices(Vec<ast::ItemRef<'a, kw::func>>),

    /// This element segment has a list of optional function indices,
    /// represented as expressions using `ref.func` and `ref.null`.
    Exprs {
        /// The desired type of each expression below.
        ty: ast::RefType<'a>,
        /// The expressions, currently optional function indices, in this
        /// segment.
        exprs: Vec<Option<ast::ItemRef<'a, kw::func>>>,
    },
}

impl<'a> Parse<'a> for Elem<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let span = parser.parse::<kw::elem>()?.0;
        let id = parser.parse()?;

        let kind = if parser.peek::<u32>()
            || (parser.peek::<ast::LParen>() && !parser.peek::<ast::RefType>())
        {
            let table = if let Some(index) = parser.parse::<Option<ast::IndexOrRef<_>>>()? {
                index.0
            } else {
                ast::ItemRef::Item {
                    kind: kw::table(parser.prev_span()),
                    idx: ast::Index::Num(0, span),
                    exports: Vec::new(),
                }
            };
            let offset = parser.parens(|parser| {
                if parser.peek::<kw::offset>() {
                    parser.parse::<kw::offset>()?;
                }
                parser.parse()
            })?;
            ElemKind::Active { table, offset }
        } else if parser.peek::<kw::declare>() {
            parser.parse::<kw::declare>()?;
            ElemKind::Declared
        } else {
            ElemKind::Passive
        };
        let payload = parser.parse()?;
        Ok(Elem {
            span,
            id,
            kind,
            payload,
        })
    }
}

impl<'a> Parse<'a> for ElemPayload<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        ElemPayload::parse_tail(parser, parser.parse()?)
    }
}

impl<'a> ElemPayload<'a> {
    fn parse_tail(parser: Parser<'a>, ty: Option<ast::RefType<'a>>) -> Result<Self> {
        let ty = match ty {
            None => {
                parser.parse::<Option<kw::func>>()?;
                ast::RefType::func()
            }
            Some(ty) => ty,
        };
        if let ast::HeapType::Func = ty.heap {
            if parser.peek::<ast::IndexOrRef<kw::func>>() {
                let mut elems = Vec::new();
                while !parser.is_empty() {
                    elems.push(parser.parse::<ast::IndexOrRef<_>>()?.0);
                }
                return Ok(ElemPayload::Indices(elems));
            }
        }
        let mut exprs = Vec::new();
        while !parser.is_empty() {
            let func = parser.parens(|p| match p.parse::<Option<kw::item>>()? {
                Some(_) => {
                    if parser.peek::<ast::LParen>() {
                        parser.parens(|p| parse_ref_func(p, ty))
                    } else {
                        parse_ref_func(parser, ty)
                    }
                }
                None => parse_ref_func(parser, ty),
            })?;
            exprs.push(func);
        }
        Ok(ElemPayload::Exprs { exprs, ty })
    }
}

fn parse_ref_func<'a>(
    parser: Parser<'a>,
    ty: ast::RefType<'a>,
) -> Result<Option<ast::ItemRef<'a, kw::func>>> {
    let mut l = parser.lookahead1();
    if l.peek::<kw::ref_null>() {
        parser.parse::<kw::ref_null>()?;
        let null_ty: ast::HeapType = parser.parse()?;
        if ty.heap != null_ty {
            return Err(parser.error("elem segment item doesn't match elem segment type"));
        }
        Ok(None)
    } else if l.peek::<kw::ref_func>() {
        parser.parse::<kw::ref_func>()?;
        Ok(Some(parser.parse::<ast::IndexOrRef<_>>()?.0))
    } else {
        Err(l.error())
    }
}

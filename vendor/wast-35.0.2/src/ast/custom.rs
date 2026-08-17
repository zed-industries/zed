use crate::ast::{self, annotation, kw};
use crate::parser::{Parse, Parser, Result};

/// A wasm custom section within a module.
#[derive(Debug)]
pub struct Custom<'a> {
    /// Where this `@custom` was defined.
    pub span: ast::Span,

    /// Name of the custom section.
    pub name: &'a str,

    /// Where the custom section is being placed,
    pub place: CustomPlace,

    /// Payload of this custom section.
    pub data: Vec<&'a [u8]>,
}

/// Possible locations to place a custom section within a module.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum CustomPlace {
    /// This custom section will appear before the first section in the module.
    BeforeFirst,
    /// This custom section will be placed just before a known section.
    Before(CustomPlaceAnchor),
    /// This custom section will be placed just after a known section.
    After(CustomPlaceAnchor),
    /// This custom section will appear after the last section in the module.
    AfterLast,
}

/// Known sections that custom sections can be placed relative to.
#[derive(Debug, PartialEq, Copy, Clone)]
#[allow(missing_docs)]
pub enum CustomPlaceAnchor {
    Type,
    Import,
    Module,
    Instance,
    Alias,
    Func,
    Table,
    Memory,
    Global,
    Export,
    Start,
    Elem,
    Code,
    Data,
    Event,
}

impl<'a> Parse<'a> for Custom<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let span = parser.parse::<annotation::custom>()?.0;
        let name = parser.parse()?;
        let place = if parser.peek::<ast::LParen>() {
            parser.parens(|p| p.parse())?
        } else {
            CustomPlace::AfterLast
        };
        let mut data = Vec::new();
        while !parser.is_empty() {
            data.push(parser.parse()?);
        }
        Ok(Custom {
            span,
            name,
            place,
            data,
        })
    }
}

impl<'a> Parse<'a> for CustomPlace {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let mut l = parser.lookahead1();
        let ctor = if l.peek::<kw::before>() {
            parser.parse::<kw::before>()?;
            if l.peek::<kw::first>() {
                parser.parse::<kw::first>()?;
                return Ok(CustomPlace::BeforeFirst);
            }
            CustomPlace::Before as fn(CustomPlaceAnchor) -> _
        } else if l.peek::<kw::after>() {
            parser.parse::<kw::after>()?;
            if l.peek::<kw::last>() {
                parser.parse::<kw::last>()?;
                return Ok(CustomPlace::AfterLast);
            }
            CustomPlace::After
        } else {
            return Err(l.error());
        };
        Ok(ctor(parser.parse()?))
    }
}

impl<'a> Parse<'a> for CustomPlaceAnchor {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        if parser.peek::<kw::r#type>() {
            parser.parse::<kw::r#type>()?;
            return Ok(CustomPlaceAnchor::Type);
        }
        if parser.peek::<kw::import>() {
            parser.parse::<kw::import>()?;
            return Ok(CustomPlaceAnchor::Import);
        }
        if parser.peek::<kw::func>() {
            parser.parse::<kw::func>()?;
            return Ok(CustomPlaceAnchor::Func);
        }
        if parser.peek::<kw::table>() {
            parser.parse::<kw::table>()?;
            return Ok(CustomPlaceAnchor::Table);
        }
        if parser.peek::<kw::memory>() {
            parser.parse::<kw::memory>()?;
            return Ok(CustomPlaceAnchor::Memory);
        }
        if parser.peek::<kw::global>() {
            parser.parse::<kw::global>()?;
            return Ok(CustomPlaceAnchor::Global);
        }
        if parser.peek::<kw::export>() {
            parser.parse::<kw::export>()?;
            return Ok(CustomPlaceAnchor::Export);
        }
        if parser.peek::<kw::start>() {
            parser.parse::<kw::start>()?;
            return Ok(CustomPlaceAnchor::Start);
        }
        if parser.peek::<kw::elem>() {
            parser.parse::<kw::elem>()?;
            return Ok(CustomPlaceAnchor::Elem);
        }
        if parser.peek::<kw::code>() {
            parser.parse::<kw::code>()?;
            return Ok(CustomPlaceAnchor::Code);
        }
        if parser.peek::<kw::data>() {
            parser.parse::<kw::data>()?;
            return Ok(CustomPlaceAnchor::Data);
        }
        if parser.peek::<kw::event>() {
            parser.parse::<kw::event>()?;
            return Ok(CustomPlaceAnchor::Event);
        }
        if parser.peek::<kw::instance>() {
            parser.parse::<kw::instance>()?;
            return Ok(CustomPlaceAnchor::Instance);
        }
        if parser.peek::<kw::module>() {
            parser.parse::<kw::module>()?;
            return Ok(CustomPlaceAnchor::Module);
        }
        if parser.peek::<kw::alias>() {
            parser.parse::<kw::alias>()?;
            return Ok(CustomPlaceAnchor::Alias);
        }

        Err(parser.error("expected a valid section name"))
    }
}

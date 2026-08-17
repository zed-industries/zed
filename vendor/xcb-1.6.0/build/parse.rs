use quick_xml::events::attributes::{Attribute, Attributes};
use quick_xml::events::{BytesStart, Event as XmlEv};
use quick_xml::Reader as XmlReader;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::result;
use std::str::{self, Utf8Error};
use std::sync::Arc;

use crate::ir::{
    Doc, DocError, DocField, DocSee, EnumItem, EventSelector, Expr, ExtInfo, Field, Item, Reply,
    SwitchCase,
};

#[derive(Debug)]
pub enum Error {
    IO(Arc<io::Error>),
    Xml(quick_xml::Error),
    Utf8(Utf8Error),
    Parse(String),
}

impl From<Arc<io::Error>> for Error {
    fn from(err: Arc<io::Error>) -> Self {
        Error::IO(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IO(err.into())
    }
}

impl From<Utf8Error> for Error {
    fn from(err: Utf8Error) -> Self {
        Error::Utf8(err)
    }
}

impl From<quick_xml::Error> for Error {
    fn from(err: quick_xml::Error) -> Self {
        match err {
            quick_xml::Error::Io(e) => Error::IO(e),
            quick_xml::Error::NonDecodable(Some(e)) => Error::Utf8(e),
            e => Error::Xml(e),
        }
    }
}

pub type Result<T> = result::Result<T, Error>;

#[allow(clippy::large_enum_variant)]
pub enum Event {
    ModuleInfo {
        name: String,
        extinfo: Option<ExtInfo>,
    },
    Import(String),
    Item(Item),
    Ignore, // doctype etc.
}

pub struct Parser<B: BufRead> {
    xml: XmlReader<B>,
    buf: Vec<u8>,
}

impl<B: BufRead> Iterator for &mut Parser<B> {
    type Item = Result<Event>;

    fn next(&mut self) -> Option<Self::Item> {
        self.buf.clear();
        match self.xml.read_event_into(&mut self.buf) {
            Ok(XmlEv::Empty(ref e)) => match e.name().into_inner() {
                b"typedef" => {
                    let names: [&[u8]; 2] = [b"oldname", b"newname"];
                    let mut vals: [Option<String>; 2] = [None, None];
                    if let Err(err) = get_attributes(e.attributes(), &names, &mut vals) {
                        return Some(Err(err));
                    }
                    let [old_typ, new_typ] = vals;
                    match (old_typ, new_typ) {
                        (Some(old_typ), Some(new_typ)) => {
                            Some(Ok(Event::Item(Item::Typedef { old_typ, new_typ })))
                        }
                        _ => Some(Err(Error::Parse(
                            "typedef without newname and/or oldname".into(),
                        ))),
                    }
                }
                b"xidtype" => Some(
                    expect_attribute(e.attributes(), b"name")
                        .map(|typ| Event::Item(Item::XidType { typ })),
                ),
                b"error" => Some({
                    let start = e.to_owned();
                    self.parse_op_struct(start, b"").map(|ops| {
                        Event::Item(Item::Error {
                            number: ops.number,
                            name: ops.name,
                            fields: ops.content.fields,
                            doc: ops.content.doc,
                        })
                    })
                }),
                b"errorcopy" => Some({
                    let start = e.to_owned();
                    let nsres = self.parse_op_struct(start, b"");
                    if let Ok(ns) = nsres {
                        if ns.r#ref.is_none() {
                            return Some(Err(Error::Parse("".into())));
                        }
                        let number = ns.number;
                        let r#ref = ns.r#ref.unwrap();

                        Ok(Event::Item(Item::ErrorCopy {
                            name: ns.name,
                            number,
                            r#ref,
                        }))
                    } else {
                        Err(Error::Parse("<errorcopy> without ref attribute".into()))
                    }
                }),
                b"eventcopy" => Some({
                    let start = e.to_owned();
                    let nsres = self.parse_op_struct(start, b"");
                    if let Ok(ns) = nsres {
                        if ns.r#ref.is_none() {
                            return Some(Err(Error::Parse("".into())));
                        }
                        let number = ns.number;
                        let r#ref = ns.r#ref.unwrap();

                        Ok(Event::Item(Item::EventCopy {
                            name: ns.name,
                            number,
                            r#ref,
                        }))
                    } else {
                        Err(Error::Parse("<errorcopy> without ref attribute".into()))
                    }
                }),
                b"request" => {
                    let start = e.to_owned();
                    Some(self.parse_request(start, true).map(|req| {
                        Event::Item(Item::Request {
                            name: req.name,
                            opcode: req.opcode,
                            params: req.params,
                            reply: req.reply,
                            doc: req.doc,
                        })
                    }))
                }
                name => Some(Err(Error::Parse(format!(
                    "unexpected XML: <{} />",
                    str::from_utf8(name).unwrap()
                )))),
            },
            Ok(XmlEv::Start(ref e)) => match e.name().into_inner() {
                b"xcb" => {
                    let names: [&[u8]; 5] = [
                        b"header",
                        b"extension-xname",
                        b"extension-name",
                        b"major-version",
                        b"minor-version",
                    ];
                    let mut vals: [Option<String>; 5] = [None, None, None, None, None];
                    if let Err(err) = get_attributes(e.attributes(), &names, &mut vals) {
                        return Some(Err(err));
                    }
                    let [mod_name, xname, name, major_version, minor_version] = vals;

                    if mod_name.is_none() {
                        return Some(Err(Error::Parse("<xcb> without header attr".into())));
                    }
                    let mod_name = mod_name.unwrap();

                    let extinfo = match (xname, name, major_version, minor_version) {
                        (Some(xname), Some(name), Some(major_version), Some(minor_version)) => {
                            let major_version = major_version
                                .parse::<u32>()
                                .expect("could not parse major_version");
                            let minor_version = minor_version
                                .parse::<u32>()
                                .expect("could not parse major_version");
                            Some(ExtInfo {
                                xname,
                                name,
                                major_version,
                                minor_version,
                            })
                        }
                        (None, None, None, None) => None,
                        _ => panic!("incomplete extension info for {}", &mod_name),
                    };

                    Some(Ok(Event::ModuleInfo {
                        name: mod_name,
                        extinfo,
                    }))
                }
                b"import" => Some(self.parse_import().map(Event::Import)),
                b"xidunion" => Some(expect_attribute(e.attributes(), b"name").and_then(|typ| {
                    let xidtypesres = self.parse_xidunion_types();
                    xidtypesres.map(|xidtypes| Event::Item(Item::XidUnion { typ, xidtypes }))
                })),
                b"enum" => Some(expect_attribute(e.attributes(), b"name").and_then(|typ| {
                    let enumres = self.parse_enum_content();
                    enumres.map(|en| {
                        Event::Item(Item::Enum {
                            typ,
                            items: en.0,
                            doc: en.1,
                        })
                    })
                })),
                b"struct" => Some(expect_attribute(e.attributes(), b"name").and_then(|typ| {
                    let contentres = self.parse_struct_content(b"struct");
                    contentres.map(|content| {
                        Event::Item(Item::Struct {
                            typ,
                            fields: content.fields,
                            doc: content.doc,
                        })
                    })
                })),
                b"union" => Some(expect_attribute(e.attributes(), b"name").and_then(|typ| {
                    let contentres = self.parse_struct_content(b"union");
                    contentres.map(|content| {
                        Event::Item(Item::Union {
                            typ,
                            fields: content.fields,
                            doc: content.doc,
                        })
                    })
                })),
                b"error" => Some({
                    let start = e.to_owned();
                    self.parse_op_struct(start, b"error").map(|ops| {
                        Event::Item(Item::Error {
                            number: ops.number,
                            name: ops.name,
                            fields: ops.content.fields,
                            doc: ops.content.doc,
                        })
                    })
                }),
                b"event" => Some({
                    let start = e.to_owned();
                    self.parse_op_struct(start, b"event").map(
                        |OpStruct {
                             number,
                             name,
                             content,
                             no_seq_number,
                             xge,
                             ..
                         }| {
                            Event::Item(Item::Event {
                                number,
                                name,
                                fields: content.fields,
                                no_seq_number,
                                xge,
                                doc: content.doc,
                            })
                        },
                    )
                }),
                b"eventstruct" => Some({
                    let start = e.to_owned();
                    self.parse_eventstruct(start)
                        .map(|(typ, selectors)| Event::Item(Item::EventStruct { typ, selectors }))
                }),
                b"request" => {
                    let start = e.to_owned();
                    Some(self.parse_request(start, false).map(|req| {
                        Event::Item(Item::Request {
                            opcode: req.opcode,
                            name: req.name,
                            params: req.params,
                            reply: req.reply,
                            doc: req.doc,
                        })
                    }))
                }
                name => Some(Err(Error::Parse(format!(
                    "unexpected XML: <{}>",
                    str::from_utf8(name).unwrap()
                )))),
            },

            Ok(XmlEv::Eof) => None,

            _ => Some(Ok(Event::Ignore)),
        }
    }
}

impl Parser<BufReader<File>> {
    pub fn from_file(xml_file: &Path) -> Self {
        let mut xml = XmlReader::from_file(xml_file).unwrap();
        xml.trim_text(true);

        Parser {
            xml,
            buf: Vec::with_capacity(8 * 1024),
        }
    }
}

impl<B: BufRead> Parser<B> {
    fn expect_text(&mut self) -> Result<String> {
        match self.xml.read_event_into(&mut self.buf)? {
            XmlEv::Text(e) => Ok(str::from_utf8(e.unescape()?.as_bytes())?.into()),
            XmlEv::CData(e) => Ok(str::from_utf8(&e)?.into()),
            ev => Err(Error::Parse(format!("expected text, found {:?}", ev))),
        }
    }

    fn expect_text_trim(&mut self, close_tag: &[u8]) -> Result<String> {
        let txt = match self.xml.read_event_into(&mut self.buf)? {
            XmlEv::Text(e) => Vec::from(e.unescape()?.as_bytes()),
            XmlEv::CData(e) => Vec::from(e.into_inner()),
            XmlEv::End(e) => {
                if e.name().0 == close_tag {
                    return Ok(String::new());
                } else {
                    return Err(Error::Parse(format!(
                        "expected text, found </{}>",
                        str::from_utf8(e.name().0).unwrap()
                    )));
                }
            }
            ev => return Err(Error::Parse(format!("expected text, found {:?}", ev))),
        };
        let txt = str::from_utf8(&txt)?.trim().into();
        if !close_tag.is_empty() {
            match self.xml.read_event_into(&mut self.buf)? {
                XmlEv::End(e) => {
                    if e.name().0 == close_tag {
                        return Ok(txt);
                    }
                    Err(Error::Parse(format!(
                        "expected </{}> after text",
                        str::from_utf8(close_tag).unwrap()
                    )))
                }
                ev => Err(Error::Parse(format!(
                    "expected </{}>, found {:?}",
                    str::from_utf8(close_tag).unwrap(),
                    ev
                ))),
            }
        } else {
            Ok(txt)
        }
    }

    fn expect_start(&mut self) -> Result<Vec<u8>> {
        match self.xml.read_event_into(&mut self.buf)? {
            XmlEv::Start(e) | XmlEv::Empty(e) => Ok(Vec::from(e.name().0)),
            ev => Err(Error::Parse(format!("expected start tag, found {:?}", ev))),
        }
    }

    fn expect_close_tag(&mut self, tag: &[u8]) -> Result<()> {
        loop {
            match self.xml.read_event_into(&mut self.buf)? {
                XmlEv::End(e) => {
                    if e.name().0 == tag {
                        return Ok(());
                    } else {
                        return Err(Error::Parse(format!(
                            "expected </{}>, got </{}>",
                            str::from_utf8(tag).unwrap(),
                            str::from_utf8(e.name().0)?
                        )));
                    }
                }
                XmlEv::Comment(_) => {}
                ev => {
                    return Err(Error::Parse(format!(
                        "expected </{}>, found {:?}",
                        str::from_utf8(tag).unwrap(),
                        ev
                    )))
                }
            }
        }
    }

    fn expect_text_element(&mut self) -> Result<(Vec<u8>, String)> {
        let tag = self.expect_start()?;
        let txt = self.expect_text()?;
        self.expect_close_tag(&tag)?;
        Ok((tag, txt))
    }

    fn parse_doc(&mut self) -> Result<Doc> {
        let mut brief = None;
        let mut description = None;
        let mut example = None;
        let mut fields = Vec::new();
        let mut errors = Vec::new();
        let mut sees = Vec::new();

        loop {
            match self.xml.read_event_into(&mut self.buf)? {
                XmlEv::Start(ref e) => match e.name().0 {
                    b"brief" => {
                        brief = Some(self.expect_text_trim(b"brief")?);
                    }
                    b"description" => {
                        description = Some(self.expect_text_trim(b"description")?);
                    }
                    b"example" => {
                        example = Some(self.expect_text_trim(b"example")?);
                    }
                    b"field" => {
                        let name = expect_attribute(e.attributes(), b"name")?;
                        let text = self.expect_text_trim(b"field")?;
                        fields.push(DocField { name, text });
                    }
                    b"error" => {
                        let typ = expect_attribute(e.attributes(), b"type")?;
                        let text = self.expect_text_trim(b"error")?;
                        errors.push(DocError { typ, text });
                    }
                    b"see" => {
                        let typ = expect_attribute(e.attributes(), b"type")?;
                        let name = self.expect_text_trim(b"error")?;
                        sees.push(DocSee { typ, name });
                    }
                    _ => {}
                },
                XmlEv::Empty(ref e) => {
                    if e.name().0 == b"field" {
                        let name = expect_attribute(e.attributes(), b"name")?;
                        fields.push(DocField {
                            name,
                            text: String::new(),
                        });
                    }
                }
                XmlEv::Text(_) | XmlEv::CData(_) => {
                    return Err(Error::Parse("Unexpected doc text out of element".into()));
                }
                XmlEv::End(ref e) => {
                    if e.name().0 == b"doc" {
                        break;
                    }
                }
                _ => {}
            }
        }

        Ok(Doc {
            brief,
            description,
            fields,
            errors,
            example,
            sees,
        })
    }

    fn parse_expr_content(
        &mut self,
        attrs: Attributes,
        tag: &[u8],
        is_empty: bool,
    ) -> Result<Expr> {
        match tag {
            b"fieldref" => {
                let fr = self.expect_text_trim(b"fieldref")?;
                Ok(Expr::FieldRef(fr))
            }
            b"paramref" => {
                let pr = self.expect_text_trim(b"paramref")?;
                Ok(Expr::ParamRef(pr))
            }
            b"enumref" => {
                let name = expect_attribute(attrs, b"ref")?;
                let item = self.expect_text_trim(b"enumref")?;
                Ok(Expr::EnumRef { name, item })
            }
            b"value" => {
                let val = self.expect_text_trim(b"value")?;
                let val: usize = val.parse().map_err(|e| {
                    Error::Parse(format!("could not parse expr <value> tag: {}", e))
                })?;
                Ok(Expr::Value(val))
            }
            b"op" => {
                let op = expect_attribute(attrs, b"op")?;
                let lhs = self.parse_expr(b"")?.unwrap();
                let rhs = self.parse_expr(b"")?.unwrap();
                self.expect_close_tag(b"op")?;
                Ok(Expr::Op(op, Box::new(lhs), Box::new(rhs)))
            }
            b"unop" => {
                let unop = expect_attribute(attrs, b"op")?;
                let expr = self.parse_expr(b"")?.unwrap();
                self.expect_close_tag(b"unop")?;
                Ok(Expr::Unop(unop, Box::new(expr)))
            }
            b"popcount" => {
                let expr = self.parse_expr(b"")?.unwrap();
                self.expect_close_tag(b"popcount")?;
                Ok(Expr::Popcount(Box::new(expr)))
            }
            b"sumof" => {
                let list_ref = expect_attribute(attrs, b"ref")?;
                let expr = if is_empty {
                    None
                } else {
                    self.parse_expr(b"sumof")?
                };
                Ok(Expr::SumOf(list_ref, expr.map(Box::new)))
            }
            b"listelement-ref" => Ok(Expr::ListElementRef),
            tag => Err(Error::Parse(format!(
                "Unexpected <{}> in expression",
                str::from_utf8(tag)?
            ))),
        }
    }

    fn parse_expr(&mut self, empty_end_tag: &[u8]) -> Result<Option<Expr>> {
        match self.xml.read_event_into(&mut self.buf)? {
            XmlEv::Start(ref e) => {
                let e = e.to_owned();
                Ok(Some(self.parse_expr_content(
                    e.attributes(),
                    e.name().0,
                    false,
                )?))
            }
            XmlEv::Empty(ref e) => {
                let e = e.to_owned();
                Ok(Some(self.parse_expr_content(
                    e.attributes(),
                    e.name().0,
                    true,
                )?))
            }
            XmlEv::Comment(_) => self.parse_expr(empty_end_tag), // in case of comment, we just parse the next one
            XmlEv::End(e) => {
                if e.name().0 == empty_end_tag {
                    Ok(None)
                } else {
                    Err(Error::Parse(format!(
                        "Unexpected </{}> while parsing expression",
                        str::from_utf8(e.name().0).unwrap()
                    )))
                }
            }
            ev => Err(Error::Parse(format!(
                "Unexpected XML while parsing expression: {:?}",
                ev
            ))),
        }
    }

    fn parse_import(&mut self) -> Result<String> {
        let imp = self.expect_text()?;
        self.expect_close_tag(b"import")?;
        Ok(imp)
    }

    fn parse_xidunion_types(&mut self) -> Result<Vec<String>> {
        let mut xidtypes = Vec::new();

        loop {
            match self.xml.read_event_into(&mut self.buf)? {
                XmlEv::Start(ref e) => match e.name().0 {
                    b"type" => {
                        let typ = self.expect_text_trim(b"type")?;
                        xidtypes.push(typ);
                    }
                    tag => {
                        return Err(Error::Parse(format!(
                            "Unexpected <{}> in <xidunion>",
                            str::from_utf8(tag)?
                        )));
                    }
                },
                XmlEv::End(ref e) => match e.name().0 {
                    b"xidunion" => break,
                    _ => unreachable!(),
                },
                ev => {
                    return Err(Error::Parse(format!(
                        "Unexpected XML in <xidunion>: {:?}",
                        ev
                    )));
                }
            }
        }

        Ok(xidtypes)
    }

    fn parse_enum_content(&mut self) -> Result<(Vec<EnumItem>, Option<Doc>)> {
        let mut items = Vec::new();
        let mut doc = None;

        loop {
            match self.xml.read_event_into(&mut self.buf)? {
                XmlEv::Empty(ref e) | XmlEv::Start(ref e) => match e.name().0 {
                    b"item" => {
                        let name = expect_attribute(e.attributes(), b"name")?;
                        let (tag, value) = self.expect_text_element()?;
                        if tag != b"bit" && tag != b"value" {
                            return Err(Error::Parse(format!(
                                "expected <bit> or <value> for enum {}, got <{}>",
                                name,
                                str::from_utf8(&tag)?
                            )));
                        }
                        let value: u32 = value.parse().map_err(|e| {
                            Error::Parse(format!(
                                "failed to parse {} of enum {}: {}",
                                str::from_utf8(&tag).unwrap(),
                                name,
                                e
                            ))
                        })?;
                        let item = if tag == b"bit" {
                            EnumItem::Bit(name, value)
                        } else {
                            EnumItem::Value(name, value)
                        };
                        items.push(item);
                    }
                    b"doc" => {
                        doc = Some(self.parse_doc()?);
                    }
                    tag => {
                        return Err(Error::Parse(format!(
                            "Unexpected tag in enum: {}",
                            str::from_utf8(tag)?
                        )));
                    }
                },
                XmlEv::End(ref e) => match e.name().0 {
                    b"enum" => break,
                    b"item" => continue,
                    tag => {
                        return Err(Error::Parse(format!(
                            "Unexpected </{}> in enum",
                            str::from_utf8(tag)?,
                        )))
                    }
                },
                XmlEv::Comment(_) => {}
                ev => {
                    return Err(Error::Parse(format!("unexpected XML in enum: {:?}", ev)));
                }
            }
        }

        Ok((items, doc))
    }

    fn parse_field_content(&mut self, e: &BytesStart, empty_tag: bool) -> Result<Option<Field>> {
        if empty_tag {
            match e.name().0 {
                b"required_start_align" => {
                    // this is meant for checking if padding is correct in the  XML
                    // we simply ignore this in the rust bindings
                    Ok(None)
                }
                b"field" => {
                    let mut r#type = None;
                    let mut name = None;
                    let mut r#enum = None;
                    let mut mask = None;
                    let mut altenum = None;
                    let mut altmask = None;
                    for attr in e.attributes() {
                        match attr {
                            Ok(attr) if attr.key.0 == b"type" => {
                                r#type = Some(attr_value(&attr).unwrap());
                            }
                            Ok(attr) if attr.key.0 == b"name" => {
                                name = Some(attr_value(&attr).unwrap());
                            }
                            Ok(attr) if attr.key.0 == b"enum" => {
                                r#enum = Some(attr_value(&attr).unwrap());
                            }
                            Ok(attr) if attr.key.0 == b"mask" => {
                                mask = Some(attr_value(&attr).unwrap());
                            }
                            Ok(attr) if attr.key.0 == b"altenum" => {
                                altenum = Some(attr_value(&attr).unwrap());
                            }
                            Ok(attr) if attr.key.0 == b"altmask" => {
                                altmask = Some(attr_value(&attr).unwrap());
                            }
                            Ok(attr) => unreachable!(
                                "field attribute {}",
                                str::from_utf8(attr.key.0).unwrap()
                            ),
                            Err(err) => return Err(quick_xml::Error::InvalidAttr(err).into()),
                        }
                    }
                    if let (Some(typ), Some(name)) = (r#type, name) {
                        Ok(Some(Field::Field {
                            name,
                            typ,
                            r#enum,
                            mask,
                            altenum,
                            altmask,
                        }))
                    } else {
                        Err(Error::Parse("struct field without type and/or name".into()))
                    }
                }
                b"pad" => {
                    let names: [&[u8]; 2] = [b"bytes", b"align"];
                    let mut vals: [Option<String>; 2] = [None, None];
                    get_attributes(e.attributes(), &names, &mut vals)?;
                    let [bytes, align] = vals;
                    if bytes.is_some() && align.is_some() {
                        return Err(Error::Parse("<pad> with both align and bytes attr".into()));
                    }
                    if let Some(bytes) = bytes {
                        let val: usize = bytes.parse().map_err(|e| {
                            Error::Parse(format!("failed to parse pad bytes of struct: {}", e))
                        })?;
                        Ok(Some(Field::Pad(val)))
                    } else if let Some(align) = align {
                        let val: usize = align.parse().map_err(|e| {
                            Error::Parse(format!("failed to parse pad bytes of struct: {}", e))
                        })?;
                        Ok(Some(Field::AlignPad(val)))
                    } else {
                        Err(Error::Parse(
                            "<pad> with neither align and bytes attr".into(),
                        ))
                    }
                }
                b"list" => {
                    let mut r#type = None;
                    let mut name = None;
                    let mut r#enum = None;
                    let mut mask = None;
                    for attr in e.attributes() {
                        match attr {
                            Ok(attr) if attr.key.0 == b"type" => {
                                r#type = Some(attr_value(&attr).unwrap());
                            }
                            Ok(attr) if attr.key.0 == b"name" => {
                                name = Some(attr_value(&attr).unwrap());
                            }
                            Ok(attr) if attr.key.0 == b"enum" => {
                                r#enum = Some(attr_value(&attr).unwrap());
                            }
                            Ok(attr) if attr.key.0 == b"mask" => {
                                mask = Some(attr_value(&attr).unwrap());
                            }
                            Ok(attr) => unreachable!(
                                "field attribute {}",
                                str::from_utf8(attr.key.0).unwrap()
                            ),
                            Err(err) => return Err(quick_xml::Error::InvalidAttr(err).into()),
                        }
                    }
                    if let (Some(typ), Some(name)) = (r#type, name) {
                        Ok(Some(Field::ListNoLen {
                            name,
                            typ,
                            r#enum,
                            mask,
                        }))
                    } else {
                        Err(Error::Parse(
                            "<list> tag without type and/or name attribute".into(),
                        ))
                    }
                }
                b"valueparam" => {
                    let names: [&[u8]; 3] =
                        [b"value-mask-type", b"value-mask-name", b"value-list-name"];
                    let mut vals: [Option<String>; 3] = [None, None, None];
                    get_attributes(e.attributes(), &names, &mut vals)?;
                    let [mask_typ, mask_name, list_name] = vals;
                    if let (Some(mask_typ), Some(mask_name), Some(list_name)) =
                        (mask_typ, mask_name, list_name)
                    {
                        Ok(Some(Field::ValueParam {
                            mask_typ,
                            mask_name,
                            list_name,
                        }))
                    } else {
                        Err(Error::Parse(
                                "<valueparam> tag without value-mask-type, value-mask-name or value-list-name attribute".into(),
                            ))
                    }
                }
                b"fd" => {
                    let name = expect_attribute(e.attributes(), b"name")?;
                    Ok(Some(Field::Fd(name)))
                }
                tag => Err(Error::Parse(format!(
                    "Unexpected <{} /> in field",
                    str::from_utf8(tag)?
                ))),
            }
        } else {
            match e.name().0 {
                b"list" => {
                    let mut r#type = None;
                    let mut name = None;
                    let mut r#enum = None;
                    let mut mask = None;
                    for attr in e.attributes() {
                        match attr {
                            Ok(attr) if attr.key.0 == b"type" => {
                                r#type = Some(attr_value(&attr).unwrap());
                            }
                            Ok(attr) if attr.key.0 == b"name" => {
                                name = Some(attr_value(&attr).unwrap());
                            }
                            Ok(attr) if attr.key.0 == b"enum" => {
                                r#enum = Some(attr_value(&attr).unwrap());
                            }
                            Ok(attr) if attr.key.0 == b"mask" => {
                                mask = Some(attr_value(&attr).unwrap());
                            }
                            Ok(attr) => unreachable!(
                                "field attribute {}",
                                str::from_utf8(attr.key.0).unwrap()
                            ),
                            Err(err) => return Err(quick_xml::Error::InvalidAttr(err).into()),
                        }
                    }
                    if let (Some(typ), Some(name)) = (r#type, name) {
                        let len_expr = self.parse_expr(b"list")?;
                        Ok(Some(match len_expr {
                            Some(len_expr) => Field::List {
                                name,
                                typ,
                                len_expr,
                                r#enum,
                                mask,
                            },
                            None => Field::ListNoLen {
                                name,
                                typ,
                                r#enum,
                                mask,
                            },
                        }))
                    } else {
                        Err(Error::Parse(
                            "<list> tag without type and/or name attribute".into(),
                        ))
                    }
                }
                b"exprfield" => {
                    let names: [&[u8]; 2] = [b"type", b"name"];
                    let mut vals: [Option<String>; 2] = [None, None];
                    get_attributes(e.attributes(), &names, &mut vals)?;
                    let [typ, nam] = vals;
                    if let (Some(typ), Some(name)) = (typ, nam) {
                        let expr = self.parse_expr(b"")?.unwrap();
                        self.xml.read_to_end_into(
                            quick_xml::name::QName(b"exprfield"),
                            &mut self.buf,
                        )?;
                        Ok(Some(Field::Expr { name, typ, expr }))
                    } else {
                        Err(Error::Parse(
                            "<exprfield> tag without type and/or name attribute".into(),
                        ))
                    }
                }
                b"switch" => {
                    let name = expect_attribute(e.attributes(), b"name")?;
                    let (expr, cases) = self.parse_switch()?;
                    Ok(Some(Field::Switch(name, expr, cases)))
                }
                tag => Err(Error::Parse(format!(
                    "Unexpected <{}> in field",
                    str::from_utf8(tag)?
                ))),
            }
        }
    }

    fn parse_struct_content(&mut self, end_tag: &[u8]) -> Result<StructContent> {
        let mut fields = Vec::new();
        let mut reply = None;
        let mut doc = None;

        loop {
            match self.xml.read_event_into(&mut self.buf)? {
                XmlEv::Empty(ref e) => {
                    let e = e.to_owned();
                    let f = self.parse_field_content(&e, true)?;
                    if let Some(f) = f {
                        fields.push(f);
                    }
                }
                XmlEv::Start(ref e) => match e.name().0 {
                    b"list" | b"exprfield" | b"switch" => {
                        let e = e.to_owned();
                        let f = self.parse_field_content(&e, false)?;
                        if let Some(f) = f {
                            fields.push(f);
                        }
                    }
                    b"doc" => {
                        doc = Some(self.parse_doc()?);
                    }
                    b"reply" => {
                        let StructContent { fields, doc, .. } =
                            self.parse_struct_content(b"reply")?;
                        reply = Some(Reply { fields, doc })
                    }
                    tag => {
                        return Err(Error::Parse(format!(
                            "Unexpected <{} /> in struct: {:?}",
                            str::from_utf8(tag)?,
                            e
                        )))
                    }
                },
                XmlEv::End(ref e) => {
                    if e.name().0 == end_tag {
                        break;
                    }
                }
                XmlEv::Comment(_) => {}
                ev => {
                    return Err(Error::Parse(format!("unexpected XML in struct: {:?}", ev)));
                }
            }
        }

        Ok(StructContent { fields, reply, doc })
    }

    fn parse_op_struct(&mut self, start: BytesStart, end_tag: &[u8]) -> Result<OpStruct> {
        let names: [&[u8]; 5] = [b"name", b"number", b"ref", b"no-sequence-number", b"xge"];
        let mut vals: [Option<String>; 5] = [None, None, None, None, None];
        get_attributes(start.attributes(), &names, &mut vals)?;
        let [name, number, r#ref, no_seq_number, xge] = vals;
        // FIXME: check if true or false
        let no_seq_number = no_seq_number.is_some();
        let xge = xge.is_some();
        match (name, number) {
            (Some(name), Some(number)) => {
                let number = str::parse::<i32>(&number)
                    .map_err(|_| Error::Parse(format!("can't parse {} as number", number)))?;
                let content = if end_tag.is_empty() {
                    StructContent {
                        fields: Vec::new(),
                        doc: None,
                        reply: None,
                    }
                } else {
                    self.parse_struct_content(end_tag)?
                };
                Ok(OpStruct {
                    number,
                    name,
                    content,
                    r#ref,
                    no_seq_number,
                    xge,
                })
            }
            _ => Err(Error::Parse(format!(
                "<{}> without name or number",
                str::from_utf8(start.name().0)?
            ))),
        }
    }

    fn parse_eventstruct(&mut self, start: BytesStart) -> Result<(String, Vec<EventSelector>)> {
        let name = expect_attribute(start.attributes(), b"name")?;
        let mut selectors = Vec::new();

        loop {
            match self.xml.read_event_into(&mut self.buf)? {
                XmlEv::Empty(ref e) if e.name().0 == b"allowed" => {
                    let names: [&[u8]; 4] = [b"extension", b"xge", b"opcode-min", b"opcode-max"];
                    let mut vals: [Option<String>; 4] = [None, None, None, None];
                    get_attributes(e.attributes(), &names, &mut vals)?;
                    let [extension, xge, opcode_min, opcode_max] = vals;
                    match (extension, xge, opcode_min, opcode_max) {
                        (Some(extension), Some(xge), Some(opcode_min), Some(opcode_max)) => {
                            let xge = xge == "true";
                            let opcode_min = opcode_min
                                .parse::<u32>()
                                .expect("opcode-min must be a number");
                            let opcode_max = opcode_max
                                .parse::<u32>()
                                .expect("opcode-max must be a number");
                            selectors.push(EventSelector {
                                extension,
                                xge,
                                opcode_range: (opcode_min, opcode_max),
                            });
                        }
                        _ => {
                            return Err(Error::Parse(
                                "Incomplete <allowed> element for event selector".into(),
                            ));
                        }
                    }
                }
                XmlEv::End(ref e) if e.name().0 == b"eventstruct" => {
                    break;
                }
                XmlEv::Comment(..) => {}
                ev => {
                    return Err(Error::Parse(format!("Unexpected XML: {:#?}", ev)));
                }
            }
        }

        Ok((name, selectors))
    }

    fn parse_request(&mut self, start: BytesStart, is_empty: bool) -> Result<Request> {
        let names: [&[u8]; 2] = [b"name", b"opcode"];
        let mut vals: [Option<String>; 2] = [None, None];
        get_attributes(start.attributes(), &names, &mut vals)?;
        let [name, opcode] = vals;
        if name.is_none() && opcode.is_none() {
            return Err(Error::Parse(
                "<request> without name or opcode attributes".into(),
            ));
        }
        let name = name.unwrap();
        let opcode = opcode.unwrap();
        let opcode: u32 = str::parse(&opcode)
            .map_err(|_| Error::Parse(format!("cannot parse {} as int", &opcode)))?;
        if is_empty {
            Ok(Request {
                name,
                opcode,
                params: Vec::new(),
                doc: None,
                reply: None,
            })
        } else {
            let StructContent { fields, doc, reply } = self.parse_struct_content(b"request")?;
            Ok(Request {
                name,
                opcode,
                params: fields,
                doc,
                reply,
            })
        }
    }

    fn parse_switch_case(&mut self, name: Option<String>, end_tag: &[u8]) -> Result<SwitchCase> {
        let bit = end_tag == b"bitcase";

        let mut exprs = Vec::new();
        let mut fields = Vec::new();

        loop {
            match self.xml.read_event_into(&mut self.buf)? {
                XmlEv::Start(ref e) => {
                    let e = e.to_owned();
                    if is_expr_tag(e.name().0) {
                        let expr = self.parse_expr_content(e.attributes(), e.name().0, false)?;
                        exprs.push(expr);
                    } else {
                        let field = self.parse_field_content(&e, false)?;
                        if let Some(field) = field {
                            fields.push(field);
                        }
                    }
                }
                XmlEv::Empty(ref e) => {
                    let e = e.to_owned();
                    if is_expr_tag(e.name().0) {
                        let expr = self.parse_expr_content(e.attributes(), e.name().0, true)?;
                        exprs.push(expr);
                    } else {
                        let field = self.parse_field_content(&e, true)?;
                        if let Some(field) = field {
                            fields.push(field);
                        }
                    }
                }
                XmlEv::Comment(_) => {}
                XmlEv::End(ref e) => {
                    if e.name().0 == end_tag {
                        break;
                    }
                }
                ev => {
                    return Err(Error::Parse(format!(
                        "unexpected XML in switch case: {:?}",
                        ev
                    )))
                }
            }
        }

        Ok(SwitchCase {
            bit,
            name,
            exprs,
            fields,
        })
    }

    fn parse_switch(&mut self) -> Result<(Expr, Vec<SwitchCase>)> {
        let expr = self.parse_expr(b"")?.unwrap();

        let mut cases = Vec::new();

        loop {
            match self.xml.read_event_into(&mut self.buf)? {
                XmlEv::Start(ref e) => {
                    let names: [&[u8]; 1] = [b"name"];
                    let mut vals: [Option<String>; 1] = [None];
                    get_attributes(e.attributes(), &names, &mut vals)?;
                    let [name] = vals;
                    match e.name().0 {
                        b"bitcase" => {
                            cases.push(self.parse_switch_case(name, b"bitcase")?);
                        }
                        b"case" => {
                            cases.push(self.parse_switch_case(name, b"case")?);
                        }
                        name => {
                            return Err(Error::Parse(format!(
                                "unexpected <{}> in <switch>",
                                str::from_utf8(name).unwrap()
                            )));
                        }
                    }
                }
                XmlEv::End(ref e) => {
                    if e.name().0 == b"switch" {
                        break;
                    }
                }
                _ => {}
            }
        }

        Ok((expr, cases))
    }
}

struct OpStruct {
    number: i32,
    name: String,
    content: StructContent,
    r#ref: Option<String>,
    no_seq_number: bool,
    xge: bool,
}

struct StructContent {
    fields: Vec<Field>,
    reply: Option<Reply>,
    doc: Option<Doc>,
}

pub struct Request {
    pub name: String,
    pub opcode: u32,
    pub params: Vec<Field>,
    pub reply: Option<Reply>,
    pub doc: Option<Doc>,
}

fn is_expr_tag(tag: &[u8]) -> bool {
    matches!(
        tag,
        b"fieldref"
            | b"paramref"
            | b"op"
            | b"unop"
            | b"popcount"
            | b"value"
            | b"enumref"
            | b"sumof"
    )
}

fn attr_value(attr: &Attribute) -> Result<String> {
    let val = attr.unescape_value()?;
    Ok(str::from_utf8(val.as_bytes())?.into())
}

fn expect_attribute(attrs: Attributes, name: &[u8]) -> Result<String> {
    for attr in attrs {
        match attr {
            Ok(attr) => {
                if attr.key.0 == name {
                    return attr_value(&attr);
                }
            }
            Err(err) => return Err(quick_xml::Error::InvalidAttr(err).into()),
        }
    }
    Err(Error::Parse(format!(
        "could not find expected attribute: {}",
        str::from_utf8(name).unwrap()
    )))
}

fn get_attributes(attrs: Attributes, names: &[&[u8]], output: &mut [Option<String>]) -> Result<()> {
    assert_eq!(names.len(), output.len());
    for attr in attrs {
        match attr {
            Ok(attr) => {
                for (i, nam) in names.iter().enumerate() {
                    if attr.key.0 == *nam {
                        output[i] = Some(attr_value(&attr)?);
                    }
                }
            }
            Err(err) => return Err(quick_xml::Error::InvalidAttr(err).into()),
        }
    }
    Ok(())
}

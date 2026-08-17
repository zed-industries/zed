#![allow(clippy::useless_format)]

use crate::*;
use roxmltree::Node;

pub fn parse_config<'a>(
    xml_doc: &'a roxmltree::Document,
) -> Result<impl Iterator<Item = Result<ConfigPart>> + 'a> {
    let fontconfig = xml_doc.root_element();

    if fontconfig.tag_name().name() != "fontconfig" {
        return Err(Error::NoFontconfig);
    }

    Ok(fontconfig
        .children()
        .filter_map(|c| parse_config_part(c).transpose()))
}

fn parse_config_part(child: Node) -> Result<Option<ConfigPart>> {
    let part = match child.tag_name().name() {
        "description" => ConfigPart::Description(try_text!(child).into()),
        "alias" => {
            let mut alias = Alias::default();

            for child in child.children() {
                let families =
                    child
                        .children()
                        .filter_map(|family| match family.tag_name().name() {
                            "family" => family.text().map(Into::into),
                            _ => None,
                        });

                match child.tag_name().name() {
                    "family" => {
                        alias.alias = try_text!(child).into();
                    }
                    "prefer" => {
                        alias.prefer.extend(families);
                    }
                    "accept" => {
                        alias.accept.extend(families);
                    }
                    "default" => {
                        alias.default.extend(families);
                    }
                    _ => {}
                }
            }

            ConfigPart::Alias(alias)
        }
        "dir" => {
            let mut dir = Dir::default();

            parse_attrs!(child, {
                "prefix" => dir.prefix,
            }, {
                "salt" => dir.salt,
            });

            dir.path = try_text!(child).into();

            ConfigPart::Dir(dir)
        }
        "reset-dirs" => ConfigPart::ResetDirs,
        "remap-dir" => {
            let mut dir = RemapDir::default();

            parse_attrs!(child, {
                "prefix" => dir.prefix,
            }, {
                "salt" => dir.salt,
                "as-path" => dir.as_path,
            });

            dir.path = try_text!(child).into();

            ConfigPart::RemapDir(dir)
        }
        "cachedir" => {
            let mut dir = CacheDir::default();

            parse_attrs!(child, {
                "prefix" => dir.prefix,
            });

            dir.path = try_text!(child).into();

            ConfigPart::CacheDir(dir)
        }
        "include" => {
            let mut dir = Include::default();
            let mut ignore_missing = "";

            parse_attrs!(child, {
                "prefix" => dir.prefix,
            }, {
                "ignore_missing" => ignore_missing,
            });

            dir.ignore_missing = matches!(ignore_missing, "yes");
            dir.path = try_text!(child).into();

            ConfigPart::Include(dir)
        }
        "config" => {
            let mut config = Config::default();

            for child in child.children() {
                match child.tag_name().name() {
                    "rescan" => {
                        if let Some(int) = child.first_element_child() {
                            if int.tag_name().name() == "int" {
                                config.rescans.push(try_text!(int).parse()?);
                            }
                        }
                    }
                    "blank" => {
                        if let Some(child) = child.first_element_child() {
                            config.blanks.push(parse_int_or_range(child)?);
                        }
                    }
                    _ => {}
                }
            }

            ConfigPart::Config(config)
        }
        "selectfont" => {
            let mut s = SelectFont::default();

            for child in child.children() {
                let matches = child.children().filter_map(|c| match c.tag_name().name() {
                    "pattern" => {
                        let patelts = c.children().filter_map(|patelt| {
                            if patelt.tag_name().name() == "patelt" {
                                let mut kind = PropertyKind::default();
                                parse_attrs_opt!(patelt, {
                                    "name" => kind,
                                });
                                parse_expr(patelt.first_element_child()?)
                                    .ok()
                                    .map(|expr| kind.make_property(expr))
                            } else {
                                None
                            }
                        });
                        Some(FontMatch::Pattern(patelts.collect()))
                    }
                    "glob" => c.text().map(Into::into).map(FontMatch::Glob),
                    _ => None,
                });

                match child.tag_name().name() {
                    "acceptfont" => {
                        s.accepts.extend(matches);
                    }
                    "rejectfont" => {
                        s.rejects.extend(matches);
                    }
                    _ => {}
                }
            }

            ConfigPart::SelectFont(s)
        }
        "match" => {
            let mut m = Match::default();

            parse_attrs!(child, {
                "target" => m.target,
            });

            for child in child.children() {
                match child.tag_name().name() {
                    "test" => {
                        let mut t = Test::default();
                        let mut kind = PropertyKind::default();

                        parse_attrs!(child, {
                            "name" => kind,
                            "qual" => t.qual,
                            "target" => t.target,
                            "compare" => t.compare,
                        });

                        t.value = kind.make_property(parse_expr(
                            child
                                .first_element_child()
                                .ok_or_else(|| Error::InvalidFormat(format!("Empty test value")))?,
                        )?);

                        m.tests.push(t);
                    }

                    "edit" => {
                        let mut e = Edit::default();
                        let mut kind = PropertyKind::default();

                        parse_attrs!(child, {
                            "name" => kind,
                            "mode" => e.mode,
                            "binding" => e.binding,
                        });

                        e.value = kind.make_property(parse_expr(
                            child
                                .first_element_child()
                                .ok_or_else(|| Error::InvalidFormat(format!("Empty edit value")))?,
                        )?);

                        m.edits.push(e);
                    }
                    _ => {}
                }
            }

            ConfigPart::Match(m)
        }
        _ => {
            return Ok(None);
        }
    };

    Ok(Some(part))
}

fn parse_int_or_range(node: Node) -> Result<IntOrRange> {
    let mut texts = get_texts(&node);

    match node.tag_name().name() {
        "int" => Ok(IntOrRange::Int(try_text!(node).parse()?)),
        "range" => Ok(IntOrRange::Range(
            try_next!(texts, "Expect int").parse()?,
            try_next!(texts, "Expect int").parse()?,
        )),
        _ => Err(Error::InvalidFormat(format!("Expect IntOrRange"))),
    }
}

fn parse_expr(node: Node) -> Result<Expression> {
    let mut exprs = get_exprs(&node);
    let mut texts = get_texts(&node);

    macro_rules! next {
        ($iter:expr) => {
            try_next!($iter, "Expect expression")
        };
    }

    match node.tag_name().name() {
        "string" => Ok(Value::String(try_text!(node).into()).into()),
        "langset" => Ok(Value::LangSet(try_text!(node).into()).into()),
        "double" => Ok(Value::Double(try_text!(node).parse()?).into()),
        "int" => Ok(Value::Int(try_text!(node).parse()?).into()),
        "bool" => Ok(Value::Bool(try_text!(node).parse()?).into()),
        "const" => Ok(Value::Constant(try_text!(node).parse()?).into()),
        "matrix" => Ok(Expression::Matrix(Box::new([
            next!(exprs)?,
            next!(exprs)?,
            next!(exprs)?,
            next!(exprs)?,
        ]))),
        "charset" => {
            let charset = node
                .children()
                .filter_map(|c| parse_int_or_range(c).ok())
                .collect();

            Ok(Value::CharSet(charset).into())
        }
        "range" => Ok(Value::Range(next!(texts).parse()?, next!(texts).parse()?).into()),
        "name" => {
            let mut target = PropertyTarget::default();
            parse_attrs!(node, {
                "target" => target,
            });
            let kind = try_text!(node).parse()?;

            Ok(Value::Property(target, kind).into())
        }
        name => {
            if let Ok(list_op) = name.parse() {
                Ok(Expression::List(
                    list_op,
                    exprs.collect::<Result<Vec<_>>>()?,
                ))
            } else if let Ok(unary_op) = name.parse() {
                Ok(Expression::Unary(unary_op, Box::new(next!(exprs)?)))
            } else if let Ok(binary_op) = name.parse() {
                Ok(Expression::Binary(
                    binary_op,
                    Box::new([next!(exprs)?, next!(exprs)?]),
                ))
            } else if let Ok(ternary_op) = name.parse() {
                Ok(Expression::Ternary(
                    ternary_op,
                    Box::new([next!(exprs)?, next!(exprs)?, next!(exprs)?]),
                ))
            } else {
                Err(Error::InvalidFormat(format!(
                    "Unknown expression: {:?}",
                    node.tag_name(),
                )))
            }
        }
    }
}

fn get_exprs<'a>(node: &'a Node) -> impl Iterator<Item = Result<Expression>> + 'a {
    node.children().filter_map(|n| {
        if n.is_element() {
            Some(parse_expr(n))
        } else {
            None
        }
    })
}

fn get_texts<'a>(node: &'a Node) -> impl Iterator<Item = &'a str> {
    node.children()
        .filter_map(|n| if n.is_element() { n.text() } else { None })
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! make_parse_failed_test {
        ($name:ident, $test_fn:ident, $text:expr,) => {
            #[test]
            #[should_panic]
            fn $name() {
                let doc = roxmltree::Document::parse($text).expect("Parsing xml");
                let node = doc.root_element();
                $test_fn(node).expect("Run parse");
            }
        };
    }

    macro_rules! make_parse_test {
        ($name:ident, $test_fn:ident, $text:expr, $value:expr,) => {
            #[test]
            fn $name() {
                let doc = roxmltree::Document::parse($text).expect("Parsing xml");
                let node = doc.root_element();
                let ret = $test_fn(node).expect("Run parse");
                let expected = $value;
                k9::assert_equal!(expected, ret);
            }
        };
    }

    make_parse_test!(
        test_parse_charset,
        parse_expr,
        "<charset><range><int>0</int><int>123</int></range></charset>",
        Expression::from(vec![IntOrRange::Range(0, 123)]),
    );

    make_parse_test!(
        test_parse_int,
        parse_expr,
        "<int>123</int>",
        Expression::from(123),
    );

    make_parse_failed_test!(test_parse_invalid_int, parse_expr, "<int>123f</int>",);

    make_parse_test!(
        test_parse_range,
        parse_expr,
        "<range><int>0</int><int>10</int></range>",
        Expression::from(Value::Range(0, 10)),
    );

    make_parse_failed_test!(
        test_parse_invalid_range,
        parse_expr,
        "<range>0<int>10</int></range>",
    );

    make_parse_test!(
        test_langset,
        parse_expr,
        "<langset>ko-KR</langset>",
        Expression::from(Value::LangSet("ko-KR".into())),
    );
}

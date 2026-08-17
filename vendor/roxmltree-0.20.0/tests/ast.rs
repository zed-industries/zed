use roxmltree::*;

use std::fmt;
use std::fmt::Write;
use std::fs;
use std::io::Read;
use std::path;

#[derive(Clone, Copy, PartialEq)]
struct TStr<'a>(pub &'a str);

impl<'a> fmt::Debug for TStr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

trait HasExtension {
    fn has_extension(&self, ext: &str) -> bool;
}

impl HasExtension for path::Path {
    fn has_extension(&self, ext: &str) -> bool {
        if let Some(e) = self.extension() {
            e == ext
        } else {
            false
        }
    }
}

fn actual_test(path: &str) {
    let path = path::Path::new(path);
    let expected = load_file(&path.with_extension("yaml"));

    let opt = ParsingOptions {
        allow_dtd: true,
        ..roxmltree::ParsingOptions::default()
    };

    let input_xml = load_file(path);
    let doc = match Document::parse_with_options(&input_xml, opt) {
        Ok(v) => v,
        Err(e) => {
            assert_eq!(TStr(&format!("error: \"{}\"", e)), TStr(expected.trim()));
            return;
        }
    };

    assert_eq!(TStr(&to_yaml(&doc)), TStr(&expected));
}

fn load_file(path: &path::Path) -> String {
    let mut file = fs::File::open(path).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    text
}

fn to_yaml(doc: &Document) -> String {
    let mut s = String::new();
    _to_yaml(doc, &mut s).unwrap();
    s
}

fn _to_yaml(doc: &Document, s: &mut String) -> Result<(), fmt::Error> {
    if !doc.root().has_children() {
        return write!(s, "Document:");
    }

    macro_rules! writeln_indented {
        ($depth:expr, $f:expr, $fmt:expr) => {
            for _ in 0..$depth { write!($f, "  ")?; }
            writeln!($f, $fmt)?;
        };
        ($depth:expr, $f:expr, $fmt:expr, $($arg:tt)*) => {
            for _ in 0..$depth { write!($f, "  ")?; }
            writeln!($f, $fmt, $($arg)*)?;
        };
    }

    fn print_children(parent: Node, depth: usize, s: &mut String) -> Result<(), fmt::Error> {
        for child in parent.children() {
            match child.node_type() {
                NodeType::Element => {
                    writeln_indented!(depth, s, "- Element:");

                    match child.tag_name().namespace() {
                        Some(ns) => {
                            if ns.is_empty() {
                                writeln_indented!(
                                    depth + 2,
                                    s,
                                    "tag_name: {}",
                                    child.tag_name().name()
                                );
                            } else {
                                writeln_indented!(
                                    depth + 2,
                                    s,
                                    "tag_name: {}@{}",
                                    child.tag_name().name(),
                                    ns
                                );
                            }
                        }
                        None => {
                            writeln_indented!(
                                depth + 2,
                                s,
                                "tag_name: {}",
                                child.tag_name().name()
                            );
                        }
                    }

                    let attributes = child.attributes();
                    if attributes.len() != 0 {
                        let mut attrs = Vec::new();
                        for attr in attributes {
                            match attr.namespace() {
                                Some(ns) => {
                                    attrs.push((format!("{}@{}", attr.name(), ns), attr.value()));
                                }
                                None => {
                                    attrs.push((attr.name().to_string(), attr.value()));
                                }
                            }
                        }
                        attrs.sort_by(|a, b| a.0.cmp(&b.0));

                        writeln_indented!(depth + 2, s, "attributes:");
                        for (name, value) in attrs {
                            writeln_indented!(depth + 3, s, "{}: {:?}", name, value);
                        }
                    }

                    let namespaces = child.namespaces();
                    if namespaces.len() != 0 {
                        let mut ns_list = Vec::new();
                        for ns in namespaces {
                            let name = ns.name().unwrap_or("None");
                            let uri = if ns.uri().is_empty() {
                                "\"\""
                            } else {
                                ns.uri()
                            };
                            ns_list.push((name, uri));
                        }
                        ns_list.sort_by(|a, b| a.0.cmp(b.0));

                        writeln_indented!(depth + 2, s, "namespaces:");
                        for (name, uri) in ns_list {
                            writeln_indented!(depth + 3, s, "{}: {}", name, uri);
                        }
                    }

                    if child.has_children() {
                        writeln_indented!(depth + 2, s, "children:");
                        print_children(child, depth + 3, s)?;
                    }
                }
                NodeType::Text => {
                    writeln_indented!(depth, s, "- Text: {:?}", child.text().unwrap());
                }
                NodeType::Comment => {
                    writeln_indented!(depth, s, "- Comment: {:?}", child.text().unwrap());
                }
                NodeType::PI => {
                    if child.parent().unwrap().is_root() {
                        continue;
                    }

                    writeln_indented!(depth, s, "- PI:");

                    let pi = child.pi().unwrap();
                    writeln_indented!(depth + 2, s, "target: {:?}", pi.target);
                    if let Some(value) = pi.value {
                        writeln_indented!(depth + 2, s, "value: {:?}", value);
                    }
                }
                NodeType::Root => {}
            }
        }

        Ok(())
    }

    writeln!(s, "Document:")?;
    print_children(doc.root(), 1, s)?;

    Ok(())
}

macro_rules! test {
    ($name:ident) => {
        #[test]
        fn $name() {
            actual_test(&format!("tests/files/{}.xml", stringify!($name)))
        }
    };
}

test!(attrs_001);
test!(attrs_002);
test!(attrs_003);
test!(attrs_004);
test!(attrs_005);
test!(attrs_006);
test!(attrs_err_001);
test!(attrs_err_002);
test!(cdata_001);
test!(cdata_002);
test!(cdata_003);
test!(cdata_004);
test!(cdata_005);
test!(cdata_006);
test!(cdata_007);
test!(comments_001);
test!(elems_err_001);
test!(elems_err_002);
test!(entity_001);
test!(entity_002);
test!(entity_003);
test!(entity_004);
test!(entity_005);
test!(entity_006);
test!(entity_007);
test!(entity_008);
test!(entity_009);
test!(entity_010);
test!(entity_011);
test!(entity_012);
test!(entity_013);
test!(entity_014);
test!(entity_err_001);
test!(entity_err_002);
test!(entity_err_003);
test!(entity_err_004);
test!(entity_err_005);
test!(entity_err_006);
test!(entity_err_007);
test!(entity_err_008);
test!(entity_err_009);
test!(ns_001);
test!(ns_002);
test!(ns_003);
test!(ns_004);
test!(ns_005);
test!(ns_006);
test!(ns_007);
test!(ns_008);
test!(ns_009);
test!(ns_010);
test!(ns_011);
test!(ns_012);
test!(ns_013);
test!(ns_014);
test!(ns_015);
test!(ns_016);
test!(ns_017);
test!(ns_err_001);
test!(ns_err_002);
test!(ns_err_003);
test!(ns_err_004);
test!(ns_err_005);
test!(ns_err_006);
test!(ns_err_007);
test!(ns_err_008);
test!(ns_err_009);
test!(ns_err_010);
test!(ns_err_011);
test!(ns_err_012);
test!(ns_err_013);
test!(text_001);
test!(text_002);
test!(text_003);
test!(text_004);
test!(text_005);
test!(text_006);
test!(text_007);
test!(text_008);
test!(text_009);
test!(text_010);
test!(text_011);
test!(text_012);
test!(tree_001);
test!(tree_002);
// test!(tree_003); // unsupported
test!(tree_err_001);
test!(tree_err_002);
test!(tree_err_003);

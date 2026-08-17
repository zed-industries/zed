#![allow(clippy::bool_assert_comparison)]

use roxmltree::*;

#[test]
fn error_size() {
    assert!(::std::mem::size_of::<Error>() <= 64);
}

#[test]
fn root_element_01() {
    let data = "\
<!-- comment -->
<e/>
";

    let doc = Document::parse(data).unwrap();
    let node = doc.root_element();
    assert_eq!(node.tag_name().name(), "e");
}

#[test]
fn get_text_01() {
    let data = "\
<root>
    Text1
    <item>
        Text2
    </item>
    Text3
</root>
";

    let doc = Document::parse(data).unwrap();
    let root = doc.root_element();

    assert_eq!(root.text(), Some("\n    Text1\n    "));
    assert_eq!(root.tail(), None);

    let item = root.children().nth(1).unwrap();

    assert_eq!(item.text(), Some("\n        Text2\n    "));
    assert_eq!(item.tail(), Some("\n    Text3\n"));
}

#[test]
fn get_text_02() {
    let data = "<root>&apos;</root>";

    let doc = Document::parse(data).unwrap();
    let root = doc.root_element();

    assert_eq!(root.text(), Some("'"));
}

#[test]
fn api_01() {
    let data = "\
<e a:attr='a_ns' b:attr='b_ns' attr='no_ns' xmlns:b='http://www.ietf.org' \
    xmlns:a='http://www.w3.org' xmlns='http://www.uvic.ca'/>
";

    let doc = Document::parse(data).unwrap();
    let p = doc.root_element();

    assert_eq!(p.attribute("attr"), Some("no_ns"));
    assert_eq!(p.has_attribute("attr"), true);

    assert_eq!(p.attribute(("http://www.w3.org", "attr")), Some("a_ns"));
    assert_eq!(p.has_attribute(("http://www.w3.org", "attr")), true);

    assert_eq!(p.attribute("attr2"), None);
    assert_eq!(p.has_attribute("attr2"), false);

    assert_eq!(p.attribute(("http://www.w2.org", "attr")), None);
    assert_eq!(p.has_attribute(("http://www.w2.org", "attr")), false);

    assert_eq!(p.attribute("b"), None);
    assert_eq!(p.has_attribute("b"), false);

    assert_eq!(p.attribute("xmlns"), None);
    assert_eq!(p.has_attribute("xmlns"), false);
}

#[test]
fn get_pi() {
    let data = "\
<?target value?>
<root/>
";

    let doc = Document::parse(data).unwrap();
    let node = doc.root().first_child().unwrap();
    assert_eq!(
        node.pi(),
        Some(PI {
            target: "target",
            value: Some("value")
        })
    );
}

#[test]
fn lookup_prefix_01() {
    let data = "<e xmlns:n1='http://www.w3.org' n1:a='b1'/>";

    let doc = Document::parse(data).unwrap();
    let node = doc.root_element();
    assert_eq!(node.lookup_prefix("http://www.w3.org"), Some("n1"));
    assert_eq!(node.lookup_prefix("http://www.w4.org"), None);
}

#[test]
fn lookup_prefix_02() {
    let data = "<e xml:space='preserve'/>";

    let doc = Document::parse(data).unwrap();
    let node = doc.root_element();
    assert_eq!(node.lookup_prefix(NS_XML_URI), Some("xml"));
}

#[test]
fn lookup_namespace_uri() {
    let data = "<e xmlns:n1='http://www.w3.org' xmlns='http://www.w4.org'/>";

    let doc = Document::parse(data).unwrap();
    let node = doc.root_element();
    assert_eq!(
        node.lookup_namespace_uri(Some("n1")),
        Some("http://www.w3.org")
    );
    assert_eq!(node.lookup_namespace_uri(None), Some("http://www.w4.org"));
    assert_eq!(node.lookup_namespace_uri(Some("n2")), None);
}

#[cfg(feature = "positions")]
#[test]
fn text_pos_01() {
    let data = "\
<e a='b'>
    <!-- comment -->
    <p>Text</p>
</e>
";

    let doc = Document::parse(data).unwrap();
    let node = doc.root_element();

    assert_eq!(
        doc.text_pos_at(doc.root().range().start),
        TextPos::new(1, 1)
    );
    assert_eq!(doc.text_pos_at(doc.root().range().end), TextPos::new(5, 1));

    assert_eq!(doc.text_pos_at(node.range().start), TextPos::new(1, 1));
    assert_eq!(doc.text_pos_at(node.range().end), TextPos::new(4, 5));

    if let Some(attr) = node.attribute_node("a") {
        assert_eq!(doc.text_pos_at(attr.range().start), TextPos::new(1, 4));
        assert_eq!(doc.text_pos_at(attr.range().end), TextPos::new(1, 9));
        assert_eq!(doc.text_pos_at(attr.range_qname().start), TextPos::new(1, 4));
        assert_eq!(doc.text_pos_at(attr.range_qname().end), TextPos::new(1, 5));
        assert_eq!(doc.text_pos_at(attr.range_value().start), TextPos::new(1, 7));
        assert_eq!(doc.text_pos_at(attr.range_value().end), TextPos::new(1, 8));
    }

    // first child is a text/whitespace, not a comment
    let comm = node.first_child().unwrap().next_sibling().unwrap();
    assert_eq!(doc.text_pos_at(comm.range().start), TextPos::new(2, 5));

    let p = comm.next_sibling().unwrap().next_sibling().unwrap();
    assert_eq!(doc.text_pos_at(p.range().start), TextPos::new(3, 5));

    let text = p.first_child().unwrap();
    assert_eq!(doc.text_pos_at(text.range().start), TextPos::new(3, 8));
}

#[cfg(feature = "positions")]
#[test]
fn text_pos_02() {
    let data = "<n1:e xmlns:n1='http://www.w3.org' n1:a='b'/>";

    let doc = Document::parse(data).unwrap();
    let node = doc.root_element();

    assert_eq!(doc.text_pos_at(node.range().start), TextPos::new(1, 1));

    if let Some(attr) = node.attribute_node(("http://www.w3.org", "a")) {
        assert_eq!(doc.text_pos_at(attr.range().start), TextPos::new(1, 36));
        assert_eq!(doc.text_pos_at(attr.range().end), TextPos::new(1, 44));
        assert_eq!(doc.text_pos_at(attr.range_qname().start), TextPos::new(1, 36));
        assert_eq!(doc.text_pos_at(attr.range_qname().end), TextPos::new(1, 40));
        assert_eq!(doc.text_pos_at(attr.range_value().start), TextPos::new(1, 42));
        assert_eq!(doc.text_pos_at(attr.range_value().end), TextPos::new(1, 43));
    }
}

#[cfg(feature = "positions")]
#[test]
fn text_pos_03() {
    let data = "\
<!-- comment -->
<e/>
";

    let doc = Document::parse(data).unwrap();
    let node = doc.root_element();

    assert_eq!(doc.text_pos_at(node.range().start), TextPos::new(2, 1));
    assert_eq!(doc.text_pos_at(node.range().end), TextPos::new(2, 5));
}

#[cfg(feature = "positions")]
#[test]
fn text_pos_04() {
    let data = "<n1:e xmlns:n1='http://www.w3.org' n1:a=''/>";

    let doc = Document::parse(data).unwrap();
    let node = doc.root_element();

    if let Some(attr) = node.attribute_node("a") {
        assert_eq!(doc.text_pos_at(attr.range().start), TextPos::new(1, 36));
        assert_eq!(doc.text_pos_at(attr.range().end), TextPos::new(1, 43));
        assert_eq!(doc.text_pos_at(attr.range_qname().start), TextPos::new(1, 36));
        assert_eq!(doc.text_pos_at(attr.range_qname().end), TextPos::new(1, 40));
        assert_eq!(doc.text_pos_at(attr.range_value().start), TextPos::new(1, 42));
        assert_eq!(doc.text_pos_at(attr.range_value().end), TextPos::new(1, 42));
}
}

#[cfg(feature = "positions")]
#[test]
fn text_pos_05() {
    let data = "<n1:e xmlns:n1='http://www.w3.org' n1:a  =   'b'/>";

    let doc = Document::parse(data).unwrap();
    let node = doc.root_element();

    if let Some(attr) = node.attribute_node("a") {
        assert_eq!(doc.text_pos_at(attr.range().start), TextPos::new(1, 36));
        assert_eq!(doc.text_pos_at(attr.range().end), TextPos::new(1, 48));
        assert_eq!(doc.text_pos_at(attr.range_qname().start), TextPos::new(1, 36));
        assert_eq!(doc.text_pos_at(attr.range_qname().end), TextPos::new(1, 40));
        assert_eq!(doc.text_pos_at(attr.range_value().start), TextPos::new(1, 47));
        assert_eq!(doc.text_pos_at(attr.range_value().end), TextPos::new(1, 48));
    }
}

#[cfg(feature = "positions")]
#[test]
fn text_pos_06() {
    //              0         1         2         3         4         5         6         7         8         9        10        11        12        13        14        15        16        17        18        19        20        21        22        23        24        25        26
    let data = "<e a                                                                                                    =                                                                                                                                                                'b'/>";

    let doc = Document::parse(data).unwrap();
    let node = doc.root_element();

    if let Some(attr) = node.attribute_node("a") {
        assert_eq!(doc.text_pos_at(attr.range().start), TextPos::new(1, 4));
        assert_eq!(doc.text_pos_at(attr.range().end), TextPos::new(1, 269));
        assert_eq!(doc.text_pos_at(attr.range_qname().start), TextPos::new(1, 4));
        assert_eq!(doc.text_pos_at(attr.range_qname().end), TextPos::new(1, 5));
        attr.range_value(); // unreliable since >254 spaces around equal sign, but still shouldn't panic
    }
}

#[test]
fn next_sibling_element_01() {
    let data = "<root><a/><b/><c/></root>";

    let doc = roxmltree::Document::parse(data).unwrap();

    let root = doc.root_element();
    let a = root.first_element_child().unwrap();
    let b = a.next_sibling_element().unwrap();
    let c = b.next_sibling_element().unwrap();
    assert!(c.next_sibling_element().is_none());

    assert_eq!(root.tag_name().name(), "root");
    assert_eq!(a.tag_name().name(), "a");
    assert_eq!(b.tag_name().name(), "b");
    assert_eq!(c.tag_name().name(), "c");
}

#[test]
fn next_prev_element_01() {
    let data = "<root><a/><b/><c/></root>";

    let doc = roxmltree::Document::parse(data).unwrap();

    let root = doc.root_element();
    let c = root.last_element_child().unwrap();
    let b = c.prev_sibling_element().unwrap();
    let a = b.prev_sibling_element().unwrap();
    assert!(a.prev_sibling_element().is_none());

    assert_eq!(root.tag_name().name(), "root");
    assert_eq!(a.tag_name().name(), "a");
    assert_eq!(b.tag_name().name(), "b");
    assert_eq!(c.tag_name().name(), "c");
}

#[test]
fn nodes_document_order() {
    let data = "<root><a/><b/><c/></root>";

    let doc = roxmltree::Document::parse(data).unwrap();
    let root = doc.root_element();
    let a = root.first_element_child().unwrap();
    let b = a.next_sibling_element().unwrap();
    let c = b.next_sibling_element().unwrap();

    let mut elems = vec![&b, &c, &a];
    elems.sort();
    assert!(elems[0] == &a);
    assert!(elems[1] == &b);
    assert!(elems[2] == &c);
}

#[test]
fn lifetimes() {
    fn f<'a, 'd, F, R>(doc: &'a roxmltree::Document<'d>, fun: F) -> R
    where
        F: Fn(&'a roxmltree::Document<'d>) -> R,
    {
        fun(doc)
    }

    let doc = roxmltree::Document::parse("<e xmlns='http://www.w3.org'/>").unwrap();

    let _ = f(&doc, |d| d.root());
    let _ = f(&doc, |d| d.root().document());
    let _ = f(&doc, |d| d.root().tag_name());
    let _ = f(&doc, |d| d.root().tag_name().namespace());
    let _ = f(&doc, |d| d.root().tag_name().name());
    let _ = f(&doc, |d| d.root().default_namespace());
    let _ = f(&doc, |d| d.root().lookup_prefix(""));
    let _ = f(&doc, |d| d.root().lookup_namespace_uri(None));
    let _ = f(&doc, |d| d.root().attribute("a"));
    let _ = f(&doc, |d| d.root().attribute_node("a"));
    let _ = f(&doc, |d| d.root().attributes());
    let _ = f(&doc, |d| d.root().namespaces());
    let _ = f(&doc, |d| d.root().text());
    let _ = f(&doc, |d| d.root().tail());
    let _ = f(&doc, |d| d.root().pi());
}

#[test]
fn tag_name_lifetime() {
    fn get_tag_name<'a, 'input>(node: &'a Node<'a, 'input>) -> &'input str {
        node.tag_name().name()
    }

    let data = "<e xmlns='http://www.w3.org' />";
    let doc = roxmltree::Document::parse(data).unwrap();
    let root = doc.root_element();
    assert_eq!(get_tag_name(&root), "e");
}

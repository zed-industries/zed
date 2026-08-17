use roxmltree::*;

// Document.getElementsByTagName()
#[test]
fn get_elements_by_tag_name() {
    let data = "\
<!-- comment -->
<svg>
    <rect/>
    <text>Text</text>
    <g>
        <!-- comment -->
        <rect/>
    </g>
</svg>
";

    let doc = Document::parse(data).unwrap();

    let nodes: Vec<Node> = doc
        .descendants()
        .filter(|n| n.has_tag_name("rect"))
        .collect();
    assert_eq!(nodes.len(), 2);
}

// Document.getElementsByTagNameNS()
#[test]
fn get_elements_by_tag_name_ns() {
    let data = "\
<!-- comment -->
<svg xmlns:q='http://www.w3.org/'>
    <rect/>
    <text>Text</text>
    <g>
        <!-- comment -->
        <q:rect/>
    </g>
</svg>
";

    let doc = Document::parse(data).unwrap();

    let nodes: Vec<Node> = doc
        .descendants()
        .filter(|n| n.has_tag_name(("http://www.w3.org/", "rect")))
        .collect();
    assert_eq!(nodes.len(), 1);
}

// ParentNode.childElementCount
#[test]
fn child_element_count() {
    let data = "\
<svg>
    <rect/>
    <!-- comment -->
    <rect/>
    <!-- comment -->
    <rect/>
</svg>
";

    let doc = Document::parse(data).unwrap();
    let svg_elem = doc.root_element();

    let count = svg_elem.children().filter(|n| n.is_element()).count();
    assert_eq!(count, 3);
}

// ParentNode.children
#[test]
fn children() {
    let data = "\
<svg>
    <rect/>
    <!-- comment -->
    <rect/>
    <!-- comment -->
    <rect/>
</svg>
";

    let doc = Document::parse(data).unwrap();
    let svg_elem = doc.root_element();

    let count = svg_elem.children().filter(|n| n.is_element()).count();
    assert_eq!(count, 3);
}

// ParentNode.firstElementChild
#[test]
fn first_element_child() {
    let data = "\
<svg>
    <!-- comment -->
    <rect/>
</svg>
";

    let doc = Document::parse(data).unwrap();
    let svg_elem = doc.root_element();

    let elem = svg_elem.first_element_child().unwrap();
    assert!(elem.has_tag_name("rect"));

    // or

    let elem = svg_elem.children().find(|n| n.is_element()).unwrap();
    assert!(elem.has_tag_name("rect"));
}

// ParentNode.lastElementChild
#[test]
fn last_element_child() {
    let data = "\
<svg>
    <!-- comment -->
    <rect/>
    <!-- comment -->
</svg>
";

    let doc = Document::parse(data).unwrap();
    let svg_elem = doc.root_element();

    let elem = svg_elem.last_element_child().unwrap();
    assert!(elem.has_tag_name("rect"));

    // or

    let elem = svg_elem
        .children()
        .filter(|n| n.is_element())
        .last()
        .unwrap();
    assert!(elem.has_tag_name("rect"));
}

// Document.getElementById
#[test]
fn get_element_by_id() {
    let data = "\
<svg id='svg1'>
    <circle id='circle1'/>
    <g>
        <rect id='rect1'/>
    </g>
</svg>
";

    let doc = Document::parse(data).unwrap();
    let elem = doc
        .descendants()
        .find(|n| n.attribute("id") == Some("rect1"))
        .unwrap();
    assert!(elem.has_tag_name("rect"));
}

// Node.ownerDocument
#[test]
fn owner_document() {
    let doc = Document::parse("<svg/>").unwrap();
    let _elem = doc.root_element();
}

// Node.parentElement
#[test]
fn parent_element() {
    let data = "\
<svg>
    <!-- comment -->
    <rect/>
</svg>
";

    let doc = Document::parse(data).unwrap();
    let rect = doc.descendants().find(|n| n.has_tag_name("rect")).unwrap();
    assert!(rect.parent_element().unwrap().has_tag_name("svg"));

    // or

    assert!(rect
        .ancestors()
        .skip(1)
        .find(|n| n.is_element())
        .unwrap()
        .has_tag_name("svg"));
}

// Node.contains
#[test]
fn contains() {
    let data = "\
<svg>
    <rect/>
</svg>
";

    let doc = Document::parse(data).unwrap();
    let svg = doc.root_element();
    let rect = svg.first_child().unwrap();

    assert!(svg.descendants().any(|n| n == rect));
}

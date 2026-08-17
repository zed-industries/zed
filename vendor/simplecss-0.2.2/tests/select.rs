// Copyright 2019 the SimpleCSS Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Select

use simplecss::*;

struct XmlNode<'a, 'input: 'a>(roxmltree::Node<'a, 'input>);

impl<'a, 'input: 'a> XmlNode<'a, 'input> {
    fn select(&self, text: &str) -> Vec<roxmltree::Node<'a, 'input>> {
        let selectors = Selector::parse(text).unwrap();
        let mut nodes = Vec::new();
        for node in self.0.descendants().filter(|n| n.is_element()) {
            if selectors.matches(&XmlNode(node)) {
                nodes.push(node);
            }
        }

        nodes
    }
}

impl Element for XmlNode<'_, '_> {
    fn parent_element(&self) -> Option<Self> {
        self.0.parent_element().map(XmlNode)
    }

    fn prev_sibling_element(&self) -> Option<Self> {
        self.0.prev_sibling_element().map(XmlNode)
    }

    fn has_local_name(&self, local_name: &str) -> bool {
        self.0.tag_name().name() == local_name
    }

    fn attribute_matches(&self, local_name: &str, operator: AttributeOperator<'_>) -> bool {
        match self.0.attribute(local_name) {
            Some(value) => operator.matches(value),
            None => false,
        }
    }

    fn pseudo_class_matches(&self, class: PseudoClass<'_>) -> bool {
        match class {
            PseudoClass::FirstChild => self.prev_sibling_element().is_none(),
            _ => false,
        }
    }
}

macro_rules! match_single {
    ($doc:expr, $selector:expr) => {{
        let nodes = XmlNode($doc.root_element()).select($selector);
        assert_eq!(nodes.len(), 1);
        nodes[0].attribute("id").unwrap()
    }};
}

macro_rules! match_none {
    ($doc:expr, $selector:expr) => {{
        assert_eq!(XmlNode($doc.root_element()).select($selector).len(), 0);
    }};
}

#[test]
fn select_01() {
    let doc = roxmltree::Document::parse("<div id='div1'/>").unwrap();
    assert_eq!(match_single!(doc, "*"), "div1");
}

#[test]
fn select_02() {
    let doc = roxmltree::Document::parse("<div id='div1'/>").unwrap();
    assert_eq!(match_single!(doc, "div"), "div1");
    match_none!(doc, "p");
}

#[test]
fn select_03() {
    let doc = roxmltree::Document::parse("<div id='div1'/>").unwrap();
    assert_eq!(match_single!(doc, "#div1"), "div1");
    match_none!(doc, "#d1");
}

#[test]
fn select_04() {
    let doc = roxmltree::Document::parse("<div id='div1'/>").unwrap();
    match_none!(doc, "p#div1");
}

#[test]
fn select_05() {
    let doc = roxmltree::Document::parse(
        "\
<div id='div1'>
    <p id='p1'/>
</div>
",
    )
    .unwrap();

    assert_eq!(match_single!(doc, "div p"), "p1");
}

#[test]
fn select_06() {
    let doc = roxmltree::Document::parse(
        "\
<div id='div1'>
    <g id='g1'>
        <p id='p1'/>
    </g>
</div>
",
    )
    .unwrap();

    assert_eq!(match_single!(doc, "div p"), "p1");
}

#[test]
fn select_07() {
    let doc = roxmltree::Document::parse(
        "\
<div id='div1'>
    <div id='div2'>
        <g id='g1'>
            <p id='p1'/>
        </g>
    </div>
</div>
",
    )
    .unwrap();

    assert_eq!(match_single!(doc, "div p"), "p1");
}

#[test]
fn select_08() {
    let doc = roxmltree::Document::parse(
        "\
<div id='div1'>
    <g id='g1'>
        <p id='p1'>
            <div/>
        </p>
    </g>
</div>
",
    )
    .unwrap();

    assert_eq!(match_single!(doc, "div p"), "p1");
}

#[test]
fn select_09() {
    let doc = roxmltree::Document::parse(
        "\
<div id='div1'>
    <g id='g1'>
        <p id='p1'/>
    </g>
</div>
",
    )
    .unwrap();

    assert_eq!(match_single!(doc, "div g p"), "p1");
}

#[test]
fn select_10() {
    let doc = roxmltree::Document::parse(
        "\
<div id='div1'>
    <q id='g1'>
        <p id='p1'/>
    </q>
</div>
",
    )
    .unwrap();

    match_none!(doc, "div g p");
}

#[test]
fn select_11() {
    let doc = roxmltree::Document::parse(
        "\
<div id='div1'>
    <g id='g1'>
        <p id='p1'/>
    </g>
</div>
",
    )
    .unwrap();

    assert_eq!(match_single!(doc, "div * p"), "p1");
}

#[test]
fn select_12() {
    let doc = roxmltree::Document::parse(
        "\
<div id='div1'>
    <p id='p1'>
        <rect id='rect1'/>
        <rect id='rect2' color='green'/>
    </p>
</div>
",
    )
    .unwrap();

    assert_eq!(match_single!(doc, "div p *[color]"), "rect2");
    assert_eq!(match_single!(doc, "div p [color]"), "rect2");
}

#[test]
fn select_13() {
    let doc = roxmltree::Document::parse(
        "\
<div id='div1'>
    <p id='p1'/>
</div>
",
    )
    .unwrap();

    assert_eq!(match_single!(doc, "div > p"), "p1");
}

#[test]
fn select_14() {
    let doc = roxmltree::Document::parse(
        "\
<p id='p1'/>
",
    )
    .unwrap();

    match_none!(doc, "div > p");
}

#[test]
fn select_15() {
    let doc = roxmltree::Document::parse(
        "\
<div id='div1'>
    <g id='g1'>
        <p id='p1'/>
    </g>
</div>
",
    )
    .unwrap();

    match_none!(doc, "div > p");
}

#[test]
fn select_16() {
    let doc = roxmltree::Document::parse(
        "\
<div id='div1'>
    <p>
        <ol>
            <li>
                <g>
                    <p id='p1'/>
                </g>
            </li>
        </ol>
    </p>
</div>
",
    )
    .unwrap();

    assert_eq!(match_single!(doc, "div ol>li p"), "p1");
}

#[test]
fn select_17() {
    let doc = roxmltree::Document::parse(
        "\
<div id='div1'>
    <p>
        <ol>
            <g>
                <li>
                    <g>
                        <p id='p1'/>
                    </g>
                </li>
            </g>
        </ol>
    </p>
</div>
",
    )
    .unwrap();

    match_none!(doc, "div ol>li p");
}

#[test]
fn select_18() {
    let doc = roxmltree::Document::parse(
        "\
<div id='div1'>
    <g/>
    <p id='p1'/>
</div>
",
    )
    .unwrap();

    assert_eq!(match_single!(doc, "g + p"), "p1");
}

#[test]
fn select_19() {
    let doc = roxmltree::Document::parse(
        "\
<div id='div1'>
    <test/>
    <g/>
    <p id='p1'/>
</div>
",
    )
    .unwrap();

    assert_eq!(match_single!(doc, "g + p"), "p1");
}

#[test]
fn select_20() {
    let doc = roxmltree::Document::parse(
        "\
<div id='div1'>
    <p id='p1'/>
    <g/>
</div>
",
    )
    .unwrap();

    match_none!(doc, "g + p");
}

#[test]
fn select_21() {
    let doc = roxmltree::Document::parse(
        "\
<div id='div1'>
    <p id='p1'/>
</div>
",
    )
    .unwrap();

    match_none!(doc, "div + p");
}

#[test]
fn select_22() {
    let doc = roxmltree::Document::parse(
        "\
<div id='div1'>
    <p id='p1'/>
</div>
",
    )
    .unwrap();

    assert_eq!(match_single!(doc, "[id=p1]"), "p1");
}

#[test]
fn select_23() {
    let doc = roxmltree::Document::parse(
        "\
<div id='div1'>
    <p id='p1' class='test warn'/>
</div>
",
    )
    .unwrap();

    assert_eq!(match_single!(doc, "[class~=warn]"), "p1");
}

#[test]
fn select_24() {
    let doc = roxmltree::Document::parse(
        "\
<div id='div1'>
    <p id='p1' class='test warn'/>
</div>
",
    )
    .unwrap();

    match_none!(doc, "[class~='test warn']");
}

#[test]
fn select_25() {
    let doc = roxmltree::Document::parse(
        "\
<div id='div1'>
    <p id='p1' lang='en'/>
</div>
",
    )
    .unwrap();

    assert_eq!(match_single!(doc, "[lang=en]"), "p1");
    assert_eq!(match_single!(doc, "[lang|=en]"), "p1");
}

#[test]
fn select_26() {
    let doc = roxmltree::Document::parse(
        "\
<div id='div1'>
    <p id='p1' lang='en-US'/>
</div>
",
    )
    .unwrap();

    assert_eq!(match_single!(doc, "[lang='en-US']"), "p1");
    assert_eq!(match_single!(doc, "[lang|=en]"), "p1");
}

#[test]
fn select_27() {
    let doc = roxmltree::Document::parse(
        "\
<div id='div1'>
    <p id='p1' class='pastoral blue aqua marine'/>
</div>
",
    )
    .unwrap();

    assert_eq!(match_single!(doc, ".marine.pastoral"), "p1");
}

#[test]
fn select_28() {
    let doc = roxmltree::Document::parse(
        "\
<div id='div1'>
    <p id='p1'/>
</div>
",
    )
    .unwrap();

    assert_eq!(match_single!(doc, "p:first-child"), "p1");
}

#[test]
fn select_29() {
    let doc = roxmltree::Document::parse(
        "\
<div id='div1'>
    <rect/>
    <p id='p1'/>
</div>
",
    )
    .unwrap();

    match_none!(doc, "p:first-child");
}

#[test]
fn select_30() {
    let doc = roxmltree::Document::parse(
        "\
<div id='div1'>
    <p id='p1'/>
    <p id='p2'/>
</div>
",
    )
    .unwrap();

    let nodes = XmlNode(doc.root_element()).select(":first-child");
    assert_eq!(nodes.len(), 2);
    assert_eq!(nodes[0].attribute("id").unwrap(), "div1");
    assert_eq!(nodes[1].attribute("id").unwrap(), "p1");
}

#[test]
fn to_string() {
    let selectors = Selector::parse("a > b").unwrap();
    assert_eq!(selectors.to_string(), "a > b");
}

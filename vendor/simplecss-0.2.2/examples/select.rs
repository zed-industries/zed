// Copyright 2019 the SimpleCSS Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Select

struct XmlNode<'a, 'input: 'a>(roxmltree::Node<'a, 'input>);

impl<'a, 'input: 'a> XmlNode<'a, 'input> {
    fn select(&self, text: &str) -> Option<roxmltree::Node<'a, 'input>> {
        let selectors = simplecss::Selector::parse(text)?;
        self.0
            .descendants()
            .filter(|n| n.is_element())
            .find(|&node| selectors.matches(&XmlNode(node)))
    }
}

impl simplecss::Element for XmlNode<'_, '_> {
    fn parent_element(&self) -> Option<Self> {
        self.0.parent_element().map(XmlNode)
    }

    fn prev_sibling_element(&self) -> Option<Self> {
        self.0
            .prev_siblings()
            .filter(|n| n.is_element())
            .nth(0)
            .map(XmlNode)
    }

    fn has_local_name(&self, local_name: &str) -> bool {
        self.0.tag_name().name() == local_name
    }

    fn attribute_matches(
        &self,
        local_name: &str,
        operator: simplecss::AttributeOperator<'_>,
    ) -> bool {
        match self.0.attribute(local_name) {
            Some(value) => operator.matches(value),
            None => false,
        }
    }

    fn pseudo_class_matches(&self, class: simplecss::PseudoClass<'_>) -> bool {
        match class {
            simplecss::PseudoClass::FirstChild => self.prev_sibling_element().is_none(),
            _ => false, // Since we are querying a static XML we can ignore other pseudo-classes.
        }
    }
}

fn main() {
    let doc = roxmltree::Document::parse(
        "<svg>
            <g>
                <rect id='rect1' class='round blue'/>
                <rect id='rect2' color='red'/>
            </g>
        </svg>",
    )
    .unwrap();
    let root = XmlNode(doc.root_element());

    assert_eq!(
        root.select("rect:first-child")
            .unwrap()
            .attribute("id")
            .unwrap(),
        "rect1",
        "selected wrong element"
    );

    assert_eq!(
        root.select("[color=red]").unwrap().attribute("id").unwrap(),
        "rect2",
        "selected wrong element"
    );

    assert_eq!(
        root.select("svg rect").unwrap().attribute("id").unwrap(),
        "rect1",
        "selected wrong element"
    );

    assert_eq!(
        root.select("svg > g > rect")
            .unwrap()
            .attribute("id")
            .unwrap(),
        "rect1",
        "selected wrong element"
    );

    assert_eq!(
        root.select(".blue").unwrap().attribute("id").unwrap(),
        "rect1",
        "selected wrong element"
    );
}

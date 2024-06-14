use std::cell::RefCell;
use std::collections::HashSet;
use std::sync::OnceLock;

use html5ever::Attribute;

/// Returns a [`HashSet`] containing the HTML elements that are inline by default.
///
/// [MDN: List of "inline" elements](https://yari-demos.prod.mdn.mozit.cloud/en-US/docs/Web/HTML/Inline_elements)
fn inline_elements() -> &'static HashSet<&'static str> {
    static INLINE_ELEMENTS: OnceLock<HashSet<&str>> = OnceLock::new();
    &INLINE_ELEMENTS.get_or_init(|| {
        HashSet::from_iter([
            "a", "abbr", "acronym", "audio", "b", "bdi", "bdo", "big", "br", "button", "canvas",
            "cite", "code", "data", "datalist", "del", "dfn", "em", "embed", "i", "iframe", "img",
            "input", "ins", "kbd", "label", "map", "mark", "meter", "noscript", "object", "output",
            "picture", "progress", "q", "ruby", "s", "samp", "script", "select", "slot", "small",
            "span", "strong", "sub", "sup", "svg", "template", "textarea", "time", "tt", "u",
            "var", "video", "wbr",
        ])
    })
}

#[derive(Debug, Clone)]
pub struct HtmlElement {
    tag: String,
    pub(crate) attrs: RefCell<Vec<Attribute>>,
}

impl HtmlElement {
    pub fn new(tag: String, attrs: RefCell<Vec<Attribute>>) -> Self {
        Self { tag, attrs }
    }

    pub fn tag(&self) -> &str {
        &self.tag
    }

    /// Returns whether this [`HtmlElement`] is an inline element.
    pub fn is_inline(&self) -> bool {
        inline_elements().contains(self.tag.as_str())
    }

    /// Returns the attribute with the specified name.
    pub fn attr(&self, name: &str) -> Option<String> {
        self.attrs
            .borrow()
            .iter()
            .find(|attr| attr.name.local.to_string() == name)
            .map(|attr| attr.value.to_string())
    }

    /// Returns the list of classes on this [`HtmlElement`].
    pub fn classes(&self) -> Vec<String> {
        self.attrs
            .borrow()
            .iter()
            .find(|attr| attr.name.local.to_string() == "class")
            .map(|attr| {
                attr.value
                    .split(' ')
                    .map(|class| class.trim().to_string())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
    }

    /// Returns whether this [`HtmlElement`] has the specified class.
    pub fn has_class(&self, class: &str) -> bool {
        self.has_any_classes(&[class])
    }

    /// Returns whether this [`HtmlElement`] has any of the specified classes.
    pub fn has_any_classes(&self, classes: &[&str]) -> bool {
        self.attrs.borrow().iter().any(|attr| {
            attr.name.local.to_string() == "class"
                && attr
                    .value
                    .split(' ')
                    .any(|class| classes.contains(&class.trim()))
        })
    }
}

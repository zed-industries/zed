use indexmap::IndexMap;
use strum::{EnumIter, IntoEnumIterator};

use crate::html_element::HtmlElement;
use crate::markdown_writer::{HandleTag, HandlerOutcome, MarkdownWriter, StartTagOutcome};

pub struct RustdocHeadingHandler;

impl HandleTag for RustdocHeadingHandler {
    fn should_handle(&self, _tag: &str) -> bool {
        // We're only handling text, so we don't need to visit any tags.
        false
    }

    fn handle_text(&mut self, text: &str, writer: &mut MarkdownWriter) -> HandlerOutcome {
        if writer.is_inside("h1")
            || writer.is_inside("h2")
            || writer.is_inside("h3")
            || writer.is_inside("h4")
            || writer.is_inside("h5")
            || writer.is_inside("h6")
        {
            let text = text
                .trim_matches(|char| char == '\n' || char == '\r' || char == '§')
                .replace('\n', " ");
            writer.push_str(&text);

            return HandlerOutcome::Handled;
        }

        HandlerOutcome::NoOp
    }
}

pub struct RustdocCodeHandler;

impl HandleTag for RustdocCodeHandler {
    fn should_handle(&self, tag: &str) -> bool {
        match tag {
            "pre" | "code" => true,
            _ => false,
        }
    }

    fn handle_tag_start(
        &mut self,
        tag: &HtmlElement,
        writer: &mut MarkdownWriter,
    ) -> StartTagOutcome {
        match tag.tag.as_str() {
            "code" => {
                if !writer.is_inside("pre") {
                    writer.push_str("`");
                }
            }
            "pre" => {
                let classes = tag.classes();
                let is_rust = classes.iter().any(|class| class == "rust");
                let language = is_rust
                    .then(|| "rs")
                    .or_else(|| {
                        classes.iter().find_map(|class| {
                            if let Some((_, language)) = class.split_once("language-") {
                                Some(language.trim())
                            } else {
                                None
                            }
                        })
                    })
                    .unwrap_or("");

                writer.push_str(&format!("\n\n```{language}\n"));
            }
            _ => {}
        }

        StartTagOutcome::Continue
    }

    fn handle_tag_end(&mut self, tag: &HtmlElement, writer: &mut MarkdownWriter) {
        match tag.tag.as_str() {
            "code" => {
                if !writer.is_inside("pre") {
                    writer.push_str("`");
                }
            }
            "pre" => writer.push_str("\n```\n"),
            _ => {}
        }
    }

    fn handle_text(&mut self, text: &str, writer: &mut MarkdownWriter) -> HandlerOutcome {
        if writer.is_inside("pre") {
            writer.push_str(&text);
            return HandlerOutcome::Handled;
        }

        HandlerOutcome::NoOp
    }
}

const RUSTDOC_ITEM_NAME_CLASS: &str = "item-name";

pub struct RustdocItemHandler;

impl RustdocItemHandler {
    /// Returns whether we're currently inside of an `.item-name` element, which
    /// rustdoc uses to display Rust items in a list.
    fn is_inside_item_name(writer: &MarkdownWriter) -> bool {
        writer
            .current_element_stack()
            .iter()
            .any(|element| element.has_class(RUSTDOC_ITEM_NAME_CLASS))
    }
}

impl HandleTag for RustdocItemHandler {
    fn should_handle(&self, tag: &str) -> bool {
        match tag {
            "div" | "span" => true,
            _ => false,
        }
    }

    fn handle_tag_start(
        &mut self,
        tag: &HtmlElement,
        writer: &mut MarkdownWriter,
    ) -> StartTagOutcome {
        match tag.tag.as_str() {
            "div" | "span" => {
                if Self::is_inside_item_name(writer) && tag.has_class("stab") {
                    writer.push_str(" [");
                }
            }
            _ => {}
        }

        StartTagOutcome::Continue
    }

    fn handle_tag_end(&mut self, tag: &HtmlElement, writer: &mut MarkdownWriter) {
        match tag.tag.as_str() {
            "div" | "span" => {
                if tag.has_class(RUSTDOC_ITEM_NAME_CLASS) {
                    writer.push_str(": ");
                }

                if Self::is_inside_item_name(writer) && tag.has_class("stab") {
                    writer.push_str("]");
                }
            }
            _ => {}
        }
    }

    fn handle_text(&mut self, text: &str, writer: &mut MarkdownWriter) -> HandlerOutcome {
        if Self::is_inside_item_name(writer)
            && !writer.is_inside("span")
            && !writer.is_inside("code")
        {
            writer.push_str(&format!("`{text}`"));
            return HandlerOutcome::Handled;
        }

        HandlerOutcome::NoOp
    }
}

pub struct RustdocChromeRemover;

impl HandleTag for RustdocChromeRemover {
    fn should_handle(&self, tag: &str) -> bool {
        match tag {
            "head" | "script" | "nav" | "summary" | "button" | "div" | "span" => true,
            _ => false,
        }
    }

    fn handle_tag_start(
        &mut self,
        tag: &HtmlElement,
        _writer: &mut MarkdownWriter,
    ) -> StartTagOutcome {
        match tag.tag.as_str() {
            "head" | "script" | "nav" => return StartTagOutcome::Skip,
            "summary" => {
                if tag.has_class("hideme") {
                    return StartTagOutcome::Skip;
                }
            }
            "button" => {
                if tag.attr("id").as_deref() == Some("copy-path") {
                    return StartTagOutcome::Skip;
                }
            }
            "div" | "span" => {
                let classes_to_skip = ["nav-container", "sidebar-elems", "out-of-band"];
                if tag.has_any_classes(&classes_to_skip) {
                    return StartTagOutcome::Skip;
                }
            }
            _ => {}
        }

        StartTagOutcome::Continue
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, EnumIter)]
pub enum RustdocItemKind {
    Mod,
    Macro,
    Struct,
    Enum,
    Constant,
    Trait,
    Function,
    TypeAlias,
    AttributeMacro,
    DeriveMacro,
}

impl RustdocItemKind {
    const fn class(&self) -> &'static str {
        match self {
            Self::Mod => "mod",
            Self::Macro => "macro",
            Self::Struct => "struct",
            Self::Enum => "enum",
            Self::Constant => "constant",
            Self::Trait => "trait",
            Self::Function => "fn",
            Self::TypeAlias => "type",
            Self::AttributeMacro => "attr",
            Self::DeriveMacro => "derive",
        }
    }
}

#[derive(Debug, Clone)]
pub struct RustdocItem {
    pub kind: RustdocItemKind,
    pub name: String,
}

impl RustdocItem {
    pub fn url_path(&self) -> String {
        let name = &self.name;
        match self.kind {
            RustdocItemKind::Mod => format!("{name}/index.html"),
            RustdocItemKind::Macro
            | RustdocItemKind::Struct
            | RustdocItemKind::Enum
            | RustdocItemKind::Constant
            | RustdocItemKind::Trait
            | RustdocItemKind::Function
            | RustdocItemKind::TypeAlias
            | RustdocItemKind::AttributeMacro
            | RustdocItemKind::DeriveMacro => {
                format!("{kind}.{name}.html", kind = self.kind.class())
            }
        }
    }
}

pub struct RustdocItemCollector {
    pub items: IndexMap<(RustdocItemKind, String), RustdocItem>,
}

impl RustdocItemCollector {
    pub fn new() -> Self {
        Self {
            items: IndexMap::new(),
        }
    }

    fn parse_item(tag: &HtmlElement) -> Option<RustdocItem> {
        if tag.tag.as_str() != "a" {
            return None;
        }

        let href = tag.attr("href")?;
        if href == "#" {
            return None;
        }

        for kind in RustdocItemKind::iter() {
            if tag.has_class(kind.class()) {
                let name = href
                    .trim_start_matches(&format!("{}.", kind.class()))
                    .trim_end_matches("/index.html")
                    .trim_end_matches(".html");

                return Some(RustdocItem {
                    kind,
                    name: name.to_owned(),
                });
            }
        }

        None
    }
}

impl HandleTag for RustdocItemCollector {
    fn should_handle(&self, tag: &str) -> bool {
        tag == "a"
    }

    fn handle_tag_start(
        &mut self,
        tag: &HtmlElement,
        writer: &mut MarkdownWriter,
    ) -> StartTagOutcome {
        match tag.tag.as_str() {
            "a" => {
                let is_reexport = writer.current_element_stack().iter().any(|element| {
                    if let Some(id) = element.attr("id") {
                        id.starts_with("reexport.")
                    } else {
                        false
                    }
                });

                if !is_reexport {
                    if let Some(item) = Self::parse_item(tag) {
                        self.items.insert((item.kind, item.name.clone()), item);
                    }
                }
            }
            _ => {}
        }

        StartTagOutcome::Continue
    }
}

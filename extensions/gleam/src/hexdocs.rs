use std::cell::RefCell;
use std::collections::BTreeSet;
use std::io::Read;
use std::rc::Rc;

use html_to_markdown::markdown::{
    HeadingHandler, ListHandler, ParagraphHandler, StyledTextHandler, TableHandler,
};
use html_to_markdown::{
    convert_html_to_markdown, HandleTag, HandlerOutcome, HtmlElement, MarkdownWriter,
    StartTagOutcome, TagHandler,
};
use zed_extension_api::{self as zed, HttpRequest, KeyValueStore, Result};

pub fn index(package: String, database: &KeyValueStore) -> Result<()> {
    let response = zed::fetch(&HttpRequest {
        url: format!("https://hexdocs.pm/{package}"),
    })?;

    let (package_root_markdown, modules) = convert_hexdocs_to_markdown(response.body.as_bytes())?;

    database.insert(&package, &package_root_markdown)?;

    for module in modules {
        let response = zed::fetch(&HttpRequest {
            url: format!("https://hexdocs.pm/{package}/{module}.html"),
        })?;

        let (markdown, _modules) = convert_hexdocs_to_markdown(response.body.as_bytes())?;

        database.insert(&module, &markdown)?;
    }

    Ok(())
}

pub fn convert_hexdocs_to_markdown(html: impl Read) -> Result<(String, Vec<String>)> {
    let module_collector = Rc::new(RefCell::new(GleamModuleCollector::new()));

    let mut handlers: Vec<TagHandler> = vec![
        module_collector.clone(),
        Rc::new(RefCell::new(ParagraphHandler)),
        Rc::new(RefCell::new(HeadingHandler)),
        Rc::new(RefCell::new(ListHandler)),
        Rc::new(RefCell::new(TableHandler::new())),
        Rc::new(RefCell::new(StyledTextHandler)),
    ];

    let markdown = convert_html_to_markdown(html, &mut handlers)
        .map_err(|err| format!("failed to convert docs to Markdown {err}"))?;

    let modules = module_collector
        .borrow()
        .modules
        .iter()
        .cloned()
        .collect::<Vec<_>>();

    dbg!(&modules);

    Ok((markdown, modules))
}

pub struct GleamModuleCollector {
    modules: BTreeSet<String>,
    has_seen_modules_header: bool,
}

impl GleamModuleCollector {
    pub fn new() -> Self {
        Self {
            modules: BTreeSet::new(),
            has_seen_modules_header: false,
        }
    }

    fn parse_module(tag: &HtmlElement) -> Option<String> {
        if tag.tag() != "a" {
            return None;
        }

        let href = tag.attr("href")?;
        if href.starts_with('#') || href.starts_with("https://") || href.starts_with("../") {
            return None;
        }

        let module_name = href.trim_start_matches("./").trim_end_matches(".html");

        Some(module_name.to_owned())
    }
}

impl HandleTag for GleamModuleCollector {
    fn should_handle(&self, tag: &str) -> bool {
        match tag {
            "h2" | "a" => true,
            _ => false,
        }
    }

    fn handle_tag_start(
        &mut self,
        tag: &HtmlElement,
        writer: &mut MarkdownWriter,
    ) -> StartTagOutcome {
        match tag.tag() {
            "a" => {
                dbg!(self.has_seen_modules_header, writer.is_inside("li"), &tag);
                if self.has_seen_modules_header && writer.is_inside("li") {
                    if let Some(module_name) = Self::parse_module(tag) {
                        self.modules.insert(module_name);
                    }
                }
            }
            _ => {}
        }

        StartTagOutcome::Continue
    }

    fn handle_text(&mut self, text: &str, writer: &mut MarkdownWriter) -> HandlerOutcome {
        if writer.is_inside("nav") && writer.is_inside("h2") && text == "Modules" {
            self.has_seen_modules_header = true;
        }

        HandlerOutcome::NoOp
    }
}

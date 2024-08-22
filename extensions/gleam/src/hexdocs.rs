use std::cell::RefCell;
use std::collections::BTreeSet;
use std::io::{self, Read};
use std::rc::Rc;

use html_to_markdown::markdown::{
    HeadingHandler, ListHandler, ParagraphHandler, StyledTextHandler, TableHandler,
};
use html_to_markdown::{
    convert_html_to_markdown, HandleTag, HandlerOutcome, HtmlElement, MarkdownWriter,
    StartTagOutcome, TagHandler,
};
use zed_extension_api::{
    http_client::{HttpMethod, HttpRequest, RedirectPolicy},
    KeyValueStore, Result,
};

pub fn index(package: String, database: &KeyValueStore) -> Result<()> {
    let headers = vec![(
        "User-Agent".to_string(),
        "Zed (Gleam Extension)".to_string(),
    )];

    let response = HttpRequest::builder()
        .method(HttpMethod::Get)
        .url(format!("https://hexdocs.pm/{package}"))
        .headers(headers.clone())
        .redirect_policy(RedirectPolicy::FollowAll)
        .build()?
        .fetch()?;

    let (package_root_markdown, modules) =
        convert_hexdocs_to_markdown(&mut io::Cursor::new(&response.body))?;

    database.insert(&package, &package_root_markdown)?;

    for module in modules {
        let response = HttpRequest::builder()
            .method(HttpMethod::Get)
            .url(format!("https://hexdocs.pm/{package}/{module}.html"))
            .headers(headers.clone())
            .redirect_policy(RedirectPolicy::FollowAll)
            .build()?
            .fetch()?;

        let (markdown, _modules) =
            convert_hexdocs_to_markdown(&mut io::Cursor::new(&response.body))?;

        database.insert(&format!("{module} ({package})"), &markdown)?;
    }

    Ok(())
}

pub fn convert_hexdocs_to_markdown(html: impl Read) -> Result<(String, Vec<String>)> {
    let module_collector = Rc::new(RefCell::new(GleamModuleCollector::new()));

    let mut handlers: Vec<TagHandler> = vec![
        module_collector.clone(),
        Rc::new(RefCell::new(GleamChromeRemover)),
        Rc::new(RefCell::new(NavSkipper::new(ParagraphHandler))),
        Rc::new(RefCell::new(NavSkipper::new(HeadingHandler))),
        Rc::new(RefCell::new(NavSkipper::new(ListHandler))),
        Rc::new(RefCell::new(NavSkipper::new(TableHandler::new()))),
        Rc::new(RefCell::new(NavSkipper::new(StyledTextHandler))),
    ];

    let markdown = convert_html_to_markdown(html, &mut handlers)
        .map_err(|err| format!("failed to convert docs to Markdown {err}"))?;

    let modules = module_collector
        .borrow()
        .modules
        .iter()
        .cloned()
        .collect::<Vec<_>>();

    Ok((markdown, modules))
}

/// A higher-order handler that skips all content from the `nav`.
///
/// We still need to traverse the `nav` for collecting information, but
/// we don't want to include any of its content in the resulting Markdown.
pub struct NavSkipper<T: HandleTag> {
    handler: T,
}

impl<T: HandleTag> NavSkipper<T> {
    pub fn new(handler: T) -> Self {
        Self { handler }
    }
}

impl<T: HandleTag> HandleTag for NavSkipper<T> {
    fn should_handle(&self, tag: &str) -> bool {
        tag == "nav" || self.handler.should_handle(tag)
    }

    fn handle_tag_start(
        &mut self,
        tag: &HtmlElement,
        writer: &mut MarkdownWriter,
    ) -> StartTagOutcome {
        if writer.is_inside("nav") {
            return StartTagOutcome::Continue;
        }

        self.handler.handle_tag_start(tag, writer)
    }

    fn handle_tag_end(&mut self, tag: &HtmlElement, writer: &mut MarkdownWriter) {
        if writer.is_inside("nav") {
            return;
        }

        self.handler.handle_tag_end(tag, writer)
    }

    fn handle_text(&mut self, text: &str, writer: &mut MarkdownWriter) -> HandlerOutcome {
        if writer.is_inside("nav") {
            return HandlerOutcome::Handled;
        }

        self.handler.handle_text(text, writer)
    }
}

pub struct GleamChromeRemover;

impl HandleTag for GleamChromeRemover {
    fn should_handle(&self, tag: &str) -> bool {
        match tag {
            "head" | "script" | "style" | "svg" | "header" | "footer" | "a" => true,
            _ => false,
        }
    }

    fn handle_tag_start(
        &mut self,
        tag: &HtmlElement,
        _writer: &mut MarkdownWriter,
    ) -> StartTagOutcome {
        match tag.tag() {
            "head" | "script" | "style" | "svg" | "header" | "footer" => {
                return StartTagOutcome::Skip;
            }
            "a" => {
                if tag.attr("onclick").is_some() {
                    return StartTagOutcome::Skip;
                }
            }
            _ => {}
        }

        StartTagOutcome::Continue
    }
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

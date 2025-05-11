use std::cell::RefCell;
use std::io::Read;
use std::rc::Rc;

use anyhow::Result;
use html_to_markdown::markdown::{
    HeadingHandler, ListHandler, ParagraphHandler, StyledTextHandler, TableHandler,
};
use html_to_markdown::{
    HandleTag, HandlerOutcome, HtmlElement, MarkdownWriter, StartTagOutcome, TagHandler,
    convert_html_to_markdown,
};
use indexmap::IndexSet;
use strum::IntoEnumIterator;

use crate::{RustdocItem, RustdocItemKind};

/// Converts the provided rustdoc HTML to Markdown.
pub fn convert_rustdoc_to_markdown(html: impl Read) -> Result<(String, Vec<RustdocItem>)> {
    let item_collector = Rc::new(RefCell::new(RustdocItemCollector::new()));

    let mut handlers: Vec<TagHandler> = vec![
        Rc::new(RefCell::new(ParagraphHandler)),
        Rc::new(RefCell::new(HeadingHandler)),
        Rc::new(RefCell::new(ListHandler)),
        Rc::new(RefCell::new(TableHandler::new())),
        Rc::new(RefCell::new(StyledTextHandler)),
        Rc::new(RefCell::new(RustdocChromeRemover)),
        Rc::new(RefCell::new(RustdocHeadingHandler)),
        Rc::new(RefCell::new(RustdocCodeHandler)),
        Rc::new(RefCell::new(RustdocItemHandler)),
        item_collector.clone(),
    ];

    let markdown = convert_html_to_markdown(html, &mut handlers)?;

    let items = item_collector
        .borrow()
        .items
        .iter()
        .cloned()
        .collect::<Vec<_>>();

    Ok((markdown, items))
}

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
                .trim_matches(|char| char == '\n' || char == '\r')
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
        matches!(tag, "pre" | "code")
    }

    fn handle_tag_start(
        &mut self,
        tag: &HtmlElement,
        writer: &mut MarkdownWriter,
    ) -> StartTagOutcome {
        match tag.tag() {
            "code" => {
                if !writer.is_inside("pre") {
                    writer.push_str("`");
                }
            }
            "pre" => {
                let classes = tag.classes();
                let is_rust = classes.iter().any(|class| class == "rust");
                let language = is_rust
                    .then_some("rs")
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
        match tag.tag() {
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
            writer.push_str(text);
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
        matches!(tag, "div" | "span")
    }

    fn handle_tag_start(
        &mut self,
        tag: &HtmlElement,
        writer: &mut MarkdownWriter,
    ) -> StartTagOutcome {
        match tag.tag() {
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
        match tag.tag() {
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
        matches!(
            tag,
            "head" | "script" | "nav" | "summary" | "button" | "a" | "div" | "span"
        )
    }

    fn handle_tag_start(
        &mut self,
        tag: &HtmlElement,
        _writer: &mut MarkdownWriter,
    ) -> StartTagOutcome {
        match tag.tag() {
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
            "a" => {
                if tag.has_any_classes(&["anchor", "doc-anchor", "src"]) {
                    return StartTagOutcome::Skip;
                }
            }
            "div" | "span" => {
                if tag.has_any_classes(&["nav-container", "sidebar-elems", "out-of-band"]) {
                    return StartTagOutcome::Skip;
                }
            }

            _ => {}
        }

        StartTagOutcome::Continue
    }
}

pub struct RustdocItemCollector {
    pub items: IndexSet<RustdocItem>,
}

impl RustdocItemCollector {
    pub fn new() -> Self {
        Self {
            items: IndexSet::new(),
        }
    }

    fn parse_item(tag: &HtmlElement) -> Option<RustdocItem> {
        if tag.tag() != "a" {
            return None;
        }

        let href = tag.attr("href")?;
        if href.starts_with('#') || href.starts_with("https://") || href.starts_with("../") {
            return None;
        }

        for kind in RustdocItemKind::iter() {
            if tag.has_class(kind.class()) {
                let mut parts = href.trim_end_matches("/index.html").split('/');

                if let Some(last_component) = parts.next_back() {
                    let last_component = match last_component.split_once('#') {
                        Some((component, _fragment)) => component,
                        None => last_component,
                    };

                    let name = last_component
                        .trim_start_matches(&format!("{}.", kind.class()))
                        .trim_end_matches(".html");

                    return Some(RustdocItem {
                        kind,
                        name: name.into(),
                        path: parts.map(Into::into).collect(),
                    });
                }
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
        if tag.tag() == "a" {
            let is_reexport = writer.current_element_stack().iter().any(|element| {
                if let Some(id) = element.attr("id") {
                    id.starts_with("reexport.") || id.starts_with("method.")
                } else {
                    false
                }
            });

            if !is_reexport {
                if let Some(item) = Self::parse_item(tag) {
                    self.items.insert(item);
                }
            }
        }

        StartTagOutcome::Continue
    }
}

#[cfg(test)]
mod tests {
    use html_to_markdown::{TagHandler, convert_html_to_markdown};
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use super::*;

    fn rustdoc_handlers() -> Vec<TagHandler> {
        vec![
            Rc::new(RefCell::new(ParagraphHandler)),
            Rc::new(RefCell::new(HeadingHandler)),
            Rc::new(RefCell::new(ListHandler)),
            Rc::new(RefCell::new(TableHandler::new())),
            Rc::new(RefCell::new(StyledTextHandler)),
            Rc::new(RefCell::new(RustdocChromeRemover)),
            Rc::new(RefCell::new(RustdocHeadingHandler)),
            Rc::new(RefCell::new(RustdocCodeHandler)),
            Rc::new(RefCell::new(RustdocItemHandler)),
        ]
    }

    #[test]
    fn test_main_heading_buttons_get_removed() {
        let html = indoc! {r##"
            <div class="main-heading">
                <h1>Crate <a class="mod" href="#">serde</a><button id="copy-path" title="Copy item path to clipboard">Copy item path</button></h1>
                <span class="out-of-band">
                    <a class="src" href="../src/serde/lib.rs.html#1-340">source</a> · <button id="toggle-all-docs" title="collapse all docs">[<span>−</span>]</button>
                </span>
            </div>
        "##};
        let expected = indoc! {"
            # Crate serde
        "}
        .trim();

        assert_eq!(
            convert_html_to_markdown(html.as_bytes(), &mut rustdoc_handlers()).unwrap(),
            expected
        )
    }

    #[test]
    fn test_single_paragraph() {
        let html = indoc! {r#"
            <p>In particular, the last point is what sets <code>axum</code> apart from other frameworks.
            <code>axum</code> doesn’t have its own middleware system but instead uses
            <a href="https://docs.rs/tower-service/0.3.2/x86_64-unknown-linux-gnu/tower_service/trait.Service.html" title="trait tower_service::Service"><code>tower::Service</code></a>. This means <code>axum</code> gets timeouts, tracing, compression,
            authorization, and more, for free. It also enables you to share middleware with
            applications written using <a href="http://crates.io/crates/hyper"><code>hyper</code></a> or <a href="http://crates.io/crates/tonic"><code>tonic</code></a>.</p>
        "#};
        let expected = indoc! {"
            In particular, the last point is what sets `axum` apart from other frameworks. `axum` doesn’t have its own middleware system but instead uses `tower::Service`. This means `axum` gets timeouts, tracing, compression, authorization, and more, for free. It also enables you to share middleware with applications written using `hyper` or `tonic`.
        "}
        .trim();

        assert_eq!(
            convert_html_to_markdown(html.as_bytes(), &mut rustdoc_handlers()).unwrap(),
            expected
        )
    }

    #[test]
    fn test_multiple_paragraphs() {
        let html = indoc! {r##"
            <h2 id="serde"><a class="doc-anchor" href="#serde">§</a>Serde</h2>
            <p>Serde is a framework for <em><strong>ser</strong></em>ializing and <em><strong>de</strong></em>serializing Rust data
            structures efficiently and generically.</p>
            <p>The Serde ecosystem consists of data structures that know how to serialize
            and deserialize themselves along with data formats that know how to
            serialize and deserialize other things. Serde provides the layer by which
            these two groups interact with each other, allowing any supported data
            structure to be serialized and deserialized using any supported data format.</p>
            <p>See the Serde website <a href="https://serde.rs/">https://serde.rs/</a> for additional documentation and
            usage examples.</p>
            <h3 id="design"><a class="doc-anchor" href="#design">§</a>Design</h3>
            <p>Where many other languages rely on runtime reflection for serializing data,
            Serde is instead built on Rust’s powerful trait system. A data structure
            that knows how to serialize and deserialize itself is one that implements
            Serde’s <code>Serialize</code> and <code>Deserialize</code> traits (or uses Serde’s derive
            attribute to automatically generate implementations at compile time). This
            avoids any overhead of reflection or runtime type information. In fact in
            many situations the interaction between data structure and data format can
            be completely optimized away by the Rust compiler, leaving Serde
            serialization to perform the same speed as a handwritten serializer for the
            specific selection of data structure and data format.</p>
        "##};
        let expected = indoc! {"
            ## Serde

            Serde is a framework for _**ser**_ializing and _**de**_serializing Rust data structures efficiently and generically.

            The Serde ecosystem consists of data structures that know how to serialize and deserialize themselves along with data formats that know how to serialize and deserialize other things. Serde provides the layer by which these two groups interact with each other, allowing any supported data structure to be serialized and deserialized using any supported data format.

            See the Serde website https://serde.rs/ for additional documentation and usage examples.

            ### Design

            Where many other languages rely on runtime reflection for serializing data, Serde is instead built on Rust’s powerful trait system. A data structure that knows how to serialize and deserialize itself is one that implements Serde’s `Serialize` and `Deserialize` traits (or uses Serde’s derive attribute to automatically generate implementations at compile time). This avoids any overhead of reflection or runtime type information. In fact in many situations the interaction between data structure and data format can be completely optimized away by the Rust compiler, leaving Serde serialization to perform the same speed as a handwritten serializer for the specific selection of data structure and data format.
        "}
        .trim();

        assert_eq!(
            convert_html_to_markdown(html.as_bytes(), &mut rustdoc_handlers()).unwrap(),
            expected
        )
    }

    #[test]
    fn test_styled_text() {
        let html = indoc! {r#"
            <p>This text is <strong>bolded</strong>.</p>
            <p>This text is <em>italicized</em>.</p>
        "#};
        let expected = indoc! {"
            This text is **bolded**.

            This text is _italicized_.
        "}
        .trim();

        assert_eq!(
            convert_html_to_markdown(html.as_bytes(), &mut rustdoc_handlers()).unwrap(),
            expected
        )
    }

    #[test]
    fn test_rust_code_block() {
        let html = indoc! {r#"
            <pre class="rust rust-example-rendered"><code><span class="kw">use </span>axum::extract::{Path, Query, Json};
            <span class="kw">use </span>std::collections::HashMap;

            <span class="comment">// `Path` gives you the path parameters and deserializes them.
            </span><span class="kw">async fn </span>path(Path(user_id): Path&lt;u32&gt;) {}

            <span class="comment">// `Query` gives you the query parameters and deserializes them.
            </span><span class="kw">async fn </span>query(Query(params): Query&lt;HashMap&lt;String, String&gt;&gt;) {}

            <span class="comment">// Buffer the request body and deserialize it as JSON into a
            // `serde_json::Value`. `Json` supports any type that implements
            // `serde::Deserialize`.
            </span><span class="kw">async fn </span>json(Json(payload): Json&lt;serde_json::Value&gt;) {}</code></pre>
        "#};
        let expected = indoc! {"
            ```rs
            use axum::extract::{Path, Query, Json};
            use std::collections::HashMap;

            // `Path` gives you the path parameters and deserializes them.
            async fn path(Path(user_id): Path<u32>) {}

            // `Query` gives you the query parameters and deserializes them.
            async fn query(Query(params): Query<HashMap<String, String>>) {}

            // Buffer the request body and deserialize it as JSON into a
            // `serde_json::Value`. `Json` supports any type that implements
            // `serde::Deserialize`.
            async fn json(Json(payload): Json<serde_json::Value>) {}
            ```
        "}
        .trim();

        assert_eq!(
            convert_html_to_markdown(html.as_bytes(), &mut rustdoc_handlers()).unwrap(),
            expected
        )
    }

    #[test]
    fn test_toml_code_block() {
        let html = indoc! {r##"
            <h2 id="required-dependencies"><a class="doc-anchor" href="#required-dependencies">§</a>Required dependencies</h2>
            <p>To use axum there are a few dependencies you have to pull in as well:</p>
            <div class="example-wrap"><pre class="language-toml"><code>[dependencies]
            axum = &quot;&lt;latest-version&gt;&quot;
            tokio = { version = &quot;&lt;latest-version&gt;&quot;, features = [&quot;full&quot;] }
            tower = &quot;&lt;latest-version&gt;&quot;
            </code></pre></div>
        "##};
        let expected = indoc! {r#"
            ## Required dependencies

            To use axum there are a few dependencies you have to pull in as well:

            ```toml
            [dependencies]
            axum = "<latest-version>"
            tokio = { version = "<latest-version>", features = ["full"] }
            tower = "<latest-version>"

            ```
        "#}
        .trim();

        assert_eq!(
            convert_html_to_markdown(html.as_bytes(), &mut rustdoc_handlers()).unwrap(),
            expected
        )
    }

    #[test]
    fn test_item_table() {
        let html = indoc! {r##"
            <h2 id="structs" class="section-header">Structs<a href="#structs" class="anchor">§</a></h2>
            <ul class="item-table">
            <li><div class="item-name"><a class="struct" href="struct.Error.html" title="struct axum::Error">Error</a></div><div class="desc docblock-short">Errors that can happen when using axum.</div></li>
            <li><div class="item-name"><a class="struct" href="struct.Extension.html" title="struct axum::Extension">Extension</a></div><div class="desc docblock-short">Extractor and response for extensions.</div></li>
            <li><div class="item-name"><a class="struct" href="struct.Form.html" title="struct axum::Form">Form</a><span class="stab portability" title="Available on crate feature `form` only"><code>form</code></span></div><div class="desc docblock-short">URL encoded extractor and response.</div></li>
            <li><div class="item-name"><a class="struct" href="struct.Json.html" title="struct axum::Json">Json</a><span class="stab portability" title="Available on crate feature `json` only"><code>json</code></span></div><div class="desc docblock-short">JSON Extractor / Response.</div></li>
            <li><div class="item-name"><a class="struct" href="struct.Router.html" title="struct axum::Router">Router</a></div><div class="desc docblock-short">The router type for composing handlers and services.</div></li></ul>
            <h2 id="functions" class="section-header">Functions<a href="#functions" class="anchor">§</a></h2>
            <ul class="item-table">
            <li><div class="item-name"><a class="fn" href="fn.serve.html" title="fn axum::serve">serve</a><span class="stab portability" title="Available on crate feature `tokio` and (crate features `http1` or `http2`) only"><code>tokio</code> and (<code>http1</code> or <code>http2</code>)</span></div><div class="desc docblock-short">Serve the service with the supplied listener.</div></li>
            </ul>
        "##};
        let expected = indoc! {r#"
            ## Structs

            - `Error`: Errors that can happen when using axum.
            - `Extension`: Extractor and response for extensions.
            - `Form` [`form`]: URL encoded extractor and response.
            - `Json` [`json`]: JSON Extractor / Response.
            - `Router`: The router type for composing handlers and services.

            ## Functions

            - `serve` [`tokio` and (`http1` or `http2`)]: Serve the service with the supplied listener.
        "#}
        .trim();

        assert_eq!(
            convert_html_to_markdown(html.as_bytes(), &mut rustdoc_handlers()).unwrap(),
            expected
        )
    }

    #[test]
    fn test_table() {
        let html = indoc! {r##"
            <h2 id="feature-flags"><a class="doc-anchor" href="#feature-flags">§</a>Feature flags</h2>
            <p>axum uses a set of <a href="https://doc.rust-lang.org/cargo/reference/features.html#the-features-section">feature flags</a> to reduce the amount of compiled and
            optional dependencies.</p>
            <p>The following optional features are available:</p>
            <div><table><thead><tr><th>Name</th><th>Description</th><th>Default?</th></tr></thead><tbody>
            <tr><td><code>http1</code></td><td>Enables hyper’s <code>http1</code> feature</td><td>Yes</td></tr>
            <tr><td><code>http2</code></td><td>Enables hyper’s <code>http2</code> feature</td><td>No</td></tr>
            <tr><td><code>json</code></td><td>Enables the <a href="struct.Json.html" title="struct axum::Json"><code>Json</code></a> type and some similar convenience functionality</td><td>Yes</td></tr>
            <tr><td><code>macros</code></td><td>Enables optional utility macros</td><td>No</td></tr>
            <tr><td><code>matched-path</code></td><td>Enables capturing of every request’s router path and the <a href="extract/struct.MatchedPath.html" title="struct axum::extract::MatchedPath"><code>MatchedPath</code></a> extractor</td><td>Yes</td></tr>
            <tr><td><code>multipart</code></td><td>Enables parsing <code>multipart/form-data</code> requests with <a href="extract/struct.Multipart.html" title="struct axum::extract::Multipart"><code>Multipart</code></a></td><td>No</td></tr>
            <tr><td><code>original-uri</code></td><td>Enables capturing of every request’s original URI and the <a href="extract/struct.OriginalUri.html" title="struct axum::extract::OriginalUri"><code>OriginalUri</code></a> extractor</td><td>Yes</td></tr>
            <tr><td><code>tokio</code></td><td>Enables <code>tokio</code> as a dependency and <code>axum::serve</code>, <code>SSE</code> and <code>extract::connect_info</code> types.</td><td>Yes</td></tr>
            <tr><td><code>tower-log</code></td><td>Enables <code>tower</code>’s <code>log</code> feature</td><td>Yes</td></tr>
            <tr><td><code>tracing</code></td><td>Log rejections from built-in extractors</td><td>Yes</td></tr>
            <tr><td><code>ws</code></td><td>Enables WebSockets support via <a href="extract/ws/index.html" title="mod axum::extract::ws"><code>extract::ws</code></a></td><td>No</td></tr>
            <tr><td><code>form</code></td><td>Enables the <code>Form</code> extractor</td><td>Yes</td></tr>
            <tr><td><code>query</code></td><td>Enables the <code>Query</code> extractor</td><td>Yes</td></tr>
            </tbody></table>
        "##};
        let expected = indoc! {r#"
            ## Feature flags

            axum uses a set of feature flags to reduce the amount of compiled and optional dependencies.

            The following optional features are available:

            | Name | Description | Default? |
            | --- | --- | --- |
            | `http1` | Enables hyper’s `http1` feature | Yes |
            | `http2` | Enables hyper’s `http2` feature | No |
            | `json` | Enables the `Json` type and some similar convenience functionality | Yes |
            | `macros` | Enables optional utility macros | No |
            | `matched-path` | Enables capturing of every request’s router path and the `MatchedPath` extractor | Yes |
            | `multipart` | Enables parsing `multipart/form-data` requests with `Multipart` | No |
            | `original-uri` | Enables capturing of every request’s original URI and the `OriginalUri` extractor | Yes |
            | `tokio` | Enables `tokio` as a dependency and `axum::serve`, `SSE` and `extract::connect_info` types. | Yes |
            | `tower-log` | Enables `tower`’s `log` feature | Yes |
            | `tracing` | Log rejections from built-in extractors | Yes |
            | `ws` | Enables WebSockets support via `extract::ws` | No |
            | `form` | Enables the `Form` extractor | Yes |
            | `query` | Enables the `Query` extractor | Yes |
        "#}
        .trim();

        assert_eq!(
            convert_html_to_markdown(html.as_bytes(), &mut rustdoc_handlers()).unwrap(),
            expected
        )
    }
}

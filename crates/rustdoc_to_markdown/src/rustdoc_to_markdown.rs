//! Provides conversion from rustdoc's HTML output to Markdown.

#![deny(missing_docs)]

mod markdown_writer;

use std::io::Read;

use anyhow::{Context, Result};
use html5ever::driver::ParseOpts;
use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::TreeBuilderOpts;
use markup5ever_rcdom::RcDom;

use crate::markdown_writer::MarkdownWriter;

/// Converts the provided rustdoc HTML to Markdown.
pub fn convert_rustdoc_to_markdown(mut html: impl Read) -> Result<String> {
    let parse_options = ParseOpts {
        tree_builder: TreeBuilderOpts {
            drop_doctype: true,
            ..Default::default()
        },
        ..Default::default()
    };
    let dom = parse_document(RcDom::default(), parse_options)
        .from_utf8()
        .read_from(&mut html)
        .context("failed to parse rustdoc HTML")?;

    let markdown_writer = MarkdownWriter::new();
    let markdown = markdown_writer
        .run(&dom.document)
        .context("failed to convert rustdoc to HTML")?;

    Ok(markdown)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use super::*;

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
            convert_rustdoc_to_markdown(html.as_bytes()).unwrap(),
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
            convert_rustdoc_to_markdown(html.as_bytes()).unwrap(),
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
            convert_rustdoc_to_markdown(html.as_bytes()).unwrap(),
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
            convert_rustdoc_to_markdown(html.as_bytes()).unwrap(),
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

            axum uses a set of feature flags to reduce the amount of compiled and
            optional dependencies.The following optional features are available:

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
            convert_rustdoc_to_markdown(html.as_bytes()).unwrap(),
            expected
        )
    }
}

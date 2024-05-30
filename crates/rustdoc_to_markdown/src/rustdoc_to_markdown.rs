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
}

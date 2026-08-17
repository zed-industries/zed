use pulldown_cmark::{html, CowStr, Event, LinkType, Options, Parser, Tag};
use regex::RegexBuilder;
use std::io::Write;

/// This example demonstrates how to normalize the href of a wikilink. The
/// details of this implementation can be tweaked for different use cases.
fn main() {
    let markdown_input: &str = r#"
Example provided by [[https://example.org/]].
Some people might prefer the wikilink syntax for autolinks.

Wanna go for a [[Wiki Walk]]?"#;

    let parser = Parser::new_ext(markdown_input, Options::ENABLE_WIKILINKS).map(|event| {
        if let Event::Start(Tag::Link {
            link_type: LinkType::WikiLink { has_pothole },
            dest_url,
            title,
            id,
        }) = event
        {
            let new_link = normalize_wikilink(dest_url);
            Event::Start(Tag::Link {
                link_type: LinkType::WikiLink { has_pothole },
                dest_url: new_link,
                title,
                id,
            })
        } else {
            event
        }
    });

    // Write to anything implementing the `Write` trait. This could also be a file
    // or network socket.
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(b"\nHTML output:\n").unwrap();
    html::write_html_io(&mut handle, parser).unwrap();
}

/// Performs wikilink normalization.
fn normalize_wikilink(link: CowStr) -> CowStr {
    // your wiki is stored at "/wiki"
    let prefix: &str = "/wiki";
    if link.is_empty() {
        return link;
    }

    // check if the link is absolute, if it is, return as is
    // according to RFC 3986; https://www.rfc-editor.org/rfc/rfc3986
    let is_absolute = RegexBuilder::new("^(?:[a-z][a-z0-9+\\-.]*:)?//")
        .case_insensitive(true)
        .build()
        .expect("valid regex");

    if is_absolute.is_match(&link) {
        return link;
    }

    let mut result = String::with_capacity(link.len() + 2);
    let mut i = 0;
    let mut mark = 0;
    let mut in_whitespace = false;

    result.push_str(prefix);

    if !link.starts_with('/') {
        result.push('/');
    }

    while i < link.len() {
        if !in_whitespace && link.as_bytes()[i].is_ascii_whitespace() {
            in_whitespace = true;
            result.push_str(&link[mark..i]);
        } else if in_whitespace && !link.as_bytes()[i].is_ascii_whitespace() {
            result.push('_');
            mark = i;
            in_whitespace = false;
        }

        i += 1;
    }

    if !in_whitespace {
        result.push_str(&link[mark..]);
    }
    if !result.ends_with('/') {
        result.push('/');
    }
    result.into()
}

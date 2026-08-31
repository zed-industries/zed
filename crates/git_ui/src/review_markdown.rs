use gpui::{App, AppContext as _, Entity, Window};
use language::LanguageRegistry;
use markdown::{Markdown, MarkdownElement, MarkdownFont, MarkdownStyle};
use pulldown_cmark::{CodeBlockKind, Event, Parser, Tag};
use std::sync::Arc;

pub(crate) fn source(body: &str) -> String {
    let mut replacements = Vec::new();
    for (event, range) in Parser::new(body).into_offset_iter() {
        match event {
            Event::Start(Tag::Image { .. }) if body[range.clone()].starts_with('!') => {
                replacements.push((range.start..range.start + 1, String::new()));
            }
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(info)))
                if info.as_ref() == "suggestion" || info.starts_with("suggestion:") =>
            {
                let line = body[range.clone()].lines().next().unwrap_or_default();
                if let Some(offset) = line.find("suggestion") {
                    replacements.push((
                        range.start + offset..range.start + offset + info.len(),
                        String::new(),
                    ));
                    replacements.push((
                        range.start..range.start,
                        "**Suggested code (display only)**\n\n".into(),
                    ));
                }
            }
            _ => {}
        }
    }
    replacements.sort_by_key(|(range, _)| std::cmp::Reverse(range.start));
    let mut result = body.to_owned();
    for (range, replacement) in replacements {
        result.replace_range(range, &replacement);
    }
    let unsafe_links: Vec<_> = Parser::new(&result)
        .into_offset_iter()
        .filter_map(|(event, range)| match event {
            Event::Start(Tag::Link { dest_url, .. }) if !safe_url(&dest_url) => Some(range.start),
            _ => None,
        })
        .collect();
    for offset in unsafe_links.into_iter().rev() {
        result.insert(offset, '\\');
    }
    result
}

fn safe_url(value: &str) -> bool {
    url::Url::parse(value).is_ok_and(|url| matches!(url.scheme(), "https" | "http"))
}

pub(crate) fn new(body: &str, languages: Arc<LanguageRegistry>, cx: &mut App) -> Entity<Markdown> {
    cx.new(|cx| Markdown::new(source(body).into(), Some(languages), None, cx))
}

pub(crate) fn render(markdown: Entity<Markdown>, window: &Window, cx: &App) -> MarkdownElement {
    MarkdownElement::new(
        markdown,
        MarkdownStyle::themed(MarkdownFont::Preview, window, cx),
    )
    .on_url_click(|url, _, cx| {
        if safe_url(&url) {
            cx.open_url(&url);
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn images_become_links_but_code_stays_literal() {
        assert_eq!(
            source("![diagram](https://example.com/x.png)\n\n`![code](x)`"),
            "[diagram](https://example.com/x.png)\n\n`![code](x)`"
        );
        assert_eq!(
            source("![diagram][image]\n\n[image]: https://example.com/x.png"),
            "[diagram][image]\n\n[image]: https://example.com/x.png"
        );
    }

    #[test]
    fn suggestions_are_labelled_without_changing_their_code() {
        assert_eq!(
            source("```suggestion\nlet x = 1;\n```"),
            "**Suggested code (display only)**\n\n```\nlet x = 1;\n```"
        );
    }

    #[test]
    fn unsafe_links_and_image_links_cannot_open_local_files_or_commands() {
        let rendered =
            source("[run](command:run) ![local](file:///private/file) [site](https://example.com)");
        let urls: Vec<_> = Parser::new(&rendered)
            .filter_map(|event| match event {
                Event::Start(Tag::Link { dest_url, .. }) => Some(dest_url.to_string()),
                _ => None,
            })
            .collect();
        assert_eq!(urls, ["https://example.com"]);
    }
}

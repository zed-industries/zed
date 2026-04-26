use acp_thread::MentionUri;
use agent_client_protocol::schema as acp;
use util::paths::PathStyle;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum StickyUserMessageSegment {
    Text(String),
    Mention { uri: MentionUri, label: String },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct StickyUserMessagePreview {
    pub(crate) segments: Vec<StickyUserMessageSegment>,
    pub(crate) text: String,
    pub(crate) has_more_message_content: bool,
}

pub(crate) fn parse_sticky_user_message_preview(
    chunks: &[acp::ContentBlock],
    path_style: PathStyle,
) -> StickyUserMessagePreview {
    let (segments, has_more_message_content) = segmented_preview_line(chunks, path_style);
    let text = segments.iter().map(segment_text).collect();

    StickyUserMessagePreview {
        segments,
        text,
        has_more_message_content,
    }
}

fn reference_segment(uri: &str, path_style: PathStyle) -> StickyUserMessageSegment {
    match MentionUri::parse(uri, path_style) {
        Ok(uri) => StickyUserMessageSegment::Mention {
            label: format!("@{}", uri.name()),
            uri,
        },
        Err(_) => StickyUserMessageSegment::Text(uri.to_string()),
    }
}

fn resource_link_segment(
    resource_link: &acp::ResourceLink,
    path_style: PathStyle,
) -> StickyUserMessageSegment {
    match MentionUri::parse(&resource_link.uri, path_style) {
        Ok(uri) => StickyUserMessageSegment::Mention {
            label: format!("@{}", uri.name()),
            uri,
        },
        Err(_) => StickyUserMessageSegment::Text(format!("@{}", resource_link.name)),
    }
}

fn segment_text(segment: &StickyUserMessageSegment) -> &str {
    match segment {
        StickyUserMessageSegment::Text(text) => text,
        StickyUserMessageSegment::Mention { label, .. } => label,
    }
}

fn trim_segments(mut segments: Vec<StickyUserMessageSegment>) -> Vec<StickyUserMessageSegment> {
    while matches!(segments.first(), Some(StickyUserMessageSegment::Text(text)) if text.trim_start().is_empty())
    {
        segments.remove(0);
    }

    while matches!(segments.last(), Some(StickyUserMessageSegment::Text(text)) if text.trim_end().is_empty())
    {
        segments.pop();
    }

    if let Some(StickyUserMessageSegment::Text(text)) = segments.first_mut() {
        *text = text.trim_start().to_string();
    }

    if let Some(StickyUserMessageSegment::Text(text)) = segments.last_mut() {
        *text = text.trim_end().to_string();
    }

    segments
        .into_iter()
        .filter(
            |segment| !matches!(segment, StickyUserMessageSegment::Text(text) if text.is_empty()),
        )
        .collect()
}

fn segmented_preview_line(
    chunks: &[acp::ContentBlock],
    path_style: PathStyle,
) -> (Vec<StickyUserMessageSegment>, bool) {
    fn finish_line(
        current_line_segments: &mut Vec<StickyUserMessageSegment>,
        first_non_empty_line_segments: &mut Option<Vec<StickyUserMessageSegment>>,
    ) -> bool {
        let trimmed_segments = trim_segments(std::mem::take(current_line_segments));
        if trimmed_segments.is_empty() {
            return false;
        }

        if first_non_empty_line_segments.is_none() {
            *first_non_empty_line_segments = Some(trimmed_segments);
            false
        } else {
            true
        }
    }

    let mut first_non_empty_line_segments = None;
    let mut current_line_segments = Vec::new();
    let mut has_more_message_content = false;

    for chunk in chunks {
        match chunk {
            acp::ContentBlock::Text(text_content) => {
                let mut lines = text_content.text.split('\n').peekable();
                while let Some(line) = lines.next() {
                    if !line.is_empty() {
                        current_line_segments
                            .push(StickyUserMessageSegment::Text(line.to_string()));
                    }

                    if lines.peek().is_some()
                        && finish_line(
                            &mut current_line_segments,
                            &mut first_non_empty_line_segments,
                        )
                    {
                        has_more_message_content = true;
                        break;
                    }
                }
            }
            acp::ContentBlock::ResourceLink(resource_link) => {
                current_line_segments.push(resource_link_segment(resource_link, path_style));
            }
            acp::ContentBlock::Resource(acp::EmbeddedResource {
                resource: acp::EmbeddedResourceResource::TextResourceContents(resource),
                ..
            }) => current_line_segments.push(reference_segment(&resource.uri, path_style)),
            acp::ContentBlock::Image(acp::ImageContent { uri, .. }) => {
                current_line_segments.push(
                    uri.as_deref()
                        .map(|uri| reference_segment(uri, path_style))
                        .unwrap_or_else(|| StickyUserMessageSegment::Mention {
                            uri: MentionUri::PastedImage {
                                name: "Image".to_string(),
                            },
                            label: "@Image".to_string(),
                        }),
                );
            }
            _ => {}
        }

        if has_more_message_content {
            break;
        }
    }

    if !has_more_message_content
        && finish_line(
            &mut current_line_segments,
            &mut first_non_empty_line_segments,
        )
    {
        has_more_message_content = true;
    }

    (
        first_non_empty_line_segments
            .unwrap_or_else(|| vec![StickyUserMessageSegment::Text("Message".to_string())]),
        has_more_message_content,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uses_structured_labels_for_references() {
        let chunks = vec![
            acp::ContentBlock::Text(acp::TextContent::new("Check ")),
            acp::ContentBlock::ResourceLink(acp::ResourceLink::new(
                "main.rs",
                "file:///project/main.rs",
            )),
            acp::ContentBlock::Text(acp::TextContent::new(" and ")),
            acp::ContentBlock::Resource(acp::EmbeddedResource::new(
                acp::EmbeddedResourceResource::TextResourceContents(
                    acp::TextResourceContents::new("fn main() {}", "file:///project/lib.rs"),
                ),
            )),
        ];

        let preview = parse_sticky_user_message_preview(&chunks, PathStyle::Posix);

        assert!(matches!(
            preview.segments.as_slice(),
            [
                StickyUserMessageSegment::Text(_),
                StickyUserMessageSegment::Mention { .. },
                StickyUserMessageSegment::Text(_),
                StickyUserMessageSegment::Mention { .. }
            ]
        ));
        assert_eq!(preview.text, "Check @main.rs and @lib.rs");
        assert!(!preview.has_more_message_content);
    }

    #[test]
    fn uses_image_label_instead_of_markdown_placeholder() {
        let chunks = vec![
            acp::ContentBlock::Image(
                acp::ImageContent::new("ignored", "image/png")
                    .uri("zed:///agent/pasted-image?name=Diagram"),
            ),
            acp::ContentBlock::Text(acp::TextContent::new("\nExplain this diagram")),
        ];

        let preview = parse_sticky_user_message_preview(&chunks, PathStyle::Posix);

        assert!(matches!(
            preview.segments.as_slice(),
            [StickyUserMessageSegment::Mention { .. }]
        ));
        assert_eq!(preview.text, "@Diagram");
        assert!(preview.has_more_message_content);
    }

    #[test]
    fn falls_back_to_message_for_empty_preview_content() {
        let chunks = vec![acp::ContentBlock::Text(acp::TextContent::new("\n   \n"))];

        let preview = parse_sticky_user_message_preview(&chunks, PathStyle::Posix);

        assert_eq!(
            preview.segments,
            vec![StickyUserMessageSegment::Text("Message".to_string())]
        );
        assert_eq!(preview.text, "Message");
        assert!(!preview.has_more_message_content);
    }

    #[test]
    fn trims_surrounding_whitespace_on_first_non_empty_line() {
        let chunks = vec![acp::ContentBlock::Text(acp::TextContent::new(
            "\n   hello world   \n",
        ))];

        let preview = parse_sticky_user_message_preview(&chunks, PathStyle::Posix);

        assert_eq!(
            preview.segments,
            vec![StickyUserMessageSegment::Text("hello world".to_string())]
        );
        assert_eq!(preview.text, "hello world");
        assert!(!preview.has_more_message_content);
    }

    #[test]
    fn marks_has_more_content_when_later_non_empty_lines_exist() {
        let chunks = vec![acp::ContentBlock::Text(acp::TextContent::new(
            "\nFirst line\n\nSecond line",
        ))];

        let preview = parse_sticky_user_message_preview(&chunks, PathStyle::Posix);

        assert_eq!(
            preview.segments,
            vec![StickyUserMessageSegment::Text("First line".to_string())]
        );
        assert_eq!(preview.text, "First line");
        assert!(preview.has_more_message_content);
    }

    #[test]
    fn falls_back_to_resource_name_for_invalid_resource_link_uri() {
        let chunks = vec![acp::ContentBlock::ResourceLink(acp::ResourceLink::new(
            "notes.md",
            "not a valid uri",
        ))];

        let preview = parse_sticky_user_message_preview(&chunks, PathStyle::Posix);

        assert_eq!(
            preview.segments,
            vec![StickyUserMessageSegment::Text("@notes.md".to_string())]
        );
        assert_eq!(preview.text, "@notes.md");
        assert!(!preview.has_more_message_content);
    }

    #[test]
    fn falls_back_to_raw_uri_for_invalid_embedded_resource_uri() {
        let chunks = vec![acp::ContentBlock::Resource(acp::EmbeddedResource::new(
            acp::EmbeddedResourceResource::TextResourceContents(acp::TextResourceContents::new(
                "contents",
                "not a valid uri",
            )),
        ))];

        let preview = parse_sticky_user_message_preview(&chunks, PathStyle::Posix);

        assert_eq!(
            preview.segments,
            vec![StickyUserMessageSegment::Text(
                "not a valid uri".to_string()
            )]
        );
        assert_eq!(preview.text, "not a valid uri");
        assert!(!preview.has_more_message_content);
    }
}

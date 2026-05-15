use agent_client_protocol::schema as acp;
use gpui::{AnyElement, App, AvailableSpace, Pixels, Window};
use ui::prelude::*;
use util::paths::PathStyle;

use super::{UserMessageContentSegment, parse_content_block};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct StickyUserMessagePreview {
    pub(crate) segments: Vec<UserMessageContentSegment>,
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

fn segment_text(segment: &UserMessageContentSegment) -> &str {
    match segment {
        UserMessageContentSegment::Text(text) => text,
        UserMessageContentSegment::Mention { label, .. } => label,
    }
}

fn sticky_user_message_display_segments(
    segments: Vec<UserMessageContentSegment>,
) -> Vec<UserMessageContentSegment> {
    let mut merged_segments = Vec::new();

    for segment in segments {
        match segment {
            UserMessageContentSegment::Text(text) => {
                if let Some(UserMessageContentSegment::Text(previous_text)) =
                    merged_segments.last_mut()
                {
                    previous_text.push_str(&text);
                } else {
                    merged_segments.push(UserMessageContentSegment::Text(text));
                }
            }
            UserMessageContentSegment::Mention { .. } => merged_segments.push(segment),
        }
    }

    merged_segments
        .into_iter()
        .filter_map(|segment| match segment {
            UserMessageContentSegment::Text(text) => {
                let text = text.trim();
                (!text.is_empty()).then(|| UserMessageContentSegment::Text(text.to_string()))
            }
            UserMessageContentSegment::Mention { .. } => Some(segment),
        })
        .collect()
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct StickyUserMessageFit {
    pub(crate) visible_segment_count: usize,
    pub(crate) show_ellipsis: bool,
}

fn fit_sticky_user_message_segments(
    segment_widths: &[Pixels],
    gap_width: Pixels,
    ellipsis_width: Pixels,
    available_width: Pixels,
    has_more_message_content: bool,
) -> StickyUserMessageFit {
    let mut visible_segment_count = segment_widths.len();
    let mut show_ellipsis = has_more_message_content;

    if segment_widths.len() > 1 {
        loop {
            show_ellipsis =
                has_more_message_content || visible_segment_count < segment_widths.len();
            if visible_segment_count == 0
                || sticky_user_message_preview_width(
                    segment_widths,
                    visible_segment_count,
                    gap_width,
                    ellipsis_width,
                    show_ellipsis,
                ) <= available_width
            {
                break;
            }
            visible_segment_count -= 1;
        }
    }

    StickyUserMessageFit {
        visible_segment_count,
        show_ellipsis,
    }
}

fn sticky_user_message_preview_width(
    segment_widths: &[Pixels],
    visible_segment_count: usize,
    gap_width: Pixels,
    ellipsis_width: Pixels,
    show_ellipsis: bool,
) -> Pixels {
    let segment_width = segment_widths
        .iter()
        .take(visible_segment_count)
        .fold(Pixels::ZERO, |sum, width| sum + *width);
    let child_count = visible_segment_count + usize::from(show_ellipsis);
    let gap_width = if child_count > 1 {
        gap_width * (child_count - 1) as f32
    } else {
        Pixels::ZERO
    };
    let ellipsis_width = if show_ellipsis {
        ellipsis_width
    } else {
        Pixels::ZERO
    };

    segment_width + gap_width + ellipsis_width
}

pub(crate) fn render_sticky_user_message_preview(
    segments: Vec<UserMessageContentSegment>,
    has_more_message_content: bool,
    available_width: Pixels,
    rem_size: Pixels,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let segments = sticky_user_message_display_segments(segments);
    let render_segment =
        |index: usize, segment: UserMessageContentSegment, truncate: bool, cx: &mut App| {
            match segment {
                UserMessageContentSegment::Text(text) => Label::new(text)
                    .size(LabelSize::Small)
                    .color(Color::Default)
                    .map(|this| {
                        if truncate {
                            this.truncate()
                        } else {
                            this.single_line().flex_none()
                        }
                    })
                    .into_any_element(),
                UserMessageContentSegment::Mention { uri, label } => h_flex()
                    .id(("sticky-user-message-mention", index))
                    .flex_none()
                    .h_5()
                    .px_1p5()
                    .gap_1()
                    .items_center()
                    .rounded_sm()
                    .border_1()
                    .border_color(cx.theme().colors().border_variant)
                    .bg(cx.theme().colors().element_background)
                    .child(
                        Icon::from_path(uri.icon_path(cx))
                            .size(IconSize::XSmall)
                            .color(Color::Muted),
                    )
                    .child(
                        Label::new(label)
                            .size(LabelSize::Small)
                            .color(Color::Default),
                    )
                    .into_any_element(),
            }
        };
    let render_ellipsis = || {
        Label::new("…")
            .size(LabelSize::Small)
            .color(Color::Muted)
            .flex_shrink_0()
            .into_any_element()
    };
    let measure = |element: &mut AnyElement, window: &mut Window, cx: &mut App| {
        window.with_rem_size(Some(rem_size), |window| {
            element
                .layout_as_root(AvailableSpace::min_size(), window, cx)
                .width
        })
    };

    let mut ellipsis = render_ellipsis();
    let ellipsis_width = measure(&mut ellipsis, window, cx);
    let segment_widths = segments
        .iter()
        .cloned()
        .enumerate()
        .map(|(index, segment)| {
            let mut element = render_segment(index, segment, false, cx);
            measure(&mut element, window, cx)
        })
        .collect::<Vec<_>>();
    let fit = fit_sticky_user_message_segments(
        &segment_widths,
        rems(0.25).to_pixels(rem_size),
        ellipsis_width,
        available_width,
        has_more_message_content,
    );

    let include_dropped_text = segments
        .get(fit.visible_segment_count)
        .is_some_and(|s| matches!(s, UserMessageContentSegment::Text(_)));
    let visible_segment_count = fit.visible_segment_count + usize::from(include_dropped_text);
    let truncated_text_index = if include_dropped_text {
        Some(fit.visible_segment_count)
    } else if segments.len() == 1
        && matches!(segments.first(), Some(UserMessageContentSegment::Text(_)))
    {
        Some(0)
    } else {
        None
    };

    let mut rendered_segments = segments
        .into_iter()
        .take(visible_segment_count)
        .enumerate()
        .map(|(index, segment)| {
            let truncate = Some(index) == truncated_text_index;
            render_segment(index, segment, truncate, cx)
        })
        .collect::<Vec<_>>();
    let show_ellipsis = !include_dropped_text
        && (fit.show_ellipsis || visible_segment_count < segment_widths.len());
    if show_ellipsis {
        rendered_segments.push(render_ellipsis());
    }

    h_flex()
        .min_w_0()
        .flex_1()
        .overflow_hidden()
        .gap_1()
        .items_center()
        .children(rendered_segments)
        .into_any_element()
}

fn trim_segments(mut segments: Vec<UserMessageContentSegment>) -> Vec<UserMessageContentSegment> {
    while matches!(segments.first(), Some(UserMessageContentSegment::Text(text)) if text.trim_start().is_empty())
    {
        segments.remove(0);
    }

    while matches!(segments.last(), Some(UserMessageContentSegment::Text(text)) if text.trim_end().is_empty())
    {
        segments.pop();
    }

    if let Some(UserMessageContentSegment::Text(text)) = segments.first_mut() {
        *text = text.trim_start().to_string();
    }

    if let Some(UserMessageContentSegment::Text(text)) = segments.last_mut() {
        *text = text.trim_end().to_string();
    }

    segments
        .into_iter()
        .filter(
            |segment| !matches!(segment, UserMessageContentSegment::Text(text) if text.is_empty()),
        )
        .collect()
}

fn segmented_preview_line(
    chunks: &[acp::ContentBlock],
    path_style: PathStyle,
) -> (Vec<UserMessageContentSegment>, bool) {
    fn finish_line(
        current_line_segments: &mut Vec<UserMessageContentSegment>,
        first_non_empty_line_segments: &mut Option<Vec<UserMessageContentSegment>>,
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
        match parse_content_block(chunk, path_style) {
            Some(UserMessageContentSegment::Text(text)) => {
                let mut lines = text.split('\n').peekable();
                while let Some(line) = lines.next() {
                    if !line.is_empty() {
                        current_line_segments
                            .push(UserMessageContentSegment::Text(line.to_string()));
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
            Some(segment) => {
                current_line_segments.push(segment);
            }
            None => {}
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
            .unwrap_or_else(|| vec![UserMessageContentSegment::Text("Message".to_string())]),
        has_more_message_content,
    )
}

#[cfg(test)]
mod tests {
    use acp_thread::MentionUri;

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
                UserMessageContentSegment::Text(_),
                UserMessageContentSegment::Mention { .. },
                UserMessageContentSegment::Text(_),
                UserMessageContentSegment::Mention { .. }
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
            [UserMessageContentSegment::Mention { .. }]
        ));
        assert_eq!(preview.text, "@Diagram");
        assert!(preview.has_more_message_content);
    }

    #[test]
    fn display_segments_coalesce_adjacent_text() {
        let segments = sticky_user_message_display_segments(vec![
            UserMessageContentSegment::Text("hel".to_string()),
            UserMessageContentSegment::Text("lo world".to_string()),
        ]);

        assert_eq!(
            segments,
            vec![UserMessageContentSegment::Text("hello world".to_string())]
        );
    }

    #[test]
    fn fit_segments_keeps_everything_that_fits() {
        let fit = fit_sticky_user_message_segments(
            &[gpui::px(10.0), gpui::px(10.0)],
            gpui::px(1.0),
            gpui::px(3.0),
            gpui::px(21.0),
            false,
        );

        assert_eq!(fit.visible_segment_count, 2);
        assert!(!fit.show_ellipsis);
    }

    #[test]
    fn fit_segments_removes_trailing_segments_to_make_room_for_ellipsis() {
        let fit = fit_sticky_user_message_segments(
            &[gpui::px(10.0), gpui::px(10.0), gpui::px(10.0)],
            gpui::px(1.0),
            gpui::px(3.0),
            gpui::px(25.0),
            false,
        );

        assert_eq!(fit.visible_segment_count, 2);
        assert!(fit.show_ellipsis);
    }

    #[test]
    fn display_segments_drop_separator_whitespace() {
        let segments = vec![
            UserMessageContentSegment::Text("Hello ".to_string()),
            UserMessageContentSegment::Mention {
                uri: MentionUri::PastedImage {
                    name: "one".to_string(),
                },
                label: "@one".to_string(),
            },
            UserMessageContentSegment::Text(" ".to_string()),
            UserMessageContentSegment::Mention {
                uri: MentionUri::PastedImage {
                    name: "two".to_string(),
                },
                label: "@two".to_string(),
            },
        ];

        let segments = sticky_user_message_display_segments(segments);

        assert_eq!(
            segments,
            vec![
                UserMessageContentSegment::Text("Hello".to_string()),
                UserMessageContentSegment::Mention {
                    uri: MentionUri::PastedImage {
                        name: "one".to_string(),
                    },
                    label: "@one".to_string(),
                },
                UserMessageContentSegment::Mention {
                    uri: MentionUri::PastedImage {
                        name: "two".to_string(),
                    },
                    label: "@two".to_string(),
                },
            ]
        );
    }

    #[test]
    fn falls_back_to_message_for_empty_preview_content() {
        let chunks = vec![acp::ContentBlock::Text(acp::TextContent::new("\n   \n"))];

        let preview = parse_sticky_user_message_preview(&chunks, PathStyle::Posix);

        assert_eq!(
            preview.segments,
            vec![UserMessageContentSegment::Text("Message".to_string())]
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
            vec![UserMessageContentSegment::Text("hello world".to_string())]
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
            vec![UserMessageContentSegment::Text("First line".to_string())]
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
            vec![UserMessageContentSegment::Text("@notes.md".to_string())]
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
            vec![UserMessageContentSegment::Text(
                "not a valid uri".to_string()
            )]
        );
        assert_eq!(preview.text, "not a valid uri");
        assert!(!preview.has_more_message_content);
    }
}

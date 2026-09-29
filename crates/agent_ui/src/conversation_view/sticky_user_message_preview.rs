//! Condensed, single-line previews of user messages for the agent panel's
//! sticky header.
//!
//! The header has room for exactly one line, so a preview is built from the
//! first non-empty line of the message and then fitted to the width available
//! in the panel. Previews are derived from structured [`acp_v2::ContentBlock`]s
//! rather than from rendered markdown, which keeps mentions and images as
//! labelled chips instead of raw URIs or placeholder syntax.

use std::ops::Range;

use agent_client_protocol::schema::v2 as acp_v2;
use gpui::{AnyElement, App, AvailableSpace, HighlightStyle, Pixels, StyledText, Window};
use ui::{LabelLike, prelude::*};
use util::paths::PathStyle;

use super::{UserMessageContentSegment, parse_content_block};

/// Shown when a message has no renderable text, so the header never collapses
/// into an empty strip.
const EMPTY_PREVIEW_PLACEHOLDER: &str = "Message";

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct StickyUserMessagePreview {
    pub(crate) segments: Vec<UserMessageContentSegment>,
    /// Flattened text of `segments`, used to assert preview contents in tests.
    pub(crate) text: String,
    /// Whether the message continues past the previewed line. Drives the
    /// trailing ellipsis even when every segment happens to fit.
    pub(crate) has_more_message_content: bool,
}

/// A search match inside one display segment, in that segment's byte offsets.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct StickyUserMessageSearchHighlight {
    range: Range<usize>,
    is_active: bool,
}

/// Search matches for a preview, indexed in parallel with the display segments
/// returned by [`sticky_user_message_display_segments`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct StickyUserMessageSearchHighlights {
    segment_ranges: Vec<Vec<StickyUserMessageSearchHighlight>>,
}

pub(crate) fn parse_sticky_user_message_preview(
    blocks: &[acp_v2::ContentBlock],
    path_style: PathStyle,
) -> StickyUserMessagePreview {
    let (segments, has_more_message_content) = segmented_preview_line(blocks, path_style);
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

/// Prepares parsed segments for display by merging adjacent text runs and
/// dropping whitespace-only ones.
///
/// Parsing can split a single visual run of text across several segments, and
/// the whitespace that separated two mentions in the source message would
/// otherwise render as a stray gap between two chips.
fn sticky_user_message_display_segments(
    segments: Vec<UserMessageContentSegment>,
) -> Vec<UserMessageContentSegment> {
    let mut merged_segments: Vec<UserMessageContentSegment> = Vec::new();

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

/// Maps thread-search matches onto the preview's display segments.
///
/// `search_ranges` is called once per segment with that segment's text and
/// returns the matches within it. `active_match_index` is the index of the
/// active match *within this message*, counted in display order.
pub(crate) fn sticky_user_message_search_highlights(
    segments: &[UserMessageContentSegment],
    mut search_ranges: impl FnMut(&str) -> Vec<Range<usize>>,
    active_match_index: Option<usize>,
) -> Option<StickyUserMessageSearchHighlights> {
    let mut match_index = 0;
    let mut has_ranges = false;

    let segment_ranges = sticky_user_message_display_segments(segments.to_vec())
        .iter()
        .map(|segment| {
            search_ranges(segment_text(segment))
                .into_iter()
                .map(|range| {
                    has_ranges = true;
                    let is_active = active_match_index == Some(match_index);
                    match_index += 1;
                    StickyUserMessageSearchHighlight { range, is_active }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    has_ranges.then_some(StickyUserMessageSearchHighlights { segment_ranges })
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct StickyUserMessageFit {
    visible_segment_count: usize,
    show_ellipsis: bool,
}

/// Determines how many segments fit in `available_width`, dropping trailing
/// segments until the row fits.
///
/// The ellipsis is part of the budget: hiding a segment adds the ellipsis,
/// which itself consumes width, so the loop re-checks the total each time. A
/// lone segment is never dropped, since truncating its text is preferable to
/// rendering an empty preview.
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
    search_highlights: Option<StickyUserMessageSearchHighlights>,
    has_more_message_content: bool,
    available_width: Pixels,
    rem_size: Pixels,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let segments = sticky_user_message_display_segments(segments);

    let render_text = |text: String,
                       highlights: Option<&Vec<StickyUserMessageSearchHighlight>>,
                       truncate: bool,
                       cx: &mut App| {
        let Some(highlights) = highlights.filter(|highlights| !highlights.is_empty()) else {
            return Label::new(text)
                .size(LabelSize::Small)
                .color(Color::Default)
                .map(|this| {
                    if truncate {
                        this.truncate()
                    } else {
                        this.single_line().flex_none()
                    }
                })
                .into_any_element();
        };

        let colors = cx.theme().colors();
        let highlights = highlights.iter().map(|highlight| {
            (
                highlight.range.clone(),
                HighlightStyle {
                    background_color: Some(if highlight.is_active {
                        colors.search_active_match_background
                    } else {
                        colors.search_match_background
                    }),
                    ..Default::default()
                },
            )
        });
        let label = LabelLike::new()
            .size(LabelSize::Small)
            .color(Color::Default)
            .map(|this| {
                if truncate {
                    this.truncate()
                } else {
                    this.single_line()
                }
            })
            .child(StyledText::new(text).with_highlights(highlights));

        if truncate {
            div().min_w_0().child(label).into_any_element()
        } else {
            div().flex_none().child(label).into_any_element()
        }
    };

    let render_segment = |index: usize,
                          segment: UserMessageContentSegment,
                          truncate: bool,
                          cx: &mut App| {
        let highlights = search_highlights
            .as_ref()
            .and_then(|highlights| highlights.segment_ranges.get(index));
        match segment {
            UserMessageContentSegment::Text(text) => render_text(text, highlights, truncate, cx),
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
                .child(render_text(label, highlights, false, cx))
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

    // Segments are measured at their natural width so the fit calculation can
    // decide which ones survive before anything is committed to the tree.
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

    // When the first dropped segment is text, keep it and let it truncate
    // instead: a clipped sentence reads better than an abrupt ellipsis, and it
    // fills the leftover space that dropping it entirely would waste.
    let include_dropped_text = segments
        .get(fit.visible_segment_count)
        .is_some_and(|segment| matches!(segment, UserMessageContentSegment::Text(_)));
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

/// Drops leading and trailing whitespace-only text segments and trims the
/// edges of the ones that remain.
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

/// Extracts the first non-empty line of the message as preview segments.
///
/// Returns the segments plus whether any further non-empty content follows, so
/// the caller can show a trailing ellipsis. Scanning stops as soon as a second
/// non-empty line is found, since only the first is ever displayed.
fn segmented_preview_line(
    blocks: &[acp_v2::ContentBlock],
    path_style: PathStyle,
) -> (Vec<UserMessageContentSegment>, bool) {
    /// Commits the pending line. Returns `true` once a *second* non-empty line
    /// has been seen, which signals the caller to stop scanning.
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

    for block in blocks {
        match parse_content_block(block, path_style) {
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
            Some(segment) => current_line_segments.push(segment),
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
        first_non_empty_line_segments.unwrap_or_else(|| {
            vec![UserMessageContentSegment::Text(
                EMPTY_PREVIEW_PLACEHOLDER.to_string(),
            )]
        }),
        has_more_message_content,
    )
}

#[cfg(test)]
mod tests {
    use acp_thread::MentionUri;

    use super::*;

    fn pasted_image_mention(name: &str) -> UserMessageContentSegment {
        UserMessageContentSegment::Mention {
            uri: MentionUri::PastedImage {
                name: name.to_string(),
            },
            label: format!("@{name}"),
        }
    }

    #[test]
    fn uses_structured_labels_for_references() {
        let blocks = vec![
            acp_v2::ContentBlock::Text(acp_v2::TextContent::new("Check ")),
            acp_v2::ContentBlock::ResourceLink(acp_v2::ResourceLink::new(
                "main.rs",
                "file:///project/main.rs",
            )),
            acp_v2::ContentBlock::Text(acp_v2::TextContent::new(" and ")),
            acp_v2::ContentBlock::Resource(acp_v2::EmbeddedResource::new(
                acp_v2::EmbeddedResourceResource::TextResourceContents(
                    acp_v2::TextResourceContents::new("fn main() {}", "file:///project/lib.rs"),
                ),
            )),
        ];

        let preview = parse_sticky_user_message_preview(&blocks, PathStyle::Unix);

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
        let blocks = vec![
            acp_v2::ContentBlock::Image(
                acp_v2::ImageContent::new("ignored", "image/png")
                    .uri("zed:///agent/pasted-image?name=Diagram"),
            ),
            acp_v2::ContentBlock::Text(acp_v2::TextContent::new("\nExplain this diagram")),
        ];

        let preview = parse_sticky_user_message_preview(&blocks, PathStyle::Unix);

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
    fn display_segments_drop_separator_whitespace() {
        let segments = sticky_user_message_display_segments(vec![
            UserMessageContentSegment::Text("Hello ".to_string()),
            pasted_image_mention("one"),
            UserMessageContentSegment::Text(" ".to_string()),
            pasted_image_mention("two"),
        ]);

        assert_eq!(
            segments,
            vec![
                UserMessageContentSegment::Text("Hello".to_string()),
                pasted_image_mention("one"),
                pasted_image_mention("two"),
            ]
        );
    }

    #[test]
    fn fit_segments_keeps_everything_that_fits() {
        let fit = fit_sticky_user_message_segments(
            &[px(10.0), px(10.0)],
            px(1.0),
            px(3.0),
            px(21.0),
            false,
        );

        assert_eq!(fit.visible_segment_count, 2);
        assert!(!fit.show_ellipsis);
    }

    #[test]
    fn fit_segments_removes_trailing_segments_to_make_room_for_ellipsis() {
        let fit = fit_sticky_user_message_segments(
            &[px(10.0), px(10.0), px(10.0)],
            px(1.0),
            px(3.0),
            px(25.0),
            false,
        );

        assert_eq!(fit.visible_segment_count, 2);
        assert!(fit.show_ellipsis);
    }

    #[test]
    fn fit_segments_shows_ellipsis_when_message_continues_past_preview() {
        let fit = fit_sticky_user_message_segments(&[px(10.0)], px(1.0), px(3.0), px(100.0), true);

        assert_eq!(fit.visible_segment_count, 1);
        assert!(fit.show_ellipsis);
    }

    #[test]
    fn search_highlights_are_mapped_to_display_segments() {
        let segments = vec![
            UserMessageContentSegment::Text("Check ".to_string()),
            pasted_image_mention("main.rs"),
            UserMessageContentSegment::Text(" and main.rs".to_string()),
        ];

        let highlights = sticky_user_message_search_highlights(
            &segments,
            |text| {
                text.match_indices("main")
                    .map(|(start, text)| start..start + text.len())
                    .collect()
            },
            Some(1),
        )
        .expect("expected matches in sticky preview");

        assert_eq!(highlights.segment_ranges[0], Vec::new());
        assert_eq!(highlights.segment_ranges[1].len(), 1);
        assert_eq!(highlights.segment_ranges[1][0].range, 1..5);
        assert!(!highlights.segment_ranges[1][0].is_active);
        assert_eq!(highlights.segment_ranges[2].len(), 1);
        assert_eq!(highlights.segment_ranges[2][0].range, 4..8);
        assert!(highlights.segment_ranges[2][0].is_active);
    }

    #[test]
    fn search_highlights_are_absent_without_matches() {
        let segments = vec![UserMessageContentSegment::Text("Check this".to_string())];

        let highlights = sticky_user_message_search_highlights(&segments, |_| Vec::new(), None);

        assert_eq!(highlights, None);
    }

    #[test]
    fn falls_back_to_placeholder_for_empty_preview_content() {
        let blocks = vec![acp_v2::ContentBlock::Text(acp_v2::TextContent::new(
            "\n   \n",
        ))];

        let preview = parse_sticky_user_message_preview(&blocks, PathStyle::Unix);

        assert_eq!(
            preview.segments,
            vec![UserMessageContentSegment::Text(
                EMPTY_PREVIEW_PLACEHOLDER.to_string()
            )]
        );
        assert_eq!(preview.text, EMPTY_PREVIEW_PLACEHOLDER);
        assert!(!preview.has_more_message_content);
    }

    #[test]
    fn trims_surrounding_whitespace_on_first_non_empty_line() {
        let blocks = vec![acp_v2::ContentBlock::Text(acp_v2::TextContent::new(
            "\n   hello world   \n",
        ))];

        let preview = parse_sticky_user_message_preview(&blocks, PathStyle::Unix);

        assert_eq!(
            preview.segments,
            vec![UserMessageContentSegment::Text("hello world".to_string())]
        );
        assert_eq!(preview.text, "hello world");
        assert!(!preview.has_more_message_content);
    }

    #[test]
    fn marks_has_more_content_when_later_non_empty_lines_exist() {
        let blocks = vec![acp_v2::ContentBlock::Text(acp_v2::TextContent::new(
            "\nFirst line\n\nSecond line",
        ))];

        let preview = parse_sticky_user_message_preview(&blocks, PathStyle::Unix);

        assert_eq!(
            preview.segments,
            vec![UserMessageContentSegment::Text("First line".to_string())]
        );
        assert_eq!(preview.text, "First line");
        assert!(preview.has_more_message_content);
    }

    #[test]
    fn falls_back_to_resource_name_for_invalid_resource_link_uri() {
        let blocks = vec![acp_v2::ContentBlock::ResourceLink(
            acp_v2::ResourceLink::new("notes.md", "not a valid uri"),
        )];

        let preview = parse_sticky_user_message_preview(&blocks, PathStyle::Unix);

        assert_eq!(
            preview.segments,
            vec![UserMessageContentSegment::Text("@notes.md".to_string())]
        );
        assert_eq!(preview.text, "@notes.md");
        assert!(!preview.has_more_message_content);
    }

    #[test]
    fn falls_back_to_raw_uri_for_invalid_embedded_resource_uri() {
        let blocks = vec![acp_v2::ContentBlock::Resource(
            acp_v2::EmbeddedResource::new(acp_v2::EmbeddedResourceResource::TextResourceContents(
                acp_v2::TextResourceContents::new("contents", "not a valid uri"),
            )),
        )];

        let preview = parse_sticky_user_message_preview(&blocks, PathStyle::Unix);

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

use super::super::*;
use super::geometry::node_left_top;
use super::model::SequenceSvgModel;
use crate::sequence::sequence_text_dimensions_height_px;
use rustc_hash::FxHashMap;

pub(super) fn render_sequence_box_frames_and_rect_blocks(
    out: &mut String,
    model: &SequenceSvgModel,
    nodes_by_id: &FxHashMap<&str, &LayoutNode>,
    actor_label_font_size: f64,
    box_margin: f64,
    box_text_margin: f64,
) {
    // Mermaid renders "box" frames as root-level `<g><rect class="rect"/>...</g>` nodes before actors.
    // Mermaid renders boxes "behind" other elements; multiple boxes end up reversed in DOM order.
    let has_box_titles = model
        .boxes
        .iter()
        .any(|b| b.name.as_deref().is_some_and(|s| !s.trim().is_empty()));
    let max_box_title_height = if has_box_titles {
        // Mermaid uses `utils.calculateTextDimensions(...).height` for box titles.
        // With 16px fonts this ends up as 17px, and is used for the actor `starty` bump.
        let line_h = sequence_text_dimensions_height_px(actor_label_font_size);
        model
            .boxes
            .iter()
            .filter_map(|b| b.name.as_deref())
            .map(|s| crate::text::split_html_br_lines(s).len().max(1) as f64 * line_h)
            .fold(0.0, f64::max)
    } else {
        0.0
    };

    for b in model.boxes.iter().rev() {
        let pad_x = (box_margin * 2.0 + box_text_margin).max(0.0);
        let pad_top = (box_margin + box_text_margin + max_box_title_height).max(0.0);
        let pad_bottom = (box_margin * 2.0).max(0.0);

        let mut min_x = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut min_top_y = f64::INFINITY;
        let mut max_bottom_y = f64::NEG_INFINITY;

        for actor_key in &b.actor_keys {
            let top_id = format!("actor-top-{actor_key}");
            let bottom_id = format!("actor-bottom-{actor_key}");
            let Some(top) = nodes_by_id.get(top_id.as_str()).copied() else {
                continue;
            };
            let Some(bottom) = nodes_by_id.get(bottom_id.as_str()).copied() else {
                continue;
            };

            let (top_x, top_y) = node_left_top(top);
            min_x = min_x.min(top_x);
            max_x = max_x.max(top_x + top.width);
            min_top_y = min_top_y.min(top_y);

            let (_bottom_x, bottom_y) = node_left_top(bottom);
            max_bottom_y = max_bottom_y.max(bottom_y + bottom.height);
        }

        if !min_x.is_finite()
            || !max_x.is_finite()
            || !min_top_y.is_finite()
            || !max_bottom_y.is_finite()
        {
            continue;
        }

        let x = min_x - pad_x;
        let w = (max_x - min_x) + pad_x * 2.0;
        let y = min_top_y - pad_top;
        let h = (max_bottom_y - min_top_y) + pad_top + pad_bottom;

        out.push_str("<g>");
        let _ = write!(
            out,
            r#"<rect x="{x}" y="{y}" fill="{fill}" stroke="rgb(0,0,0, 0.5)" width="{w}" height="{h}" class="rect"/>"#,
            x = fmt(x),
            y = fmt(y),
            w = fmt(w),
            h = fmt(h),
            fill = escape_xml_display(&b.fill),
        );
        if let Some(name) = b.name.as_deref() {
            let cx = x + (w / 2.0);
            // Mermaid's `drawBox(...)` places the title at `box.y + boxTextMargin + textMaxHeight/2`.
            // In upstream, `box.y` is the `verticalPos` passed to `addActorRenderingData`, i.e. 0.
            let box_y = min_top_y - (box_margin + max_box_title_height);
            let text_y = box_y + box_text_margin + max_box_title_height / 2.0;
            let _ = write!(
                out,
                r#"<text x="{x}" y="{y}" dominant-baseline="central" alignment-baseline="central" class="text" style="text-anchor: middle; font-size: 16px; font-weight: 400;"><tspan x="{x}" dy="0">{text}</tspan></text>"#,
                x = fmt(cx),
                y = fmt(text_y),
                text = escape_xml_display(name)
            );
        }
        out.push_str("</g>");
    }

    // Mermaid renders `rect` blocks as root-level `<rect class="rect"/>` nodes before actors.
    {
        #[derive(Debug, Clone, Copy)]
        struct RectBlock<'a> {
            fill: &'a str,
            x: f64,
            y: f64,
            w: f64,
            h: f64,
        }

        fn contains(a: &RectBlock<'_>, b: &RectBlock<'_>) -> bool {
            const EPS: f64 = 1e-9;
            a.x <= b.x + EPS
                && a.y <= b.y + EPS
                && (a.x + a.w) >= (b.x + b.w) - EPS
                && (a.y + a.h) >= (b.y + b.h) - EPS
        }

        let mut rects: Vec<RectBlock<'_>> = Vec::new();
        for msg in &model.messages {
            if msg.message_type != 22 {
                continue;
            }
            let fill = msg.message_text();
            let node_id = format!("rect-{}", msg.id);
            let Some(n) = nodes_by_id.get(node_id.as_str()).copied() else {
                continue;
            };
            let (x, y) = node_left_top(n);
            rects.push(RectBlock {
                fill,
                x,
                y,
                w: n.width,
                h: n.height,
            });
        }

        // Mermaid's emitted order for nested `rect` blocks is not strictly tied to parse order.
        // Match its DOM ordering semantics by keeping parents before contained children and
        // sorting unrelated rectangles by vertical position (lower blocks first).
        rects.sort_by(|a, b| {
            if contains(a, b) && !contains(b, a) {
                return std::cmp::Ordering::Less;
            }
            if contains(b, a) && !contains(a, b) {
                return std::cmp::Ordering::Greater;
            }
            b.y.partial_cmp(&a.y)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.x.partial_cmp(&b.x).unwrap_or(std::cmp::Ordering::Equal))
        });

        for r in rects {
            let _ = write!(
                out,
                r#"<rect x="{x}" y="{y}" fill="{fill}" width="{w}" height="{h}" class="rect"/>"#,
                x = fmt(r.x),
                y = fmt(r.y),
                w = fmt(r.w),
                h = fmt(r.h),
                fill = escape_xml_display(r.fill)
            );
        }
    }
}

//! Flowchart node shape dispatch.

use std::fmt::Write as _;

use crate::svg::parity::flowchart::escape_attr;
use crate::svg::parity::fmt;

pub(in super::super) fn render_flowchart_v2_shape(
    out: &mut String,
    ctx: &crate::svg::parity::flowchart::types::FlowchartRenderCtx<'_>,
    common: &super::super::FlowchartNodeRenderCommon<'_>,
    label: &mut super::super::FlowchartNodeLabelState<'_>,
    details: &mut crate::svg::parity::flowchart::types::FlowchartRenderDetails,
) -> bool {
    let shape = common.shape;
    let layout_node = common.layout_node;
    let style = common.style;

    match shape {
        // Flowchart v2 shapes with no label group are handled earlier.

        // Flowchart v2 hourglass/collate: Mermaid clears `node.label` but still emits an empty
        // label group via `labelHelper(...)`, preserving the parsed label type.
        "hourglass" | "collate" => {
            label.text = "";
            super::render_hourglass_collate(out, common, details);
        }

        // Flowchart v2 card/notched-rectangle.
        "notch-rect" | "notched-rectangle" | "card" => {
            super::render_notched_rectangle(out, common);
        }

        // Flowchart v2 delay / half-rounded rectangle.
        "delay" | "half-rounded-rectangle" => {
            super::render_delay(out, ctx, common, label, details);
        }

        // Flowchart v2 lined cylinder (Disk storage).
        "lin-cyl" | "disk" | "lined-cylinder" => {
            super::render_lined_cylinder(out, common, label);
        }

        // Flowchart v2 curved trapezoid (Display).
        "curv-trap" | "display" | "curved-trapezoid" => {
            super::render_curved_trapezoid(out, ctx, common, label, details);
        }

        // Flowchart v2 divided rectangle (Divided process).
        "div-rect" | "div-proc" | "divided-rectangle" | "divided-process" => {
            super::render_divided_rect(out, common, label, details);
        }

        // Flowchart v2 notched pentagon (Loop limit).
        "notch-pent" | "loop-limit" | "notched-pentagon" => {
            super::render_notched_pentagon(out, ctx, common, label, details);
        }

        // Flowchart v2 bow tie rectangle (Stored data).
        "bow-rect" | "stored-data" | "bow-tie-rectangle" => {
            super::render_bow_tie_rect(out, ctx, common, label, details);
        }

        // Flowchart v2 datastore: rectangular node with only top and bottom borders.
        "datastore" | "data-store" => {
            super::render_datastore(out, common);
        }

        // Flowchart v2 tagged rectangle (Tagged process).
        "tag-rect" | "tagged-rectangle" | "tag-proc" | "tagged-process" => {
            super::render_tag_rect(out, ctx, common, label, details);
        }

        // Flowchart v2 wave edged rectangle (Document).
        "doc" | "document" => {
            super::render_wave_document(out, ctx, common, label, details);
        }

        // Flowchart v2 lined wave edged rectangle (Lined document).
        "lin-doc" | "lined-document" => {
            super::render_lined_wave_document(out, ctx, common, label, details);
        }

        // Flowchart v2 tagged wave edged rectangle (Tagged document).
        "tag-doc" | "tagged-document" => {
            super::render_tagged_wave_document(out, ctx, common, label, details);
        }

        // Flowchart v2 triangle (Extract).
        "tri" | "extract" | "triangle" => {
            super::render_triangle_extract(out, ctx, common, label, details);
        }

        // Flowchart v2 shaded process / lined rectangle.
        "lin-rect" | "lined-rectangle" | "lined-process" | "lin-proc" | "shaded-process" => {
            super::render_shaded_process(out, common, label, details);
        }

        // Flowchart v2 curly brace/comment shapes (rendering-elements).
        "comment" | "brace" | "brace-l" | "brace-r" | "braces" => {
            super::render_curly_brace_comment(out, ctx, common, label, details);
        }

        "imageSquare" => {
            if super::try_render_image_square(out, ctx, common, label, details) {
                return true;
            }
        }
        "icon" => {
            if super::try_render_icon(out, ctx, common, label, details) {
                return true;
            }
        }
        "iconSquare" => {
            if super::try_render_icon_square(out, ctx, common, label, details) {
                return true;
            }
        }
        "manual-file" | "flipped-triangle" | "flip-tri" => {
            super::render_manual_file(out, ctx, common, label, details);
        }
        "manual-input" | "sloped-rectangle" | "sl-rect" => {
            super::render_manual_input(out, ctx, common, label, details);
        }
        "docs" | "documents" | "st-doc" | "stacked-document" => {
            super::render_stacked_document(out, ctx, common, label, details);
        }
        "procs" | "processes" | "st-rect" | "stacked-rectangle" => {
            super::render_stacked_rectangle(out, common, label, details);
        }
        "paper-tape" | "flag" => {
            super::render_paper_tape(out, ctx, common, label, details);
        }
        "subroutine" | "fr-rect" | "subproc" | "subprocess" | "framed-rectangle" => {
            super::render_subroutine(out, common);
        }
        "cylinder" | "cyl" | "db" | "database" => {
            super::render_cylinder(out, ctx, common, label);
        }
        "h-cyl" | "das" | "horizontal-cylinder" => {
            super::render_horizontal_cylinder(out, ctx, common, label);
        }
        "win-pane" | "internal-storage" | "window-pane" => {
            super::render_window_pane(out, common, label, details);
        }
        "diamond" | "question" | "diam" | "decision" => {
            super::render_diamond(out, common);
        }
        "circle" => {
            super::render_circle(out, common);
        }
        "doublecircle" | "dbl-circ" | "double-circle" => {
            super::render_double_circle(out, common);
        }
        "roundedRect" | "rounded" | "event" => {
            super::render_rounded_rect(out, ctx, common, details);
        }
        "note" => {
            super::render_note(out, ctx, common, details);
        }
        "stadium" | "terminal" | "pill" => {
            super::render_stadium(out, ctx, common, label, details);
        }
        "hexagon" | "hex" | "prepare" => {
            super::render_hexagon(out, common, details);
        }
        "lean_right" | "lean-r" | "lean-right" | "in-out" => {
            super::render_lean_right(out, common);
        }
        "lean_left" | "lean-l" | "lean-left" | "out-in" => {
            super::render_lean_left(out, common);
        }
        "trapezoid" | "trap-b" | "priority" | "trapezoid-bottom" => {
            super::render_trapezoid(out, common);
        }
        "inv_trapezoid" | "inv-trapezoid" | "trap-t" | "manual" | "trapezoid-top" => {
            super::render_inv_trapezoid(out, common);
        }
        "odd" => {
            super::render_odd(out, common, label, details);
        }
        "text" => {
            // Mermaid `text.ts`: invisible rect used only to size/position the label.
            let w = layout_node.width.max(0.0);
            let h = layout_node.height.max(0.0);
            let _ = write!(
                out,
                r#"<rect class="text" style="{}" rx="0" ry="0" x="{}" y="{}" width="{}" height="{}"/>"#,
                escape_attr(style),
                fmt(-w / 2.0),
                fmt(-h / 2.0),
                fmt(w),
                fmt(h)
            );
        }
        _ => {
            let w = layout_node.width.max(1.0);
            let h = layout_node.height.max(1.0);
            let _ = write!(
                out,
                r#"<rect class="basic label-container" style="{}" x="{}" y="{}" width="{}" height="{}"/>"#,
                escape_attr(style),
                fmt(-w / 2.0),
                fmt(-h / 2.0),
                fmt(w),
                fmt(h)
            );
        }
    }

    false
}

use super::super::*;

pub(super) fn sequence_css(
    diagram_id: &str,
    font_size_px: f64,
    effective_config: &serde_json::Value,
) -> String {
    // Mirrors Mermaid 11.15 `diagrams/sequence/styles.js` + shared base stylesheet ordering.
    // Keep `:root` last (matches upstream fixtures).
    let id = escape_xml(diagram_id);
    let theme = PresentationTheme::new(effective_config).sequence_diagram();
    let font = theme.common.font_family_css.as_str();
    let text_color = theme.common.text_color.as_str();
    let error_bkg = theme.common.error_bkg.as_str();
    let error_text = theme.common.error_text.as_str();
    let line_color = theme.common.line_color.as_str();
    let mut out = String::new();
    let _ = write!(
        &mut out,
        r#"#{}{{font-family:{};font-size:{}px;fill:{};}}"#,
        id,
        font,
        fmt(font_size_px),
        text_color
    );
    out.push_str(
        r#"@keyframes edge-animation-frame{from{stroke-dashoffset:0;}}@keyframes dash{to{stroke-dashoffset:0;}}"#,
    );
    let _ = write!(
        &mut out,
        r#"#{} .edge-animation-slow{{stroke-dasharray:9,5!important;stroke-dashoffset:900;animation:dash 50s linear infinite;stroke-linecap:round;}}#{} .edge-animation-fast{{stroke-dasharray:9,5!important;stroke-dashoffset:900;animation:dash 20s linear infinite;stroke-linecap:round;}}"#,
        id, id
    );
    let _ = write!(
        &mut out,
        r#"#{} .error-icon{{fill:{};}}#{} .error-text{{fill:{};stroke:{};}}"#,
        id, error_bkg, id, error_text, error_text
    );
    let _ = write!(
        &mut out,
        r#"#{} .edge-thickness-normal{{stroke-width:1px;}}#{} .edge-thickness-thick{{stroke-width:3.5px;}}#{} .edge-pattern-solid{{stroke-dasharray:0;}}#{} .edge-thickness-invisible{{stroke-width:0;fill:none;}}#{} .edge-pattern-dashed{{stroke-dasharray:3;}}#{} .edge-pattern-dotted{{stroke-dasharray:2;}}"#,
        id, id, id, id, id, id
    );
    let _ = write!(
        &mut out,
        r#"#{} .marker{{fill:{};stroke:{};}}#{} .marker.cross{{stroke:{};}}"#,
        id, line_color, line_color, id, line_color
    );
    let _ = write!(
        &mut out,
        r#"#{} svg{{font-family:{};font-size:{}px;}}#{} p{{margin:0;}}"#,
        id,
        font,
        fmt(font_size_px),
        id
    );

    // Sequence styles.
    let actor_border = theme.actor_border.as_str();
    let actor_fill = theme.actor_fill.as_str();
    let stroke_width = theme.stroke_width.as_str();
    let drop_shadow = theme.drop_shadow.as_str();
    let note_border = theme.note_border.as_str();
    let note_fill = theme.note_fill.as_str();
    let actor_text = theme.actor_text.as_str();
    let actor_line = theme.actor_line.as_str();
    let signal_color = theme.signal_color.as_str();
    let sequence_number = theme.sequence_number.as_str();
    let signal_text = theme.signal_text.as_str();
    let label_box_border = theme.label_box_border.as_str();
    let label_box_fill = theme.label_box_fill.as_str();
    let label_text = theme.label_text.as_str();
    let loop_text = theme.loop_text.as_str();
    let note_text = theme.note_text.as_str();
    let activation_fill = theme.activation_fill.as_str();
    let activation_border = theme.activation_border.as_str();
    let node_border = theme.node_border.as_str();
    let label_box_filter = theme.label_box_filter.as_str();
    let note_font_weight = theme.note_font_weight.as_str();

    let _ = write!(
        &mut out,
        r#"#{} .actor{{stroke:{};fill:{};stroke-width:{};}}"#,
        id, actor_border, actor_fill, stroke_width
    );
    let _ = write!(
        &mut out,
        r#"#{} text.actor>tspan{{fill:{};stroke:none;}}"#,
        id, actor_text
    );
    let _ = write!(&mut out, r#"#{} .actor-line{{stroke:{};}}"#, id, actor_line);
    let _ = write!(
        &mut out,
        r#"#{} .innerArc{{stroke-width:1.5;stroke-dasharray:none;}}"#,
        id
    );
    let _ = write!(
        &mut out,
        r#"#{} .messageLine0{{stroke-width:1.5;stroke-dasharray:none;stroke:{};}}"#,
        id, signal_color
    );
    let _ = write!(
        &mut out,
        r#"#{} .messageLine1{{stroke-width:1.5;stroke-dasharray:2,2;stroke:{};}}"#,
        id, signal_color
    );
    let _ = write!(
        &mut out,
        r#"#{} [id$="-arrowhead"] path{{fill:{};stroke:{};}}"#,
        id, signal_color, signal_color
    );
    let _ = write!(
        &mut out,
        r#"#{} .sequenceNumber{{fill:{};}}"#,
        id, sequence_number
    );
    let _ = write!(
        &mut out,
        r#"#{} [id$="-sequencenumber"]{{fill:{};}}"#,
        id, signal_color
    );
    let _ = write!(
        &mut out,
        r#"#{} [id$="-crosshead"] path{{fill:{};stroke:{};}}"#,
        id, signal_color, signal_color
    );
    let _ = write!(
        &mut out,
        r#"#{} .messageText{{fill:{};stroke:none;}}"#,
        id, signal_text
    );
    let _ = write!(
        &mut out,
        r#"#{} .labelBox{{stroke:{};fill:{};filter:{};}}"#,
        id, label_box_border, label_box_fill, label_box_filter
    );
    let _ = write!(
        &mut out,
        r#"#{} .labelText,#{} .labelText>tspan{{fill:{};stroke:none;}}"#,
        id, id, label_text
    );
    let _ = write!(
        &mut out,
        r#"#{} .loopText,#{} .loopText>tspan{{fill:{};stroke:none;}}"#,
        id, id, loop_text
    );
    let _ = write!(
        &mut out,
        r#"#{} .sectionTitle,#{} .sectionTitle>tspan{{fill:{};stroke:none;}}"#,
        id, id, loop_text
    );
    let _ = write!(
        &mut out,
        r#"#{} .loopLine{{stroke-width:2px;stroke-dasharray:2,2;stroke:{};fill:{};}}"#,
        id, label_box_border, label_box_border
    );
    let _ = write!(
        &mut out,
        r#"#{} .note{{stroke:{};fill:{};}}"#,
        id, note_border, note_fill
    );
    let _ = write!(
        &mut out,
        r#"#{} .noteText,#{} .noteText>tspan{{fill:{};stroke:none;{}}}"#,
        id, id, note_text, note_font_weight
    );
    let _ = write!(
        &mut out,
        r#"#{} .activation0{{fill:{};stroke:{};}}#{} .activation1{{fill:{};stroke:{};}}#{} .activation2{{fill:{};stroke:{};}}"#,
        id,
        activation_fill,
        activation_border,
        id,
        activation_fill,
        activation_border,
        id,
        activation_fill,
        activation_border
    );
    let _ = write!(&mut out, r#"#{} .actorPopupMenu{{position:absolute;}}"#, id);
    let _ = write!(
        &mut out,
        r#"#{} .actorPopupMenuPanel{{position:absolute;fill:{};box-shadow:0px 8px 16px 0px rgba(0,0,0,0.2);filter:drop-shadow(3px 5px 2px rgb(0 0 0 / 0.4));}}"#,
        id, actor_fill
    );
    let _ = write!(
        &mut out,
        r#"#{} .actor-man line{{stroke:{};fill:{};}}"#,
        id, actor_border, actor_fill
    );
    let _ = write!(
        &mut out,
        r#"#{} .actor-man circle,#{} line{{stroke:{};fill:{};stroke-width:2px;}}"#,
        id, id, actor_border, actor_fill
    );
    let _ = write!(
        &mut out,
        r#"#{} g rect.rect{{filter:{};stroke:{};}}"#,
        id, drop_shadow, node_border
    );
    let _ = write!(
        &mut out,
        r#"#{} :root{{--mermaid-font-family:{};}}"#,
        id, font
    );
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn sequence_css_uses_configured_font_size() {
        let css = sequence_css("seq", 24.0, &json!({}));

        assert!(css.contains(
            r#"#seq{font-family:"trebuchet ms",verdana,arial,sans-serif;font-size:24px;fill:#333;}"#
        ));
        assert!(css.contains(r#"#seq svg{font-family:"trebuchet ms",verdana,arial,sans-serif;font-size:24px;}#seq p{margin:0;}"#));
    }

    #[test]
    fn sequence_css_honors_mermaid_11_15_theme_options() {
        let cfg = json!({
            "look": "neo",
            "themeVariables": {
                "fontFamily": "Inter, Arial",
                "textColor": "#abc001",
                "errorBkgColor": "#100000",
                "errorTextColor": "#ffeeee",
                "lineColor": "#123456",
                "actorBorder": "#220000",
                "actorBkg": "#330000",
                "strokeWidth": 2,
                "dropShadow": "drop-shadow(1px 2px 3px rgba(0,0,0,.4))",
                "actorTextColor": "#fafafa",
                "actorLineColor": "#444444",
                "signalColor": "#555555",
                "sequenceNumberColor": "#666666",
                "signalTextColor": "#777777",
                "labelBoxBorderColor": "#888888",
                "labelBoxBkgColor": "#999999",
                "labelTextColor": "#aaaaaa",
                "loopTextColor": "#bbbbbb",
                "noteBorderColor": "#cccccc",
                "noteBkgColor": "#dddddd",
                "noteTextColor": "#eeeeee",
                "noteFontWeight": 600,
                "activationBkgColor": "#010203",
                "activationBorderColor": "#040506",
                "nodeBorder": "#070809"
            }
        });

        let css = sequence_css("seq", 16.0, &cfg);

        assert!(css.contains(r#"#seq{font-family:Inter,Arial;font-size:16px;fill:#abc001;}"#));
        assert!(css.contains(
            r#"#seq .error-icon{fill:#100000;}#seq .error-text{fill:#ffeeee;stroke:#ffeeee;}"#
        ));
        assert!(css.contains(
            r#"#seq .marker{fill:#123456;stroke:#123456;}#seq .marker.cross{stroke:#123456;}"#
        ));
        assert!(css.contains(r#"#seq .actor{stroke:#220000;fill:#330000;stroke-width:2;}"#));
        assert!(css.contains(r#"#seq text.actor>tspan{fill:#fafafa;stroke:none;}"#));
        assert!(css.contains(r#"#seq .actor-line{stroke:#444444;}"#));
        assert!(css.contains(
            r#"#seq .messageLine0{stroke-width:1.5;stroke-dasharray:none;stroke:#555555;}"#
        ));
        assert!(css.contains(r#"#seq .sequenceNumber{fill:#666666;}"#));
        assert!(css.contains(r#"#seq .messageText{fill:#777777;stroke:none;}"#));
        assert!(css.contains(r#"#seq .labelBox{stroke:#888888;fill:#999999;filter:drop-shadow(1px 2px 3px rgba(0,0,0,.4));}"#));
        assert!(
            css.contains(
                r#"#seq .sectionTitle,#seq .sectionTitle>tspan{fill:#bbbbbb;stroke:none;}"#
            )
        );
        assert!(css.contains(r#"#seq .note{stroke:#cccccc;fill:#dddddd;}"#));
        assert!(css.contains(
            r#"#seq .noteText,#seq .noteText>tspan{fill:#eeeeee;stroke:none;font-weight:600;}"#
        ));
        assert!(css.contains(r#"#seq .activation0{fill:#010203;stroke:#040506;}"#));
        assert!(css.contains(
            r#"#seq g rect.rect{filter:drop-shadow(1px 2px 3px rgba(0,0,0,.4));stroke:#070809;}"#
        ));
    }
}

use super::super::*;

pub(super) fn class_css(
    diagram_id: &str,
    effective_config: &serde_json::Value,
    font_family: &str,
    font_size_css: &str,
) -> String {
    let id = escape_xml(diagram_id);
    let theme = PresentationTheme::new(effective_config).class_diagram();
    let font_family = normalize_css_font_family(font_family);
    let font_family = if font_family.is_empty() {
        "\"trebuchet ms\",verdana,arial,sans-serif"
    } else {
        font_family.as_str()
    };
    let class_text = theme.class_text.as_str();
    let note_text = theme.note_text.as_str();
    let line_color = theme.common.line_color.as_str();
    let main_bkg = theme.main_bkg.as_str();
    let node_border = theme.node_border.as_str();
    let class_group_text = theme.class_group_text.as_str();
    let cluster_bkg = theme.cluster_bkg.as_str();
    let cluster_border = theme.cluster_border.as_str();
    let title_color = theme.title_color.as_str();
    let text_color = theme.text_color.as_str();
    let stroke_width = theme.stroke_width.as_str();

    let mut out = String::new();
    let _ = write!(
        &mut out,
        r#"#{}{{font-family:{};font-size:{};fill:{};}}"#,
        id.as_str(),
        font_family,
        font_size_css,
        class_text
    );
    let _ = write!(
        &mut out,
        r#"#{} p{{margin:0;}}#{} g.classGroup text{{fill:{};stroke:none;font-family:{};font-size:10px;}}#{} g.classGroup text .title{{font-weight:bolder;}}#{} .cluster-label text{{fill:{};}}#{} .cluster-label span{{color:{};}}#{} .cluster-label span p{{background-color:transparent;}}#{} .cluster rect{{fill:{};stroke:{};stroke-width:1px;}}#{} .cluster text{{fill:{};}}#{} .cluster span{{color:{};}}#{} .nodeLabel,#{} .edgeLabel{{color:{};}}#{} .noteLabel .nodeLabel,#{} .noteLabel .edgeLabel{{color:{};}}#{} .label text{{fill:{};}}#{} .label span{{fill:{};color:{};}}"#,
        id.as_str(),
        id.as_str(),
        class_group_text,
        font_family,
        id.as_str(),
        id.as_str(),
        title_color,
        id.as_str(),
        title_color,
        id.as_str(),
        id.as_str(),
        cluster_bkg,
        cluster_border,
        id.as_str(),
        title_color,
        id.as_str(),
        title_color,
        id.as_str(),
        id.as_str(),
        class_text,
        id.as_str(),
        id.as_str(),
        note_text,
        id.as_str(),
        class_text,
        id.as_str(),
        class_text,
        class_text
    );
    let _ = write!(
        &mut out,
        r#"#{} .edgeLabel .label rect{{fill:{};}}#{} .labelBkg{{background:{}}}#{} .edgeLabel .label span{{background:{}}}#{} .classTitle{{font-weight:bolder;}}#{} .node rect,#{} .node circle,#{} .node ellipse,#{} .node polygon,#{} .node path{{fill:{};stroke:{};stroke-width:{}}}#{} .divider{{stroke:{};stroke-width:1;}}#{} g.classGroup rect{{fill:{};stroke:{};}}#{} g.classGroup line{{stroke:{};stroke-width:1;}}#{} .classLabel .box{{stroke:none;stroke-width:0;fill:{};opacity:0.5;}}#{} .classLabel .label{{fill:{};font-size:10px;}}#{} .relation{{stroke:{};stroke-width:{};fill:none;}}#{} .dashed-line{{stroke-dasharray:3;}}#{} .dotted-line{{stroke-dasharray:1 2;}}"#,
        id.as_str(),
        main_bkg,
        id.as_str(),
        main_bkg,
        id.as_str(),
        main_bkg,
        id.as_str(),
        id.as_str(),
        id.as_str(),
        id.as_str(),
        id.as_str(),
        id.as_str(),
        main_bkg,
        node_border,
        stroke_width,
        id.as_str(),
        node_border,
        id.as_str(),
        main_bkg,
        node_border,
        id.as_str(),
        node_border,
        id.as_str(),
        main_bkg,
        id.as_str(),
        node_border,
        id.as_str(),
        line_color,
        stroke_width,
        id.as_str(),
        id.as_str()
    );
    let _ = write!(
        &mut out,
        r#"#{} [id$="-compositionStart"],#{} .composition,#{} [id$="-compositionEnd"]{{fill:{}!important;stroke:{}!important;stroke-width:1;}}#{} [id$="-dependencyStart"],#{} .dependency,#{} [id$="-dependencyEnd"]{{fill:{}!important;stroke:{}!important;stroke-width:1;}}"#,
        id.as_str(),
        id.as_str(),
        id.as_str(),
        line_color,
        line_color,
        id.as_str(),
        id.as_str(),
        id.as_str(),
        line_color,
        line_color
    );
    let _ = write!(
        &mut out,
        r#"#{} [id$="-extensionStart"],#{} .extension,#{} [id$="-extensionEnd"],#{} [id$="-aggregationStart"],#{} .aggregation,#{} [id$="-aggregationEnd"]{{fill:transparent!important;stroke:{}!important;stroke-width:1;}}#{} [id$="-lollipopStart"],#{} .lollipop,#{} [id$="-lollipopEnd"]{{fill:{}!important;stroke:{}!important;stroke-width:1;}}"#,
        id.as_str(),
        id.as_str(),
        id.as_str(),
        id.as_str(),
        id.as_str(),
        id.as_str(),
        line_color,
        id.as_str(),
        id.as_str(),
        id.as_str(),
        main_bkg,
        line_color
    );
    let _ = write!(
        &mut out,
        r#"#{} g.clickable{{cursor:pointer;}}#{} .edgeTerminals{{font-size:11px;line-height:initial;}}#{} .classTitleText,#{} .classDiagramTitleText{{text-anchor:middle;font-size:18px;fill:{};}}"#,
        id.as_str(),
        id.as_str(),
        id.as_str(),
        id.as_str(),
        text_color
    );
    out
}

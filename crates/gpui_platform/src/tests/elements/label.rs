use gpui::color::Color;
use gpui::elements::Label;
use gpui::fonts::{Properties as FontProperties, Weight};

#[crate::test(self)]
fn test_layout_label_with_highlights(cx: &mut crate::AppContext) {
    let default_style = TextStyle::new(
        "Menlo",
        12.,
        Default::default(),
        Default::default(),
        Default::default(),
        Color::black(),
        cx.font_cache(),
    )
    .unwrap();
    let highlight_style = TextStyle::new(
        "Menlo",
        12.,
        *FontProperties::new().weight(Weight::BOLD),
        Default::default(),
        Default::default(),
        Color::new(255, 0, 0, 255),
        cx.font_cache(),
    )
    .unwrap();
    let label = Label::new(
        ".αβγδε.ⓐⓑⓒⓓⓔ.abcde.".to_string(),
        LabelStyle {
            text: default_style.clone(),
            highlight_text: Some(highlight_style.clone()),
        },
    )
    .with_highlights(vec![
        ".α".len(),
        ".αβ".len(),
        ".αβγδ".len(),
        ".αβγδε.ⓐ".len(),
        ".αβγδε.ⓐⓑ".len(),
    ]);

    let default_run_style = RunStyle {
        font_id: default_style.font_id,
        color: default_style.color,
        underline: default_style.underline,
    };
    let highlight_run_style = RunStyle {
        font_id: highlight_style.font_id,
        color: highlight_style.color,
        underline: highlight_style.underline,
    };
    let runs = label.compute_runs();
    assert_eq!(
        runs.as_slice(),
        &[
            (".α".len(), default_run_style),
            ("βγ".len(), highlight_run_style),
            ("δ".len(), default_run_style),
            ("ε".len(), highlight_run_style),
            (".ⓐ".len(), default_run_style),
            ("ⓑⓒ".len(), highlight_run_style),
            ("ⓓⓔ.abcde.".len(), default_run_style),
        ]
    );
}

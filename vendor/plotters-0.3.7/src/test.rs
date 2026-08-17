use crate::prelude::*;

#[cfg(feature = "svg_backend")]
#[test]
fn regression_test_issue_267() {
    let p1 = (338, 122);
    let p2 = (365, 122);

    let mut backend = SVGBackend::new("blub.png", (800, 600));

    backend
        .draw_line(p1, p2, &RGBColor(0, 0, 0).stroke_width(0))
        .unwrap();
}

#[test]
fn from_trait_impl_rgba_color() {
    let rgb = RGBColor(1, 2, 3);
    let c = RGBAColor::from(rgb);

    assert_eq!(c.rgb(), rgb.rgb());
}

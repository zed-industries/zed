use crate::{convert, Unit::*};
use ttf_parser::colr::{self, ClipBox, CompositeMode, GradientExtend, Paint, Painter};
use ttf_parser::{cpal, GlyphId, RgbaColor};

#[test]
fn basic() {
    let cpal_data = convert(&[
        UInt16(0),  // version
        UInt16(3),  // number of palette entries
        UInt16(1),  // number of palettes
        UInt16(3),  // number of colors
        UInt32(14), // offset to colors
        UInt16(0),  // index of palette 0's first color
        UInt8(10), UInt8(15), UInt8(20), UInt8(25), // color 0
        UInt8(30), UInt8(35), UInt8(40), UInt8(45), // color 1
        UInt8(50), UInt8(55), UInt8(60), UInt8(65), // color 2
    ]);

    let colr_data = convert(&[
        UInt16(0),  // version
        UInt16(3),  // number of base glyphs
        UInt32(14), // offset to base glyphs
        UInt32(32), // offset to layers
        UInt16(4),  // number of layers
        UInt16(2), UInt16(2), UInt16(2), // base glyph 0 (id 2)
        UInt16(3), UInt16(0), UInt16(3), // base glyph 1 (id 3)
        UInt16(7), UInt16(1), UInt16(1), // base glyph 2 (id 7)
        UInt16(10), UInt16(2), // layer 0
        UInt16(11), UInt16(1), // layer 1
        UInt16(12), UInt16(2), // layer 2
        UInt16(13), UInt16(0), // layer 3
    ]);

    let cpal = cpal::Table::parse(&cpal_data).unwrap();
    let colr = colr::Table::parse(cpal, &colr_data).unwrap();
    let paint = |id| {
        let mut painter = VecPainter(vec![]);
        colr.paint(GlyphId(id), 0, &mut painter, &[], RgbaColor::new(0, 0, 0, 255)).map(|_| painter.0)
    };

    let a = RgbaColor::new(20, 15, 10, 25);
    let b = RgbaColor::new(40, 35, 30, 45);
    let c = RgbaColor::new(60, 55, 50, 65);

    assert_eq!(cpal.get(0, 0), Some(a));
    assert_eq!(cpal.get(0, 1), Some(b));
    assert_eq!(cpal.get(0, 2), Some(c));
    assert_eq!(cpal.get(0, 3), None);
    assert_eq!(cpal.get(1, 0), None);

    assert!(!colr.contains(GlyphId(1)));
    assert!(colr.contains(GlyphId(2)));
    assert!(colr.contains(GlyphId(3)));
    assert!(!colr.contains(GlyphId(4)));
    assert!(!colr.contains(GlyphId(5)));
    assert!(!colr.contains(GlyphId(6)));
    assert!(colr.contains(GlyphId(7)));

    let a = CustomPaint::Solid(a);
    let b = CustomPaint::Solid(b);
    let c = CustomPaint::Solid(c);

    assert_eq!(paint(1), None);

    assert_eq!(
        paint(2).unwrap(), vec![
            Command::OutlineGlyph(GlyphId(12)),
            Command::Paint(c.clone()),
            Command::OutlineGlyph(GlyphId(13)),
            Command::Paint(a.clone())]
    );

    assert_eq!(paint(3).unwrap(), vec![
        Command::OutlineGlyph(GlyphId(10)),
        Command::Paint(c.clone()),
        Command::OutlineGlyph(GlyphId(11)),
        Command::Paint(b.clone()),
        Command::OutlineGlyph(GlyphId(12)),
        Command::Paint(c.clone()),
    ]);

    assert_eq!(paint(7).unwrap(), vec![
        Command::OutlineGlyph(GlyphId(11)),
        Command::Paint(b.clone()),
    ]);
}

#[derive(Clone, Debug, PartialEq)]
struct CustomStop(f32, RgbaColor);

#[derive(Clone, Debug, PartialEq)]
enum CustomPaint {
    Solid(RgbaColor),
    LinearGradient(f32, f32, f32, f32, f32, f32, GradientExtend, Vec<CustomStop>),
    RadialGradient(f32, f32, f32, f32, f32, f32, GradientExtend, Vec<CustomStop>),
    SweepGradient(f32, f32, f32, f32, GradientExtend, Vec<CustomStop>),
}

#[derive(Clone, Debug, PartialEq)]
enum Command {
    OutlineGlyph(GlyphId),
    Paint(CustomPaint),
    PushLayer(CompositeMode),
    PopLayer,
    Transform(ttf_parser::Transform),
    PopTransform,
    PushClip,
    PushClipBox(ClipBox),
    PopClip,
}

struct VecPainter(Vec<Command>);

impl<'a> Painter<'a> for VecPainter {
    fn outline_glyph(&mut self, glyph_id: GlyphId) {
        self.0.push(Command::OutlineGlyph(glyph_id));
    }

    fn paint(&mut self, paint: Paint<'a>) {
        let custom_paint = match paint {
            Paint::Solid(color) => CustomPaint::Solid(color),
            Paint::LinearGradient(lg) => CustomPaint::LinearGradient(lg.x0, lg.y0,
                                                                     lg.x1, lg.y1,
                                                                     lg.x2, lg.y2,
                                                                     lg.extend, lg.stops(0, &[]).map(|stop| CustomStop(stop.stop_offset, stop.color)).collect()),
            Paint::RadialGradient(rg) => CustomPaint::RadialGradient(rg.x0, rg.y0,
                                                                     rg.r0, rg.r1,
                                                                     rg.x1, rg.y1,
                                                                     rg.extend, rg.stops(0, &[]).map(|stop| CustomStop(stop.stop_offset, stop.color)).collect()),
            Paint::SweepGradient(sg) => CustomPaint::SweepGradient(sg.center_x, sg.center_y,
                                                                   sg.start_angle, sg.end_angle,
                                                                   sg.extend, sg.stops(0, &[]).map(|stop| CustomStop(stop.stop_offset, stop.color)).collect()),
        };

        self.0.push(Command::Paint(custom_paint));
    }

    fn push_layer(&mut self, mode: colr::CompositeMode) {
        self.0.push(Command::PushLayer(mode));
    }

    fn pop_layer(&mut self) {
        self.0.push(Command::PopLayer)
    }

    fn push_transform(&mut self, transform: ttf_parser::Transform) {
        self.0.push(Command::Transform(transform))
    }

    fn pop_transform(&mut self) {
        self.0.push(Command::PopTransform)
    }

    fn push_clip(&mut self) {
        self.0.push(Command::PushClip)
    }

    fn push_clip_box(&mut self, clipbox: ClipBox) {
        self.0.push(Command::PushClipBox(clipbox))
    }

    fn pop_clip(&mut self) {
        self.0.push(Command::PopClip)
    }
}

// A static and variable COLRv1 test font from Google Fonts:
// https://github.com/googlefonts/color-fonts
static COLR1_STATIC: &[u8] = include_bytes!("../fonts/colr_1.ttf");
static COLR1_VARIABLE: &[u8] = include_bytes!("../fonts/colr_1_variable.ttf");

mod colr1_static {
    use ttf_parser::{Face, GlyphId, RgbaColor};
    use ttf_parser::colr::ClipBox;
    use ttf_parser::colr::CompositeMode::*;
    use ttf_parser::colr::GradientExtend::*;
    use crate::colr::{COLR1_STATIC, Command, CustomStop, VecPainter};
    use crate::colr::Command::*;
    use crate::colr::CustomPaint::*;

    #[test]
    fn linear_gradient() {
        let face = Face::parse(COLR1_STATIC, 0).unwrap();
        let mut vec_painter = VecPainter(vec![]);
        face.paint_color_glyph(GlyphId(9), 0, RgbaColor::new(0, 0, 0, 255), &mut vec_painter);
        assert_eq!(vec_painter.0, vec![
            PushClipBox(ClipBox { x_min: 100.0, y_min: 250.0, x_max: 900.0, y_max: 950.0 }),
            OutlineGlyph(GlyphId(9)),
            PushClip,
            Paint(LinearGradient(100.0, 250.0, 900.0, 250.0, 100.0, 300.0, Repeat, vec![
                CustomStop(0.2000122, RgbaColor { red: 255, green: 0, blue: 0, alpha: 255 }),
                CustomStop(0.7999878, RgbaColor { red: 0, green: 0, blue: 255, alpha: 255 })])),
            PopClip,
            PopClip]
        )
    }

    #[test]
    fn sweep_gradient() {
        let face = Face::parse(COLR1_STATIC, 0).unwrap();
        let mut vec_painter = VecPainter(vec![]);
        face.paint_color_glyph(GlyphId(13), 0, RgbaColor::new(0, 0, 0, 255), &mut vec_painter);
        assert_eq!(vec_painter.0, vec![
            PushClipBox(ClipBox { x_min: 0.0, y_min: 0.0, x_max: 1000.0, y_max: 1000.0 }),
            OutlineGlyph(GlyphId(176)),
            PushClip,
            Paint(SweepGradient(500.0, 600.0, -0.666687, 0.666687, Pad, vec![
                CustomStop(0.25, RgbaColor { red: 250, green: 240, blue: 230, alpha: 255 }),
                CustomStop(0.416687, RgbaColor { red: 0, green: 0, blue: 255, alpha: 255 }),
                CustomStop(0.583313, RgbaColor { red: 255, green: 0, blue: 0, alpha: 255 }),
                CustomStop(0.75, RgbaColor { red: 47, green: 79, blue: 79, alpha: 255 })])),
            PopClip,
            PopClip]
        )
    }

    #[test]
    fn scale_around_center() {
        let face = Face::parse(COLR1_STATIC, 0).unwrap();
        let mut vec_painter = VecPainter(vec![]);
        face.paint_color_glyph(GlyphId(84), 0, RgbaColor::new(0, 0, 0, 255), &mut vec_painter);
        assert_eq!(vec_painter.0, vec![
            PushLayer(SourceOver),
            OutlineGlyph(GlyphId(3)),
            PushClip,
            Paint(Solid(RgbaColor { red: 0, green: 0, blue: 255, alpha: 127 })),
            PopClip,
            PushLayer(DestinationOver),
            Transform(ttf_parser::Transform::new_translate(500.0, 500.0)),
            Transform(ttf_parser::Transform::new_scale(0.5, 1.5)),
            Transform(ttf_parser::Transform::new_translate(-500.0, -500.0)),
            OutlineGlyph(
                GlyphId(3)),
            PushClip,
            Paint(Solid(RgbaColor { red: 255, green: 165, blue: 0, alpha: 178 })),
            PopClip,
            PopTransform,
            PopTransform,
            PopTransform,
            PopLayer,
            PopLayer]
        )
    }

    #[test]
    fn scale() {
        let face = Face::parse(COLR1_STATIC, 0).unwrap();
        let mut vec_painter = VecPainter(vec![]);
        face.paint_color_glyph(GlyphId(86), 0, RgbaColor::new(0, 0, 0, 255), &mut vec_painter);
        assert!(vec_painter.0.contains(&Transform(ttf_parser::Transform::new_scale(0.5, 1.5))))
    }

    #[test]
    fn radial_gradient() {
        let face = Face::parse(COLR1_STATIC, 0).unwrap();
        let mut vec_painter = VecPainter(vec![]);
        face.paint_color_glyph(GlyphId(93), 0, RgbaColor::new(0, 0, 0, 255), &mut vec_painter);
        assert_eq!(vec_painter.0, vec![
            PushClipBox(ClipBox { x_min: 0.0, y_min: 0.0, x_max: 1000.0, y_max: 1000.0 }),
            OutlineGlyph(GlyphId(2)),
            PushClip,
            Paint(RadialGradient(166.0, 768.0, 0.0, 256.0, 166.0, 768.0, Pad, vec![
                CustomStop(0.0, RgbaColor { red: 0, green: 128, blue: 0, alpha: 255 }),
                CustomStop(0.5, RgbaColor { red: 255, green: 255, blue: 255, alpha: 255 }),
                CustomStop(1.0, RgbaColor { red: 255, green: 0, blue: 0, alpha: 255 })])),
            PopClip,
            PopClip]
        )
    }

    #[test]
    fn rotate() {
        let face = Face::parse(COLR1_STATIC, 0).unwrap();
        let mut vec_painter = VecPainter(vec![]);
        face.paint_color_glyph(GlyphId(99), 0, RgbaColor::new(0, 0, 0, 255), &mut vec_painter);
        assert!(vec_painter.0.contains(&Transform(ttf_parser::Transform::new_rotate(0.055541992))))
    }

    #[test]
    fn rotate_around_center() {
        let face = Face::parse(COLR1_STATIC, 0).unwrap();
        let mut vec_painter = VecPainter(vec![]);
        face.paint_color_glyph(GlyphId(101), 0, RgbaColor::new(0, 0, 0, 255), &mut vec_painter);
        assert_eq!(vec_painter.0, vec![
            PushLayer(SourceOver),
            OutlineGlyph(GlyphId(3)),
            PushClip,
            Paint(Solid(RgbaColor { red: 0, green: 0, blue: 255, alpha: 127 })),
            PopClip,
            PushLayer(DestinationOver),
            Transform(ttf_parser::Transform::new_translate(500.0, 500.0)),
            Transform(ttf_parser::Transform::new_rotate(0.13891602)),
            Transform(ttf_parser::Transform::new_translate(-500.0, -500.0)),
            OutlineGlyph(GlyphId(3)),
            PushClip,
            Paint(Solid(RgbaColor { red: 255, green: 165, blue: 0, alpha: 178 })),
            PopClip,
            PopTransform,
            PopTransform,
            PopTransform,
            PopLayer,
            PopLayer,
        ]
        )
    }

    #[test]
    fn skew() {
        let face = Face::parse(COLR1_STATIC, 0).unwrap();
        let mut vec_painter = VecPainter(vec![]);
        face.paint_color_glyph(GlyphId(103), 0, RgbaColor::new(0, 0, 0, 255), &mut vec_painter);
        assert!(vec_painter.0.contains(&Transform(ttf_parser::Transform::new_skew(0.13891602, 0.0))));
    }

    #[test]
    fn skew_around_center() {
        let face = Face::parse(COLR1_STATIC, 0).unwrap();
        let mut vec_painter = VecPainter(vec![]);
        face.paint_color_glyph(GlyphId(104), 0, RgbaColor::new(0, 0, 0, 255), &mut vec_painter);
        assert_eq!(vec_painter.0, vec![
            PushLayer(SourceOver),
            OutlineGlyph(GlyphId(3)),
            PushClip,
            Paint(Solid(RgbaColor { red: 0, green: 0, blue: 255, alpha: 127 })),
            PopClip,
            PushLayer(DestinationOver),
            Transform(ttf_parser::Transform::new_translate(500.0, 500.0)),
            Transform(ttf_parser::Transform::new_skew(0.13891602, 0.0)),
            Transform(ttf_parser::Transform::new_translate(-500.0, -500.0)),
            OutlineGlyph(GlyphId(3)),
            PushClip,
            Paint(Solid(RgbaColor { red: 255, green: 165, blue: 0, alpha: 178 })),
            PopClip,
            PopTransform,
            PopTransform,
            PopTransform,
            PopLayer,
            PopLayer])
    }

    #[test]
    fn transform() {
        let face = Face::parse(COLR1_STATIC, 0).unwrap();
        let mut vec_painter = VecPainter(vec![]);
        face.paint_color_glyph(GlyphId(109), 0, RgbaColor::new(0, 0, 0, 255), &mut vec_painter);

        assert!(vec_painter.0.contains(&Transform(ttf_parser::Transform {
                a: 1.0,
                b: 0.0,
                c: 0.0,
                d: 1.0,
                e: 125.0,
                f: 125.0
            }
        )));
    }

    #[test]
    fn translate() {
        let face = Face::parse(COLR1_STATIC, 0).unwrap();
        let mut vec_painter = VecPainter(vec![]);
        face.paint_color_glyph(GlyphId(114), 0, RgbaColor::new(0, 0, 0, 255), &mut vec_painter);

        assert!(vec_painter.0.contains(&Transform(ttf_parser::Transform::new_translate(0.0, 100.0))));
    }

    #[test]
    fn composite() {
        let face = Face::parse(COLR1_STATIC, 0).unwrap();
        let mut vec_painter = VecPainter(vec![]);
        face.paint_color_glyph(GlyphId(131), 0, RgbaColor::new(0, 0, 0, 255), &mut vec_painter);

        assert!(vec_painter.0.contains(&Command::PushLayer(Xor)));
    }

    #[test]
    fn cyclic_dependency() {
        let face = Face::parse(COLR1_STATIC, 0).unwrap();
        let mut vec_painter = VecPainter(vec![]);
        face.paint_color_glyph(GlyphId(179), 0, RgbaColor::new(0, 0, 0, 255), &mut vec_painter);
    }
}


mod colr1_variable {
    use ttf_parser::{Face, GlyphId, RgbaColor, Tag};
    use ttf_parser::colr::ClipBox;
    use ttf_parser::colr::GradientExtend::*;
    use crate::colr::{COLR1_STATIC, COLR1_VARIABLE, CustomStop, VecPainter};
    use crate::colr::Command::*;
    use crate::colr::CustomPaint::*;

    #[test]
    fn sweep_gradient() {
        let mut face = Face::parse(COLR1_VARIABLE, 0).unwrap();
        face.set_variation(Tag::from_bytes(b"SWPS"), 45.0);
        face.set_variation(Tag::from_bytes(b"SWPE"), 58.0);
        let mut vec_painter = VecPainter(vec![]);
        face.paint_color_glyph(GlyphId(13), 0, RgbaColor::new(0, 0, 0, 255), &mut vec_painter);
        assert!(vec_painter.0.contains(&Paint(SweepGradient(500.0, 600.0, -0.416687, 0.9888916, Pad, vec![
            CustomStop(0.25, RgbaColor { red: 250, green: 240, blue: 230, alpha: 255 }),
            CustomStop(0.416687, RgbaColor { red: 0, green: 0, blue: 255, alpha: 255 }),
            CustomStop(0.583313, RgbaColor { red: 255, green: 0, blue: 0, alpha: 255 }),
            CustomStop(0.75, RgbaColor { red: 47, green: 79, blue: 79, alpha: 255 })]))
        ));
    }

    #[test]
    fn scale_around_center() {
        let mut face = Face::parse(COLR1_VARIABLE, 0).unwrap();
        face.set_variation(Tag::from_bytes(b"SCSX"), 1.1);
        face.set_variation(Tag::from_bytes(b"SCSY"), -0.9);
        let mut vec_painter = VecPainter(vec![]);
        face.paint_color_glyph(GlyphId(84), 0, RgbaColor::new(0, 0, 0, 255), &mut vec_painter);
        assert!(vec_painter.0.contains(&Transform(ttf_parser::Transform::new_scale(1.599942, 0.60009766))))
    }

    #[test]
    fn scale() {
        let mut face = Face::parse(COLR1_VARIABLE, 0).unwrap();
        face.set_variation(Tag::from_bytes(b"SCSX"), 1.1);
        face.set_variation(Tag::from_bytes(b"SCSY"), -0.9);
        let mut vec_painter = VecPainter(vec![]);
        face.paint_color_glyph(GlyphId(86), 0, RgbaColor::new(0, 0, 0, 255), &mut vec_painter);
        assert!(vec_painter.0.contains(&Transform(ttf_parser::Transform::new_scale(1.599942, 0.60009766))))
    }

    #[test]
    fn radial_gradient() {
        let face = Face::parse(COLR1_STATIC, 0).unwrap();
        let mut vec_painter = VecPainter(vec![]);
        face.paint_color_glyph(GlyphId(93), 0, RgbaColor::new(0, 0, 0, 255), &mut vec_painter);
        assert_eq!(vec_painter.0, vec![
            PushClipBox(ClipBox { x_min: 0.0, y_min: 0.0, x_max: 1000.0, y_max: 1000.0 }),
            OutlineGlyph(GlyphId(2)),
            PushClip,
            Paint(RadialGradient(166.0, 768.0, 0.0, 256.0, 166.0, 768.0, Pad, vec![
                CustomStop(0.0, RgbaColor { red: 0, green: 128, blue: 0, alpha: 255 }),
                CustomStop(0.5, RgbaColor { red: 255, green: 255, blue: 255, alpha: 255 }),
                CustomStop(1.0, RgbaColor { red: 255, green: 0, blue: 0, alpha: 255 })])),
            PopClip,
            PopClip]
        )
    }

    #[test]
    fn rotate() {
        let mut face = Face::parse(COLR1_VARIABLE, 0).unwrap();
        face.set_variation(Tag::from_bytes(b"ROTA"), 150.0);
        let mut vec_painter = VecPainter(vec![]);
        face.paint_color_glyph(GlyphId(99), 0, RgbaColor::new(0, 0, 0, 255), &mut vec_painter);
        assert!(vec_painter.0.contains(&Transform(ttf_parser::Transform::new_rotate(0.87341005))))
    }

    #[test]
    fn rotate_around_center() {
        let mut face = Face::parse(COLR1_VARIABLE, 0).unwrap();
        face.set_variation(Tag::from_bytes(b"ROTA"), 150.0);
        let mut vec_painter = VecPainter(vec![]);
        face.paint_color_glyph(GlyphId(101), 0, RgbaColor::new(0, 0, 0, 255), &mut vec_painter);
        assert!(vec_painter.0.contains(&Transform(ttf_parser::Transform::new_rotate(0.9336252))))
    }

    #[test]
    fn skew() {
        let mut face = Face::parse(COLR1_VARIABLE, 0).unwrap();
        face.set_variation(Tag::from_bytes(b"SKXA"), 46.0);
        let mut vec_painter = VecPainter(vec![]);
        face.paint_color_glyph(GlyphId(103), 0, RgbaColor::new(0, 0, 0, 255), &mut vec_painter);
        assert!(vec_painter.0.contains(&Transform(ttf_parser::Transform::new_skew(0.3944702, 0.0))));
    }

    #[test]
    fn skew_around_center() {
        let mut face = Face::parse(COLR1_VARIABLE, 0).unwrap();
        face.set_variation(Tag::from_bytes(b"SKXA"), 46.0);
        let mut vec_painter = VecPainter(vec![]);
        face.paint_color_glyph(GlyphId(104), 0, RgbaColor::new(0, 0, 0, 255), &mut vec_painter);
        assert!(vec_painter.0.contains(&Transform(ttf_parser::Transform::new_skew(0.3944702, 0.0))));
    }

    #[test]
    fn transform() {
        let mut face = Face::parse(COLR1_VARIABLE, 0).unwrap();
        face.set_variation(Tag::from_bytes(b"TRDX"), 150.0);
        let mut vec_painter = VecPainter(vec![]);
        face.paint_color_glyph(GlyphId(109), 0, RgbaColor::new(0, 0, 0, 255), &mut vec_painter);

        assert!(vec_painter.0.contains(&Transform(ttf_parser::Transform {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
            e: 274.9939,
            f: 125.0
        }
        )));
    }

    #[test]
    fn translate() {
        let mut face = Face::parse(COLR1_VARIABLE, 0).unwrap();
        face.set_variation(Tag::from_bytes(b"TLDX"), 100.0);
        let mut vec_painter = VecPainter(vec![]);
        face.paint_color_glyph(GlyphId(114), 0, RgbaColor::new(0, 0, 0, 255), &mut vec_painter);

        assert!(vec_painter.0.contains(&Transform(ttf_parser::Transform::new_translate(99.975586, 100.0))));
    }
}

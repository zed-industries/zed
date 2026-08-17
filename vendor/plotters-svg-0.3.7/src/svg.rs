/*!
The SVG image drawing backend
*/

use plotters_backend::{
    text_anchor::{HPos, VPos},
    BackendColor, BackendCoord, BackendStyle, BackendTextStyle, DrawingBackend, DrawingErrorKind,
    FontStyle, FontTransform,
};

use std::fmt::Write as _;
use std::fs::File;
#[allow(unused_imports)]
use std::io::Cursor;
use std::io::{BufWriter, Error, Write};
use std::path::Path;

fn make_svg_color(color: BackendColor) -> String {
    let (r, g, b) = color.rgb;
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

fn make_svg_opacity(color: BackendColor) -> String {
    format!("{}", color.alpha)
}

enum Target<'a> {
    File(String, &'a Path),
    Buffer(&'a mut String),
}

impl Target<'_> {
    fn get_mut(&mut self) -> &mut String {
        match self {
            Target::File(ref mut buf, _) => buf,
            Target::Buffer(buf) => buf,
        }
    }
}

enum SVGTag {
    Svg,
    Circle,
    Line,
    Polygon,
    Polyline,
    Rectangle,
    Text,
    #[allow(dead_code)]
    Image,
}

impl SVGTag {
    fn to_tag_name(&self) -> &'static str {
        match self {
            SVGTag::Svg => "svg",
            SVGTag::Circle => "circle",
            SVGTag::Line => "line",
            SVGTag::Polyline => "polyline",
            SVGTag::Rectangle => "rect",
            SVGTag::Text => "text",
            SVGTag::Image => "image",
            SVGTag::Polygon => "polygon",
        }
    }
}

/// The SVG image drawing backend
pub struct SVGBackend<'a> {
    target: Target<'a>,
    size: (u32, u32),
    tag_stack: Vec<SVGTag>,
    saved: bool,
}

impl<'a> SVGBackend<'a> {
    fn escape_and_push(buf: &mut String, value: &str) {
        value.chars().for_each(|c| match c {
            '<' => buf.push_str("&lt;"),
            '>' => buf.push_str("&gt;"),
            '&' => buf.push_str("&amp;"),
            '"' => buf.push_str("&quot;"),
            '\'' => buf.push_str("&apos;"),
            other => buf.push(other),
        });
    }
    fn open_tag(&mut self, tag: SVGTag, attr: &[(&str, &str)], close: bool) {
        let buf = self.target.get_mut();
        buf.push('<');
        buf.push_str(tag.to_tag_name());
        for (key, value) in attr {
            buf.push(' ');
            buf.push_str(key);
            buf.push_str("=\"");
            Self::escape_and_push(buf, value);
            buf.push('\"');
        }
        if close {
            buf.push_str("/>\n");
        } else {
            self.tag_stack.push(tag);
            buf.push_str(">\n");
        }
    }

    fn close_tag(&mut self) -> bool {
        if let Some(tag) = self.tag_stack.pop() {
            let buf = self.target.get_mut();
            buf.push_str("</");
            buf.push_str(tag.to_tag_name());
            buf.push_str(">\n");
            return true;
        }
        false
    }

    fn init_svg_file(&mut self, size: (u32, u32)) {
        self.open_tag(
            SVGTag::Svg,
            &[
                ("width", &format!("{}", size.0)),
                ("height", &format!("{}", size.1)),
                ("viewBox", &format!("0 0 {} {}", size.0, size.1)),
                ("xmlns", "http://www.w3.org/2000/svg"),
            ],
            false,
        );
    }

    /// Create a new SVG drawing backend
    pub fn new<T: AsRef<Path> + ?Sized>(path: &'a T, size: (u32, u32)) -> Self {
        let mut ret = Self {
            target: Target::File(String::default(), path.as_ref()),
            size,
            tag_stack: vec![],
            saved: false,
        };

        ret.init_svg_file(size);
        ret
    }

    /// Create a new SVG drawing backend and store the document into a String buffer
    pub fn with_string(buf: &'a mut String, size: (u32, u32)) -> Self {
        let mut ret = Self {
            target: Target::Buffer(buf),
            size,
            tag_stack: vec![],
            saved: false,
        };

        ret.init_svg_file(size);

        ret
    }
}

impl<'a> DrawingBackend for SVGBackend<'a> {
    type ErrorType = Error;

    fn get_size(&self) -> (u32, u32) {
        self.size
    }

    fn ensure_prepared(&mut self) -> Result<(), DrawingErrorKind<Error>> {
        Ok(())
    }

    fn present(&mut self) -> Result<(), DrawingErrorKind<Error>> {
        if !self.saved {
            while self.close_tag() {}
            match self.target {
                Target::File(ref buf, path) => {
                    let outfile = File::create(path).map_err(DrawingErrorKind::DrawingError)?;
                    let mut outfile = BufWriter::new(outfile);
                    outfile
                        .write_all(buf.as_ref())
                        .map_err(DrawingErrorKind::DrawingError)?;
                }
                Target::Buffer(_) => {}
            }
            self.saved = true;
        }
        Ok(())
    }

    fn draw_pixel(
        &mut self,
        point: BackendCoord,
        color: BackendColor,
    ) -> Result<(), DrawingErrorKind<Error>> {
        if color.alpha == 0.0 {
            return Ok(());
        }
        self.open_tag(
            SVGTag::Rectangle,
            &[
                ("x", &format!("{}", point.0)),
                ("y", &format!("{}", point.1)),
                ("width", "1"),
                ("height", "1"),
                ("stroke", "none"),
                ("opacity", &make_svg_opacity(color)),
                ("fill", &make_svg_color(color)),
            ],
            true,
        );
        Ok(())
    }

    fn draw_line<S: BackendStyle>(
        &mut self,
        from: BackendCoord,
        to: BackendCoord,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        if style.color().alpha == 0.0 {
            return Ok(());
        }
        self.open_tag(
            SVGTag::Line,
            &[
                ("opacity", &make_svg_opacity(style.color())),
                ("stroke", &make_svg_color(style.color())),
                ("stroke-width", &format!("{}", style.stroke_width())),
                ("x1", &format!("{}", from.0)),
                ("y1", &format!("{}", from.1)),
                ("x2", &format!("{}", to.0)),
                ("y2", &format!("{}", to.1)),
            ],
            true,
        );
        Ok(())
    }

    fn draw_rect<S: BackendStyle>(
        &mut self,
        upper_left: BackendCoord,
        bottom_right: BackendCoord,
        style: &S,
        fill: bool,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        if style.color().alpha == 0.0 {
            return Ok(());
        }

        let (fill, stroke) = if !fill {
            ("none".to_string(), make_svg_color(style.color()))
        } else {
            (make_svg_color(style.color()), "none".to_string())
        };

        self.open_tag(
            SVGTag::Rectangle,
            &[
                ("x", &format!("{}", upper_left.0)),
                ("y", &format!("{}", upper_left.1)),
                ("width", &format!("{}", bottom_right.0 - upper_left.0)),
                ("height", &format!("{}", bottom_right.1 - upper_left.1)),
                ("opacity", &make_svg_opacity(style.color())),
                ("fill", &fill),
                ("stroke", &stroke),
            ],
            true,
        );

        Ok(())
    }

    fn draw_path<S: BackendStyle, I: IntoIterator<Item = BackendCoord>>(
        &mut self,
        path: I,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        if style.color().alpha == 0.0 {
            return Ok(());
        }
        self.open_tag(
            SVGTag::Polyline,
            &[
                ("fill", "none"),
                ("opacity", &make_svg_opacity(style.color())),
                ("stroke", &make_svg_color(style.color())),
                ("stroke-width", &format!("{}", style.stroke_width())),
                (
                    "points",
                    &path.into_iter().fold(String::new(), |mut s, (x, y)| {
                        write!(s, "{},{} ", x, y).ok();
                        s
                    }),
                ),
            ],
            true,
        );
        Ok(())
    }

    fn fill_polygon<S: BackendStyle, I: IntoIterator<Item = BackendCoord>>(
        &mut self,
        path: I,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        if style.color().alpha == 0.0 {
            return Ok(());
        }
        self.open_tag(
            SVGTag::Polygon,
            &[
                ("opacity", &make_svg_opacity(style.color())),
                ("fill", &make_svg_color(style.color())),
                (
                    "points",
                    &path.into_iter().fold(String::new(), |mut s, (x, y)| {
                        write!(s, "{},{} ", x, y).ok();
                        s
                    }),
                ),
            ],
            true,
        );
        Ok(())
    }

    fn draw_circle<S: BackendStyle>(
        &mut self,
        center: BackendCoord,
        radius: u32,
        style: &S,
        fill: bool,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        if style.color().alpha == 0.0 {
            return Ok(());
        }
        let (stroke, fill) = if !fill {
            (make_svg_color(style.color()), "none".to_string())
        } else {
            ("none".to_string(), make_svg_color(style.color()))
        };
        self.open_tag(
            SVGTag::Circle,
            &[
                ("cx", &format!("{}", center.0)),
                ("cy", &format!("{}", center.1)),
                ("r", &format!("{}", radius)),
                ("opacity", &make_svg_opacity(style.color())),
                ("fill", &fill),
                ("stroke", &stroke),
                ("stroke-width", &format!("{}", style.stroke_width())),
            ],
            true,
        );
        Ok(())
    }

    fn draw_text<S: BackendTextStyle>(
        &mut self,
        text: &str,
        style: &S,
        pos: BackendCoord,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let color = style.color();
        if color.alpha == 0.0 {
            return Ok(());
        }

        let (x0, y0) = pos;
        let text_anchor = match style.anchor().h_pos {
            HPos::Left => "start",
            HPos::Right => "end",
            HPos::Center => "middle",
        };

        let dy = match style.anchor().v_pos {
            VPos::Top => "0.76em",
            VPos::Center => "0.5ex",
            VPos::Bottom => "-0.5ex",
        };

        #[cfg(feature = "debug")]
        {
            let ((fx0, fy0), (fx1, fy1)) =
                font.layout_box(text).map_err(DrawingErrorKind::FontError)?;
            let x0 = match style.anchor().h_pos {
                HPos::Left => x0,
                HPos::Center => x0 - fx1 / 2 + fx0 / 2,
                HPos::Right => x0 - fx1 + fx0,
            };
            let y0 = match style.anchor().v_pos {
                VPos::Top => y0,
                VPos::Center => y0 - fy1 / 2 + fy0 / 2,
                VPos::Bottom => y0 - fy1 + fy0,
            };
            self.draw_rect(
                (x0, y0),
                (x0 + fx1 - fx0, y0 + fy1 - fy0),
                &crate::prelude::RED,
                false,
            )
            .unwrap();
            self.draw_circle((x0, y0), 2, &crate::prelude::RED, false)
                .unwrap();
        }

        let mut attrs = vec![
            ("x", format!("{}", x0)),
            ("y", format!("{}", y0)),
            ("dy", dy.to_owned()),
            ("text-anchor", text_anchor.to_string()),
            ("font-family", style.family().as_str().to_string()),
            ("font-size", format!("{}", style.size() / 1.24)),
            ("opacity", make_svg_opacity(color)),
            ("fill", make_svg_color(color)),
        ];

        match style.style() {
            FontStyle::Normal => {}
            FontStyle::Bold => attrs.push(("font-weight", "bold".to_string())),
            other_style => attrs.push(("font-style", other_style.as_str().to_string())),
        };

        let trans = style.transform();
        match trans {
            FontTransform::Rotate90 => {
                attrs.push(("transform", format!("rotate(90, {}, {})", x0, y0)))
            }
            FontTransform::Rotate180 => {
                attrs.push(("transform", format!("rotate(180, {}, {})", x0, y0)));
            }
            FontTransform::Rotate270 => {
                attrs.push(("transform", format!("rotate(270, {}, {})", x0, y0)));
            }
            _ => {}
        }

        self.open_tag(
            SVGTag::Text,
            attrs
                .iter()
                .map(|(a, b)| (*a, b.as_ref()))
                .collect::<Vec<_>>()
                .as_ref(),
            false,
        );

        Self::escape_and_push(self.target.get_mut(), text);
        self.target.get_mut().push('\n');

        self.close_tag();

        Ok(())
    }

    #[cfg(all(not(target_arch = "wasm32"), feature = "image"))]
    fn blit_bitmap(
        &mut self,
        pos: BackendCoord,
        (w, h): (u32, u32),
        src: &[u8],
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        use image::codecs::png::PngEncoder;
        use image::ImageEncoder;

        let mut data = vec![0; 0];

        {
            let cursor = Cursor::new(&mut data);

            let encoder = PngEncoder::new(cursor);

            let color = image::ColorType::Rgb8;

            encoder.write_image(src, w, h, color).map_err(|e| {
                DrawingErrorKind::DrawingError(Error::new(
                    std::io::ErrorKind::Other,
                    format!("Image error: {}", e),
                ))
            })?;
        }

        let padding = (3 - data.len() % 3) % 3;
        data.resize(data.len() + padding, 0);

        let mut rem_bits = 0;
        let mut rem_num = 0;

        fn cvt_base64(from: u8) -> char {
            (if from < 26 {
                b'A' + from
            } else if from < 52 {
                b'a' + from - 26
            } else if from < 62 {
                b'0' + from - 52
            } else if from == 62 {
                b'+'
            } else {
                b'/'
            })
            .into()
        }

        let mut buf = String::new();
        buf.push_str("data:png;base64,");

        for byte in data {
            let value = (rem_bits << (6 - rem_num)) | (byte >> (rem_num + 2));
            rem_bits = byte & ((1 << (2 + rem_num)) - 1);
            rem_num += 2;

            buf.push(cvt_base64(value));
            if rem_num == 6 {
                buf.push(cvt_base64(rem_bits));
                rem_bits = 0;
                rem_num = 0;
            }
        }

        for _ in 0..padding {
            buf.pop();
            buf.push('=');
        }

        self.open_tag(
            SVGTag::Image,
            &[
                ("x", &format!("{}", pos.0)),
                ("y", &format!("{}", pos.1)),
                ("width", &format!("{}", w)),
                ("height", &format!("{}", h)),
                ("href", buf.as_str()),
            ],
            true,
        );

        Ok(())
    }
}

impl Drop for SVGBackend<'_> {
    fn drop(&mut self) {
        if !self.saved {
            // drop should not panic, so we ignore a failed present
            let _ = self.present();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use plotters::element::Circle;
    use plotters::prelude::{
        ChartBuilder, Color, IntoDrawingArea, IntoFont, SeriesLabelPosition, TextStyle, BLACK,
        BLUE, RED, WHITE,
    };
    use plotters::style::text_anchor::{HPos, Pos, VPos};
    use std::fs;
    use std::path::Path;

    static DST_DIR: &str = "target/test/svg";

    fn checked_save_file(name: &str, content: &str) {
        /*
          Please use the SVG file to manually verify the results.
        */
        assert!(!content.is_empty());
        fs::create_dir_all(DST_DIR).unwrap();
        let file_name = format!("{}.svg", name);
        let file_path = Path::new(DST_DIR).join(file_name);
        println!("{:?} created", file_path);
        fs::write(file_path, &content).unwrap();
    }

    fn draw_mesh_with_custom_ticks(tick_size: i32, test_name: &str) {
        let mut content: String = Default::default();
        {
            let root = SVGBackend::with_string(&mut content, (500, 500)).into_drawing_area();

            let mut chart = ChartBuilder::on(&root)
                .caption("This is a test", ("sans-serif", 20u32))
                .set_all_label_area_size(40u32)
                .build_cartesian_2d(0..10, 0..10)
                .unwrap();

            chart
                .configure_mesh()
                .set_all_tick_mark_size(tick_size)
                .draw()
                .unwrap();
        }

        checked_save_file(test_name, &content);

        assert!(content.contains("This is a test"));
    }

    #[test]
    fn test_draw_mesh_no_ticks() {
        draw_mesh_with_custom_ticks(0, "test_draw_mesh_no_ticks");
    }

    #[test]
    fn test_draw_mesh_negative_ticks() {
        draw_mesh_with_custom_ticks(-10, "test_draw_mesh_negative_ticks");
    }

    #[test]
    fn test_text_alignments() {
        let mut content: String = Default::default();
        {
            let mut root = SVGBackend::with_string(&mut content, (500, 500));

            let style = TextStyle::from(("sans-serif", 20).into_font())
                .pos(Pos::new(HPos::Right, VPos::Top));
            root.draw_text("right-align", &style, (150, 50)).unwrap();

            let style = style.pos(Pos::new(HPos::Center, VPos::Top));
            root.draw_text("center-align", &style, (150, 150)).unwrap();

            let style = style.pos(Pos::new(HPos::Left, VPos::Top));
            root.draw_text("left-align", &style, (150, 200)).unwrap();
        }

        checked_save_file("test_text_alignments", &content);

        for svg_line in content.split("</text>") {
            if let Some(anchor_and_rest) = svg_line.split("text-anchor=\"").nth(1) {
                if anchor_and_rest.starts_with("end") {
                    assert!(anchor_and_rest.contains("right-align"))
                }
                if anchor_and_rest.starts_with("middle") {
                    assert!(anchor_and_rest.contains("center-align"))
                }
                if anchor_and_rest.starts_with("start") {
                    assert!(anchor_and_rest.contains("left-align"))
                }
            }
        }
    }

    #[test]
    fn test_text_draw() {
        let mut content: String = Default::default();
        {
            let root = SVGBackend::with_string(&mut content, (1500, 800)).into_drawing_area();
            let root = root
                .titled("Image Title", ("sans-serif", 60).into_font())
                .unwrap();

            let mut chart = ChartBuilder::on(&root)
                .caption("All anchor point positions", ("sans-serif", 20u32))
                .set_all_label_area_size(40u32)
                .build_cartesian_2d(0..100i32, 0..50i32)
                .unwrap();

            chart
                .configure_mesh()
                .disable_x_mesh()
                .disable_y_mesh()
                .x_desc("X Axis")
                .y_desc("Y Axis")
                .draw()
                .unwrap();

            let ((x1, y1), (x2, y2), (x3, y3)) = ((-30, 30), (0, -30), (30, 30));

            for (dy, trans) in [
                FontTransform::None,
                FontTransform::Rotate90,
                FontTransform::Rotate180,
                FontTransform::Rotate270,
            ]
            .iter()
            .enumerate()
            {
                for (dx1, h_pos) in [HPos::Left, HPos::Right, HPos::Center].iter().enumerate() {
                    for (dx2, v_pos) in [VPos::Top, VPos::Center, VPos::Bottom].iter().enumerate() {
                        let x = 150_i32 + (dx1 as i32 * 3 + dx2 as i32) * 150;
                        let y = 120 + dy as i32 * 150;
                        let draw = |x, y, text| {
                            root.draw(&Circle::new((x, y), 3, &BLACK.mix(0.5))).unwrap();
                            let style = TextStyle::from(("sans-serif", 20).into_font())
                                .pos(Pos::new(*h_pos, *v_pos))
                                .transform(trans.clone());
                            root.draw_text(text, &style, (x, y)).unwrap();
                        };
                        draw(x + x1, y + y1, "dood");
                        draw(x + x2, y + y2, "dog");
                        draw(x + x3, y + y3, "goog");
                    }
                }
            }
        }

        checked_save_file("test_text_draw", &content);

        assert_eq!(content.matches("dog").count(), 36);
        assert_eq!(content.matches("dood").count(), 36);
        assert_eq!(content.matches("goog").count(), 36);
    }

    #[test]
    fn test_text_clipping() {
        let mut content: String = Default::default();
        {
            let (width, height) = (500_i32, 500_i32);
            let root = SVGBackend::with_string(&mut content, (width as u32, height as u32))
                .into_drawing_area();

            let style = TextStyle::from(("sans-serif", 20).into_font())
                .pos(Pos::new(HPos::Center, VPos::Center));
            root.draw_text("TOP LEFT", &style, (0, 0)).unwrap();
            root.draw_text("TOP CENTER", &style, (width / 2, 0))
                .unwrap();
            root.draw_text("TOP RIGHT", &style, (width, 0)).unwrap();

            root.draw_text("MIDDLE LEFT", &style, (0, height / 2))
                .unwrap();
            root.draw_text("MIDDLE RIGHT", &style, (width, height / 2))
                .unwrap();

            root.draw_text("BOTTOM LEFT", &style, (0, height)).unwrap();
            root.draw_text("BOTTOM CENTER", &style, (width / 2, height))
                .unwrap();
            root.draw_text("BOTTOM RIGHT", &style, (width, height))
                .unwrap();
        }

        checked_save_file("test_text_clipping", &content);
    }

    #[test]
    fn test_series_labels() {
        let mut content = String::default();
        {
            let (width, height) = (500, 500);
            let root = SVGBackend::with_string(&mut content, (width, height)).into_drawing_area();

            let mut chart = ChartBuilder::on(&root)
                .caption("All series label positions", ("sans-serif", 20u32))
                .set_all_label_area_size(40u32)
                .build_cartesian_2d(0..50i32, 0..50i32)
                .unwrap();

            chart
                .configure_mesh()
                .disable_x_mesh()
                .disable_y_mesh()
                .draw()
                .unwrap();

            chart
                .draw_series(std::iter::once(Circle::new((5, 15), 5u32, &RED)))
                .expect("Drawing error")
                .label("Series 1")
                .legend(|(x, y)| Circle::new((x, y), 3u32, RED.filled()));

            chart
                .draw_series(std::iter::once(Circle::new((5, 15), 10u32, &BLUE)))
                .expect("Drawing error")
                .label("Series 2")
                .legend(|(x, y)| Circle::new((x, y), 3u32, BLUE.filled()));

            for pos in vec![
                SeriesLabelPosition::UpperLeft,
                SeriesLabelPosition::MiddleLeft,
                SeriesLabelPosition::LowerLeft,
                SeriesLabelPosition::UpperMiddle,
                SeriesLabelPosition::MiddleMiddle,
                SeriesLabelPosition::LowerMiddle,
                SeriesLabelPosition::UpperRight,
                SeriesLabelPosition::MiddleRight,
                SeriesLabelPosition::LowerRight,
                SeriesLabelPosition::Coordinate(70, 70),
            ]
            .into_iter()
            {
                chart
                    .configure_series_labels()
                    .border_style(&BLACK.mix(0.5))
                    .position(pos)
                    .draw()
                    .expect("Drawing error");
            }
        }

        checked_save_file("test_series_labels", &content);
    }

    #[test]
    fn test_draw_pixel_alphas() {
        let mut content = String::default();
        {
            let (width, height) = (100_i32, 100_i32);
            let root = SVGBackend::with_string(&mut content, (width as u32, height as u32))
                .into_drawing_area();
            root.fill(&WHITE).unwrap();

            for i in -20..20 {
                let alpha = i as f64 * 0.1;
                root.draw_pixel((50 + i, 50 + i), &BLACK.mix(alpha))
                    .unwrap();
            }
        }

        checked_save_file("test_draw_pixel_alphas", &content);
    }
}

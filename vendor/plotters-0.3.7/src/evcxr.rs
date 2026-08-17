use crate::coord::Shift;
use crate::drawing::{DrawingArea, IntoDrawingArea};
use plotters_backend::DrawingBackend;
use plotters_svg::SVGBackend;
use std::fs::File;
use std::io::Write;

#[cfg(feature = "evcxr_bitmap")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "evcxr_bitmap")))]
use plotters_bitmap::BitMapBackend;

/// The wrapper for the generated SVG
pub struct SVGWrapper(String, String);

impl SVGWrapper {
    /// Displays the contents of the `SVGWrapper` struct.
    pub fn evcxr_display(&self) {
        println!("{:?}", self);
    }
    /// Sets the style of the `SVGWrapper` struct.
    pub fn style<S: Into<String>>(mut self, style: S) -> Self {
        self.1 = style.into();
        self
    }
}

impl std::fmt::Debug for SVGWrapper {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        let svg = self.0.as_str();
        write!(
            formatter,
            "EVCXR_BEGIN_CONTENT text/html\n<div style=\"{}\">{}</div>\nEVCXR_END_CONTENT",
            self.1, svg
        )
    }
}

/// Start drawing an evcxr figure
pub fn evcxr_figure<
    Draw: FnOnce(DrawingArea<SVGBackend, Shift>) -> Result<(), Box<dyn std::error::Error>>,
>(
    size: (u32, u32),
    draw: Draw,
) -> SVGWrapper {
    let mut buffer = "".to_string();
    let root = SVGBackend::with_string(&mut buffer, size).into_drawing_area();
    draw(root).expect("Drawing failure");
    SVGWrapper(buffer, "".to_string())
}

/// An evcxr figure that can save to the local file system and render in a notebook.
pub fn evcxr_figure_with_saving<
    Draw: FnOnce(DrawingArea<SVGBackend, Shift>) -> Result<(), Box<dyn std::error::Error>>,
>(
    filename: &str,
    size: (u32, u32),
    draw: Draw,
) -> SVGWrapper {
    let mut buffer = "".to_string();
    let root = SVGBackend::with_string(&mut buffer, size).into_drawing_area();
    draw(root).expect("Drawing failure");

    let mut file = File::create(filename).expect("Unable to create file");
    file.write_all(buffer.as_bytes())
        .expect("Unable to write data");

    SVGWrapper(buffer, "".to_string())
}
/// Start drawing an evcxr figure
#[cfg(feature = "evcxr_bitmap")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "evcxr_bitmap")))]
pub fn evcxr_bitmap_figure<
    Draw: FnOnce(DrawingArea<BitMapBackend, Shift>) -> Result<(), Box<dyn std::error::Error>>,
>(
    size: (u32, u32),
    draw: Draw,
) -> SVGWrapper {
    const PIXEL_SIZE: usize = 3;

    let mut buf = vec![0; (size.0 as usize) * (size.1 as usize) * PIXEL_SIZE];

    let root = BitMapBackend::with_buffer(&mut buf, size).into_drawing_area();
    draw(root).expect("Drawing failure");
    let mut buffer = "".to_string();
    {
        let mut svg_root = SVGBackend::with_string(&mut buffer, size);
        svg_root
            .blit_bitmap((0, 0), size, &buf)
            .expect("Failure converting to SVG");
    }
    SVGWrapper(buffer, "".to_string())
}

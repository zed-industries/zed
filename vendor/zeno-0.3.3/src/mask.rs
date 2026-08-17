//! Mask generator.

use super::geometry::{Origin, Placement, Transform, Vector};
use super::path_data::{apply, PathData};
use super::scratch::Scratch;
use super::style::{Fill, Style};
#[allow(unused)]
use super::F32Ext;

use crate::lib::Vec;
use core::cell::RefCell;

/// The desired output image format for rendering.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Format {
    /// 8-bit alpha mask.
    Alpha,
    /// 32-bit RGBA subpixel mask with 1/3 pixel offsets for the red and
    /// blue channels.
    Subpixel,
    /// 32-bit RGBA subpixel mask with custom offsets.
    CustomSubpixel([f32; 3]),
}

impl Format {
    /// Creates a format for BGRA subpixel rendering.
    pub fn subpixel_bgra() -> Self {
        Self::CustomSubpixel([0.3, 0., -0.3])
    }

    /// Returns the necessary buffer size to hold an image of the specified
    /// width and height with this format.
    pub fn buffer_size(self, width: u32, height: u32) -> usize {
        (width
            * height
            * match self {
                Self::Alpha => 1,
                _ => 4,
            }) as usize
    }
}

impl Default for Format {
    fn default() -> Self {
        Self::Alpha
    }
}

/// Builder for configuring and rendering a mask.
pub struct Mask<'a, 's, D> {
    data: D,
    style: Style<'a>,
    transform: Option<Transform>,
    format: Format,
    origin: Origin,
    offset: Vector,
    render_offset: Vector,
    width: u32,
    height: u32,
    explicit_size: bool,
    has_size: bool,
    bounds_offset: Vector,
    scratch: RefCell<Option<&'s mut Scratch>>,
}

impl<'a, 's, D> Mask<'a, 's, D>
where
    D: PathData,
{
    /// Creates a new mask builder for the specified path data.
    pub fn new(data: D) -> Self {
        Self {
            data,
            style: Style::Fill(Fill::NonZero),
            transform: None,
            format: Format::Alpha,
            origin: Origin::TopLeft,
            offset: Vector::ZERO,
            render_offset: Vector::ZERO,
            width: 0,
            height: 0,
            explicit_size: false,
            has_size: false,
            bounds_offset: Vector::ZERO,
            scratch: RefCell::new(None),
        }
    }

    /// Creates a new mask builder for the specified path data and scratch memory.
    pub fn with_scratch(data: D, scratch: &'s mut Scratch) -> Self {
        Self {
            data,
            style: Style::Fill(Fill::NonZero),
            transform: None,
            format: Format::Alpha,
            origin: Origin::TopLeft,
            offset: Vector::ZERO,
            render_offset: Vector::ZERO,
            width: 0,
            height: 0,
            explicit_size: false,
            has_size: false,
            bounds_offset: Vector::ZERO,
            scratch: RefCell::new(Some(scratch)),
        }
    }

    /// Sets the style of the path. The default is a non-zero fill.
    pub fn style(&mut self, style: impl Into<Style<'a>>) -> &mut Self {
        self.style = style.into();
        self
    }

    /// Sets the transformation matrix of the path.
    pub fn transform(&mut self, transform: Option<Transform>) -> &mut Self {
        self.transform = transform;
        self
    }

    /// Sets the desired format of the mask. The default value is an 8-bit
    /// alpha format.
    pub fn format(&mut self, format: Format) -> &mut Self {
        self.format = format;
        self
    }

    /// Sets the origin that defines the coordinate system for the mask.
    pub fn origin(&mut self, origin: Origin) -> &mut Self {
        self.origin = origin;
        self
    }

    /// Sets the offset for the path's rendered bounds. To translate both the
    /// path and its rendered bounding box, set both [`Self::offset`] and
    /// [`Self::render_offset`].
    pub fn offset(&mut self, offset: impl Into<Vector>) -> &mut Self {
        self.offset = offset.into();
        self
    }

    /// Sets an explicit size for the mask. If left unspecified, the size will
    /// be computed from the bounding box of the path after applying any
    /// relevant style, offset and transform.
    pub fn size(&mut self, width: u32, height: u32) -> &mut Self {
        self.width = width;
        self.height = height;
        self.explicit_size = true;
        self.has_size = true;
        self
    }

    /// Sets an additional rendering offset for the mask. This offset does not
    /// affect bounds or size computations and is only applied during
    /// rendering.
    pub fn render_offset(&mut self, offset: impl Into<Vector>) -> &mut Self {
        self.render_offset = offset.into();
        self
    }

    /// Invokes a closure with the format, width and height of the mask provided
    /// as arguments. This is primarily useful for preparing a target buffer without
    /// interrupting the call chain.
    pub fn inspect(&mut self, mut f: impl FnMut(Format, u32, u32)) -> &mut Self {
        self.ensure_size();
        f(self.format, self.width, self.height);
        self
    }

    /// Renders the mask into a byte buffer. If specified, the pitch describes
    /// the number of bytes between subsequent rows of the target buffer. This
    /// is primarily useful for rendering into tiled images such as texture
    /// atlases. If left unspecified, the buffer is assumed to be linear and
    /// tightly packed.
    pub fn render_into(&self, buffer: &mut [u8], pitch: Option<usize>) -> Placement {
        let (offset, placement) = self.placement();
        let pitch = match pitch {
            Some(pitch) => pitch,
            _ => {
                placement.width as usize
                    * match self.format {
                        Format::Alpha => 1,
                        _ => 4,
                    }
            }
        };
        render(self, offset, &placement, buffer, pitch);
        placement
    }

    /// Renders the mask to a newly allocated buffer.
    pub fn render(&self) -> (Vec<u8>, Placement) {
        let mut buf = Vec::new();
        let (offset, placement) = self.placement();
        buf.resize(
            self.format.buffer_size(placement.width, placement.height),
            0,
        );
        let pitch = placement.width as usize
            * match self.format {
                Format::Alpha => 1,
                _ => 4,
            };
        render(self, offset, &placement, &mut buf, pitch);
        (buf, placement)
    }

    fn ensure_size(&mut self) {
        if self.has_size {
            return;
        }
        let (offset, placement) = self.placement();
        self.bounds_offset = offset;
        self.width = placement.width;
        self.height = placement.height;
        self.explicit_size = false;
        self.has_size = true;
    }

    fn placement(&self) -> (Vector, Placement) {
        let mut placement = Placement {
            left: 0,
            top: 0,
            width: self.width,
            height: self.height,
        };
        let mut offset = self.offset;
        if self.explicit_size {
            return (offset, placement);
        } else if !self.has_size {
            let mut scratch = self.scratch.borrow_mut();
            let mut bounds = if let Some(scratch) = scratch.as_mut() {
                scratch.bounds(&self.data, self.style, self.transform)
            } else {
                super::bounds(&self.data, self.style, self.transform)
            };
            bounds.min = (bounds.min + self.offset).floor();
            bounds.max = (bounds.max + self.offset).ceil();
            offset = Vector::new(-bounds.min.x, -bounds.min.y);
            placement.width = bounds.width() as u32;
            placement.height = bounds.height() as u32;
        } else {
            offset = self.bounds_offset;
        }
        placement.left = -offset.x as i32;
        placement.top = if self.origin == Origin::BottomLeft {
            (-offset.y).floor() + self.height as f32
        } else {
            -offset.y
        } as i32;
        (offset, placement)
    }
}

#[allow(clippy::needless_lifetimes)]
pub fn render<'a, 'c, D>(
    mask: &'a Mask<'a, 'c, D>,
    offset: Vector,
    placement: &Placement,
    buf: &mut [u8],
    pitch: usize,
) where
    D: PathData,
{
    let y_up = mask.origin == Origin::BottomLeft;
    let (is_subpx, subpx) = match mask.format {
        Format::Alpha => (false, [Vector::ZERO; 3]),
        Format::Subpixel => (
            true,
            [Vector::new(-0.3, 0.), Vector::ZERO, Vector::new(0.3, 0.)],
        ),
        Format::CustomSubpixel(subpx) => (
            true,
            [
                Vector::new(subpx[0], 0.),
                Vector::new(subpx[1], 0.),
                Vector::new(subpx[2], 0.),
            ],
        ),
    };
    let fill = match mask.style {
        Style::Fill(fill) => fill,
        _ => Fill::NonZero,
    };
    let w = placement.width;
    let h = placement.height;
    let shift = offset + mask.render_offset;
    let data = &mask.data;
    let style = mask.style;
    let transform = mask.transform;
    let mut scratch = mask.scratch.borrow_mut();
    use super::raster::{AdaptiveStorage, Rasterizer};
    if let Some(scratch) = scratch.as_mut() {
        let mut ras = Rasterizer::new(&mut scratch.render);
        let inner = &mut scratch.inner;
        if is_subpx {
            ras.rasterize_write(
                shift + subpx[0],
                w,
                h,
                &mut |r| {
                    inner.apply(data, &style, transform, r);
                },
                fill,
                pitch,
                y_up,
                &mut |row_offset, x, count, coverage| {
                    let buf = &mut buf[row_offset..];
                    let mut i = 0;
                    let mut j = x * 4;
                    while i < count {
                        buf[j] = coverage;
                        i += 1;
                        j += 4;
                    }
                },
            );
            ras.rasterize_write(
                shift + subpx[1],
                w,
                h,
                &mut |r| {
                    inner.apply(data, &style, transform, r);
                },
                fill,
                pitch,
                y_up,
                &mut |row_offset, x, count, coverage| {
                    let buf = &mut buf[row_offset..];
                    let mut i = 0;
                    let mut j = x * 4 + 1;
                    while i < count {
                        buf[j] = coverage;
                        i += 1;
                        j += 4;
                    }
                },
            );
            ras.rasterize_write(
                shift + subpx[2],
                w,
                h,
                &mut |r| {
                    inner.apply(data, &style, transform, r);
                },
                fill,
                pitch,
                y_up,
                &mut |row_offset, x, count, coverage| {
                    let buf = &mut buf[row_offset..];
                    let mut i = 0;
                    let mut j = x * 4 + 2;
                    while i < count {
                        buf[j] = coverage;
                        i += 1;
                        j += 4;
                    }
                },
            );
        } else {
            ras.rasterize(
                shift,
                w,
                h,
                &mut |r| {
                    inner.apply(data, &style, transform, r);
                },
                fill,
                buf,
                pitch,
                y_up,
            );
        }
    } else {
        let mut storage = AdaptiveStorage::new();
        let mut ras = Rasterizer::new(&mut storage);
        if is_subpx {
            ras.rasterize_write(
                shift + subpx[0],
                w,
                h,
                &mut |r| {
                    apply(data, style, transform, r);
                },
                fill,
                pitch,
                y_up,
                &mut |row_offset, x, count, coverage| {
                    let buf = &mut buf[row_offset..];
                    let mut i = 0;
                    let mut j = x * 4;
                    while i < count {
                        buf[j] = coverage;
                        i += 1;
                        j += 4;
                    }
                },
            );
            ras.rasterize_write(
                shift + subpx[1],
                w,
                h,
                &mut |r| {
                    apply(data, style, transform, r);
                },
                fill,
                pitch,
                y_up,
                &mut |row_offset, x, count, coverage| {
                    let buf = &mut buf[row_offset..];
                    let mut i = 0;
                    let mut j = x * 4 + 1;
                    while i < count {
                        buf[j] = coverage;
                        i += 1;
                        j += 4;
                    }
                },
            );
            ras.rasterize_write(
                shift + subpx[2],
                w,
                h,
                &mut |r| {
                    apply(data, style, transform, r);
                },
                fill,
                pitch,
                y_up,
                &mut |row_offset, x, count, coverage| {
                    let buf = &mut buf[row_offset..];
                    let mut i = 0;
                    let mut j = x * 4 + 2;
                    while i < count {
                        buf[j] = coverage;
                        i += 1;
                        j += 4;
                    }
                },
            );
        } else {
            ras.rasterize(
                shift,
                w,
                h,
                &mut |r| {
                    apply(data, style, transform, r);
                },
                fill,
                buf,
                pitch,
                y_up,
            );
        }
    }
}

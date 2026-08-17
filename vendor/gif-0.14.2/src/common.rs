use alloc::borrow::Cow;
use alloc::vec::Vec;

#[cfg(feature = "color_quant")]
use alloc::collections::{BTreeMap, BTreeSet};

/// Disposal method
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum DisposalMethod {
    /// `StreamingDecoder` is not required to take any action.
    Any = 0,
    /// Do not dispose.
    Keep = 1,
    /// Restore to background color.
    Background = 2,
    /// Restore to previous.
    Previous = 3,
}

impl DisposalMethod {
    /// Converts `u8` to `Option<Self>`
    #[must_use]
    pub const fn from_u8(n: u8) -> Option<Self> {
        match n {
            0 => Some(Self::Any),
            1 => Some(Self::Keep),
            2 => Some(Self::Background),
            3 => Some(Self::Previous),
            _ => None,
        }
    }
}

/// Known GIF block labels.
///
/// Note that the block uniquely specifies the layout of bytes that follow and how they are
/// framed. For example, the header always has a fixed length but is followed by a variable amount
/// of additional data. An image descriptor may be followed by a local color table depending on
/// information read in it. Therefore, it doesn't make sense to continue parsing after encountering
/// an unknown block as the semantics of following bytes are unclear.
///
/// The extension block provides a common framing for an arbitrary amount of application specific
/// data which may be ignored.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Block {
    /// Image block.
    Image = 0x2C,
    /// Extension block.
    Extension = 0x21,
    /// Image trailer.
    Trailer = 0x3B,
}

impl Block {
    /// Converts `u8` to `Option<Self>`
    #[must_use]
    pub const fn from_u8(n: u8) -> Option<Self> {
        match n {
            0x2C => Some(Self::Image),
            0x21 => Some(Self::Extension),
            0x3B => Some(Self::Trailer),
            _ => None,
        }
    }
}

/// A newtype wrapper around an arbitrary extension ID.
///
/// An extension is some amount of byte data organized in sub-blocks so that one can skip over it
/// without knowing the semantics. Though technically you likely want to use a `Application`
/// extension, the library tries to stay flexible here.
///
/// This allows us to customize the set of impls compared to a raw `u8`. It also clarifies the
/// intent and gives some inherent methods for interoperability with known extension types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AnyExtension(pub u8);

/// Known GIF extension labels.
///
/// These are extensions which may be interpreted by the library and to which a specification with
/// the internal data layout is known.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Extension {
    /// Plain Text extension.
    ///
    /// This instructs the decoder to render a text as characters in a grid of cells, in a
    /// mono-spaced font of its choosing. This is seldom actually implemented and ignored by
    /// ImageMagick. The color is always taken from the global table which further complicates any
    /// use. No real information on the frame sequencing of this block is available in the
    /// standard.
    Text = 0x01,
    /// Control extension.
    Control = 0xF9,
    /// Comment extension.
    Comment = 0xFE,
    /// Application extension.
    ///
    /// See [ImageMagick] for an idea of commonly recognized extensions.
    ///
    /// [ImageMagick]: https://github.com/ImageMagick/ImageMagick/blob/b0b58c6303195928060f55f9c3ca8233ab7f7733/coders/gif.c#L1128
    Application = 0xFF,
}

impl AnyExtension {
    /// Decode the label as a known extension.
    #[must_use]
    pub const fn into_known(self) -> Option<Extension> {
        Extension::from_u8(self.0)
    }
}

impl From<Extension> for AnyExtension {
    fn from(ext: Extension) -> Self {
        Self(ext as u8)
    }
}

impl Extension {
    /// Converts `u8` to a `Extension` if it is known.
    #[must_use]
    pub const fn from_u8(n: u8) -> Option<Self> {
        match n {
            0x01 => Some(Self::Text),
            0xF9 => Some(Self::Control),
            0xFE => Some(Self::Comment),
            0xFF => Some(Self::Application),
            _ => None,
        }
    }
}

/// A GIF frame
#[derive(Debug, Clone)]
pub struct Frame<'a> {
    /// Frame delay in units of 10 ms.
    pub delay: u16,
    /// Disposal method.
    pub dispose: DisposalMethod,
    /// Transparent index (if available).
    pub transparent: Option<u8>,
    /// True if the frame needs user input to be displayed.
    pub needs_user_input: bool,
    /// Offset from the top border of the canvas.
    pub top: u16,
    /// Offset from the left border of the canvas.
    pub left: u16,
    /// Width of the frame.
    pub width: u16,
    /// Height of the frame.
    pub height: u16,
    /// True if the image is interlaced.
    pub interlaced: bool,
    /// Frame local color palette if available.
    pub palette: Option<Vec<u8>>,
    /// Buffer containing the image data.
    /// Only indices unless configured differently.
    pub buffer: Cow<'a, [u8]>,
}

impl Default for Frame<'_> {
    fn default() -> Self {
        Frame {
            delay: 0,
            dispose: DisposalMethod::Keep,
            transparent: None,
            needs_user_input: false,
            top: 0,
            left: 0,
            width: 0,
            height: 0,
            interlaced: false,
            palette: None,
            buffer: Cow::Borrowed(&[]),
        }
    }
}

impl Frame<'static> {
    /// Creates a frame from pixels in RGBA format.
    ///
    /// This is a lossy method. The `gif` format does not support arbitrary alpha but only a 1-bit
    /// transparency mask per pixel. Any non-zero alpha value will be interpreted as a fully opaque
    /// pixel. Additionally, only 256 colors can appear in a single frame. The palette will be
    /// reduced by the NeuQuant algorithm if necessary. Different frames have independent palettes.
    ///
    /// *Note: This method is not optimized for speed.*
    ///
    /// # Panics:
    /// *   If the length of pixels does not equal `width * height * 4`.
    #[cfg(feature = "color_quant")]
    #[track_caller]
    pub fn from_rgba(width: u16, height: u16, pixels: &mut [u8]) -> Self {
        Frame::from_rgba_speed(width, height, pixels, 1)
    }

    /// Creates a frame from pixels in RGBA format.
    ///
    /// `speed` is a value in the range [1, 30].
    /// The higher the value the faster it runs at the cost of image quality.
    /// A `speed` of 10 is a good compromise between speed and quality.
    ///
    /// This is a lossy method. The `gif` format does not support arbitrary alpha but only a 1-bit
    /// transparency mask per pixel. Any non-zero alpha value will be interpreted as a fully opaque
    /// pixel. Additionally, only 256 colors can appear in a single frame. The palette will be
    /// reduced by the NeuQuant algorithm if necessary. Different frames have independent palettes.
    ///
    /// # Panics:
    /// *   If the length of pixels does not equal `width * height * 4`.
    /// *   If `speed < 1` or `speed > 30`
    #[cfg(feature = "color_quant")]
    #[track_caller]
    pub fn from_rgba_speed(width: u16, height: u16, pixels: &mut [u8], speed: i32) -> Self {
        assert_eq!(width as usize * height as usize * 4, pixels.len(), "Too much or too little pixel data for the given width and height to create a GIF Frame");
        assert!(
            speed >= 1 && speed <= 30,
            "speed needs to be in the range [1, 30]"
        );
        let mut transparent: Option<[u8; 4]> = None;
        for pix in pixels.chunks_exact_mut(4) {
            if pix[3] != 0 {
                pix[3] = 0xFF;
                continue;
            }

            if let Some([r, g, b, a]) = transparent {
                pix[0] = r;
                pix[1] = g;
                pix[2] = b;
                pix[3] = a;
            } else {
                transparent = Some([pix[0], pix[1], pix[2], pix[3]]);
            }
        }

        // Attempt to build a palette of all colors. If we go over 256 colors,
        // switch to the NeuQuant algorithm.
        let mut colors: BTreeSet<(u8, u8, u8, u8)> = BTreeSet::new();
        for pixel in pixels.chunks_exact(4) {
            if colors.insert((pixel[0], pixel[1], pixel[2], pixel[3])) && colors.len() > 256 {
                // > 256 colours, let's use NeuQuant.
                let nq = color_quant::NeuQuant::new(speed, 256, pixels);

                return Frame {
                    width,
                    height,
                    buffer: Cow::Owned(
                        pixels
                            .chunks_exact(4)
                            .map(|pix| nq.index_of(pix) as u8)
                            .collect(),
                    ),
                    palette: Some(nq.color_map_rgb()),
                    transparent: transparent.map(|t| nq.index_of(&t) as u8),
                    ..Frame::default()
                };
            }
        }

        // Palette size <= 256 elements, we can build an exact palette.
        let mut colors_vec: Vec<(u8, u8, u8, u8)> = colors.into_iter().collect();
        colors_vec.sort_unstable();
        let palette = colors_vec
            .iter()
            .flat_map(|&(r, g, b, _a)| [r, g, b])
            .collect();
        let colors_lookup: BTreeMap<(u8, u8, u8, u8), u8> =
            colors_vec.into_iter().zip(0..=255).collect();

        let index_of = |pixel: &[u8]| {
            colors_lookup
                .get(&(pixel[0], pixel[1], pixel[2], pixel[3]))
                .copied()
                .unwrap_or(0)
        };

        Frame {
            width,
            height,
            buffer: Cow::Owned(pixels.chunks_exact(4).map(index_of).collect()),
            palette: Some(palette),
            transparent: transparent.map(|t| index_of(&t)),
            ..Frame::default()
        }
    }

    /// Creates a frame from pixels in LumaAlpha format (grayscale pixels with transparency).
    ///
    /// This is a lossy method. The `gif` format does not support arbitrary alpha but only a 1-bit
    /// transparency mask per pixel. Any non-zero alpha value will be interpreted as a fully opaque
    /// pixel. The least used color will be used to indicate alpha and replace with the closest other color
    /// in the image. Different frames have independent palettes.
    ///
    /// # Panics:
    /// *   If the length of pixels does not equal `width * height * 2`.
    pub fn from_grayscale_with_alpha(width: u16, height: u16, pixels: &[u8]) -> Self {
        assert_eq!(width as usize * height as usize * 2, pixels.len(), "Too much or too little pixel data for the given width and height to create a GIF Frame");

        // Input is in LumaA format.
        // Count the occurrences of all the colors, then pick the least common color as alpha.
        let mut num_transparent_pixels: u32 = 0;
        let mut color_frequencies: [u32; 256] = [0; 256];
        for pixel in pixels.chunks_exact(2) {
            let color = pixel[0];
            let alpha = pixel[1];
            // do not count colors in fully transparent pixels
            if alpha == 0 {
                num_transparent_pixels += 1;
            } else {
                color_frequencies[color as usize] += 1;
            }
        }

        let grayscale_palette: Vec<u8> = (0..=255).flat_map(|i| [i, i, i]).collect();

        // If there were no fully transparent pixels, do not allocate a color to transparency in the GIF
        // and return immediately with the generic grayscale palette
        if num_transparent_pixels == 0 {
            let stripped_alpha: Vec<u8> = pixels.chunks_exact(2).map(|pixel| pixel[0]).collect();
            return Frame {
                width,
                height,
                buffer: Cow::Owned(stripped_alpha),
                palette: Some(grayscale_palette),
                transparent: None,
                ..Frame::default()
            };
        }

        // Choose the color that will be our alpha color
        let least_used_color = color_frequencies
            .iter()
            .enumerate()
            .min_by_key(|(_, &value)| value)
            .map(|(index, _)| index as u8)
            .expect("input slice is empty");

        // pick the less used color out of the neighbours as the replacement color
        let replacement_color = if least_used_color == 255 {
            254
        } else if least_used_color == 0 {
            1
        } else if color_frequencies[(least_used_color - 1) as usize]
            < color_frequencies[(least_used_color + 1) as usize]
        {
            least_used_color - 1
        } else {
            least_used_color + 1
        };

        // Strip alpha and replace fully transparent pixels with the chosen color
        let paletted: Vec<u8> = pixels
            .chunks_exact(2)
            .map(|pixel| {
                let color = pixel[0];
                let alpha = pixel[1];
                if alpha == 0 {
                    least_used_color
                } else if color == least_used_color {
                    replacement_color
                } else {
                    color
                }
            })
            .collect();

        Frame {
            width,
            height,
            buffer: Cow::Owned(paletted),
            palette: Some(grayscale_palette),
            transparent: Some(least_used_color),
            ..Frame::default()
        }
    }

    /// Creates a frame from a palette and indexed pixels.
    ///
    /// # Panics:
    /// *   If the length of pixels does not equal `width * height`.
    /// *   If the length of palette > `256 * 3`.
    #[track_caller]
    pub fn from_palette_pixels(
        width: u16,
        height: u16,
        pixels: impl Into<Vec<u8>>,
        palette: impl Into<Vec<u8>>,
        transparent: Option<u8>,
    ) -> Self {
        let pixels = pixels.into();
        let palette = palette.into();
        assert_eq!(
            width as usize * height as usize,
            pixels.len(),
            "Too many or too little pixels for the given width and height to create a GIF Frame"
        );
        assert!(
            palette.len() <= 256 * 3,
            "Too many palette values to create a GIF Frame"
        );

        Frame {
            width,
            height,
            buffer: Cow::Owned(pixels),
            palette: Some(palette),
            transparent,
            ..Frame::default()
        }
    }

    /// Creates a frame from indexed pixels in the global palette.
    ///
    /// # Panics:
    /// *   If the length of pixels does not equal `width * height`.
    #[track_caller]
    pub fn from_indexed_pixels(
        width: u16,
        height: u16,
        pixels: impl Into<Vec<u8>>,
        transparent: Option<u8>,
    ) -> Self {
        let pixels = pixels.into();
        assert_eq!(
            width as usize * height as usize,
            pixels.len(),
            "Too many or too little pixels for the given width and height to create a GIF Frame"
        );

        Frame {
            width,
            height,
            buffer: Cow::Owned(pixels),
            palette: None,
            transparent,
            ..Frame::default()
        }
    }

    /// Creates a frame from pixels in RGB format.
    ///
    /// This is a lossy method. In the `gif` format only 256 colors can appear in a single frame.
    /// The palette will be reduced by the NeuQuant algorithm if necessary. Different frames have
    /// independent palettes.
    ///
    /// *Note: This method is not optimized for speed.*
    ///
    /// # Panics:
    /// *   If the length of pixels does not equal `width * height * 3`.
    #[cfg(feature = "color_quant")]
    #[must_use]
    #[track_caller]
    pub fn from_rgb(width: u16, height: u16, pixels: &[u8]) -> Self {
        Frame::from_rgb_speed(width, height, pixels, 1)
    }

    /// Creates a frame from pixels in RGB format.
    ///
    /// `speed` is a value in the range [1, 30].
    ///
    /// This is a lossy method. In the `gif` format only 256 colors can appear in a single frame.
    /// The palette will be reduced by the NeuQuant algorithm if necessary. Different frames have
    /// independent palettes.
    ///
    /// The higher the value the faster it runs at the cost of image quality.
    /// A `speed` of 10 is a good compromise between speed and quality.
    ///
    /// # Panics:
    /// *   If the length of pixels does not equal `width * height * 3`.
    /// *   If `speed < 1` or `speed > 30`
    #[cfg(feature = "color_quant")]
    #[must_use]
    #[track_caller]
    pub fn from_rgb_speed(width: u16, height: u16, pixels: &[u8], speed: i32) -> Self {
        assert_eq!(width as usize * height as usize * 3, pixels.len(), "Too much or too little pixel data for the given width and height to create a GIF Frame");
        let mut vec: Vec<u8> = Vec::new();
        vec.try_reserve_exact(pixels.len() + width as usize * height as usize)
            .expect("OOM");
        for v in pixels.chunks_exact(3) {
            vec.extend_from_slice(&[v[0], v[1], v[2], 0xFF]);
        }
        Frame::from_rgba_speed(width, height, &mut vec, speed)
    }

    /// Leaves empty buffer and empty palette behind
    #[inline]
    pub(crate) fn take(&mut self) -> Self {
        Frame {
            delay: self.delay,
            dispose: self.dispose,
            transparent: self.transparent,
            needs_user_input: self.needs_user_input,
            top: self.top,
            left: self.left,
            width: self.width,
            height: self.height,
            interlaced: self.interlaced,
            palette: core::mem::take(&mut self.palette),
            buffer: core::mem::replace(&mut self.buffer, Cow::Borrowed(&[])),
        }
    }
}

#[test]
#[cfg(feature = "color_quant")]
// Creating the `colors_lookup` hashmap in Frame::from_rgba_speed panics due to
// overflow while bypassing NeuQuant and zipping a RangeFrom with 256 colors.
// Changing .zip(0_u8..) to .zip(0_u8..=255) fixes this issue.
fn rgba_speed_avoid_panic_256_colors() {
    let side = 16;
    let pixel_data: Vec<u8> = (0..=255).flat_map(|a| [a, a, a]).collect();
    let _ = Frame::from_rgb(side, side, &pixel_data);
}

//! YCbCr to RGB conversion for video surfaces, chosen per pixel buffer from its pixel format
//! (video or full range) and its `kCVImageBufferYCbCrMatrixKey` attachment.

use std::ptr;

use anyhow::{Result, bail};
use objc2_core_foundation::CFString;
use objc2_core_video::{
    CVPixelBuffer, CVPixelBufferGetPixelFormatType, kCVImageBufferYCbCrMatrix_ITU_R_709_2,
    kCVImageBufferYCbCrMatrix_ITU_R_2020, kCVImageBufferYCbCrMatrixKey,
    kCVPixelFormatType_420YpCbCr8BiPlanarFullRange,
    kCVPixelFormatType_420YpCbCr8BiPlanarVideoRange,
};

/// A column-major 4×4 matrix, laid out like the shader's `float4x4`. It maps
/// `(Y, Cb, Cr, 1)`, each sampled in `0..=1`, to `(R, G, B, 1)`.
pub(crate) type ColorTransform = [[f32; 4]; 4];

/// The YCbCr matrix a buffer was encoded with.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum YCbCrMatrix {
    /// ITU-R BT.601 (SD video). Also the default for buffers without a matrix attachment.
    Bt601,
    /// ITU-R BT.709 (HD video).
    Bt709,
    /// ITU-R BT.2020 (UHD and HDR video).
    Bt2020,
}

impl YCbCrMatrix {
    /// The red and blue luma coefficients, `Kr` and `Kb`.
    fn coefficients(self) -> (f32, f32) {
        match self {
            Self::Bt601 => (0.299, 0.114),
            Self::Bt709 => (0.2126, 0.0722),
            Self::Bt2020 => (0.2627, 0.0593),
        }
    }
}

/// The range a buffer's 8-bit samples use.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum YCbCrRange {
    /// Luma 0–255, chroma 0–255 around 128.
    Full,
    /// "Video" or "TV" range: luma 16–235, chroma 16–240 around 128.
    Video,
}

/// The transform that draws `buffer` with the right colors. Fails for pixel formats the
/// surface pipeline cannot sample.
pub(crate) fn surface_color_transform(buffer: &CVPixelBuffer) -> Result<ColorTransform> {
    let range = match CVPixelBufferGetPixelFormatType(buffer) {
        format if format == kCVPixelFormatType_420YpCbCr8BiPlanarFullRange => YCbCrRange::Full,
        format if format == kCVPixelFormatType_420YpCbCr8BiPlanarVideoRange => YCbCrRange::Video,
        format => bail!("unsupported surface pixel format: {format:#x}"),
    };
    Ok(ycbcr_to_rgb(matrix_of(buffer), range))
}

/// The matrix `buffer` is tagged with, or BT.601 when it carries no (known) tag.
fn matrix_of(buffer: &CVPixelBuffer) -> YCbCrMatrix {
    // SAFETY: the key is Core Video's constant string, and a null attachment mode is allowed.
    let attachment = unsafe { buffer.attachment(kCVImageBufferYCbCrMatrixKey, ptr::null_mut()) };
    let Some(tag) = attachment.and_then(|value| value.downcast::<CFString>().ok()) else {
        return YCbCrMatrix::Bt601;
    };
    let tag = tag.to_string();
    // SAFETY: these are Core Video's constant strings.
    let (bt709, bt2020) = unsafe {
        (
            kCVImageBufferYCbCrMatrix_ITU_R_709_2.to_string(),
            kCVImageBufferYCbCrMatrix_ITU_R_2020.to_string(),
        )
    };
    if tag == bt709 {
        YCbCrMatrix::Bt709
    } else if tag == bt2020 {
        YCbCrMatrix::Bt2020
    } else {
        YCbCrMatrix::Bt601
    }
}

/// The YCbCr to RGB transform for 8-bit samples in `range`, encoded with `matrix`.
pub(crate) fn ycbcr_to_rgb(matrix: YCbCrMatrix, range: YCbCrRange) -> ColorTransform {
    let (kr, kb) = matrix.coefficients();
    let kg = 1.0 - kr - kb;
    // Scale and offset that bring luma to 0..=1 and chroma to -0.5..=0.5.
    let (luma_scale, luma_offset, chroma_scale) = match range {
        YCbCrRange::Full => (1.0, 0.0, 1.0),
        YCbCrRange::Video => (255.0 / 219.0, 16.0 / 255.0, 255.0 / 224.0),
    };
    let chroma_offset = 128.0 / 255.0;
    let y = [luma_scale; 3];
    let cb = [
        0.0,
        -2.0 * kb * (1.0 - kb) / kg * chroma_scale,
        2.0 * (1.0 - kb) * chroma_scale,
    ];
    let cr = [
        2.0 * (1.0 - kr) * chroma_scale,
        -2.0 * kr * (1.0 - kr) / kg * chroma_scale,
        0.0,
    ];
    let offset: [f32; 3] = std::array::from_fn(|channel| {
        -(y[channel] * luma_offset + (cb[channel] + cr[channel]) * chroma_offset)
    });
    [
        [y[0], y[1], y[2], 0.0],
        [cb[0], cb[1], cb[2], 0.0],
        [cr[0], cr[1], cr[2], 0.0],
        [offset[0], offset[1], offset[2], 1.0],
    ]
}

#[cfg(test)]
mod tests {
    use super::{ColorTransform, YCbCrMatrix, YCbCrRange, ycbcr_to_rgb};

    /// Applies `transform` to 8-bit `(Y, Cb, Cr)` and returns 8-bit RGB.
    fn apply(transform: ColorTransform, ycbcr: [u8; 3]) -> [i32; 3] {
        let input = [ycbcr[0], ycbcr[1], ycbcr[2]].map(|v| f32::from(v) / 255.0);
        std::array::from_fn(|row| {
            let value = transform[0][row] * input[0]
                + transform[1][row] * input[1]
                + transform[2][row] * input[2]
                + transform[3][row];
            (value * 255.0).round() as i32
        })
    }

    #[test]
    fn full_range_bt601_keeps_the_previous_hard_coded_matrix() {
        let transform = ycbcr_to_rgb(YCbCrMatrix::Bt601, YCbCrRange::Full);
        let previous = [
            [1.0, 1.0, 1.0, 0.0],
            [0.0, -0.3441, 1.7720, 0.0],
            [1.4020, -0.7141, 0.0, 0.0],
            [-0.7010, 0.5291, -0.8860, 1.0],
        ];
        for (column, expected) in transform.iter().zip(previous) {
            for (value, expected) in column.iter().zip(expected) {
                // The old offsets assumed a chroma midpoint of 0.5 rather than 128/255.
                assert!((value - expected).abs() < 0.01, "{transform:?}");
            }
        }
    }

    #[test]
    fn video_range_maps_black_and_white_to_the_ends() {
        for matrix in [YCbCrMatrix::Bt601, YCbCrMatrix::Bt709, YCbCrMatrix::Bt2020] {
            let transform = ycbcr_to_rgb(matrix, YCbCrRange::Video);
            assert_eq!(apply(transform, [16, 128, 128]), [0, 0, 0]);
            assert_eq!(apply(transform, [235, 128, 128]), [255, 255, 255]);
        }
    }

    #[test]
    fn video_range_bt709_decodes_hd_colour_bars() {
        let transform = ycbcr_to_rgb(YCbCrMatrix::Bt709, YCbCrRange::Video);
        // 75% green, RGB (0, 191, 0), is (134, 63, 52) in BT.709 video range.
        let [r, g, b] = apply(transform, [134, 63, 52]);
        assert!(
            r.abs() <= 2 && (g - 191).abs() <= 2 && b.abs() <= 2,
            "{:?}",
            [r, g, b]
        );
        // Read as BT.601 instead, the same samples come out visibly wrong.
        let wrong = apply(
            ycbcr_to_rgb(YCbCrMatrix::Bt601, YCbCrRange::Video),
            [134, 63, 52],
        );
        assert!(wrong[0] > 10, "{wrong:?}");
    }
}

use std::sync::OnceLock;

use crate::{
    atlas::SoftwareAtlasTexture,
    lower::IRect,
    text_correction::{AlphaLut, AlphaLut3},
};

#[cfg(target_arch = "x86_64")]
#[path = "kernels_avx2.rs"]
mod avx2;

type FillKernel = unsafe fn(&mut [u32], usize, usize, IRect, u32);

struct KernelTable {
    fill_opaque: FillKernel,
    fill_blend: FillKernel,
    #[cfg(target_arch = "x86_64")]
    avx2: bool,
    simd_level: &'static str,
}

static KERNELS: OnceLock<KernelTable> = OnceLock::new();

fn kernels() -> &'static KernelTable {
    KERNELS.get_or_init(|| {
        #[cfg(target_arch = "x86_64")]
        if std::is_x86_feature_detected!("avx2") {
            return KernelTable {
                fill_opaque: avx2::fill_opaque,
                fill_blend: avx2::fill_blend,
                avx2: true,
                simd_level: "AVX2",
            };
        }
        KernelTable {
            fill_opaque: fill_opaque_scalar,
            fill_blend: fill_blend_scalar,
            #[cfg(target_arch = "x86_64")]
            avx2: false,
            simd_level: "scalar",
        }
    })
}

pub(crate) fn simd_level() -> &'static str {
    kernels().simd_level
}

pub(crate) fn fill_opaque(
    pixels: &mut [u32],
    stride: usize,
    band_y: usize,
    rect: IRect,
    color: u32,
) {
    // SAFETY: the runtime-selected function accepts the same slices and validates row ranges.
    unsafe { (kernels().fill_opaque)(pixels, stride, band_y, rect, color) }
}

pub(crate) fn fill_blend(
    pixels: &mut [u32],
    stride: usize,
    band_y: usize,
    rect: IRect,
    color: u32,
) {
    // SAFETY: the runtime-selected function accepts the same slices and validates row ranges.
    unsafe { (kernels().fill_blend)(pixels, stride, band_y, rect, color) }
}

pub(crate) unsafe fn fill_opaque_scalar(
    pixels: &mut [u32],
    stride: usize,
    band_y: usize,
    rect: IRect,
    color: u32,
) {
    for row in rows_mut(pixels, stride, band_y, rect) {
        row.fill(color | 0xff00_0000);
    }
}

pub(crate) unsafe fn fill_blend_scalar(
    pixels: &mut [u32],
    stride: usize,
    band_y: usize,
    rect: IRect,
    color: u32,
) {
    let alpha = (color >> 24) as u8;
    for row in rows_mut(pixels, stride, band_y, rect) {
        for destination in row {
            *destination = blend_pixel(*destination, color, alpha);
        }
    }
}

pub(crate) fn fill_gradient(
    pixels: &mut [u32],
    stride: usize,
    band_y: usize,
    rect: IRect,
    lut: &[u32; 256],
    t0: f32,
    dt_dx: f32,
    dt_dy: f32,
) {
    for y in rect.y0..rect.y1 {
        let local_y = y as usize - band_y;
        let start = local_y * stride + rect.x0 as usize;
        let row = &mut pixels[start..start + rect.width() as usize];
        let mut t = t0 + (rect.x0 as f32 + 0.5) * dt_dx + (y as f32 + 0.5) * dt_dy;
        for destination in row {
            let index = (t.clamp(0.0, 1.0) * 255.0).round() as usize;
            let color = lut[index];
            *destination = blend_pixel(*destination, color, (color >> 24) as u8);
            t += dt_dx;
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn blit_mono(
    pixels: &mut [u32],
    stride: usize,
    band_y: usize,
    rect: IRect,
    destination: IRect,
    untransformed: IRect,
    inverse: Option<[f32; 6]>,
    texture: &SoftwareAtlasTexture,
    tile_origin: (i32, i32),
    tile_size: (i32, i32),
    color: u32,
    lut: &AlphaLut,
) {
    #[cfg(target_arch = "x86_64")]
    if kernels().avx2
        && inverse.is_none()
        && destination.width() == tile_size.0
        && destination.height() == tile_size.1
        && texture.bytes_per_pixel() == 1
    {
        // SAFETY: AVX2 was detected at runtime and the one-to-one source rectangle is validated.
        unsafe {
            avx2::blit_mono(
                pixels,
                stride,
                band_y,
                rect,
                destination,
                texture,
                tile_origin,
                color,
                lut,
            )
        };
        return;
    }
    blit_mono_scalar(
        pixels,
        stride,
        band_y,
        rect,
        destination,
        untransformed,
        inverse,
        texture,
        tile_origin,
        tile_size,
        color,
        lut,
    );
}

#[allow(clippy::too_many_arguments)]
fn blit_mono_scalar(
    pixels: &mut [u32],
    stride: usize,
    band_y: usize,
    rect: IRect,
    destination: IRect,
    untransformed: IRect,
    inverse: Option<[f32; 6]>,
    texture: &SoftwareAtlasTexture,
    tile_origin: (i32, i32),
    tile_size: (i32, i32),
    color: u32,
    lut: &AlphaLut,
) {
    for y in rect.y0..rect.y1 {
        let local_y = y as usize - band_y;
        for x in rect.x0..rect.x1 {
            let Some((source_x, source_y)) =
                source_position(x, y, destination, untransformed, inverse, tile_size)
            else {
                continue;
            };
            let source_index = atlas_index(texture, tile_origin, source_x, source_y);
            let Some(coverage) = texture.pixels().get(source_index) else {
                continue;
            };
            let alpha = lut.0[*coverage as usize] as u8;
            let destination_index = local_y * stride + x as usize;
            pixels[destination_index] = blend_pixel(pixels[destination_index], color, alpha);
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn blit_subpixel(
    pixels: &mut [u32],
    stride: usize,
    band_y: usize,
    rect: IRect,
    destination: IRect,
    untransformed: IRect,
    inverse: Option<[f32; 6]>,
    texture: &SoftwareAtlasTexture,
    tile_origin: (i32, i32),
    tile_size: (i32, i32),
    color: u32,
    lut: &AlphaLut3,
    is_bgr: bool,
) {
    #[cfg(target_arch = "x86_64")]
    if kernels().avx2
        && inverse.is_none()
        && destination.width() == tile_size.0
        && destination.height() == tile_size.1
        && texture.bytes_per_pixel() == 4
    {
        // SAFETY: AVX2 was detected at runtime and the one-to-one source rectangle is validated.
        unsafe {
            avx2::blit_subpixel(
                pixels,
                stride,
                band_y,
                rect,
                destination,
                texture,
                tile_origin,
                color,
                lut,
                is_bgr,
            )
        };
        return;
    }
    blit_subpixel_scalar(
        pixels,
        stride,
        band_y,
        rect,
        destination,
        untransformed,
        inverse,
        texture,
        tile_origin,
        tile_size,
        color,
        lut,
        is_bgr,
    );
}

#[allow(clippy::too_many_arguments)]
fn blit_subpixel_scalar(
    pixels: &mut [u32],
    stride: usize,
    band_y: usize,
    rect: IRect,
    destination: IRect,
    untransformed: IRect,
    inverse: Option<[f32; 6]>,
    texture: &SoftwareAtlasTexture,
    tile_origin: (i32, i32),
    tile_size: (i32, i32),
    color: u32,
    lut: &AlphaLut3,
    is_bgr: bool,
) {
    for y in rect.y0..rect.y1 {
        let local_y = y as usize - band_y;
        for x in rect.x0..rect.x1 {
            let Some((source_x, source_y)) =
                source_position(x, y, destination, untransformed, inverse, tile_size)
            else {
                continue;
            };
            let source_index = atlas_index(texture, tile_origin, source_x, source_y);
            let Some(sample) = texture.pixels().get(source_index..source_index + 3) else {
                continue;
            };
            let coverage = if is_bgr {
                [sample[2], sample[1], sample[0]]
            } else {
                [sample[0], sample[1], sample[2]]
            };
            let destination_index = local_y * stride + x as usize;
            pixels[destination_index] = blend_subpixel(
                pixels[destination_index],
                color,
                [
                    lut.0[0][coverage[0] as usize] as u8,
                    lut.0[1][coverage[1] as usize] as u8,
                    lut.0[2][coverage[2] as usize] as u8,
                ],
            );
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn blit_polychrome(
    pixels: &mut [u32],
    stride: usize,
    band_y: usize,
    rect: IRect,
    destination: IRect,
    texture: &SoftwareAtlasTexture,
    tile_origin: (i32, i32),
    tile_size: (i32, i32),
    opacity: u8,
    grayscale: bool,
) {
    #[cfg(target_arch = "x86_64")]
    if kernels().avx2
        && destination.width() == tile_size.0
        && destination.height() == tile_size.1
        && texture.bytes_per_pixel() == 4
    {
        // SAFETY: AVX2 was detected at runtime and the one-to-one source rectangle is validated.
        unsafe {
            avx2::blit_polychrome(
                pixels,
                stride,
                band_y,
                rect,
                destination,
                texture,
                tile_origin,
                opacity,
                grayscale,
            )
        };
        return;
    }
    blit_polychrome_scalar(
        pixels,
        stride,
        band_y,
        rect,
        destination,
        texture,
        tile_origin,
        tile_size,
        opacity,
        grayscale,
    );
}

#[allow(clippy::too_many_arguments)]
fn blit_polychrome_scalar(
    pixels: &mut [u32],
    stride: usize,
    band_y: usize,
    rect: IRect,
    destination: IRect,
    texture: &SoftwareAtlasTexture,
    tile_origin: (i32, i32),
    tile_size: (i32, i32),
    opacity: u8,
    grayscale: bool,
) {
    for y in rect.y0..rect.y1 {
        let local_y = y as usize - band_y;
        for x in rect.x0..rect.x1 {
            let Some((source_x, source_y)) =
                source_position(x, y, destination, destination, None, tile_size)
            else {
                continue;
            };
            let source_index = atlas_index(texture, tile_origin, source_x, source_y);
            let Some(source) = texture.pixels().get(source_index..source_index + 4) else {
                continue;
            };
            let (mut red, mut green, mut blue) = (source[2], source[1], source[0]);
            if grayscale {
                let luma =
                    (u32::from(red) * 54 + u32::from(green) * 183 + u32::from(blue) * 19) / 256;
                red = luma as u8;
                green = luma as u8;
                blue = luma as u8;
            }
            let alpha = ((u16::from(source[3]) * u16::from(opacity) + 127) / 255) as u8;
            let color = (u32::from(red) << 16) | (u32::from(green) << 8) | u32::from(blue);
            let destination_index = local_y * stride + x as usize;
            pixels[destination_index] = blend_pixel(pixels[destination_index], color, alpha);
        }
    }
}

fn rows_mut(
    pixels: &mut [u32],
    stride: usize,
    band_y: usize,
    rect: IRect,
) -> impl Iterator<Item = &mut [u32]> {
    let start_row = rect.y0 as usize - band_y;
    let row_count = rect.height() as usize;
    pixels
        .chunks_mut(stride)
        .skip(start_row)
        .take(row_count)
        .map(move |row| &mut row[rect.x0 as usize..rect.x1 as usize])
}

fn atlas_index(
    texture: &SoftwareAtlasTexture,
    tile_origin: (i32, i32),
    source_x: i32,
    source_y: i32,
) -> usize {
    ((tile_origin.1 + source_y) as usize * texture.size().width.0 as usize
        + (tile_origin.0 + source_x) as usize)
        * texture.bytes_per_pixel()
}

fn source_position(
    x: i32,
    y: i32,
    destination: IRect,
    untransformed: IRect,
    inverse: Option<[f32; 6]>,
    tile_size: (i32, i32),
) -> Option<(i32, i32)> {
    if inverse.is_none()
        && destination.width() == tile_size.0
        && destination.height() == tile_size.1
    {
        return (x >= destination.x0
            && x < destination.x1
            && y >= destination.y0
            && y < destination.y1)
            .then_some((x - destination.x0, y - destination.y0));
    }
    let point = [x as f32 + 0.5, y as f32 + 0.5];
    let point = if let Some([a, b, c, d, translation_x, translation_y]) = inverse {
        [
            a * point[0] + b * point[1] + translation_x,
            c * point[0] + d * point[1] + translation_y,
        ]
    } else {
        point
    };
    let source_bounds = if inverse.is_some() {
        untransformed
    } else {
        destination
    };
    if point[0] < source_bounds.x0 as f32
        || point[1] < source_bounds.y0 as f32
        || point[0] >= source_bounds.x1 as f32
        || point[1] >= source_bounds.y1 as f32
    {
        return None;
    }
    let u = (point[0] - source_bounds.x0 as f32) / source_bounds.width() as f32;
    let v = (point[1] - source_bounds.y0 as f32) / source_bounds.height() as f32;
    Some((
        (u * tile_size.0 as f32)
            .floor()
            .clamp(0.0, (tile_size.0 - 1) as f32) as i32,
        (v * tile_size.1 as f32)
            .floor()
            .clamp(0.0, (tile_size.1 - 1) as f32) as i32,
    ))
}

pub(crate) fn blend_pixel(destination: u32, source: u32, alpha: u8) -> u32 {
    if alpha == 0 {
        return destination | 0xff00_0000;
    }
    if alpha == 255 {
        return source | 0xff00_0000;
    }
    let blend = |shift: u32| {
        let destination = ((destination >> shift) & 0xff) as i32;
        let source = ((source >> shift) & 0xff) as i32;
        (destination + (((source - destination) * i32::from(alpha) + 128) >> 8)) as u32
    };
    0xff00_0000 | (blend(16) << 16) | (blend(8) << 8) | blend(0)
}

fn blend_subpixel(destination: u32, source: u32, alpha: [u8; 3]) -> u32 {
    let blend = |shift: u32, alpha: u8| {
        let destination = ((destination >> shift) & 0xff) as i32;
        let source = ((source >> shift) & 0xff) as i32;
        if alpha == 255 {
            source as u32
        } else {
            (destination + (((source - destination) * i32::from(alpha) + 128) >> 8)) as u32
        }
    };
    0xff00_0000 | (blend(16, alpha[0]) << 16) | (blend(8, alpha[1]) << 8) | blend(0, alpha[2])
}

#[cfg(test)]
mod tests {
    #[cfg(target_arch = "x86_64")]
    use std::borrow::Cow;

    #[cfg(target_arch = "x86_64")]
    use gpui::{
        AtlasKey, DevicePixels, ImageId, PlatformAtlas, RenderImageParams, RenderSvgParams,
        SharedString, Size,
    };
    use rand::{Rng, SeedableRng, rngs::StdRng};

    #[cfg(target_arch = "x86_64")]
    use crate::atlas::SoftwareAtlas;

    use super::*;

    #[test]
    fn integer_sprite_coordinates_match_general_sampling() {
        for width in [1, 2, 6, 13, 37, 1025] {
            for origin in [-10, 0, 3, 1000] {
                let destination = IRect {
                    x0: origin,
                    y0: -3,
                    x1: origin + width,
                    y1: 7,
                };
                for y in -4..8 {
                    for x in origin - 1..=origin + width {
                        assert_eq!(
                            source_position(x, y, destination, destination, None, (width, 10)),
                            source_position(
                                x,
                                y,
                                destination,
                                destination,
                                Some([1.0, 0.0, 0.0, 1.0, 0.0, 0.0]),
                                (width, 10)
                            ),
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn selected_fill_kernels_match_scalar_reference() {
        let mut random = StdRng::seed_from_u64(0x5eed);
        for width in 1..40 {
            let rect = IRect {
                x0: 3,
                y0: 0,
                x1: 3 + width,
                y1: 2,
            };
            let mut actual = (0..100)
                .map(|_| random.random::<u32>() | 0xff00_0000)
                .collect::<Vec<_>>();
            let mut expected = actual.clone();
            let color = random.random::<u32>();
            fill_blend(&mut actual, 50, 0, rect, color);
            // SAFETY: the test provides a valid two-row framebuffer and in-bounds rectangle.
            unsafe { fill_blend_scalar(&mut expected, 50, 0, rect, color) };
            assert_eq!(actual, expected, "width {width}");
        }
    }

    #[cfg(target_arch = "x86_64")]
    #[test]
    fn avx2_blits_match_scalar_reference() {
        if !std::is_x86_feature_detected!("avx2") {
            return;
        }
        let mut random = StdRng::seed_from_u64(0xa72_5eed);
        for width in [3, 8, 13, 37] {
            let tile_size = Size {
                width: DevicePixels(width),
                height: DevicePixels(2),
            };
            let destination = IRect {
                x0: 3,
                y0: 0,
                x1: 3 + width,
                y1: 2,
            };
            let stride = width as usize + 7;

            let mut mono_pixels = (0..width * 2)
                .map(|_| random.random::<u8>())
                .collect::<Vec<_>>();
            mono_pixels[0] = 17;
            let mono_atlas = SoftwareAtlas::new();
            let mono_key = AtlasKey::Svg(RenderSvgParams {
                path: SharedString::from(format!("mono-{width}")),
                size: tile_size,
            });
            let mono_tile = mono_atlas
                .get_or_insert_with(&mono_key, &mut || {
                    Ok(Some((tile_size, Cow::Borrowed(&mono_pixels))))
                })
                .expect("mono atlas insertion failed")
                .expect("mono atlas builder returned no tile");
            let mono_state = mono_atlas.lock();
            let mono_texture = mono_state
                .texture(mono_tile.texture_id)
                .expect("mono texture was retired");
            let mut mono_lut = AlphaLut(std::array::from_fn(|_| u32::from(random.random::<u8>())));
            mono_lut.0[17] = 255;
            let color = random.random::<u32>();
            let mut actual = (0..stride * 2)
                .map(|_| random.random::<u32>() | 0xff00_0000)
                .collect::<Vec<_>>();
            let mut expected = actual.clone();
            // SAFETY: AVX2 was detected and all source and destination rectangles are in bounds.
            unsafe {
                avx2::blit_mono(
                    &mut actual,
                    stride,
                    0,
                    destination,
                    destination,
                    mono_texture,
                    (mono_tile.bounds.origin.x.0, mono_tile.bounds.origin.y.0),
                    color,
                    &mono_lut,
                )
            };
            blit_mono_scalar(
                &mut expected,
                stride,
                0,
                destination,
                destination,
                destination,
                None,
                mono_texture,
                (mono_tile.bounds.origin.x.0, mono_tile.bounds.origin.y.0),
                (width, 2),
                color,
                &mono_lut,
            );
            assert_eq!(actual, expected, "monochrome width {width}");

            let mut color_pixels = (0..width * 2 * 4)
                .map(|_| random.random::<u8>())
                .collect::<Vec<_>>();
            color_pixels[..3].fill(17);
            color_pixels[3] = 255;
            let color_atlas = SoftwareAtlas::new();
            let color_key = AtlasKey::Image(RenderImageParams {
                image_id: ImageId(width as usize),
                frame_index: 0,
            });
            let color_tile = color_atlas
                .get_or_insert_with(&color_key, &mut || {
                    Ok(Some((tile_size, Cow::Borrowed(&color_pixels))))
                })
                .expect("color atlas insertion failed")
                .expect("color atlas builder returned no tile");
            let color_state = color_atlas.lock();
            let color_texture = color_state
                .texture(color_tile.texture_id)
                .expect("color texture was retired");

            for is_bgr in [false, true] {
                let mut subpixel_lut = AlphaLut3(std::array::from_fn(|_| {
                    std::array::from_fn(|_| u32::from(random.random::<u8>()))
                }));
                for channel in &mut subpixel_lut.0 {
                    channel[17] = 255;
                }
                let mut actual = (0..stride * 2)
                    .map(|_| random.random::<u32>() | 0xff00_0000)
                    .collect::<Vec<_>>();
                let mut expected = actual.clone();
                // SAFETY: AVX2 was detected and all source and destination rectangles are in bounds.
                unsafe {
                    avx2::blit_subpixel(
                        &mut actual,
                        stride,
                        0,
                        destination,
                        destination,
                        color_texture,
                        (color_tile.bounds.origin.x.0, color_tile.bounds.origin.y.0),
                        color,
                        &subpixel_lut,
                        is_bgr,
                    )
                };
                blit_subpixel_scalar(
                    &mut expected,
                    stride,
                    0,
                    destination,
                    destination,
                    destination,
                    None,
                    color_texture,
                    (color_tile.bounds.origin.x.0, color_tile.bounds.origin.y.0),
                    (width, 2),
                    color,
                    &subpixel_lut,
                    is_bgr,
                );
                assert_eq!(actual, expected, "subpixel width {width}, is_bgr {is_bgr}");
            }

            for grayscale in [false, true] {
                let opacity = if grayscale {
                    random.random::<u8>()
                } else {
                    255
                };
                let mut actual = (0..stride * 2)
                    .map(|_| random.random::<u32>() | 0xff00_0000)
                    .collect::<Vec<_>>();
                let mut expected = actual.clone();
                // SAFETY: AVX2 was detected and all source and destination rectangles are in bounds.
                unsafe {
                    avx2::blit_polychrome(
                        &mut actual,
                        stride,
                        0,
                        destination,
                        destination,
                        color_texture,
                        (color_tile.bounds.origin.x.0, color_tile.bounds.origin.y.0),
                        opacity,
                        grayscale,
                    )
                };
                blit_polychrome_scalar(
                    &mut expected,
                    stride,
                    0,
                    destination,
                    destination,
                    color_texture,
                    (color_tile.bounds.origin.x.0, color_tile.bounds.origin.y.0),
                    (width, 2),
                    opacity,
                    grayscale,
                );
                assert_eq!(
                    actual, expected,
                    "polychrome width {width}, grayscale {grayscale}"
                );
            }
        }
    }
}

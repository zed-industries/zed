use std::arch::x86_64::*;

use crate::{
    atlas::SoftwareAtlasTexture,
    kernels::{blend_pixel, fill_blend_scalar},
    lower::IRect,
    text_correction::{AlphaLut, AlphaLut3},
};

#[target_feature(enable = "avx2")]
pub(super) unsafe fn fill_opaque(
    pixels: &mut [u32],
    stride: usize,
    band_y: usize,
    rect: IRect,
    color: u32,
) {
    let color = color | 0xff00_0000;
    let vector = _mm256_set1_epi32(color as i32);
    for y in rect.y0..rect.y1 {
        let start = (y as usize - band_y) * stride + rect.x0 as usize;
        let row = &mut pixels[start..start + rect.width() as usize];
        let mut chunks = row.chunks_exact_mut(8);
        for chunk in &mut chunks {
            // SAFETY: chunks_exact_mut guarantees eight writable u32 values; unaligned stores are supported.
            unsafe { _mm256_storeu_si256(chunk.as_mut_ptr().cast(), vector) };
        }
        chunks.into_remainder().fill(color);
    }
}

#[target_feature(enable = "avx2")]
pub(super) unsafe fn fill_blend(
    pixels: &mut [u32],
    stride: usize,
    band_y: usize,
    rect: IRect,
    color: u32,
) {
    let alpha = (color >> 24) as u16;
    if alpha == 0 {
        return;
    }
    if alpha == 255 {
        // SAFETY: forwarded slice and rectangle have the same contract as this function.
        unsafe { fill_opaque(pixels, stride, band_y, rect, color) };
        return;
    }
    let source = _mm256_set1_epi32(color as i32);
    let zero = _mm256_setzero_si256();
    let alpha = _mm256_set1_epi16(alpha as i16);
    let inverse_alpha = _mm256_set1_epi16((256 - ((color >> 24) & 0xff)) as i16);
    let rounding = _mm256_set1_epi16(128);
    let opaque_alpha = _mm256_set1_epi32(0xff00_0000u32 as i32);
    for y in rect.y0..rect.y1 {
        let start = (y as usize - band_y) * stride + rect.x0 as usize;
        let row = &mut pixels[start..start + rect.width() as usize];
        let mut chunks = row.chunks_exact_mut(8);
        for chunk in &mut chunks {
            // SAFETY: chunks_exact_mut guarantees eight readable and writable u32 values.
            let destination = unsafe { _mm256_loadu_si256(chunk.as_ptr().cast()) };
            let destination_low = _mm256_unpacklo_epi8(destination, zero);
            let destination_high = _mm256_unpackhi_epi8(destination, zero);
            let source_low = _mm256_unpacklo_epi8(source, zero);
            let source_high = _mm256_unpackhi_epi8(source, zero);
            let low = _mm256_srli_epi16::<8>(_mm256_add_epi16(
                _mm256_add_epi16(
                    _mm256_mullo_epi16(destination_low, inverse_alpha),
                    _mm256_mullo_epi16(source_low, alpha),
                ),
                rounding,
            ));
            let high = _mm256_srli_epi16::<8>(_mm256_add_epi16(
                _mm256_add_epi16(
                    _mm256_mullo_epi16(destination_high, inverse_alpha),
                    _mm256_mullo_epi16(source_high, alpha),
                ),
                rounding,
            ));
            let blended = _mm256_or_si256(_mm256_packus_epi16(low, high), opaque_alpha);
            // SAFETY: chunks_exact_mut guarantees eight writable u32 values.
            unsafe { _mm256_storeu_si256(chunk.as_mut_ptr().cast(), blended) };
        }
        let remainder = chunks.into_remainder();
        if !remainder.is_empty() {
            let remainder_rect = IRect {
                x0: rect.x1 - remainder.len() as i32,
                x1: rect.x1,
                y0: y,
                y1: y + 1,
            };
            // SAFETY: the remainder rectangle is within the original validated row.
            unsafe { fill_blend_scalar(pixels, stride, band_y, remainder_rect, color) };
        }
    }
}

#[target_feature(enable = "avx2")]
#[allow(clippy::too_many_arguments)]
pub(super) unsafe fn blit_mono(
    pixels: &mut [u32],
    stride: usize,
    band_y: usize,
    rect: IRect,
    destination: IRect,
    texture: &SoftwareAtlasTexture,
    tile_origin: (i32, i32),
    color: u32,
    lut: &AlphaLut,
) {
    let width = rect.width() as usize;
    let vector_width = width / 8 * 8;
    for y in rect.y0..rect.y1 {
        let destination_start = (y as usize - band_y) * stride + rect.x0 as usize;
        let source_start = ((tile_origin.1 + y - destination.y0) as usize
            * texture.size().width.0 as usize
            + (tile_origin.0 + rect.x0 - destination.x0) as usize)
            * texture.bytes_per_pixel();
        for offset in (0..vector_width).step_by(8) {
            // SAFETY: the one-to-one fast path validates the tile dimensions and this loop stays in the row.
            let coverage = unsafe {
                std::ptr::read_unaligned(
                    texture
                        .pixels()
                        .as_ptr()
                        .add(source_start + offset)
                        .cast::<u64>(),
                )
            };
            let indices = _mm256_cvtepu8_epi32(_mm_cvtsi64_si128(coverage as i64));
            // SAFETY: every gathered index is an unsigned byte and the LUT has 256 entries.
            let alpha = unsafe { _mm256_i32gather_epi32(lut.0.as_ptr().cast(), indices, 4) };
            // SAFETY: the destination chunk contains eight pixels and AVX2 was checked by the caller.
            let destination_pixels = unsafe {
                _mm256_loadu_si256(pixels.as_ptr().add(destination_start + offset).cast())
            };
            // SAFETY: all arguments are initialized AVX2 vectors.
            let blended = unsafe { blend_constant(destination_pixels, color, alpha) };
            // SAFETY: the destination chunk contains eight writable pixels.
            unsafe {
                _mm256_storeu_si256(
                    pixels.as_mut_ptr().add(destination_start + offset).cast(),
                    blended,
                )
            };
        }
        for offset in vector_width..width {
            let coverage = texture.pixels()[source_start + offset];
            let alpha = lut.0[coverage as usize] as u8;
            let index = destination_start + offset;
            pixels[index] = blend_pixel(pixels[index], color, alpha);
        }
    }
}

#[target_feature(enable = "avx2")]
#[allow(clippy::too_many_arguments)]
pub(super) unsafe fn blit_subpixel(
    pixels: &mut [u32],
    stride: usize,
    band_y: usize,
    rect: IRect,
    destination: IRect,
    texture: &SoftwareAtlasTexture,
    tile_origin: (i32, i32),
    color: u32,
    lut: &AlphaLut3,
    is_bgr: bool,
) {
    let width = rect.width() as usize;
    let vector_width = width / 8 * 8;
    let byte_width = texture.bytes_per_pixel();
    let channel_mask = _mm256_set1_epi32(0xff);
    for y in rect.y0..rect.y1 {
        let destination_start = (y as usize - band_y) * stride + rect.x0 as usize;
        let source_start = ((tile_origin.1 + y - destination.y0) as usize
            * texture.size().width.0 as usize
            + (tile_origin.0 + rect.x0 - destination.x0) as usize)
            * byte_width;
        for offset in (0..vector_width).step_by(8) {
            // SAFETY: the fast path validates 4-byte one-to-one source pixels and stays in the row.
            let source = unsafe {
                _mm256_loadu_si256(
                    texture
                        .pixels()
                        .as_ptr()
                        .add(source_start + offset * byte_width)
                        .cast(),
                )
            };
            let first = _mm256_and_si256(source, channel_mask);
            let green = _mm256_and_si256(_mm256_srli_epi32::<8>(source), channel_mask);
            let third = _mm256_and_si256(_mm256_srli_epi32::<16>(source), channel_mask);
            let (red, blue) = if is_bgr {
                (third, first)
            } else {
                (first, third)
            };
            // SAFETY: coverage lanes are in 0..=255 and every LUT has 256 entries.
            let red_alpha = unsafe { _mm256_i32gather_epi32(lut.0[0].as_ptr().cast(), red, 4) };
            // SAFETY: coverage lanes are in 0..=255 and every LUT has 256 entries.
            let green_alpha = unsafe { _mm256_i32gather_epi32(lut.0[1].as_ptr().cast(), green, 4) };
            // SAFETY: coverage lanes are in 0..=255 and every LUT has 256 entries.
            let blue_alpha = unsafe { _mm256_i32gather_epi32(lut.0[2].as_ptr().cast(), blue, 4) };
            // SAFETY: the destination chunk contains eight pixels.
            let destination_pixels = unsafe {
                _mm256_loadu_si256(pixels.as_ptr().add(destination_start + offset).cast())
            };
            let destination_red =
                _mm256_and_si256(_mm256_srli_epi32::<16>(destination_pixels), channel_mask);
            let destination_green =
                _mm256_and_si256(_mm256_srli_epi32::<8>(destination_pixels), channel_mask);
            let destination_blue = _mm256_and_si256(destination_pixels, channel_mask);
            // SAFETY: all arguments are initialized AVX2 vectors.
            let red = unsafe {
                blend_channel(
                    destination_red,
                    _mm256_set1_epi32(((color >> 16) & 0xff) as i32),
                    red_alpha,
                )
            };
            // SAFETY: all arguments are initialized AVX2 vectors.
            let green = unsafe {
                blend_channel(
                    destination_green,
                    _mm256_set1_epi32(((color >> 8) & 0xff) as i32),
                    green_alpha,
                )
            };
            // SAFETY: all arguments are initialized AVX2 vectors.
            let blue = unsafe {
                blend_channel(
                    destination_blue,
                    _mm256_set1_epi32((color & 0xff) as i32),
                    blue_alpha,
                )
            };
            let blended = _mm256_or_si256(
                _mm256_set1_epi32(0xff00_0000u32 as i32),
                _mm256_or_si256(
                    _mm256_slli_epi32::<16>(red),
                    _mm256_or_si256(_mm256_slli_epi32::<8>(green), blue),
                ),
            );
            // SAFETY: the destination chunk contains eight writable pixels.
            unsafe {
                _mm256_storeu_si256(
                    pixels.as_mut_ptr().add(destination_start + offset).cast(),
                    blended,
                )
            };
        }
        for offset in vector_width..width {
            let source_index = source_start + offset * byte_width;
            let sample = &texture.pixels()[source_index..source_index + 3];
            let coverage = if is_bgr {
                [sample[2], sample[1], sample[0]]
            } else {
                [sample[0], sample[1], sample[2]]
            };
            let index = destination_start + offset;
            pixels[index] = blend_subpixel_scalar(
                pixels[index],
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

#[target_feature(enable = "avx2")]
#[allow(clippy::too_many_arguments)]
pub(super) unsafe fn blit_polychrome(
    pixels: &mut [u32],
    stride: usize,
    band_y: usize,
    rect: IRect,
    destination: IRect,
    texture: &SoftwareAtlasTexture,
    tile_origin: (i32, i32),
    opacity: u8,
    grayscale: bool,
) {
    let width = rect.width() as usize;
    let vector_width = width / 8 * 8;
    let byte_width = texture.bytes_per_pixel();
    let channel_mask = _mm256_set1_epi32(0xff);
    for y in rect.y0..rect.y1 {
        let destination_start = (y as usize - band_y) * stride + rect.x0 as usize;
        let source_start = ((tile_origin.1 + y - destination.y0) as usize
            * texture.size().width.0 as usize
            + (tile_origin.0 + rect.x0 - destination.x0) as usize)
            * byte_width;
        for offset in (0..vector_width).step_by(8) {
            // SAFETY: the fast path validates 4-byte one-to-one source pixels and stays in the row.
            let source = unsafe {
                _mm256_loadu_si256(
                    texture
                        .pixels()
                        .as_ptr()
                        .add(source_start + offset * byte_width)
                        .cast(),
                )
            };
            let mut blue = _mm256_and_si256(source, channel_mask);
            let mut green = _mm256_and_si256(_mm256_srli_epi32::<8>(source), channel_mask);
            let mut red = _mm256_and_si256(_mm256_srli_epi32::<16>(source), channel_mask);
            let source_alpha = _mm256_and_si256(_mm256_srli_epi32::<24>(source), channel_mask);
            let product = _mm256_mullo_epi32(source_alpha, _mm256_set1_epi32(i32::from(opacity)));
            let rounded = _mm256_add_epi32(product, _mm256_set1_epi32(128));
            let alpha =
                _mm256_srli_epi32::<8>(_mm256_add_epi32(rounded, _mm256_srli_epi32::<8>(rounded)));
            if grayscale {
                let luma = _mm256_srli_epi32::<8>(_mm256_add_epi32(
                    _mm256_add_epi32(
                        _mm256_mullo_epi32(red, _mm256_set1_epi32(54)),
                        _mm256_mullo_epi32(green, _mm256_set1_epi32(183)),
                    ),
                    _mm256_mullo_epi32(blue, _mm256_set1_epi32(19)),
                ));
                red = luma;
                green = luma;
                blue = luma;
            }
            // SAFETY: the destination chunk contains eight pixels.
            let destination_pixels = unsafe {
                _mm256_loadu_si256(pixels.as_ptr().add(destination_start + offset).cast())
            };
            // SAFETY: all arguments are initialized AVX2 vectors.
            red = unsafe {
                blend_channel(
                    _mm256_and_si256(_mm256_srli_epi32::<16>(destination_pixels), channel_mask),
                    red,
                    alpha,
                )
            };
            // SAFETY: all arguments are initialized AVX2 vectors.
            green = unsafe {
                blend_channel(
                    _mm256_and_si256(_mm256_srli_epi32::<8>(destination_pixels), channel_mask),
                    green,
                    alpha,
                )
            };
            // SAFETY: all arguments are initialized AVX2 vectors.
            blue = unsafe {
                blend_channel(
                    _mm256_and_si256(destination_pixels, channel_mask),
                    blue,
                    alpha,
                )
            };
            let blended = _mm256_or_si256(
                _mm256_set1_epi32(0xff00_0000u32 as i32),
                _mm256_or_si256(
                    _mm256_slli_epi32::<16>(red),
                    _mm256_or_si256(_mm256_slli_epi32::<8>(green), blue),
                ),
            );
            // SAFETY: the destination chunk contains eight writable pixels.
            unsafe {
                _mm256_storeu_si256(
                    pixels.as_mut_ptr().add(destination_start + offset).cast(),
                    blended,
                )
            };
        }
        for offset in vector_width..width {
            let source_index = source_start + offset * byte_width;
            let source = &texture.pixels()[source_index..source_index + 4];
            let (mut red, mut green, mut blue) = (source[2], source[1], source[0]);
            if grayscale {
                let luma =
                    (u32::from(red) * 54 + u32::from(green) * 183 + u32::from(blue) * 19) / 256;
                red = luma as u8;
                green = luma as u8;
                blue = luma as u8;
            }
            let alpha = ((u16::from(source[3]) * u16::from(opacity) + 127) / 255) as u8;
            let source = (u32::from(red) << 16) | (u32::from(green) << 8) | u32::from(blue);
            let index = destination_start + offset;
            pixels[index] = blend_pixel(pixels[index], source, alpha);
        }
    }
}

#[target_feature(enable = "avx2")]
unsafe fn blend_constant(destination: __m256i, color: u32, alpha: __m256i) -> __m256i {
    let mask = _mm256_set1_epi32(0xff);
    // SAFETY: all arguments are initialized AVX2 vectors.
    let red = unsafe {
        blend_channel(
            _mm256_and_si256(_mm256_srli_epi32::<16>(destination), mask),
            _mm256_set1_epi32(((color >> 16) & 0xff) as i32),
            alpha,
        )
    };
    // SAFETY: all arguments are initialized AVX2 vectors.
    let green = unsafe {
        blend_channel(
            _mm256_and_si256(_mm256_srli_epi32::<8>(destination), mask),
            _mm256_set1_epi32(((color >> 8) & 0xff) as i32),
            alpha,
        )
    };
    // SAFETY: all arguments are initialized AVX2 vectors.
    let blue = unsafe {
        blend_channel(
            _mm256_and_si256(destination, mask),
            _mm256_set1_epi32((color & 0xff) as i32),
            alpha,
        )
    };
    _mm256_or_si256(
        _mm256_set1_epi32(0xff00_0000u32 as i32),
        _mm256_or_si256(
            _mm256_slli_epi32::<16>(red),
            _mm256_or_si256(_mm256_slli_epi32::<8>(green), blue),
        ),
    )
}

#[target_feature(enable = "avx2")]
unsafe fn blend_channel(destination: __m256i, source: __m256i, alpha: __m256i) -> __m256i {
    let inverse_alpha = _mm256_sub_epi32(_mm256_set1_epi32(256), alpha);
    let blended = _mm256_srli_epi32::<8>(_mm256_add_epi32(
        _mm256_add_epi32(
            _mm256_mullo_epi32(destination, inverse_alpha),
            _mm256_mullo_epi32(source, alpha),
        ),
        _mm256_set1_epi32(128),
    ));
    let fully_covered = _mm256_cmpeq_epi32(alpha, _mm256_set1_epi32(255));
    _mm256_blendv_epi8(blended, source, fully_covered)
}

fn blend_subpixel_scalar(destination: u32, source: u32, alpha: [u8; 3]) -> u32 {
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

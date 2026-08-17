use criterion::{black_box, criterion_group, criterion_main, Criterion};
use image::{DynamicImage, ImageBuffer, Rgba};

pub fn bench_cast_intra_colorspace(c: &mut Criterion) {
    let rgb_source =
        DynamicImage::ImageRgba8(ImageBuffer::from_pixel(256, 256, Rgba([0u8, 0, 0, 255])));

    c.bench_function("cast_dynamic_rgba8_rgb8", |b| {
        b.iter(|| black_box(&rgb_source).to_rgb8());
    });

    c.bench_function("cast_dynamic_rgba8_rgba8", |b| {
        b.iter(|| black_box(&rgb_source).to_rgba8());
    });

    c.bench_function("cast_dynamic_rgba8_rgb16", |b| {
        b.iter(|| black_box(&rgb_source).to_rgb16());
    });

    c.bench_function("cast_dynamic_rgba8_rgba16", |b| {
        b.iter(|| black_box(&rgb_source).to_rgba16());
    });

    let luma_source =
        DynamicImage::ImageLuma8(ImageBuffer::from_pixel(256, 256, image::Luma([0u8])));

    c.bench_function("cast_dynamic_luma8_luma16", |b| {
        b.iter(|| black_box(&luma_source).to_luma16());
    });

    c.bench_function("cast_dynamic_luma8_luma_alpha8", |b| {
        b.iter(|| black_box(&luma_source).to_luma_alpha8());
    });

    c.bench_function("cast_dynamic_luma8_luma_alpha16", |b| {
        b.iter(|| black_box(&luma_source).to_luma_alpha16());
    });

    let la_source =
        DynamicImage::ImageLumaA8(ImageBuffer::from_pixel(256, 256, image::LumaA([0u8, 255])));

    c.bench_function("cast_dynamic_luma_alpha8_luma_alpha16", |b| {
        b.iter(|| black_box(&la_source).to_luma_alpha16());
    });

    c.bench_function("cast_dynamic_luma_alpha8_luma8", |b| {
        b.iter(|| black_box(&la_source).to_luma8());
    });

    c.bench_function("cast_dynamic_luma_alpha8_luma16", |b| {
        b.iter(|| black_box(&la_source).to_luma16());
    });

    let la_source =
        DynamicImage::ImageLumaA16(ImageBuffer::from_pixel(256, 256, image::LumaA([0u16, 255])));

    c.bench_function("cast_dynamic_luma_alpha16_luma_alpha16", |b| {
        b.iter(|| black_box(&la_source).to_luma_alpha16());
    });

    c.bench_function("cast_dynamic_luma_alpha16_luma8", |b| {
        b.iter(|| black_box(&la_source).to_luma8());
    });

    c.bench_function("cast_dynamic_luma_alpha16_luma16", |b| {
        b.iter(|| black_box(&la_source).to_luma16());
    });
}

pub fn bench_cast_with_coefficient(c: &mut Criterion) {
    let source =
        DynamicImage::ImageRgba8(ImageBuffer::from_pixel(256, 256, Rgba([0u8, 0, 0, 255])));

    c.bench_function("cast_dynamic_rgba8_l8", |b| {
        b.iter(|| black_box(&source).to_luma8());
    });

    c.bench_function("cast_dynamic_rgba8_l16", |b| {
        b.iter(|| black_box(&source).to_luma16());
    });

    c.bench_function("cast_dynamic_rgba8_la16", |b| {
        b.iter(|| black_box(&source).to_luma_alpha16());
    });
}

criterion_group!(
    benches,
    bench_cast_intra_colorspace,
    bench_cast_with_coefficient
);
criterion_main!(benches);

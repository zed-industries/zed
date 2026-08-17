use criterion::{black_box, criterion_group, criterion_main, Criterion};
use image::{GenericImage, ImageBuffer, Rgba};

pub fn bench_copy_from(c: &mut Criterion) {
    let at = rect_from_xy_ranges(256..1280, 256..1280);

    let mut target = ImageBuffer::from_pixel(2048, 2048, Rgba([0u8, 0, 0, 255]));
    let src = ImageBuffer::from_pixel(2048, 2048, Rgba([255u8, 0, 0, 255]));
    let part = ImageBuffer::from_pixel(at.width, at.height, Rgba([255u8, 0, 0, 255]));

    let view = image::GenericImageView::view(&src, at.x, at.y, at.width, at.height);

    const BG: Rgba<u8> = Rgba([0u8, 0, 0, 255]);
    let samples = image::flat::FlatSamples::with_monocolor(&BG, at.width, at.height);
    let singular = samples.as_view().unwrap();

    let mut samples = src.as_flat_samples();
    samples.layout.width = 1024;
    samples.layout.width_stride *= 2;
    samples.layout.height = 1024;
    samples.layout.height_stride *= 2;
    let skip = samples.as_view().unwrap();

    c.bench_function("copy_from", |b| {
        b.iter(|| target.copy_from(black_box(&src), 0, 0));
    });

    c.bench_function("copy_at", |b| {
        b.iter(|| target.copy_from(black_box(&part), at.x, at.y));
    });

    c.bench_function("copy_view", |b| {
        b.iter(|| target.copy_from(black_box(&*view), at.x, at.y));
    });

    c.bench_function("copy_fill", |b| {
        b.iter(|| target.copy_from(black_box(&singular), at.x, at.y));
    });

    c.bench_function("copy_strides", |b| {
        b.iter(|| target.copy_from(black_box(&skip), at.x, at.y));
    });
}

pub fn bench_copy_subimage_from(c: &mut Criterion) {
    let vp = rect_from_xy_ranges(256..1280, 256..1280);
    let at = rect_from_xy_ranges(128..512, 128..512);

    let mut target = ImageBuffer::from_pixel(2048, 2048, Rgba([0u8, 0, 0, 255]));
    let mut target = target.sub_image(vp.x, vp.y, vp.width, vp.height);

    let src = ImageBuffer::from_pixel(vp.width, vp.height, Rgba([255u8, 0, 0, 255]));
    let part = ImageBuffer::from_pixel(at.width, at.height, Rgba([255u8, 0, 0, 255]));
    let view = image::GenericImageView::view(&src, at.x, at.y, at.width, at.height);

    const BG: Rgba<u8> = Rgba([0u8, 0, 0, 255]);
    let samples = image::flat::FlatSamples::with_monocolor(&BG, at.width, at.height);
    let singular = samples.as_view().unwrap();

    let mut samples = src.as_flat_samples();
    samples.layout.width = at.width / 2;
    samples.layout.width_stride *= 2;
    samples.layout.height = at.height / 2;
    samples.layout.height_stride *= 2;
    let skip = samples.as_view().unwrap();

    c.bench_function("copy_subimage_from", |b| {
        b.iter(|| target.copy_from(black_box(&src), 0, 0));
    });

    c.bench_function("copy_subimage_at", |b| {
        b.iter(|| target.copy_from(black_box(&part), at.x, at.y));
    });

    c.bench_function("copy_subimage_view", |b| {
        b.iter(|| target.copy_from(black_box(&*view), at.x, at.y));
    });

    c.bench_function("copy_subimage_fill", |b| {
        b.iter(|| target.copy_from(black_box(&singular), at.x, at.y));
    });

    c.bench_function("copy_subimage_strides", |b| {
        b.iter(|| target.copy_from(black_box(&skip), at.x, at.y));
    });
}

criterion_group!(benches, bench_copy_from, bench_copy_subimage_from);
criterion_main!(benches);

// Backport of the constructor.
fn rect_from_xy_ranges(x: std::ops::Range<u32>, y: std::ops::Range<u32>) -> image::math::Rect {
    image::math::Rect {
        x: x.start,
        y: y.start,
        width: x.end - x.start,
        height: y.end - y.start,
    }
}

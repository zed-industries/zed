//! Rendering benchmarks.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fastrand::Rng;
use zeno::{Command, Mask, PathBuilder, Scratch, Style};

fn drawing(c: &mut Criterion) {
    // Set up buffers for rendering.
    let mut buffer = Box::new([0u8; 1024 * 1024]);
    let mut scratch = Scratch::new();
    let mut rng = Rng::with_seed(0x12345678);

    c.bench_function("fill_square", |b| {
        let path = {
            let mut path = Vec::<Command>::new();
            path.add_rect((5.0, 5.0), 1000.0, 1000.0);
            path
        };

        b.iter(|| {
            Mask::with_scratch(&path, &mut scratch)
                .style(Style::Fill(zeno::Fill::EvenOdd))
                .render_into(&mut *buffer, None);
            black_box((&mut scratch, &mut buffer));
        });
    });

    c.bench_function("complicated_shape", |b| {
        // Create a weird, jagged circle.
        let path = {
            let (center_x, center_y) = (500.0, 500.0);
            let radius = 450.0;
            let mut path = Vec::<Command>::new();

            path.move_to((center_x, center_y));

            for i in 0..500 {
                let angle = core::f32::consts::PI * 2.0 * (i as f32) / 500.0;
                let pt_x = center_x + (angle.cos() * radius) + rng.f32();
                let pt_y = center_y + (angle.sin() * radius) + rng.f32();
                path.line_to((pt_x, pt_y));
            }

            path.close();
            path
        };

        b.iter(|| {
            Mask::with_scratch(&path, &mut scratch)
                .style(Style::Fill(zeno::Fill::EvenOdd))
                .render_into(&mut *buffer, None);
            black_box((&mut scratch, &mut buffer));
        })
    });

    c.bench_function("circle", |b| {
        let path = {
            let mut path = Vec::<Command>::new();
            path.add_circle((500.0, 500.0), 450.0);
            path
        };

        b.iter(|| {
            Mask::with_scratch(&path, &mut scratch)
                .style(Style::Fill(zeno::Fill::EvenOdd))
                .render_into(&mut *buffer, None);
            black_box((&mut scratch, &mut buffer));
        });
    });
}

criterion_group!(benches, drawing);

criterion_main!(benches);

// Copyright 2025 the Kurbo Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Stroke example.
//!
//! This example has been lightly adapted from Vello, which in turn has
//! been adapted from the Skia "tricky stroke" test.

use kurbo::{Affine, BezPath, Cap, Join, Rect, Shape, Stroke, StrokeOpts, stroke};

fn tricky_strokes() {
    const CELL_SIZE: f64 = 200.;
    const STROKE_WIDTH: f64 = 30.;
    const NUM_COLS: usize = 5;

    fn stroke_bounds(pts: &[(f64, f64); 4]) -> Rect {
        use kurbo::CubicBez;
        CubicBez::new(pts[0], pts[1], pts[2], pts[3])
            .bounding_box()
            .inflate(STROKE_WIDTH, STROKE_WIDTH)
    }

    fn map_rect_to_rect(src: &Rect, dst: &Rect) -> (Affine, f64) {
        let (scale, x_larger) = {
            let sx = dst.width() / src.width();
            let sy = dst.height() / src.height();
            (sx.min(sy), sx > sy)
        };
        let tx = dst.x0 - src.x0 * scale;
        let ty = dst.y0 - src.y0 * scale;
        let (tx, ty) = if x_larger {
            (tx + 0.5 * (dst.width() - src.width() * scale), ty)
        } else {
            (tx, ty + 0.5 * (dst.height() - src.height() * scale))
        };
        (Affine::new([scale, 0.0, 0.0, scale, tx, ty]), scale)
    }

    let tricky_cubics = [
        [(122., 737.), (348., 553.), (403., 761.), (400., 760.)],
        [(244., 520.), (244., 518.), (1141., 634.), (394., 688.)],
        [(550., 194.), (138., 130.), (1035., 246.), (288., 300.)],
        [(226., 733.), (556., 779.), (-43., 471.), (348., 683.)],
        [(268., 204.), (492., 304.), (352., 23.), (433., 412.)],
        [(172., 480.), (396., 580.), (256., 299.), (338., 677.)],
        [(731., 340.), (318., 252.), (1026., -64.), (367., 265.)],
        [(475., 708.), (62., 620.), (770., 304.), (220., 659.)],
        [(0., 0.), (128., 128.), (128., 0.), (0., 128.)], // Perfect cusp
        [(0., 0.01), (128., 127.999), (128., 0.01), (0., 127.99)], // Near-cusp
        [(0., -0.01), (128., 128.001), (128., -0.01), (0., 128.001)], // Near-cusp
        [(0., 0.), (0., -10.), (0., -10.), (0., 10.)],    // Flat line with 180
        [(10., 0.), (0., 0.), (20., 0.), (10., 0.)],      // Flat line with 2 180s
        [(39., -39.), (40., -40.), (40., -40.), (0., 0.)], // Flat diagonal with 180
        [(40., 40.), (0., 0.), (200., 200.), (0., 0.)],   // Diag w/ an internal 180
        [(0., 0.), (1e-2, 0.), (-1e-2, 0.), (0., 0.)],    // Circle
        // Flat line with no turns:
        [
            (400.75, 100.05),
            (400.75, 100.05),
            (100.05, 300.95),
            (100.05, 300.95),
        ],
        [(0.5, 0.), (0., 0.), (20., 0.), (10., 0.)], // Flat line with 2 180s
        [(10., 0.), (0., 0.), (10., 0.), (10., 0.)], // Flat line with a 180
    ];

    // Flat conic with a cusp: (1,1) (2,1) (1,1), weight: 1
    let flat_quad = [
        // moveTo(1., 1.),
        [(2., 1.), (1., 1.)],
    ];
    // Flat conic with a cusp: (1,1) (100,1) (25,1), weight: 0.3
    let flat_conic_as_quads = [
        // moveTo(1., 1.),
        [(2.232486, 1.000000), (3.471740, 1.000000)],
        [(4.710995, 1.000000), (5.949262, 1.000000)],
        [(7.187530, 1.000000), (8.417061, 1.000000)],
        [(9.646591, 1.000000), (10.859690, 1.000000)],
        [(12.072789, 1.000000), (13.261865, 1.000000)],
        [(14.450940, 1.000000), (15.608549, 1.000000)],
        [(16.766161, 1.000000), (17.885059, 1.000000)],
        [(19.003958, 1.000000), (20.077141, 1.000000)],
        [(21.150328, 1.000000), (22.171083, 1.000000)],
        [(23.191839, 1.000000), (24.153776, 1.000000)],
        [(25.115715, 1.000000), (26.012812, 1.000000)],
        [(26.909912, 1.000000), (27.736557, 1.000000)],
        [(28.563202, 1.000000), (29.314220, 1.000000)],
        [(30.065239, 1.000000), (30.735928, 1.000000)],
        [(31.406620, 1.000000), (31.992788, 1.000000)],
        [(32.578957, 1.000000), (33.076927, 1.000000)],
        [(33.574905, 1.000000), (33.981567, 1.000000)],
        [(34.388233, 1.000000), (34.701038, 1.000000)],
        [(35.013851, 1.000000), (35.230850, 1.000000)],
        [(35.447845, 1.000000), (35.567669, 1.000000)],
        [(35.687500, 1.000000), (35.709404, 1.000000)],
        [(35.731312, 1.000000), (35.655155, 1.000000)],
        [(35.579006, 1.000000), (35.405273, 1.000000)],
        [(35.231541, 1.000000), (34.961311, 1.000000)],
        [(34.691086, 1.000000), (34.326057, 1.000000)],
        [(33.961029, 1.000000), (33.503479, 1.000000)],
        [(33.045937, 1.000000), (32.498734, 1.000000)],
        [(31.951530, 1.000000), (31.318098, 1.000000)],
        [(30.684669, 1.000000), (29.968971, 1.000000)],
        [(29.253277, 1.000000), (28.459791, 1.000000)],
        [(27.666309, 1.000000), (26.800005, 1.000000)],
        [(25.933704, 1.000000), (25.000000, 1.000000)],
    ];
    // Flat conic with a cusp: (1,1) (100,1) (25,1), weight: 1.5
    let bigger_flat_conic_as_quads = [
        // moveTo(1., 1.),
        [(8.979845, 1.000000), (15.795975, 1.000000)],
        [(22.612104, 1.000000), (28.363287, 1.000000)],
        [(34.114471, 1.000000), (38.884045, 1.000000)],
        [(43.653618, 1.000000), (47.510696, 1.000000)],
        [(51.367767, 1.000000), (54.368233, 1.000000)],
        [(57.368698, 1.000000), (59.556030, 1.000000)],
        [(61.743366, 1.000000), (63.149269, 1.000000)],
        [(64.555168, 1.000000), (65.200005, 1.000000)],
        [(65.844841, 1.000000), (65.737961, 1.000000)],
        [(65.631073, 1.000000), (64.770912, 1.000000)],
        [(63.910763, 1.000000), (62.284878, 1.000000)],
        [(60.658997, 1.000000), (58.243816, 1.000000)],
        [(55.828640, 1.000000), (52.589172, 1.000000)],
        [(49.349705, 1.000000), (45.239006, 1.000000)],
        [(41.128315, 1.000000), (36.086826, 1.000000)],
        [(31.045338, 1.000000), (25.000000, 1.000000)],
    ];

    let mut idx = 0;
    let tolerance = 0.25;
    for cubic in &tricky_cubics {
        let x = (idx % NUM_COLS) as f64 * CELL_SIZE;
        let y = (idx / NUM_COLS) as f64 * CELL_SIZE;
        let cell = Rect::new(x, y, x + CELL_SIZE, y + CELL_SIZE);
        let bounds = stroke_bounds(cubic);
        let (t, s) = map_rect_to_rect(&bounds, &cell);
        let style = Stroke::new(STROKE_WIDTH / s)
            .with_caps(Cap::Butt)
            .with_join(Join::Miter);
        let mut path = BezPath::new();
        path.move_to(cubic[0]);
        path.curve_to(cubic[1], cubic[2], cubic[3]);
        let stroked = stroke(path, &style, &StrokeOpts::default(), tolerance);
        println!(
            "  <path d='{}' stroke='#000' fill='#aec' stroke-width='1'/>",
            (t * stroked).to_svg()
        );
        idx += 1;
    }

    let flat_curves = [
        flat_quad.as_slice(),
        flat_conic_as_quads.as_slice(),
        bigger_flat_conic_as_quads.as_slice(),
    ];
    for quads in flat_curves.iter() {
        let mut path = BezPath::new();
        path.move_to((1., 1.));
        for quad in quads.iter() {
            path.quad_to(quad[0], quad[1]);
        }
        let x = (idx % NUM_COLS) as f64 * CELL_SIZE;
        let y = (idx / NUM_COLS) as f64 * CELL_SIZE;
        let cell = Rect::new(x, y, x + CELL_SIZE, y + CELL_SIZE);
        let bounds = path.bounding_box().inflate(STROKE_WIDTH, STROKE_WIDTH);
        let (t, s) = map_rect_to_rect(&bounds, &cell);
        let style = Stroke::new(STROKE_WIDTH / s)
            .with_caps(Cap::Butt)
            .with_join(Join::Miter);
        let stroked = stroke(path, &style, &StrokeOpts::default(), tolerance);
        println!(
            "  <path d='{}' stroke='#000' fill='#aec' stroke-width='1'/>",
            (t * stroked).to_svg()
        );
        idx += 1;
    }
}

fn main() {
    println!("<svg width='1000' height='1000' xmlns='http://www.w3.org/2000/svg'>");
    tricky_strokes();
    println!("</svg>");
}

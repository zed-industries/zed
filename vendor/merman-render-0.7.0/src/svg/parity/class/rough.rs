use super::super::*;

fn roughjs46_next_f64(seed: &mut u32) -> f64 {
    if *seed == 0 {
        return 0.0;
    }
    let prod = seed.wrapping_mul(48_271);
    *seed = prod & 0x7fff_ffff;
    (*seed as f64) / 2_147_483_648.0
}

fn roughjs46_diverge_point(seed: &mut u32) -> f64 {
    0.2 + roughjs46_next_f64(seed) * 0.2
}

pub(super) fn class_rough_seed(_diagram_id: &str, _dom_id: &str) -> u64 {
    1
}

pub(super) fn class_rough_line_double_path_and_bounds(
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    seed: u64,
) -> (String, super::super::path_bounds::SvgPathBounds) {
    let dx = x2 - x1;
    let dy = y2 - y1;
    let mut s = seed as u32;
    let mut d = String::new();
    let mut pb = super::super::path_bounds::SvgPathBounds {
        min_x: x1,
        min_y: y1,
        max_x: x1,
        max_y: y1,
    };

    for idx in 0..2 {
        let diverge = roughjs46_diverge_point(&mut s);
        for _ in 0..10 {
            let _ = roughjs46_next_f64(&mut s);
        }

        let c1x = x1 + dx * diverge;
        let c1y = y1 + dy * diverge;
        let c2x = x1 + dx * 2.0 * diverge;
        let c2y = y1 + dy * 2.0 * diverge;
        if idx > 0 {
            d.push(' ');
        }
        let _ = write!(
            &mut d,
            "M{} {} C{} {}, {} {}, {} {}",
            fmt(x1),
            fmt(y1),
            fmt(c1x),
            fmt(c1y),
            fmt(c2x),
            fmt(c2y),
            fmt(x2),
            fmt(y2),
        );
        super::super::path_bounds::svg_path_bounds_include_cubic(
            &mut pb,
            super::super::path_bounds::CubicBezier {
                x0: x1,
                y0: y1,
                x1: c1x,
                y1: c1y,
                x2: c2x,
                y2: c2y,
                x3: x2,
                y3: y2,
            },
        );
    }

    (d, pb)
}

pub(super) fn class_rough_rect_stroke_path_and_bounds(
    left: f64,
    top: f64,
    width: f64,
    height: f64,
    seed: u64,
) -> (String, super::super::path_bounds::SvgPathBounds) {
    let right = left + width;
    let bottom = top + height;
    let mut s = seed as u32;
    let mut out = String::new();
    let mut pb_opt: Option<super::super::path_bounds::SvgPathBounds> = None;

    for (idx, (ax, ay, bx, by)) in [
        (left, top, right, top),
        (right, top, right, bottom),
        (right, bottom, left, bottom),
        (left, bottom, left, top),
    ]
    .into_iter()
    .enumerate()
    {
        let (segment, seg_pb) = class_rough_line_double_path_and_bounds(ax, ay, bx, by, s as u64);
        for _ in 0..22 {
            let _ = roughjs46_next_f64(&mut s);
        }
        if idx > 0 {
            out.push(' ');
        }
        out.push_str(&segment);
        if let Some(pb) = pb_opt.as_mut() {
            pb.min_x = pb.min_x.min(seg_pb.min_x);
            pb.min_y = pb.min_y.min(seg_pb.min_y);
            pb.max_x = pb.max_x.max(seg_pb.max_x);
            pb.max_y = pb.max_y.max(seg_pb.max_y);
        } else {
            pb_opt = Some(seg_pb);
        }
    }

    (
        out,
        pb_opt.unwrap_or(super::super::path_bounds::SvgPathBounds {
            min_x: left,
            min_y: top,
            max_x: right,
            max_y: bottom,
        }),
    )
}

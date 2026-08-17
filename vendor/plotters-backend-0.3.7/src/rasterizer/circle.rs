use crate::{BackendCoord, BackendStyle, DrawingBackend, DrawingErrorKind};

fn draw_part_a<
    B: DrawingBackend,
    Draw: FnMut(i32, (f64, f64)) -> Result<(), DrawingErrorKind<B::ErrorType>>,
>(
    height: f64,
    radius: u32,
    mut draw: Draw,
) -> Result<(), DrawingErrorKind<B::ErrorType>> {
    let half_width = (radius as f64 * radius as f64
        - (radius as f64 - height) * (radius as f64 - height))
        .sqrt();

    let x0 = (-half_width).ceil() as i32;
    let x1 = half_width.floor() as i32;

    let y0 = (radius as f64 - height).ceil();

    for x in x0..=x1 {
        let y1 = (radius as f64 * radius as f64 - x as f64 * x as f64).sqrt();
        check_result!(draw(x, (y0, y1)));
    }

    Ok(())
}

fn draw_part_b<
    B: DrawingBackend,
    Draw: FnMut(i32, (f64, f64)) -> Result<(), DrawingErrorKind<B::ErrorType>>,
>(
    from: f64,
    size: f64,
    mut draw: Draw,
) -> Result<(), DrawingErrorKind<B::ErrorType>> {
    let from = from.floor();
    for x in (from - size).floor() as i32..=from as i32 {
        check_result!(draw(x, (-x as f64, x as f64)));
    }
    Ok(())
}

fn draw_part_c<
    B: DrawingBackend,
    Draw: FnMut(i32, (f64, f64)) -> Result<(), DrawingErrorKind<B::ErrorType>>,
>(
    r: i32,
    r_limit: i32,
    mut draw: Draw,
) -> Result<(), DrawingErrorKind<B::ErrorType>> {
    let half_size = r as f64 / (2f64).sqrt();

    let (x0, x1) = ((-half_size).ceil() as i32, half_size.floor() as i32);

    for x in x0..x1 {
        let outer_y0 = ((r_limit as f64) * (r_limit as f64) - x as f64 * x as f64).sqrt();
        let inner_y0 = r as f64 - 1.0;
        let mut y1 = outer_y0.min(inner_y0);
        let y0 = ((r as f64) * (r as f64) - x as f64 * x as f64).sqrt();

        if y0 > y1 {
            y1 = y0.ceil();
            if y1 >= r as f64 {
                continue;
            }
        }

        check_result!(draw(x, (y0, y1)));
    }

    for x in x1 + 1..r {
        let outer_y0 = ((r_limit as f64) * (r_limit as f64) - x as f64 * x as f64).sqrt();
        let inner_y0 = r as f64 - 1.0;
        let y0 = outer_y0.min(inner_y0);
        let y1 = x as f64;

        if y1 < y0 {
            check_result!(draw(x, (y0, y1 + 1.0)));
            check_result!(draw(-x, (y0, y1 + 1.0)));
        }
    }

    Ok(())
}

fn draw_sweep_line<B: DrawingBackend, S: BackendStyle>(
    b: &mut B,
    style: &S,
    (x0, y0): BackendCoord,
    (dx, dy): (i32, i32),
    p0: i32,
    (s, e): (f64, f64),
) -> Result<(), DrawingErrorKind<B::ErrorType>> {
    let mut s = if dx < 0 || dy < 0 { -s } else { s };
    let mut e = if dx < 0 || dy < 0 { -e } else { e };
    if s > e {
        std::mem::swap(&mut s, &mut e);
    }

    let vs = s.ceil() - s;
    let ve = e - e.floor();

    if dx == 0 {
        check_result!(b.draw_line(
            (p0 + x0, s.ceil() as i32 + y0),
            (p0 + x0, e.floor() as i32 + y0),
            &style.color()
        ));
        check_result!(b.draw_pixel((p0 + x0, s.ceil() as i32 + y0 - 1), style.color().mix(vs)));
        check_result!(b.draw_pixel((p0 + x0, e.floor() as i32 + y0 + 1), style.color().mix(ve)));
    } else {
        check_result!(b.draw_line(
            (s.ceil() as i32 + x0, p0 + y0),
            (e.floor() as i32 + x0, p0 + y0),
            &style.color()
        ));
        check_result!(b.draw_pixel((s.ceil() as i32 + x0 - 1, p0 + y0), style.color().mix(vs)));
        check_result!(b.draw_pixel((e.floor() as i32 + x0 + 1, p0 + y0), style.color().mix(ve)));
    }

    Ok(())
}

fn draw_annulus<B: DrawingBackend, S: BackendStyle>(
    b: &mut B,
    center: BackendCoord,
    radius: (u32, u32),
    style: &S,
) -> Result<(), DrawingErrorKind<B::ErrorType>> {
    let a0 = ((radius.0 - radius.1) as f64).min(radius.0 as f64 * (1.0 - 1.0 / (2f64).sqrt()));
    let a1 = (radius.0 as f64 - a0 - radius.1 as f64).max(0.0);

    check_result!(draw_part_a::<B, _>(a0, radius.0, |p, r| draw_sweep_line(
        b,
        style,
        center,
        (0, 1),
        p,
        r
    )));
    check_result!(draw_part_a::<B, _>(a0, radius.0, |p, r| draw_sweep_line(
        b,
        style,
        center,
        (0, -1),
        p,
        r
    )));
    check_result!(draw_part_a::<B, _>(a0, radius.0, |p, r| draw_sweep_line(
        b,
        style,
        center,
        (1, 0),
        p,
        r
    )));
    check_result!(draw_part_a::<B, _>(a0, radius.0, |p, r| draw_sweep_line(
        b,
        style,
        center,
        (-1, 0),
        p,
        r
    )));

    if a1 > 0.0 {
        check_result!(draw_part_b::<B, _>(
            radius.0 as f64 - a0,
            a1.floor(),
            |h, (f, t)| {
                let f = f as i32;
                let t = t as i32;
                check_result!(b.draw_line(
                    (center.0 + h, center.1 + f),
                    (center.0 + h, center.1 + t),
                    &style.color()
                ));
                check_result!(b.draw_line(
                    (center.0 - h, center.1 + f),
                    (center.0 - h, center.1 + t),
                    &style.color()
                ));

                check_result!(b.draw_line(
                    (center.0 + f + 1, center.1 + h),
                    (center.0 + t - 1, center.1 + h),
                    &style.color()
                ));
                check_result!(b.draw_line(
                    (center.0 + f + 1, center.1 - h),
                    (center.0 + t - 1, center.1 - h),
                    &style.color()
                ));

                Ok(())
            }
        ));
    }

    check_result!(draw_part_c::<B, _>(
        radius.1 as i32,
        radius.0 as i32,
        |p, r| draw_sweep_line(b, style, center, (0, 1), p, r)
    ));
    check_result!(draw_part_c::<B, _>(
        radius.1 as i32,
        radius.0 as i32,
        |p, r| draw_sweep_line(b, style, center, (0, -1), p, r)
    ));
    check_result!(draw_part_c::<B, _>(
        radius.1 as i32,
        radius.0 as i32,
        |p, r| draw_sweep_line(b, style, center, (1, 0), p, r)
    ));
    check_result!(draw_part_c::<B, _>(
        radius.1 as i32,
        radius.0 as i32,
        |p, r| draw_sweep_line(b, style, center, (-1, 0), p, r)
    ));

    let d_inner = ((radius.1 as f64) / (2f64).sqrt()) as i32;
    let d_outer = (((radius.0 as f64) / (2f64).sqrt()) as i32).min(radius.1 as i32 - 1);
    let d_outer_actually = (radius.1 as i32).min(
        (radius.0 as f64 * radius.0 as f64 - radius.1 as f64 * radius.1 as f64 / 2.0)
            .sqrt()
            .ceil() as i32,
    );

    check_result!(b.draw_line(
        (center.0 - d_inner, center.1 - d_inner),
        (center.0 - d_outer, center.1 - d_outer),
        &style.color()
    ));
    check_result!(b.draw_line(
        (center.0 + d_inner, center.1 - d_inner),
        (center.0 + d_outer, center.1 - d_outer),
        &style.color()
    ));
    check_result!(b.draw_line(
        (center.0 - d_inner, center.1 + d_inner),
        (center.0 - d_outer, center.1 + d_outer),
        &style.color()
    ));
    check_result!(b.draw_line(
        (center.0 + d_inner, center.1 + d_inner),
        (center.0 + d_outer, center.1 + d_outer),
        &style.color()
    ));

    check_result!(b.draw_line(
        (center.0 - d_inner, center.1 + d_inner),
        (center.0 - d_outer_actually, center.1 + d_inner),
        &style.color()
    ));
    check_result!(b.draw_line(
        (center.0 + d_inner, center.1 - d_inner),
        (center.0 + d_inner, center.1 - d_outer_actually),
        &style.color()
    ));
    check_result!(b.draw_line(
        (center.0 + d_inner, center.1 + d_inner),
        (center.0 + d_inner, center.1 + d_outer_actually),
        &style.color()
    ));
    check_result!(b.draw_line(
        (center.0 + d_inner, center.1 + d_inner),
        (center.0 + d_outer_actually, center.1 + d_inner),
        &style.color()
    ));

    Ok(())
}

pub fn draw_circle<B: DrawingBackend, S: BackendStyle>(
    b: &mut B,
    center: BackendCoord,
    mut radius: u32,
    style: &S,
    mut fill: bool,
) -> Result<(), DrawingErrorKind<B::ErrorType>> {
    if style.color().alpha == 0.0 {
        return Ok(());
    }

    if !fill && style.stroke_width() != 1 {
        let inner_radius = radius - (style.stroke_width() / 2).min(radius);
        radius += style.stroke_width() / 2;
        if inner_radius > 0 {
            return draw_annulus(b, center, (radius, inner_radius), style);
        } else {
            fill = true;
        }
    }

    let min = (f64::from(radius) * (1.0 - (2f64).sqrt() / 2.0)).ceil() as i32;
    let max = (f64::from(radius) * (1.0 + (2f64).sqrt() / 2.0)).floor() as i32;

    let range = min..=max;

    let (up, down) = (
        range.start() + center.1 - radius as i32,
        range.end() + center.1 - radius as i32,
    );

    for dy in range {
        let dy = dy - radius as i32;
        let y = center.1 + dy;

        let lx = (f64::from(radius) * f64::from(radius)
            - (f64::from(dy) * f64::from(dy)).max(1e-5))
        .sqrt();

        let left = center.0 - lx.floor() as i32;
        let right = center.0 + lx.floor() as i32;

        let v = lx - lx.floor();

        let x = center.0 + dy;
        let top = center.1 - lx.floor() as i32;
        let bottom = center.1 + lx.floor() as i32;

        if fill {
            check_result!(b.draw_line((left, y), (right, y), &style.color()));
            check_result!(b.draw_line((x, top), (x, up - 1), &style.color()));
            check_result!(b.draw_line((x, down + 1), (x, bottom), &style.color()));
        } else {
            check_result!(b.draw_pixel((left, y), style.color().mix(1.0 - v)));
            check_result!(b.draw_pixel((right, y), style.color().mix(1.0 - v)));

            check_result!(b.draw_pixel((x, top), style.color().mix(1.0 - v)));
            check_result!(b.draw_pixel((x, bottom), style.color().mix(1.0 - v)));
        }

        check_result!(b.draw_pixel((left - 1, y), style.color().mix(v)));
        check_result!(b.draw_pixel((right + 1, y), style.color().mix(v)));
        check_result!(b.draw_pixel((x, top - 1), style.color().mix(v)));
        check_result!(b.draw_pixel((x, bottom + 1), style.color().mix(v)));
    }

    Ok(())
}

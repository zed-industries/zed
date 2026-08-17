use crate::{BackendCoord, BackendStyle, DrawingBackend, DrawingErrorKind};

pub fn draw_line<DB: DrawingBackend, S: BackendStyle>(
    back: &mut DB,
    mut from: BackendCoord,
    mut to: BackendCoord,
    style: &S,
) -> Result<(), DrawingErrorKind<DB::ErrorType>> {
    if style.color().alpha == 0.0 || style.stroke_width() == 0 {
        return Ok(());
    }

    if style.stroke_width() != 1 {
        // If the line is wider than 1px, then we need to make it a polygon
        let v = (i64::from(to.0 - from.0), i64::from(to.1 - from.1));
        let l = ((v.0 * v.0 + v.1 * v.1) as f64).sqrt();

        if l < 1e-5 {
            return Ok(());
        }

        let v = (v.0 as f64 / l, v.1 as f64 / l);

        let r = f64::from(style.stroke_width()) / 2.0;
        let mut trans = [(v.1 * r, -v.0 * r), (-v.1 * r, v.0 * r)];
        let mut vertices = vec![];

        for point in [from, to].iter() {
            for t in trans.iter() {
                vertices.push((
                    (f64::from(point.0) + t.0) as i32,
                    (f64::from(point.1) + t.1) as i32,
                ))
            }

            trans.swap(0, 1);
        }

        return back.fill_polygon(vertices, style);
    }

    if from.0 == to.0 {
        if from.1 > to.1 {
            std::mem::swap(&mut from, &mut to);
        }
        for y in from.1..=to.1 {
            check_result!(back.draw_pixel((from.0, y), style.color()));
        }
        return Ok(());
    }

    if from.1 == to.1 {
        if from.0 > to.0 {
            std::mem::swap(&mut from, &mut to);
        }
        for x in from.0..=to.0 {
            check_result!(back.draw_pixel((x, from.1), style.color()));
        }
        return Ok(());
    }

    let steep = (from.0 - to.0).abs() < (from.1 - to.1).abs();

    if steep {
        from = (from.1, from.0);
        to = (to.1, to.0);
    }

    let (from, to) = if from.0 > to.0 {
        (to, from)
    } else {
        (from, to)
    };

    let mut size_limit = back.get_size();

    if steep {
        size_limit = (size_limit.1, size_limit.0);
    }

    let grad = f64::from(to.1 - from.1) / f64::from(to.0 - from.0);

    let mut put_pixel = |(x, y): BackendCoord, b: f64| {
        if steep {
            back.draw_pixel((y, x), style.color().mix(b))
        } else {
            back.draw_pixel((x, y), style.color().mix(b))
        }
    };

    let y_step_limit =
        (f64::from(to.1.min(size_limit.1 as i32 - 1).max(0) - from.1) / grad).floor() as i32;

    let batch_start = (f64::from(from.1.min(size_limit.1 as i32 - 2).max(0) - from.1) / grad)
        .abs()
        .ceil() as i32
        + from.0;

    let batch_limit =
        to.0.min(size_limit.0 as i32 - 2)
            .min(from.0 + y_step_limit - 1);

    let mut y = f64::from(from.1) + f64::from(batch_start - from.0) * grad;

    for x in batch_start..=batch_limit {
        check_result!(put_pixel((x, y as i32), 1.0 + y.floor() - y));
        check_result!(put_pixel((x, y as i32 + 1), y - y.floor()));

        y += grad;
    }

    if to.0 > batch_limit && y < f64::from(to.1) {
        let x = batch_limit + 1;
        if 1.0 + y.floor() - y > 1e-5 {
            check_result!(put_pixel((x, y as i32), 1.0 + y.floor() - y));
        }
        if y - y.floor() > 1e-5 && y + 1.0 < f64::from(to.1) {
            check_result!(put_pixel((x, y as i32 + 1), y - y.floor()));
        }
    }

    Ok(())
}

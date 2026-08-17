use crate::{BackendCoord, BackendStyle, DrawingBackend, DrawingErrorKind};

pub fn draw_rect<B: DrawingBackend, S: BackendStyle>(
    b: &mut B,
    upper_left: BackendCoord,
    bottom_right: BackendCoord,
    style: &S,
    fill: bool,
) -> Result<(), DrawingErrorKind<B::ErrorType>> {
    if style.color().alpha == 0.0 {
        return Ok(());
    }
    let (upper_left, bottom_right) = (
        (
            upper_left.0.min(bottom_right.0),
            upper_left.1.min(bottom_right.1),
        ),
        (
            upper_left.0.max(bottom_right.0),
            upper_left.1.max(bottom_right.1),
        ),
    );

    if fill {
        if bottom_right.0 - upper_left.0 < bottom_right.1 - upper_left.1 {
            for x in upper_left.0..=bottom_right.0 {
                check_result!(b.draw_line((x, upper_left.1), (x, bottom_right.1), style));
            }
        } else {
            for y in upper_left.1..=bottom_right.1 {
                check_result!(b.draw_line((upper_left.0, y), (bottom_right.0, y), style));
            }
        }
    } else {
        b.draw_line(
            (upper_left.0, upper_left.1),
            (upper_left.0, bottom_right.1),
            style,
        )?;
        b.draw_line(
            (upper_left.0, upper_left.1),
            (bottom_right.0, upper_left.1),
            style,
        )?;
        b.draw_line(
            (bottom_right.0, bottom_right.1),
            (upper_left.0, bottom_right.1),
            style,
        )?;
        b.draw_line(
            (bottom_right.0, bottom_right.1),
            (bottom_right.0, upper_left.1),
            style,
        )?;
    }
    Ok(())
}

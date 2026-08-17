use super::hb_font_t;
use super::ot_shape::{hb_ot_shape_context_t, shape_internal};
use super::ot_shape_plan::hb_ot_shape_plan_t;
use crate::{script, Feature, GlyphBuffer, UnicodeBuffer};

/// Shapes the buffer content using provided font and features.
///
/// Consumes the buffer. You can then run [`GlyphBuffer::clear`] to get the [`UnicodeBuffer`] back
/// without allocating a new one.
///
/// If you plan to shape multiple strings using the same [`Face`] prefer [`shape_with_plan`].
/// This is because [`ShapePlan`] initialization is pretty slow and should preferably be called
/// once for each [`Face`].
pub fn shape(face: &hb_font_t, features: &[Feature], mut buffer: UnicodeBuffer) -> GlyphBuffer {
    buffer.0.guess_segment_properties();
    let plan = hb_ot_shape_plan_t::new(
        face,
        buffer.0.direction,
        buffer.0.script,
        buffer.0.language.as_ref(),
        features,
    );
    shape_with_plan(face, &plan, buffer)
}

/// Shapes the buffer content using the provided font and plan.
///
/// Consumes the buffer. You can then run [`GlyphBuffer::clear`] to get the [`UnicodeBuffer`] back
/// without allocating a new one.
///
/// It is up to the caller to ensure that the shape plan matches the properties of the provided
/// buffer, otherwise the shaping result will likely be incorrect.
///
/// # Panics
///
/// Will panic when debugging assertions are enabled if the buffer and plan have mismatched
/// properties.
pub fn shape_with_plan(
    face: &hb_font_t,
    plan: &hb_ot_shape_plan_t,
    buffer: UnicodeBuffer,
) -> GlyphBuffer {
    let mut buffer = buffer.0;
    buffer.guess_segment_properties();

    buffer.enter();

    debug_assert_eq!(buffer.direction, plan.direction);
    debug_assert_eq!(
        buffer.script.unwrap_or(script::UNKNOWN),
        plan.script.unwrap_or(script::UNKNOWN)
    );

    if buffer.len > 0 {
        // Save the original direction, we use it later.
        let target_direction = buffer.direction;

        #[cfg(feature = "wasm-shaper")]
        {
            super::shape_wasm::shape_with_wasm(face, plan, &mut buffer).unwrap_or_else(|| {
                shape_internal(&mut hb_ot_shape_context_t {
                    plan,
                    face,
                    buffer: &mut buffer,
                    target_direction,
                });
            });
        }
        #[cfg(not(feature = "wasm-shaper"))]
        {
            shape_internal(&mut hb_ot_shape_context_t {
                plan,
                face,
                buffer: &mut buffer,
                target_direction,
            });
        }
    }

    GlyphBuffer(buffer)
}

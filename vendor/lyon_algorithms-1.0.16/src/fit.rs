//! Fit paths into rectangles.

use crate::aabb::bounding_box;
use crate::math::*;
use crate::path::iterator::*;
use crate::path::Path;

/// The strategy to use when fitting (stretching, overflow, etc.)
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FitStyle {
    /// Stretch vertically and horizontally to fit the destination rectangle exactly.
    Stretch,
    /// Uniformly scale without overflow.
    Min,
    /// Uniformly scale with overflow.
    Max,
    /// Uniformly scale to fit horizontally.
    Horizontal,
    /// Uniformly scale to fit vertically.
    Vertical,
}

/// Computes a transform that fits a rectangle into another one.
pub fn fit_box(src_rect: &Box2D, dst_rect: &Box2D, style: FitStyle) -> Transform {
    let scale: Vector = vector(
        dst_rect.width() / src_rect.width(),
        dst_rect.height() / src_rect.height(),
    );

    let scale = match style {
        FitStyle::Stretch => scale,
        FitStyle::Min => {
            let s = f32::min(scale.x, scale.y);
            vector(s, s)
        }
        FitStyle::Max => {
            let s = f32::max(scale.x, scale.y);
            vector(s, s)
        }
        FitStyle::Horizontal => vector(scale.x, scale.x),
        FitStyle::Vertical => vector(scale.y, scale.y),
    };

    let src_center = src_rect.min.lerp(src_rect.max, 0.5);
    let dst_center = dst_rect.min.lerp(dst_rect.max, 0.5);

    Transform::translation(-src_center.x, -src_center.y)
        .then_scale(scale.x, scale.y)
        .then_translate(dst_center.to_vector())
}

/// Fits a path into a rectangle.
pub fn fit_path(path: &Path, output_rect: &Box2D, style: FitStyle) -> Path {
    let aabb = bounding_box(path.iter());
    let transform = fit_box(&aabb, output_rect, style);

    let mut builder = Path::builder();
    for evt in path.iter().transformed(&transform) {
        builder.path_event(evt)
    }

    builder.build()
}

#[test]
fn simple_fit() {
    fn approx_eq(a: &Box2D, b: &Box2D) -> bool {
        use crate::geom::euclid::approxeq::ApproxEq;
        let result = a.min.approx_eq(&b.min) && a.max.approx_eq(&b.max);
        if !result {
            std::println!("{a:?} == {b:?}");
        }
        result
    }

    let t = fit_box(
        &Box2D {
            min: point(0.0, 0.0),
            max: point(1.0, 1.0),
        },
        &Box2D {
            min: point(0.0, 0.0),
            max: point(2.0, 2.0),
        },
        FitStyle::Stretch,
    );

    assert!(approx_eq(
        &t.outer_transformed_box(&Box2D {
            min: point(0.0, 0.0),
            max: point(1.0, 1.0)
        }),
        &Box2D {
            min: point(0.0, 0.0),
            max: point(2.0, 2.0)
        },
    ));

    let t = fit_box(
        &Box2D {
            min: point(1.0, 2.0),
            max: point(5.0, 6.0),
        },
        &Box2D {
            min: point(0.0, 0.0),
            max: point(2.0, 8.0),
        },
        FitStyle::Stretch,
    );

    assert!(approx_eq(
        &t.outer_transformed_box(&Box2D {
            min: point(1.0, 2.0),
            max: point(5.0, 6.0)
        }),
        &Box2D {
            min: point(0.0, 0.0),
            max: point(2.0, 8.0)
        },
    ));

    let t = fit_box(
        &Box2D {
            min: point(1.0, 2.0),
            max: point(3.0, 6.0),
        },
        &Box2D {
            min: point(0.0, 0.0),
            max: point(2.0, 2.0),
        },
        FitStyle::Horizontal,
    );

    assert!(approx_eq(
        &t.outer_transformed_box(&Box2D {
            min: point(1.0, 2.0),
            max: point(3.0, 6.0)
        }),
        &Box2D {
            min: point(0.0, -1.0),
            max: point(2.0, 3.0)
        },
    ));

    let t = fit_box(
        &Box2D {
            min: point(1.0, 2.0),
            max: point(3.0, 4.0),
        },
        &Box2D {
            min: point(0.0, 0.0),
            max: point(4.0, 2.0),
        },
        FitStyle::Horizontal,
    );

    assert!(approx_eq(
        &t.outer_transformed_box(&Box2D {
            min: point(1.0, 2.0),
            max: point(3.0, 4.0)
        }),
        &Box2D {
            min: point(0.0, -1.0),
            max: point(4.0, 3.0)
        },
    ));
}

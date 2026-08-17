use crate::{
    bool_mask::LazySelect,
    num::{Arithmetics, Cbrt, One, PartialCmp, Powi, Real, Sqrt},
};

// Based on https://stackoverflow.com/q/4778147 and https://math.stackexchange.com/q/18686,
// picking A = (0, 0), B = (0, 1), C = (1, 1) gives us:
//
// (  sqrt(r1) * r2  ,  sqrt(r1) * (1 - r2) + sqrt(r1) * r2  ) =
// (  sqrt(r1) * r2  ,  sqrt(r1) - sqrt(r1) * r2 + sqrt(r1) * r2  ) =
// (  sqrt(r1) * r2  ,  sqrt(r1)  )
//
// `sqrt(r1)` gives us the scale of the triangle, `r2` the radius.
// Substituting, we get `x = scale * radius, y = scale` and thus for cone
// sampling: `scale = powf(r1, 1.0/3.0)` and `radius = sqrt(r2)`.

#[derive(Debug, PartialEq)]
pub(crate) struct HsvSample<T> {
    pub(crate) value: T,
    pub(crate) saturation: T,
}

#[inline]
pub(crate) fn sample_hsv<T>(r1: T, r2: T) -> HsvSample<T>
where
    T: Cbrt + Sqrt,
{
    HsvSample {
        value: r1.cbrt(),
        saturation: r2.sqrt(),
    }
}

#[inline]
pub(crate) fn invert_hsv_sample<T>(sample: HsvSample<T>) -> (T, T)
where
    T: Powi,
{
    (sample.value.powi(3), sample.saturation.powi(2))
}

#[derive(Debug, PartialEq)]
pub(crate) struct HslSample<T> {
    pub(crate) saturation: T,
    pub(crate) lightness: T,
}

#[inline]
pub(crate) fn sample_hsl<T>(r1: T, r2: T) -> HslSample<T>
where
    T: Real + One + Cbrt + Sqrt + Arithmetics + PartialCmp + Clone,
    T::Mask: LazySelect<T> + Clone,
{
    HslSample {
        saturation: r2.sqrt(),
        lightness: sample_bicone_height(r1),
    }
}

#[inline]
fn sample_bicone_height<T>(r1: T) -> T
where
    T: Real + One + Cbrt + Arithmetics + PartialCmp + Clone,
    T::Mask: LazySelect<T> + Clone,
{
    let mask = r1.lt_eq(&T::from_f64(0.5));

    // Scale it up to [0, 1] or [0, 1)
    let r1 = lazy_select! {
        if mask.clone() => r1.clone(),
        else => T::one() - &r1,
    } * T::from_f64(2.0);

    let height = r1.cbrt();

    // Turn the height back to [0, 0.5] or (0.5, 1.0]
    let height = height * T::from_f64(0.5);
    lazy_select! {
        if mask => height.clone(),
        else => T::one() - &height,
    }
}

#[inline]
pub(crate) fn invert_hsl_sample<T>(sample: HslSample<T>) -> (T, T)
where
    T: Real + Powi + Arithmetics + PartialCmp + Clone,
    T::Mask: LazySelect<T>,
{
    let HslSample {
        saturation,
        lightness,
    } = sample;

    let r1 = invert_bicone_height_sample(lightness);

    // saturation is first multiplied, then divided by h before squaring.
    // h can be completely eliminated, leaving only the saturation.
    let r2 = saturation.powi(2);

    (r1, r2)
}

fn invert_bicone_height_sample<T>(height: T) -> T
where
    T: Real + Powi + Arithmetics + PartialCmp + Clone,
    T::Mask: LazySelect<T>,
{
    lazy_select! {
        if height.lt_eq(&T::from_f64(0.5)) => {
            // ((x * 2)^3) / 2 = x^3 * 4.
            // height is multiplied by 2 to scale it up to [0, 1], becoming h.
            // h is cubed to make it r1. r1 is divided by 2 to take it back to [0, 0.5].
            height.clone().powi(3) * T::from_f64(4.0)
        },
        else => {
            let x = height.clone() - T::from_f64(1.0);
            x.powi(3) * T::from_f64(4.0) + T::from_f64(1.0)
        },
    }
}

#[cfg(test)]
mod test {
    use super::{sample_hsl, sample_hsv, HslSample, HsvSample};

    #[cfg(feature = "random")]
    #[test]
    fn sample_max_min() {
        let a = sample_hsv(0.0, 0.0);
        let b = sample_hsv(1.0, 1.0);
        assert_eq!(
            HsvSample {
                saturation: 0.0,
                value: 0.0
            },
            a
        );
        assert_eq!(
            HsvSample {
                saturation: 1.0,
                value: 1.0
            },
            b
        );
        let a = sample_hsl(0.0, 0.0);
        let b = sample_hsl(1.0, 1.0);
        assert_eq!(
            HslSample {
                saturation: 0.0,
                lightness: 0.0
            },
            a
        );
        assert_eq!(
            HslSample {
                saturation: 1.0,
                lightness: 1.0
            },
            b
        );
    }

    #[cfg(all(feature = "random", feature = "approx"))]
    #[allow(clippy::excessive_precision)]
    #[test]
    fn hsl_sampling() {
        use super::invert_hsl_sample;

        // Sanity check that sampling and inverting from sample are equivalent
        macro_rules! test_hsl {
            ( $x:expr, $y:expr ) => {{
                let hsl = sample_hsl($x, $y);
                let a = invert_hsl_sample(hsl);
                assert_relative_eq!(a.0, $x);
                assert_relative_eq!(a.1, $y);
            }};
        }

        test_hsl!(0.8464721407, 0.8271899200);
        test_hsl!(0.8797234442, 0.4924621591);
        test_hsl!(0.9179406120, 0.8771350605);
        test_hsl!(0.5458023108, 0.1154283005);
        test_hsl!(0.2691241774, 0.7881780600);
        test_hsl!(0.2085030453, 0.9975406626);
        test_hsl!(0.8483632811, 0.4955013942);
        test_hsl!(0.0857919040, 0.0652214785);
        test_hsl!(0.7152662838, 0.2788421565);
        test_hsl!(0.2973598808, 0.5585230243);
        test_hsl!(0.0936619602, 0.7289450731);
        test_hsl!(0.4364395449, 0.9362269009);
        test_hsl!(0.9802381158, 0.9742974964);
        test_hsl!(0.1666129293, 0.4396910574);
        test_hsl!(0.6190216210, 0.7175675180);
    }
}

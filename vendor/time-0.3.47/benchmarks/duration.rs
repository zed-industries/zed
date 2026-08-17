use std::time::Duration as StdDuration;

use criterion::Bencher;
use time::ext::{NumericalDuration, NumericalStdDuration};
use time::Duration;

setup_benchmark! {
    "Duration",

    fn is_zero(ben: &mut Bencher<'_>) {
        let a = (-1).nanoseconds();
        let b = 0.seconds();
        let c = 1.nanoseconds();
        ben.iter(|| a.is_zero());
        ben.iter(|| b.is_zero());
        ben.iter(|| c.is_zero());
    }

    fn is_negative(ben: &mut Bencher<'_>) {
        let a = (-1).seconds();
        let b = 0.seconds();
        let c = 1.seconds();
        ben.iter(|| a.is_negative());
        ben.iter(|| b.is_negative());
        ben.iter(|| c.is_negative());
    }

    fn is_positive(ben: &mut Bencher<'_>) {
        let a = (-1).seconds();
        let b = 0.seconds();
        let c = 1.seconds();
        ben.iter(|| a.is_positive());
        ben.iter(|| b.is_positive());
        ben.iter(|| c.is_positive());
    }

    fn abs(ben: &mut Bencher<'_>) {
        let a = 1.seconds();
        let b = 0.seconds();
        let c = (-1).seconds();
        ben.iter(|| a.abs());
        ben.iter(|| b.abs());
        ben.iter(|| c.abs());
    }

    fn unsigned_abs(ben: &mut Bencher<'_>) {
        let a = 1.seconds();
        let b = 0.seconds();
        let c = (-1).seconds();
        ben.iter(|| a.unsigned_abs());
        ben.iter(|| b.unsigned_abs());
        ben.iter(|| c.unsigned_abs());
    }

    fn new(ben: &mut Bencher<'_>) {
        ben.iter(|| Duration::new(1, 0));
        ben.iter(|| Duration::new(-1, 0));
        ben.iter(|| Duration::new(1, 2_000_000_000));

        ben.iter(|| Duration::new(0, 0));
        ben.iter(|| Duration::new(0, 1_000_000_000));
        ben.iter(|| Duration::new(-1, 1_000_000_000));
        ben.iter(|| Duration::new(-2, 1_000_000_000));

        ben.iter(|| Duration::new(1, -1));
        ben.iter(|| Duration::new(-1, 1));
        ben.iter(|| Duration::new(1, 1));
        ben.iter(|| Duration::new(-1, -1));
        ben.iter(|| Duration::new(0, 1));
        ben.iter(|| Duration::new(0, -1));

        ben.iter(|| Duration::new(-1, 1_400_000_000));
        ben.iter(|| Duration::new(-2, 1_400_000_000));
        ben.iter(|| Duration::new(-3, 1_400_000_000));
        ben.iter(|| Duration::new(1, -1_400_000_000));
        ben.iter(|| Duration::new(2, -1_400_000_000));
        ben.iter(|| Duration::new(3, -1_400_000_000));
    }

    fn weeks(ben: &mut Bencher<'_>) {
        ben.iter(|| Duration::weeks(1));
        ben.iter(|| Duration::weeks(2));
        ben.iter(|| Duration::weeks(-1));
        ben.iter(|| Duration::weeks(-2));
    }

    fn days(ben: &mut Bencher<'_>) {
        ben.iter(|| Duration::days(1));
        ben.iter(|| Duration::days(2));
        ben.iter(|| Duration::days(-1));
        ben.iter(|| Duration::days(-2));
    }

    fn hours(ben: &mut Bencher<'_>) {
        ben.iter(|| Duration::hours(1));
        ben.iter(|| Duration::hours(2));
        ben.iter(|| Duration::hours(-1));
        ben.iter(|| Duration::hours(-2));
    }

    fn minutes(ben: &mut Bencher<'_>) {
        ben.iter(|| Duration::minutes(1));
        ben.iter(|| Duration::minutes(2));
        ben.iter(|| Duration::minutes(-1));
        ben.iter(|| Duration::minutes(-2));
    }

    fn seconds(ben: &mut Bencher<'_>) {
        ben.iter(|| Duration::seconds(1));
        ben.iter(|| Duration::seconds(2));
        ben.iter(|| Duration::seconds(-1));
        ben.iter(|| Duration::seconds(-2));
    }

    fn seconds_f64(ben: &mut Bencher<'_>) {
        ben.iter(|| Duration::seconds_f64(0.5));
        ben.iter(|| Duration::seconds_f64(-0.5));
    }

    fn seconds_f32(ben: &mut Bencher<'_>) {
        ben.iter(|| Duration::seconds_f32(0.5));
        ben.iter(|| Duration::seconds_f32(-0.5));
    }

    fn saturating_seconds_f64(ben: &mut Bencher<'_>) {
        ben.iter(|| Duration::saturating_seconds_f64(0.5));
        ben.iter(|| Duration::saturating_seconds_f64(-0.5));
    }

    fn saturating_seconds_f32(ben: &mut Bencher<'_>) {
        ben.iter(|| Duration::saturating_seconds_f32(0.5));
        ben.iter(|| Duration::saturating_seconds_f32(-0.5));
    }

    fn checked_seconds_f64(ben: &mut Bencher<'_>) {
        ben.iter(|| Duration::checked_seconds_f64(0.5));
        ben.iter(|| Duration::checked_seconds_f64(-0.5));
    }

    fn checked_seconds_f32(ben: &mut Bencher<'_>) {
        ben.iter(|| Duration::checked_seconds_f32(0.5));
        ben.iter(|| Duration::checked_seconds_f32(-0.5));
    }

    fn milliseconds(ben: &mut Bencher<'_>) {
        ben.iter(|| Duration::milliseconds(1));
        ben.iter(|| Duration::milliseconds(-1));
    }

    fn microseconds(ben: &mut Bencher<'_>) {
        ben.iter(|| Duration::microseconds(1));
        ben.iter(|| Duration::microseconds(-1));
    }

    fn nanoseconds(ben: &mut Bencher<'_>) {
        ben.iter(|| Duration::nanoseconds(1));
        ben.iter(|| Duration::nanoseconds(-1));
    }

    fn whole_weeks(ben: &mut Bencher<'_>) {
        let a = Duration::weeks(1);
        let b = Duration::weeks(-1);
        let c = Duration::days(6);
        let d = Duration::days(-6);
        ben.iter(|| a.whole_weeks());
        ben.iter(|| b.whole_weeks());
        ben.iter(|| c.whole_weeks());
        ben.iter(|| d.whole_weeks());
    }

    fn whole_days(ben: &mut Bencher<'_>) {
        let a = Duration::days(1);
        let b = Duration::days(-1);
        let c = Duration::hours(23);
        let d = Duration::hours(-23);
        ben.iter(|| a.whole_days());
        ben.iter(|| b.whole_days());
        ben.iter(|| c.whole_days());
        ben.iter(|| d.whole_days());
    }

    fn whole_hours(ben: &mut Bencher<'_>) {
        let a = Duration::hours(1);
        let b = Duration::hours(-1);
        let c = Duration::minutes(59);
        let d = Duration::minutes(-59);
        ben.iter(|| a.whole_hours());
        ben.iter(|| b.whole_hours());
        ben.iter(|| c.whole_hours());
        ben.iter(|| d.whole_hours());
    }

    fn whole_minutes(ben: &mut Bencher<'_>) {
        let a = 1.minutes();
        let b = (-1).minutes();
        let c = 59.seconds();
        let d = (-59).seconds();
        ben.iter(|| a.whole_minutes());
        ben.iter(|| b.whole_minutes());
        ben.iter(|| c.whole_minutes());
        ben.iter(|| d.whole_minutes());
    }

    fn whole_seconds(ben: &mut Bencher<'_>) {
        let a = 1.seconds();
        let b = (-1).seconds();
        let c = 1.minutes();
        let d = (-1).minutes();
        ben.iter(|| a.whole_seconds());
        ben.iter(|| b.whole_seconds());
        ben.iter(|| c.whole_seconds());
        ben.iter(|| d.whole_seconds());
    }

    fn as_seconds_f64(ben: &mut Bencher<'_>) {
        let a = 1.seconds();
        let b = (-1).seconds();
        let c = 1.minutes();
        let d = (-1).minutes();
        let e = 1.5.seconds();
        let f = (-1.5).seconds();
        ben.iter(|| a.as_seconds_f64());
        ben.iter(|| b.as_seconds_f64());
        ben.iter(|| c.as_seconds_f64());
        ben.iter(|| d.as_seconds_f64());
        ben.iter(|| e.as_seconds_f64());
        ben.iter(|| f.as_seconds_f64());
    }

    fn as_seconds_f32(ben: &mut Bencher<'_>) {
        let a = 1.seconds();
        let b = (-1).seconds();
        let c = 1.minutes();
        let d = (-1).minutes();
        let e = 1.5.seconds();
        let f = (-1.5).seconds();
        ben.iter(|| a.as_seconds_f32());
        ben.iter(|| b.as_seconds_f32());
        ben.iter(|| c.as_seconds_f32());
        ben.iter(|| d.as_seconds_f32());
        ben.iter(|| e.as_seconds_f32());
        ben.iter(|| f.as_seconds_f32());
    }

    fn whole_milliseconds(ben: &mut Bencher<'_>) {
        let a = 1.seconds();
        let b = (-1).seconds();
        let c = 1.milliseconds();
        let d = (-1).milliseconds();
        ben.iter(|| a.whole_milliseconds());
        ben.iter(|| b.whole_milliseconds());
        ben.iter(|| c.whole_milliseconds());
        ben.iter(|| d.whole_milliseconds());
    }

    fn subsec_milliseconds(ben: &mut Bencher<'_>) {
        let a = 1.4.seconds();
        let b = (-1.4).seconds();
        ben.iter(|| a.subsec_milliseconds());
        ben.iter(|| b.subsec_milliseconds());
    }

    fn whole_microseconds(ben: &mut Bencher<'_>) {
        let a = 1.milliseconds();
        let b = (-1).milliseconds();
        let c = 1.microseconds();
        let d = (-1).microseconds();
        ben.iter(|| a.whole_microseconds());
        ben.iter(|| b.whole_microseconds());
        ben.iter(|| c.whole_microseconds());
        ben.iter(|| d.whole_microseconds());
    }

    fn subsec_microseconds(ben: &mut Bencher<'_>) {
        let a = 1.0004.seconds();
        let b = (-1.0004).seconds();
        ben.iter(|| a.subsec_microseconds());
        ben.iter(|| b.subsec_microseconds());
    }

    fn whole_nanoseconds(ben: &mut Bencher<'_>) {
        let a = 1.microseconds();
        let b = (-1).microseconds();
        let c = 1.nanoseconds();
        let d = (-1).nanoseconds();
        ben.iter(|| a.whole_nanoseconds());
        ben.iter(|| b.whole_nanoseconds());
        ben.iter(|| c.whole_nanoseconds());
        ben.iter(|| d.whole_nanoseconds());
    }

    fn subsec_nanoseconds(ben: &mut Bencher<'_>) {
        let a = 1.000_000_4.seconds();
        let b = (-1.000_000_4).seconds();
        ben.iter(|| a.subsec_nanoseconds());
        ben.iter(|| b.subsec_nanoseconds());
    }

    fn checked_add(ben: &mut Bencher<'_>) {
        let a = 5.seconds();
        let b = Duration::MAX;
        let c = (-5).seconds();

        let a2 = 5.seconds();
        let b2 = 1.nanoseconds();
        let c2 = 5.seconds();

        ben.iter(|| a.checked_add(a2));
        ben.iter(|| b.checked_add(b2));
        ben.iter(|| c.checked_add(c2));
    }

    fn checked_sub(ben: &mut Bencher<'_>) {
        let a = 5.seconds();
        let b = Duration::MIN;
        let c = 5.seconds();

        let a2 = 5.seconds();
        let b2 = 1.nanoseconds();
        let c2 = 10.seconds();

        ben.iter(|| a.checked_sub(a2));
        ben.iter(|| b.checked_sub(b2));
        ben.iter(|| c.checked_sub(c2));
    }

    fn checked_mul(ben: &mut Bencher<'_>) {
        let a = 5.seconds();
        let b = Duration::MAX;
        ben.iter(|| a.checked_mul(2));
        ben.iter(|| b.checked_mul(2));
    }

    fn checked_div(ben: &mut Bencher<'_>) {
        let a = 10.seconds();
        ben.iter(|| a.checked_div(2));
        ben.iter(|| a.checked_div(0));
    }

    fn saturating_add(ben: &mut Bencher<'_>) {
        let a = 5.seconds();
        let b = Duration::MAX;
        let c = Duration::MIN;
        let d = (-5).seconds();

        let a2 = 5.seconds();
        let b2 = 1.nanoseconds();
        let c2 = (-1).nanoseconds();
        let d2 = 5.seconds();

        ben.iter(|| a.saturating_add(a2));
        ben.iter(|| b.saturating_add(b2));
        ben.iter(|| c.saturating_add(c2));
        ben.iter(|| d.saturating_add(d2));
    }

    fn saturating_sub(ben: &mut Bencher<'_>) {
        let a = 5.seconds();
        let b = Duration::MIN;
        let c = Duration::MAX;
        let d = 5.seconds();

        let a2 = 5.seconds();
        let b2 = 1.nanoseconds();
        let c2 = (-1).nanoseconds();
        let d2 = 10.seconds();

        ben.iter(|| a.saturating_sub(a2));
        ben.iter(|| b.saturating_sub(b2));
        ben.iter(|| c.saturating_sub(c2));
        ben.iter(|| d.saturating_sub(d2));
    }

    fn saturating_mul(ben: &mut Bencher<'_>) {
        let a = 5.seconds();
        let b = 5.seconds();
        let c = 5.seconds();
        let d = Duration::MAX;
        let e = Duration::MIN;
        let f = Duration::MAX;
        let g = Duration::MIN;

        ben.iter(|| a.saturating_mul(2));
        ben.iter(|| b.saturating_mul(-2));
        ben.iter(|| c.saturating_mul(0));
        ben.iter(|| d.saturating_mul(2));
        ben.iter(|| e.saturating_mul(2));
        ben.iter(|| f.saturating_mul(-2));
        ben.iter(|| g.saturating_mul(-2));
    }

    fn try_from_std_duration(ben: &mut Bencher<'_>) {
        let a = 0.std_seconds();
        let b = 1.std_seconds();
        ben.iter(|| Duration::try_from(a));
        ben.iter(|| Duration::try_from(b));
    }

    fn try_to_std_duration(ben: &mut Bencher<'_>) {
        let a = 0.seconds();
        let b = 1.seconds();
        let c = (-1).seconds();
        ben.iter(|| StdDuration::try_from(a));
        ben.iter(|| StdDuration::try_from(b));
        ben.iter(|| StdDuration::try_from(c));
    }

    fn add(ben: &mut Bencher<'_>) {
        let a = 1.seconds();
        let b = 2.seconds();
        let c = 500.milliseconds();
        let d = (-1).seconds();
        ben.iter(|| a + b + c + d);
    }

    fn add_std(ben: &mut Bencher<'_>) {
        let a = 1.seconds();
        let b = 2.std_seconds();
        ben.iter(|| a + b);
    }

    fn std_add(ben: &mut Bencher<'_>) {
        let a = 1.std_seconds();
        let b = 2.seconds();
        ben.iter(|| a + b);
    }

    fn add_assign(ben: &mut Bencher<'_>) {
        let a = 1.seconds();
        let b = 500.milliseconds();
        let c = (-1).seconds();
        iter_batched_ref!(
            ben,
            || 1.seconds(),
            [
                |duration| *duration += a,
                |duration| *duration += b,
                |duration| *duration += c,
            ]
        );
    }

    fn add_assign_std(ben: &mut Bencher<'_>) {
        let a = 1.std_seconds();
        let b = 500.std_milliseconds();
        iter_batched_ref!(
            ben,
            || 1.seconds(),
            [
                |duration| *duration += a,
                |duration| *duration += b,
            ]
        );
    }

    fn neg(ben: &mut Bencher<'_>) {
        let a = 1.seconds();
        let b = (-1).seconds();
        let c = 0.seconds();
        ben.iter(|| -a);
        ben.iter(|| -b);
        ben.iter(|| -c);
    }

    fn sub(ben: &mut Bencher<'_>) {
        let a = 1.seconds();
        let b = 1.seconds();
        let c = 1_500.milliseconds();
        let d = 500.milliseconds();
        let e = 1.seconds();
        let f = (-1).seconds();
        ben.iter(|| a - b);
        ben.iter(|| b - c);
        ben.iter(|| c - d);
        ben.iter(|| d - e);
        ben.iter(|| e - f);
        ben.iter(|| f - a);
    }

    fn sub_std(ben: &mut Bencher<'_>) {
        let a = 1.seconds();
        let b = 2.std_seconds();
        ben.iter(|| a - b);
    }

    fn std_sub(ben: &mut Bencher<'_>) {
        let a = 1.std_seconds();
        let b = 2.seconds();
        ben.iter(|| a - b);
    }

    fn sub_assign(ben: &mut Bencher<'_>) {
        let a = 1.seconds();
        let b = 500.milliseconds();
        let c = (-1).seconds();
        iter_batched_ref!(
            ben,
            || 1.seconds(),
            [
                |duration| *duration -= a,
                |duration| *duration -= b,
                |duration| *duration -= c,
            ]
        );
    }

    fn mul_int(ben: &mut Bencher<'_>) {
        let d = 1.seconds();
        ben.iter(|| d * 2);
        ben.iter(|| d * -2);
    }

    fn mul_int_assign(ben: &mut Bencher<'_>) {
        iter_batched_ref!(
            ben,
            || 1.seconds(),
            [
                |duration| *duration *= 2,
                |duration| *duration *= -2,
            ]
        );
    }

    fn int_mul(ben: &mut Bencher<'_>) {
        let d = 1.seconds();
        ben.iter(|| 2 * d);
        ben.iter(|| -2 * d);
    }

    fn div_int(ben: &mut Bencher<'_>) {
        let d = 1.seconds();
        ben.iter(|| d / 2);
        ben.iter(|| d / -2);
    }

    fn div_int_assign(ben: &mut Bencher<'_>) {
        iter_batched_ref!(
            ben,
            || 1.seconds(),
            [
                |duration| *duration /= 2,
                |duration| *duration /= -2,
            ]
        );
    }

    fn div(ben: &mut Bencher<'_>) {
        let a = 1.seconds();
        let b = 0.5.seconds();
        ben.iter(|| a / b);
    }

    fn mul_float(ben: &mut Bencher<'_>) {
        let d = 1.seconds();
        ben.iter(|| d * 1.5_f32);
        ben.iter(|| d * 2.5_f32);
        ben.iter(|| d * -1.5_f32);
        ben.iter(|| d * 0_f32);
        ben.iter(|| d * 1.5_f64);
        ben.iter(|| d * 2.5_f64);
        ben.iter(|| d * -1.5_f64);
        ben.iter(|| d * 0_f64);
    }

    fn float_mul(ben: &mut Bencher<'_>) {
        let d = 1.seconds();
        ben.iter(|| 1.5_f32 * d);
        ben.iter(|| 2.5_f32 * d);
        ben.iter(|| -1.5_f32 * d);
        ben.iter(|| 0_f32 * d);
        ben.iter(|| 1.5_f64 * d);
        ben.iter(|| 2.5_f64 * d);
        ben.iter(|| -1.5_f64 * d);
        ben.iter(|| 0_f64 * d);
    }

    fn mul_float_assign(ben: &mut Bencher<'_>) {
        iter_batched_ref!(
            ben,
            || 1.seconds(),
            [
                |duration| *duration *= 1.5_f32,
                |duration| *duration *= 2.5_f32,
                |duration| *duration *= -1.5_f32,
                |duration| *duration *= 3.15_f32,
                |duration| *duration *= 1.5_f64,
                |duration| *duration *= 2.5_f64,
                |duration| *duration *= -1.5_f64,
                |duration| *duration *= 0_f64,
            ]
        );
    }

    fn div_float(ben: &mut Bencher<'_>) {
        let d = 1.seconds();
        ben.iter(|| d / 1_f32);
        ben.iter(|| d / 2_f32);
        ben.iter(|| d / -1_f32);
        ben.iter(|| d / 1_f64);
        ben.iter(|| d / 2_f64);
        ben.iter(|| d / -1_f64);
    }

    fn div_float_assign(ben: &mut Bencher<'_>) {
        iter_batched_ref!(
            ben,
            || 10.seconds(),
            [
                |duration| *duration /= 1_f32,
                |duration| *duration /= 2_f32,
                |duration| *duration /= -1_f32,
                |duration| *duration /= 1_f64,
                |duration| *duration /= 2_f64,
                |duration| *duration /= -1_f64,
            ]
        );
    }

    fn partial_eq(ben: &mut Bencher<'_>) {
        let a = 1.minutes();
        let b = (-1).minutes();
        let c = 40.seconds();
        ben.iter(|| a == b);
        ben.iter(|| c == a);
    }

    fn partial_eq_std(ben: &mut Bencher<'_>) {
        let a = (-1).seconds();
        let b = 1.std_seconds();
        let c = (-1).minutes();
        let d = 1.std_minutes();
        let e = 40.seconds();
        ben.iter(|| a == b);
        ben.iter(|| c == d);
        ben.iter(|| e == d);
    }

    fn std_partial_eq(ben: &mut Bencher<'_>) {
        let a = 1.std_seconds();
        let b = (-1).seconds();
        let c = 1.std_minutes();
        let d = (-1).minutes();
        let e = 40.std_seconds();
        let f = 1.minutes();
        ben.iter(|| a == b);
        ben.iter(|| c == d);
        ben.iter(|| e == f);
    }

    fn partial_ord(ben: &mut Bencher<'_>) {
        let a = 0.seconds();
        let b = 1.seconds();
        let c = (-1).seconds();
        let d = 1.minutes();
        let e = (-1).minutes();
        ben.iter(|| a.partial_cmp(&a));
        ben.iter(|| b.partial_cmp(&a));
        ben.iter(|| b.partial_cmp(&c));
        ben.iter(|| c.partial_cmp(&b));
        ben.iter(|| a.partial_cmp(&c));
        ben.iter(|| a.partial_cmp(&b));
        ben.iter(|| c.partial_cmp(&a));
        ben.iter(|| d.partial_cmp(&b));
        ben.iter(|| e.partial_cmp(&c));
    }

    fn partial_ord_std(ben: &mut Bencher<'_>) {
        let a = 0.seconds();
        let b = 0.std_seconds();
        let c = 1.seconds();
        let d = (-1).seconds();
        let e = 1.std_seconds();
        let f = 1.minutes();
        let g = u64::MAX.std_seconds();
        ben.iter(|| a.partial_cmp(&b));
        ben.iter(|| c.partial_cmp(&b));
        ben.iter(|| d.partial_cmp(&e));
        ben.iter(|| a.partial_cmp(&e));
        ben.iter(|| d.partial_cmp(&b));
        ben.iter(|| f.partial_cmp(&e));
        ben.iter(|| a.partial_cmp(&g));
    }

    fn std_partial_ord(ben: &mut Bencher<'_>) {
        let a = 0.std_seconds();
        let b = 0.seconds();
        let c = 1.std_seconds();
        let d = (-1).seconds();
        let e = 1.seconds();
        let f = 1.std_minutes();
        ben.iter(|| a.partial_cmp(&b));
        ben.iter(|| c.partial_cmp(&b));
        ben.iter(|| c.partial_cmp(&d));
        ben.iter(|| a.partial_cmp(&d));
        ben.iter(|| a.partial_cmp(&e));
        ben.iter(|| f.partial_cmp(&e));
    }

    fn ord(ben: &mut Bencher<'_>) {
        let a = 1.seconds();
        let b = 0.seconds();
        let c = (-1).seconds();
        let d = 1.minutes();
        let e = (-1).minutes();
        ben.iter(|| a > b);
        ben.iter(|| a > c);
        ben.iter(|| c < a);
        ben.iter(|| b > c);
        ben.iter(|| b < a);
        ben.iter(|| c < b);
        ben.iter(|| d > a);
        ben.iter(|| e < c);
    }
}

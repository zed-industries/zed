use std::hint::black_box as bb;
use std::sync::LazyLock;

use criterion::Bencher;
use time::util;

/// Generate a representative sample of all years.
fn representative_years() -> [i32; 800] {
    static DATES: LazyLock<[i32; 800]> = LazyLock::new(|| {
        let mut years = [0; _];
        for year in -400..400 {
            years[(year + 400) as usize] = year;
        }
        crate::shuffle(years)
    });

    *DATES
}

setup_benchmark! {
    "Utils",

    fn noop(ben: &mut Bencher<'_>) {
        ben.iter(|| {
            for i in representative_years() {
                let _ = bb(i);
            }
        });
    }

    fn is_leap_year(ben: &mut Bencher<'_>) {
        ben.iter(|| {
            for year in representative_years() {
                let _ = bb(util::is_leap_year(bb(year)));
            }
        });
    }

    fn days_in_year(ben: &mut Bencher<'_>) {
        ben.iter(|| {
            for year in representative_years() {
                let _ = bb(util::days_in_year(bb(year)));
            }
        });
    }

    fn weeks_in_year(ben: &mut Bencher<'_>) {
        ben.iter(|| {
            for year in representative_years() {
                let _ = bb(util::weeks_in_year(bb(year)));
            }
        });
    }
}

#[cfg(test)]
macro_rules! test {
    ($ty:ident) => {
        mod $ty {
            use quickcheck::TestResult;
            use quickcheck::quickcheck;
            use approx::relative_eq;

            use crate::stats::bivariate::regression::Slope;
            use crate::stats::bivariate::Data;

            quickcheck! {
                fn means(size: u8, start: u8,
                         offset: u8, nresamples: u8) -> TestResult {
                    let size = size as usize;
                    let start = start as usize;
                    let offset = offset as usize;
                    let nresamples = nresamples as usize;
                    if let Some(x) = crate::stats::test::vec::<$ty>(size, start) {
                        let y = crate::stats::test::vec::<$ty>(size + offset, start + offset).unwrap();
                        let data = Data::new(&x[start..], &y[start+offset..]);

                        let (x_means, y_means) = if nresamples > 0 {
                            data.bootstrap(nresamples, |d| (d.x().mean(), d.y().mean()))
                        } else {
                            return TestResult::discard();
                        };

                        let x_min = data.x().min();
                        let x_max = data.x().max();
                        let y_min = data.y().min();
                        let y_max = data.y().max();

                        TestResult::from_bool(
                            // Computed the correct number of resamples
                            x_means.len() == nresamples &&
                            y_means.len() == nresamples &&
                            // No uninitialized values
                            x_means.iter().all(|&x| {
                                (x > x_min || relative_eq!(x, x_min)) &&
                                (x < x_max || relative_eq!(x, x_max))
                            }) &&
                            y_means.iter().all(|&y| {
                                (y > y_min || relative_eq!(y, y_min)) &&
                                (y < y_max || relative_eq!(y, y_max))
                            })
                        )
                    } else {
                        TestResult::discard()
                    }
                }
            }

            quickcheck! {
                fn slope(size: u8, start: u8,
                         offset: u8, nresamples: u8) -> TestResult {
                    let size = size as usize;
                    let start = start as usize;
                    let offset = offset as usize;
                    let nresamples = nresamples as usize;
                    if let Some(x) = crate::stats::test::vec::<$ty>(size, start) {
                        let y = crate::stats::test::vec::<$ty>(size + offset, start + offset).unwrap();
                        let data = Data::new(&x[start..], &y[start+offset..]);

                        let slopes = if nresamples > 0 {
                            data.bootstrap(nresamples, |d| (Slope::fit(&d),)).0
                        } else {
                            return TestResult::discard();
                        };

                        TestResult::from_bool(
                            // Computed the correct number of resamples
                            slopes.len() == nresamples &&
                            // No uninitialized values
                            slopes.iter().all(|s| s.0 > 0.)
                        )
                    } else {
                        TestResult::discard()
                    }
                }
            }

        }
    };
}

#[cfg(test)]
mod test {
    test!(f32);
    test!(f64);
}

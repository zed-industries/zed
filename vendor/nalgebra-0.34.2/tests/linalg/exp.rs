#[cfg(test)]
mod tests {
    //https://github.com/scipy/scipy/blob/c1372d8aa90a73d8a52f135529293ff4edb98fc8/scipy/sparse/linalg/tests/test_matfuncs.py
    #[test]
    fn exp_static() {
        use nalgebra::{Matrix1, Matrix2, Matrix3};

        {
            let m = Matrix1::new(1.0);

            let f = m.exp();

            assert!(relative_eq!(f, Matrix1::new(1_f64.exp()), epsilon = 1.0e-7));
        }

        {
            let m = Matrix2::new(0.0, 1.0, 0.0, 0.0);

            assert!(relative_eq!(
                m.exp(),
                Matrix2::new(1.0, 1.0, 0.0, 1.0),
                epsilon = 1.0e-7
            ));
        }

        {
            let a: f64 = 1.0;
            let b: f64 = 2.0;
            let c: f64 = 3.0;
            let d: f64 = 4.0;
            let m = Matrix2::new(a, b, c, d);

            let delta = ((a - d).powf(2.0) + 4.0 * b * c).sqrt();
            let delta_2 = delta / 2.0;
            let ad_2 = (a + d) / 2.0;
            let m11 = ad_2.exp() * (delta * delta_2.cosh() + (a - d) * delta_2.sinh());
            let m12 = 2.0 * b * ad_2.exp() * delta_2.sinh();
            let m21 = 2.0 * c * ad_2.exp() * delta_2.sinh();
            let m22 = ad_2.exp() * (delta * delta_2.cosh() + (d - a) * delta_2.sinh());

            let f = Matrix2::new(m11, m12, m21, m22) / delta;
            assert!(relative_eq!(f, m.exp(), epsilon = 1.0e-7));
        }

        {
            // https://mathworld.wolfram.com/MatrixExponential.html
            use rand::{
                distr::{Distribution, Uniform},
                rng,
            };
            let mut rng = rng();
            let dist = Uniform::new(-10.0, 10.0)
                .expect("Failed to construct `Uniform`, should be unreachable");
            loop {
                let a: f64 = dist.sample(&mut rng);
                let b: f64 = dist.sample(&mut rng);
                let c: f64 = dist.sample(&mut rng);
                let d: f64 = dist.sample(&mut rng);
                let m = Matrix2::new(a, b, c, d);

                let delta_sq = (a - d).powf(2.0) + 4.0 * b * c;
                if delta_sq < 0.0 {
                    continue;
                }

                let delta = delta_sq.sqrt();
                let delta_2 = delta / 2.0;
                let ad_2 = (a + d) / 2.0;
                let m11 = ad_2.exp() * (delta * delta_2.cosh() + (a - d) * delta_2.sinh());
                let m12 = 2.0 * b * ad_2.exp() * delta_2.sinh();
                let m21 = 2.0 * c * ad_2.exp() * delta_2.sinh();
                let m22 = ad_2.exp() * (delta * delta_2.cosh() + (d - a) * delta_2.sinh());

                let f = Matrix2::new(m11, m12, m21, m22) / delta;
                assert!(relative_eq!(f, m.exp(), epsilon = 1.0e-7));
                break;
            }
        }

        {
            let m = Matrix3::new(1.0, 3.0, 0.0, 0.0, 1.0, 5.0, 0.0, 0.0, 2.0);

            let e1 = 1.0_f64.exp();
            let e2 = 2.0_f64.exp();

            let f = Matrix3::new(
                e1,
                3.0 * e1,
                15.0 * (e2 - 2.0 * e1),
                0.0,
                e1,
                5.0 * (e2 - e1),
                0.0,
                0.0,
                e2,
            );

            assert!(relative_eq!(f, m.exp(), epsilon = 1.0e-7));
        }
    }

    #[test]
    fn exp_dynamic() {
        use nalgebra::DMatrix;

        let m = DMatrix::from_row_slice(3, 3, &[1.0, 3.0, 0.0, 0.0, 1.0, 5.0, 0.0, 0.0, 2.0]);

        let e1 = 1.0_f64.exp();
        let e2 = 2.0_f64.exp();

        let f = DMatrix::from_row_slice(
            3,
            3,
            &[
                e1,
                3.0 * e1,
                15.0 * (e2 - 2.0 * e1),
                0.0,
                e1,
                5.0 * (e2 - e1),
                0.0,
                0.0,
                e2,
            ],
        );

        assert!(relative_eq!(f, m.exp(), epsilon = 1.0e-7));
    }

    #[test]
    fn exp_complex() {
        use nalgebra::{Complex, DMatrix, DVector, Matrix2, RealField};

        {
            let z = Matrix2::<Complex<f64>>::zeros();

            let identity = Matrix2::<Complex<f64>>::identity();

            assert!((z.exp() - identity).norm() < 1e-7);
        }

        {
            let a = Matrix2::<Complex<f64>>::new(
                Complex::<f64>::new(0.0, 1.0),
                Complex::<f64>::new(0.0, 2.0),
                Complex::<f64>::new(0.0, -1.0),
                Complex::<f64>::new(0.0, 3.0),
            );

            let b = Matrix2::<Complex<f64>>::new(
                Complex::<f64>::new(0.42645929666726, 1.89217550966333),
                Complex::<f64>::new(-2.13721484276556, -0.97811251808259),
                Complex::<f64>::new(1.06860742138278, 0.48905625904129),
                Complex::<f64>::new(-1.7107555460983, 0.91406299158075),
            );

            assert!((a.exp() - b).norm() < 1.0e-07);
        }

        {
            let d1 = Complex::<f64>::new(0.0, <f64 as RealField>::pi());
            let d2 = Complex::<f64>::new(0.0, <f64 as RealField>::frac_pi_2());
            let d3 = Complex::<f64>::new(0.0, <f64 as RealField>::frac_pi_4());

            let m = DMatrix::<Complex<f64>>::from_diagonal(&DVector::from_row_slice(&[d1, d2, d3]));

            let res = DMatrix::<Complex<f64>>::from_diagonal(&DVector::from_row_slice(&[
                d1.exp(),
                d2.exp(),
                d3.exp(),
            ]));

            assert!((m.exp() - res).norm() < 1e-07);
        }
    }
}

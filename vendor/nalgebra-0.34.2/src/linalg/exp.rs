//! This module provides the matrix exponent (exp) function to square matrices.
//!
use crate::{
    ComplexField, OMatrix, RealField,
    base::{
        DefaultAllocator,
        allocator::Allocator,
        dimension::{Const, Dim, DimMin, DimMinimum},
    },
    convert, try_convert,
};

use crate::num::Zero;

/// Precomputed factorials for integers in range `0..=34`.
/// Note: `35!` does not fit into 128 bits.
// TODO: find a better place for this array?
const FACTORIAL: [u128; 35] = [
    1,
    1,
    2,
    6,
    24,
    120,
    720,
    5040,
    40320,
    362880,
    3628800,
    39916800,
    479001600,
    6227020800,
    87178291200,
    1307674368000,
    20922789888000,
    355687428096000,
    6402373705728000,
    121645100408832000,
    2432902008176640000,
    51090942171709440000,
    1124000727777607680000,
    25852016738884976640000,
    620448401733239439360000,
    15511210043330985984000000,
    403291461126605635584000000,
    10888869450418352160768000000,
    304888344611713860501504000000,
    8841761993739701954543616000000,
    265252859812191058636308480000000,
    8222838654177922817725562880000000,
    263130836933693530167218012160000000,
    8683317618811886495518194401280000000,
    295232799039604140847618609643520000000,
];

// https://github.com/scipy/scipy/blob/c1372d8aa90a73d8a52f135529293ff4edb98fc8/scipy/sparse/linalg/matfuncs.py
struct ExpmPadeHelper<T, D>
where
    T: ComplexField,
    D: DimMin<D>,
    DefaultAllocator: Allocator<D, D> + Allocator<DimMinimum<D, D>>,
{
    use_exact_norm: bool,
    ident: OMatrix<T, D, D>,

    a: OMatrix<T, D, D>,
    a2: Option<OMatrix<T, D, D>>,
    a4: Option<OMatrix<T, D, D>>,
    a6: Option<OMatrix<T, D, D>>,
    a8: Option<OMatrix<T, D, D>>,
    a10: Option<OMatrix<T, D, D>>,

    d4_exact: Option<T::RealField>,
    d6_exact: Option<T::RealField>,
    d8_exact: Option<T::RealField>,
    d10_exact: Option<T::RealField>,

    d4_approx: Option<T::RealField>,
    d6_approx: Option<T::RealField>,
    d8_approx: Option<T::RealField>,
    d10_approx: Option<T::RealField>,
}

impl<T, D> ExpmPadeHelper<T, D>
where
    T: ComplexField,
    D: DimMin<D>,
    DefaultAllocator: Allocator<D, D> + Allocator<DimMinimum<D, D>>,
{
    fn new(a: OMatrix<T, D, D>, use_exact_norm: bool) -> Self {
        let (nrows, ncols) = a.shape_generic();
        ExpmPadeHelper {
            use_exact_norm,
            ident: OMatrix::<T, D, D>::identity_generic(nrows, ncols),
            a,
            a2: None,
            a4: None,
            a6: None,
            a8: None,
            a10: None,
            d4_exact: None,
            d6_exact: None,
            d8_exact: None,
            d10_exact: None,
            d4_approx: None,
            d6_approx: None,
            d8_approx: None,
            d10_approx: None,
        }
    }

    fn calc_a2(&mut self) {
        if self.a2.is_none() {
            self.a2 = Some(&self.a * &self.a);
        }
    }

    fn calc_a4(&mut self) {
        if self.a4.is_none() {
            self.calc_a2();
            let a2 = self.a2.as_ref().unwrap();
            self.a4 = Some(a2 * a2);
        }
    }

    fn calc_a6(&mut self) {
        if self.a6.is_none() {
            self.calc_a2();
            self.calc_a4();
            let a2 = self.a2.as_ref().unwrap();
            let a4 = self.a4.as_ref().unwrap();
            self.a6 = Some(a4 * a2);
        }
    }

    fn calc_a8(&mut self) {
        if self.a8.is_none() {
            self.calc_a2();
            self.calc_a6();
            let a2 = self.a2.as_ref().unwrap();
            let a6 = self.a6.as_ref().unwrap();
            self.a8 = Some(a6 * a2);
        }
    }

    fn calc_a10(&mut self) {
        if self.a10.is_none() {
            self.calc_a4();
            self.calc_a6();
            let a4 = self.a4.as_ref().unwrap();
            let a6 = self.a6.as_ref().unwrap();
            self.a10 = Some(a6 * a4);
        }
    }

    fn d4_tight(&mut self) -> T::RealField {
        if self.d4_exact.is_none() {
            self.calc_a4();
            self.d4_exact = Some(one_norm(self.a4.as_ref().unwrap()).powf(convert(0.25)));
        }
        self.d4_exact.clone().unwrap()
    }

    fn d6_tight(&mut self) -> T::RealField {
        if self.d6_exact.is_none() {
            self.calc_a6();
            self.d6_exact = Some(one_norm(self.a6.as_ref().unwrap()).powf(convert(1.0 / 6.0)));
        }
        self.d6_exact.clone().unwrap()
    }

    fn d8_tight(&mut self) -> T::RealField {
        if self.d8_exact.is_none() {
            self.calc_a8();
            self.d8_exact = Some(one_norm(self.a8.as_ref().unwrap()).powf(convert(1.0 / 8.0)));
        }
        self.d8_exact.clone().unwrap()
    }

    fn d10_tight(&mut self) -> T::RealField {
        if self.d10_exact.is_none() {
            self.calc_a10();
            self.d10_exact = Some(one_norm(self.a10.as_ref().unwrap()).powf(convert(1.0 / 10.0)));
        }
        self.d10_exact.clone().unwrap()
    }

    fn d4_loose(&mut self) -> T::RealField {
        if self.use_exact_norm {
            return self.d4_tight();
        }

        if self.d4_exact.is_some() {
            return self.d4_exact.clone().unwrap();
        }

        if self.d4_approx.is_none() {
            self.calc_a4();
            self.d4_approx = Some(one_norm(self.a4.as_ref().unwrap()).powf(convert(0.25)));
        }

        self.d4_approx.clone().unwrap()
    }

    fn d6_loose(&mut self) -> T::RealField {
        if self.use_exact_norm {
            return self.d6_tight();
        }

        if self.d6_exact.is_some() {
            return self.d6_exact.clone().unwrap();
        }

        if self.d6_approx.is_none() {
            self.calc_a6();
            self.d6_approx = Some(one_norm(self.a6.as_ref().unwrap()).powf(convert(1.0 / 6.0)));
        }

        self.d6_approx.clone().unwrap()
    }

    fn d8_loose(&mut self) -> T::RealField {
        if self.use_exact_norm {
            return self.d8_tight();
        }

        if self.d8_exact.is_some() {
            return self.d8_exact.clone().unwrap();
        }

        if self.d8_approx.is_none() {
            self.calc_a8();
            self.d8_approx = Some(one_norm(self.a8.as_ref().unwrap()).powf(convert(1.0 / 8.0)));
        }

        self.d8_approx.clone().unwrap()
    }

    fn d10_loose(&mut self) -> T::RealField {
        if self.use_exact_norm {
            return self.d10_tight();
        }

        if self.d10_exact.is_some() {
            return self.d10_exact.clone().unwrap();
        }

        if self.d10_approx.is_none() {
            self.calc_a10();
            self.d10_approx = Some(one_norm(self.a10.as_ref().unwrap()).powf(convert(1.0 / 10.0)));
        }

        self.d10_approx.clone().unwrap()
    }

    fn pade3(&mut self) -> (OMatrix<T, D, D>, OMatrix<T, D, D>) {
        let b: [T; 4] = [convert(120.0), convert(60.0), convert(12.0), convert(1.0)];
        self.calc_a2();
        let a2 = self.a2.as_ref().unwrap();
        let u = &self.a * (a2 * b[3].clone() + &self.ident * b[1].clone());
        let v = a2 * b[2].clone() + &self.ident * b[0].clone();
        (u, v)
    }

    fn pade5(&mut self) -> (OMatrix<T, D, D>, OMatrix<T, D, D>) {
        let b: [T; 6] = [
            convert(30240.0),
            convert(15120.0),
            convert(3360.0),
            convert(420.0),
            convert(30.0),
            convert(1.0),
        ];
        self.calc_a2();
        self.calc_a6();
        let u = &self.a
            * (self.a4.as_ref().unwrap() * b[5].clone()
                + self.a2.as_ref().unwrap() * b[3].clone()
                + &self.ident * b[1].clone());
        let v = self.a4.as_ref().unwrap() * b[4].clone()
            + self.a2.as_ref().unwrap() * b[2].clone()
            + &self.ident * b[0].clone();
        (u, v)
    }

    fn pade7(&mut self) -> (OMatrix<T, D, D>, OMatrix<T, D, D>) {
        let b: [T; 8] = [
            convert(17_297_280.0),
            convert(8_648_640.0),
            convert(1_995_840.0),
            convert(277_200.0),
            convert(25_200.0),
            convert(1_512.0),
            convert(56.0),
            convert(1.0),
        ];
        self.calc_a2();
        self.calc_a4();
        self.calc_a6();
        let u = &self.a
            * (self.a6.as_ref().unwrap() * b[7].clone()
                + self.a4.as_ref().unwrap() * b[5].clone()
                + self.a2.as_ref().unwrap() * b[3].clone()
                + &self.ident * b[1].clone());
        let v = self.a6.as_ref().unwrap() * b[6].clone()
            + self.a4.as_ref().unwrap() * b[4].clone()
            + self.a2.as_ref().unwrap() * b[2].clone()
            + &self.ident * b[0].clone();
        (u, v)
    }

    fn pade9(&mut self) -> (OMatrix<T, D, D>, OMatrix<T, D, D>) {
        let b: [T; 10] = [
            convert(17_643_225_600.0),
            convert(8_821_612_800.0),
            convert(2_075_673_600.0),
            convert(302_702_400.0),
            convert(30_270_240.0),
            convert(2_162_160.0),
            convert(110_880.0),
            convert(3_960.0),
            convert(90.0),
            convert(1.0),
        ];
        self.calc_a2();
        self.calc_a4();
        self.calc_a6();
        self.calc_a8();
        let u = &self.a
            * (self.a8.as_ref().unwrap() * b[9].clone()
                + self.a6.as_ref().unwrap() * b[7].clone()
                + self.a4.as_ref().unwrap() * b[5].clone()
                + self.a2.as_ref().unwrap() * b[3].clone()
                + &self.ident * b[1].clone());
        let v = self.a8.as_ref().unwrap() * b[8].clone()
            + self.a6.as_ref().unwrap() * b[6].clone()
            + self.a4.as_ref().unwrap() * b[4].clone()
            + self.a2.as_ref().unwrap() * b[2].clone()
            + &self.ident * b[0].clone();
        (u, v)
    }

    fn pade13_scaled(&mut self, s: u64) -> (OMatrix<T, D, D>, OMatrix<T, D, D>) {
        let b: [T; 14] = [
            convert(64_764_752_532_480_000.0),
            convert(32_382_376_266_240_000.0),
            convert(7_771_770_303_897_600.0),
            convert(1_187_353_796_428_800.0),
            convert(129_060_195_264_000.0),
            convert(10_559_470_521_600.0),
            convert(670_442_572_800.0),
            convert(33_522_128_640.0),
            convert(1_323_241_920.0),
            convert(40_840_800.0),
            convert(960_960.0),
            convert(16_380.0),
            convert(182.0),
            convert(1.0),
        ];
        let s = s as f64;

        let mb = &self.a * convert::<f64, T>(2.0_f64.powf(-s));
        self.calc_a2();
        self.calc_a4();
        self.calc_a6();
        let mb2 = self.a2.as_ref().unwrap() * convert::<f64, T>(2.0_f64.powf(-2.0 * s));
        let mb4 = self.a4.as_ref().unwrap() * convert::<f64, T>(2.0.powf(-4.0 * s));
        let mb6 = self.a6.as_ref().unwrap() * convert::<f64, T>(2.0.powf(-6.0 * s));

        let u2 = &mb6 * (&mb6 * b[13].clone() + &mb4 * b[11].clone() + &mb2 * b[9].clone());
        let u = &mb
            * (&u2
                + &mb6 * b[7].clone()
                + &mb4 * b[5].clone()
                + &mb2 * b[3].clone()
                + &self.ident * b[1].clone());
        let v2 = &mb6 * (&mb6 * b[12].clone() + &mb4 * b[10].clone() + &mb2 * b[8].clone());
        let v = v2
            + &mb6 * b[6].clone()
            + &mb4 * b[4].clone()
            + &mb2 * b[2].clone()
            + &self.ident * b[0].clone();
        (u, v)
    }
}

/// Compute `n!`
#[inline(always)]
fn factorial(n: usize) -> u128 {
    match FACTORIAL.get(n) {
        Some(f) => *f,
        None => panic!("{n}! is greater than u128::MAX"),
    }
}

/// Compute the 1-norm of a non-negative integer power of a non-negative matrix.
fn onenorm_matrix_power_nonm<T, D>(a: &OMatrix<T, D, D>, p: usize) -> T
where
    T: RealField,
    D: Dim,
    DefaultAllocator: Allocator<D, D> + Allocator<D>,
{
    let nrows = a.shape_generic().0;
    let mut v = crate::OVector::<T, D>::repeat_generic(nrows, Const::<1>, convert(1.0));
    let m = a.transpose();

    for _ in 0..p {
        v = &m * v;
    }

    v.max()
}

fn ell<T, D>(a: &OMatrix<T, D, D>, m: usize) -> u64
where
    T: ComplexField,
    D: Dim,
    DefaultAllocator: Allocator<D, D> + Allocator<D> + Allocator<D> + Allocator<D, D>,
{
    let a_abs = a.map(|x| x.abs());

    let a_abs_onenorm = onenorm_matrix_power_nonm(&a_abs, 2 * m + 1);

    if a_abs_onenorm == <T as ComplexField>::RealField::zero() {
        return 0;
    }

    // 2m choose m = (2m)!/(m! * (2m-m)!) = (2m)!/((m!)^2)
    let m_factorial = factorial(m);
    let choose_2m_m = factorial(2 * m) / (m_factorial * m_factorial);

    let abs_c_recip = choose_2m_m * factorial(2 * m + 1);
    let alpha = a_abs_onenorm / one_norm(a);
    let alpha: f64 = try_convert::<_, f64>(alpha).unwrap() / abs_c_recip as f64;

    let u = 2_f64.powf(-53.0);
    let log2_alpha_div_u = (alpha / u).log2();
    let value = (log2_alpha_div_u / (2.0 * m as f64)).ceil();
    if value > 0.0 { value as u64 } else { 0 }
}

fn solve_p_q<T, D>(u: OMatrix<T, D, D>, v: OMatrix<T, D, D>) -> OMatrix<T, D, D>
where
    T: ComplexField,
    D: DimMin<D, Output = D>,
    DefaultAllocator: Allocator<D, D> + Allocator<DimMinimum<D, D>>,
{
    let p = &u + &v;
    let q = &v - &u;

    q.lu().solve(&p).unwrap()
}

fn one_norm<T, D>(m: &OMatrix<T, D, D>) -> T::RealField
where
    T: ComplexField,
    D: Dim,
    DefaultAllocator: Allocator<D, D>,
{
    let mut max = <T as ComplexField>::RealField::zero();

    for i in 0..m.ncols() {
        let col = m.column(i);
        max = max.max(
            col.iter()
                .fold(<T as ComplexField>::RealField::zero(), |a, b| {
                    a + b.clone().abs()
                }),
        );
    }

    max
}

impl<T: ComplexField, D> OMatrix<T, D, D>
where
    D: DimMin<D, Output = D>,
    DefaultAllocator: Allocator<D, D>
        + Allocator<DimMinimum<D, D>>
        + Allocator<D>
        + Allocator<D>
        + Allocator<D, D>,
{
    /// Computes exponential of this matrix
    #[must_use]
    pub fn exp(&self) -> Self {
        // Simple case
        if self.nrows() == 1 {
            return self.map(|v| v.exp());
        }

        let mut helper = ExpmPadeHelper::new(self.clone(), true);

        let eta_1 = T::RealField::max(helper.d4_loose(), helper.d6_loose());
        if eta_1 < convert(1.495_585_217_958_292e-2) && ell(&helper.a, 3) == 0 {
            let (u, v) = helper.pade3();
            return solve_p_q(u, v);
        }

        let eta_2 = T::RealField::max(helper.d4_tight(), helper.d6_loose());
        if eta_2 < convert(2.539_398_330_063_23e-1) && ell(&helper.a, 5) == 0 {
            let (u, v) = helper.pade5();
            return solve_p_q(u, v);
        }

        let eta_3 = T::RealField::max(helper.d6_tight(), helper.d8_loose());
        if eta_3 < convert(9.504_178_996_162_932e-1) && ell(&helper.a, 7) == 0 {
            let (u, v) = helper.pade7();
            return solve_p_q(u, v);
        }
        if eta_3 < convert(2.097_847_961_257_068e0) && ell(&helper.a, 9) == 0 {
            let (u, v) = helper.pade9();
            return solve_p_q(u, v);
        }

        let eta_4 = T::RealField::max(helper.d8_loose(), helper.d10_loose());
        let eta_5 = T::RealField::min(eta_3, eta_4);
        let theta_13 = convert(4.25);

        let mut s = if eta_5 == T::RealField::zero() {
            0
        } else {
            let l2 = try_convert::<_, f64>((eta_5 / theta_13).log2().ceil()).unwrap();

            if l2 < 0.0 { 0 } else { l2 as u64 }
        };

        s += ell(
            &(&helper.a * convert::<f64, T>(2.0_f64.powf(-(s as f64)))),
            13,
        );

        let (u, v) = helper.pade13_scaled(s);
        let mut x = solve_p_q(u, v);

        for _ in 0..s {
            x = &x * &x;
        }
        x
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[allow(clippy::float_cmp)]
    fn one_norm() {
        use crate::Matrix3;
        let m = Matrix3::new(-3.0, 5.0, 7.0, 2.0, 6.0, 4.0, 0.0, 2.0, 8.0);

        assert_eq!(super::one_norm(&m), 19.0);
    }
}

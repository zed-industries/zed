#![feature(test)]

extern crate test;

use bincode::Options as _;
use core::str::FromStr;
use rust_decimal::Decimal;

macro_rules! bench_decimal_op {
    ($name:ident, $op:tt, $x:expr, $y:expr) => {
        #[bench]
        fn $name(b: &mut ::test::Bencher) {
            let x = Decimal::from_str($x).unwrap();
            let y = Decimal::from_str($y).unwrap();
            b.iter(|| {
                let result = x $op y;
                ::test::black_box(result);
            });
        }
    }
}

macro_rules! bench_fold_op {
    ($name:ident, $op:tt, $init:expr, $count:expr) => {
        #[bench]
        fn $name(b: &mut ::test::Bencher) {
            fn fold(values: &[Decimal]) -> Decimal {
                let mut acc: Decimal = $init.into();
                for value in values {
                    acc = acc $op value;
                }
                acc
            }

            let values: Vec<Decimal> = test::black_box((1..$count).map(|i| i.into()).collect());
            b.iter(|| {
                let result = fold(&values);
                ::test::black_box(result);
            });
        }
    }
}

/* Add */
bench_decimal_op!(add_self, +, "2.01", "2.01");
bench_decimal_op!(add_simple, +, "2", "1");
bench_decimal_op!(add_one, +, "2.01", "1");
bench_decimal_op!(add_two, +, "2.01", "2");
bench_decimal_op!(add_one_hundred, +, "2.01", "100");
bench_decimal_op!(add_point_zero_one, +, "2.01", "0.01");
bench_decimal_op!(add_negative_point_five, +, "2.01", "-0.5");
bench_decimal_op!(add_pi, +, "2.01", "3.1415926535897932384626433832");
bench_decimal_op!(add_negative_pi, +, "2.01", "-3.1415926535897932384626433832");

bench_fold_op!(add_10k, +, 0, 10_000);

/* Sub */
bench_decimal_op!(sub_self, -, "2.01", "2.01");
bench_decimal_op!(sub_simple, -, "2", "1");
bench_decimal_op!(sub_one, -, "2.01", "1");
bench_decimal_op!(sub_two, -, "2.01", "2");
bench_decimal_op!(sub_one_hundred, -, "2.01", "100");
bench_decimal_op!(sub_point_zero_one, -, "2.01", "0.01");
bench_decimal_op!(sub_negative_point_five, -, "2.01", "-0.5");
bench_decimal_op!(sub_pi, -, "2.01", "3.1415926535897932384626433832");
bench_decimal_op!(sub_negative_pi, -, "2.01", "-3.1415926535897932384626433832");

bench_fold_op!(sub_10k, -, 5_000_000, 10_000);

/* Mul */
bench_decimal_op!(mul_one, *, "2.01", "1");
bench_decimal_op!(mul_two, *, "2.01", "2");
bench_decimal_op!(mul_one_hundred, *, "2.01", "100");
bench_decimal_op!(mul_point_zero_one, *, "2.01", "0.01");
bench_decimal_op!(mul_negative_point_five, *, "2.01", "-0.5");
bench_decimal_op!(mul_pi, *, "2.01", "3.1415926535897932384626433832");
bench_decimal_op!(mul_negative_pi, *, "2.01", "-3.1415926535897932384626433832");

bench_fold_op!(mul_25, *, Decimal::from_str("1.1").unwrap(), 25);

/* Div */
bench_decimal_op!(div_one, /, "2.01", "1");
bench_decimal_op!(div_two, /, "2.01", "2");
bench_decimal_op!(div_one_hundred, /, "2.01", "100");
bench_decimal_op!(div_point_zero_one, /, "2.01", "0.01");
bench_decimal_op!(div_negative_point_five, /, "2.01", "-0.5");
bench_decimal_op!(div_pi, /, "2.01", "3.1415926535897932384626433832");
bench_decimal_op!(div_negative_pi, /, "2.01", "-3.1415926535897932384626433832");
bench_decimal_op!(div_no_underflow, /, "1.02343545345", "0.35454343453");
bench_fold_op!(div_10k, /, Decimal::MAX, 10_000);
bench_fold_op!(rem_10k, %, Decimal::MAX, 10_000);

/* Iteration */
struct DecimalIterator {
    count: usize,
}

impl DecimalIterator {
    fn new() -> DecimalIterator {
        DecimalIterator { count: 0 }
    }
}

impl Iterator for DecimalIterator {
    type Item = Decimal;

    fn next(&mut self) -> Option<Decimal> {
        self.count += 1;
        if self.count < 6 {
            Some(Decimal::new(314, 2))
        } else {
            None
        }
    }
}

#[bench]
fn iterator_individual(b: &mut ::test::Bencher) {
    b.iter(|| {
        let mut result = Decimal::new(0, 0);
        let iterator = DecimalIterator::new();
        for i in iterator {
            result += i;
        }
        ::test::black_box(result);
    });
}

#[bench]
fn iterator_product(b: &mut ::test::Bencher) {
    b.iter(|| {
        let result: Decimal = DecimalIterator::new().product();
        ::test::black_box(result);
    });
}

#[bench]
fn iterator_sum(b: &mut ::test::Bencher) {
    b.iter(|| {
        let result: Decimal = DecimalIterator::new().sum();
        ::test::black_box(result);
    });
}

const SAMPLE_STRS: &[&str] = &[
    "3950.123456",
    "3950",
    "0.1",
    "0.01",
    "0.001",
    "0.0001",
    "0.00001",
    "0.000001",
    "1",
    "-100",
    "-123.456",
    "119996.25",
    "1000000",
    "9999999.99999",
    "12340.56789",
];

#[bench]
fn serialize_bincode(b: &mut test::Bencher) {
    let decimals: Vec<Decimal> = SAMPLE_STRS.iter().map(|s| Decimal::from_str(s).unwrap()).collect();

    b.iter(|| {
        for d in &decimals {
            let bytes = bincode::options().serialize(d).unwrap();
            test::black_box(bytes);
        }
    })
}

#[cfg(feature = "serde-str")]
#[bench]
fn deserialize_bincode(b: &mut test::Bencher) {
    let payloads: Vec<Vec<u8>> = SAMPLE_STRS
        .iter()
        .map(|s| bincode::options().serialize(&Decimal::from_str(s).unwrap()).unwrap())
        .collect();

    b.iter(|| {
        for payload in &payloads {
            let decimal: Decimal = bincode::options().deserialize(payload).unwrap();
            test::black_box(decimal);
        }
    })
}

#[bench]
fn decimal_from_str(b: &mut test::Bencher) {
    b.iter(|| {
        for s in SAMPLE_STRS {
            let result = Decimal::from_str(s).unwrap();
            test::black_box(result);
        }
    })
}

#[bench]
fn decimal_to_string(b: &mut test::Bencher) {
    let decimals: Vec<Decimal> = SAMPLE_STRS.iter().map(|s| Decimal::from_str(s).unwrap()).collect();

    b.iter(|| {
        for s in decimals.iter() {
            let string = s.to_string();
            test::black_box(string);
        }
    })
}

#[cfg(feature = "postgres")]
#[bench]
fn to_from_sql(b: &mut ::test::Bencher) {
    use bytes::BytesMut;
    use postgres::types::{FromSql, Kind, ToSql, Type};

    let samples: Vec<Decimal> = test::black_box(SAMPLE_STRS.iter().map(|x| Decimal::from_str(x).unwrap()).collect());
    let t = Type::new("".into(), 0, Kind::Simple, "".into());
    let mut bytes: BytesMut = BytesMut::with_capacity(100).into();

    b.iter(|| {
        for _ in 0..100 {
            for sample in &samples {
                bytes.clear();
                sample.to_sql(&t, &mut bytes).unwrap();
                let result = Decimal::from_sql(&t, &bytes).unwrap();
                ::test::black_box(result);
            }
        }
    });
}

#[cfg(feature = "maths")]
mod maths {
    use rust_decimal::prelude::*;

    #[bench]
    fn powi(b: &mut ::test::Bencher) {
        // These exponents have to be fairly small because multiplcation overflows easily
        let samples = &[
            (Decimal::from_str("36.7").unwrap(), 5),
            (Decimal::from_str("0.00000007").unwrap(), 5),
            (Decimal::from(2), 64),
            (Decimal::from_str("8819287.19276555").unwrap(), 3),
            (Decimal::from_str("-8819287.19276555").unwrap(), 3),
        ];
        b.iter(|| {
            for sample in samples.iter() {
                let result = sample.0.powi(sample.1);
                ::test::black_box(result);
            }
        });
    }

    #[bench]
    fn sqrt(b: &mut ::test::Bencher) {
        let samples = &[
            Decimal::from_str("36.7").unwrap(),
            Decimal::from_str("0.00000007").unwrap(),
            Decimal::from(2),
            Decimal::from_str("8819287.19276555").unwrap(),
            Decimal::from_str("-8819287.19276555").unwrap(),
        ];
        b.iter(|| {
            for sample in samples.iter() {
                let result = sample.sqrt();
                ::test::black_box(result);
            }
        });
    }

    #[bench]
    fn exp(b: &mut ::test::Bencher) {
        let samples = &[
            Decimal::from_str("3.7").unwrap(),
            Decimal::from_str("0.07").unwrap(),
            Decimal::from(2),
            Decimal::from_str("8.19").unwrap(),
            Decimal::from_str("-8.19").unwrap(),
        ];
        b.iter(|| {
            for sample in samples.iter() {
                let result = sample.exp();
                ::test::black_box(result);
            }
        });
    }

    #[bench]
    fn exp_large(b: &mut ::test::Bencher) {
        let samples = &[
            Decimal::from_str("17.6").unwrap(),
            Decimal::from_str("34.5").unwrap(),
            Decimal::from_str("54.3").unwrap(),
        ];
        b.iter(|| {
            for sample in samples.iter() {
                let result = sample.exp();
                ::test::black_box(result);
            }
        });
    }

    #[bench]
    fn norm_cdf(b: &mut ::test::Bencher) {
        let samples = &[
            Decimal::from_str("3.7").unwrap(),
            Decimal::from_str("0.007").unwrap(),
            Decimal::from(2),
            Decimal::from_str("1.19").unwrap(),
            Decimal::from_str("-1.19").unwrap(),
        ];
        b.iter(|| {
            for sample in samples.iter() {
                let result = sample.norm_cdf();
                ::test::black_box(result);
            }
        });
    }

    #[bench]
    fn norm_pdf(b: &mut ::test::Bencher) {
        let samples = &[
            Decimal::from_str("3.7").unwrap(),
            Decimal::from_str("0.007").unwrap(),
            Decimal::from(2),
            Decimal::from_str("1.19").unwrap(),
            Decimal::from_str("-1.19").unwrap(),
        ];
        b.iter(|| {
            for sample in samples.iter() {
                let result = sample.norm_pdf();
                ::test::black_box(result);
            }
        });
    }

    #[bench]
    fn ln(b: &mut ::test::Bencher) {
        let samples = &[
            Decimal::from_str("36.7").unwrap(),
            Decimal::from_str("0.00000007").unwrap(),
            Decimal::from(2),
            Decimal::from_str("8819287.19").unwrap(),
            Decimal::from_str("-8819287.19").unwrap(),
        ];
        b.iter(|| {
            for sample in samples.iter() {
                let result = sample.ln();
                ::test::black_box(result);
            }
        });
    }

    #[bench]
    fn erf(b: &mut ::test::Bencher) {
        let samples = &[
            Decimal::from(0),
            Decimal::from(1),
            Decimal::from_str("-0.98717").unwrap(),
            Decimal::from_str("0.07").unwrap(),
            Decimal::from_str("0.1111").unwrap(),
            Decimal::from_str("0.4").unwrap(),
        ];
        b.iter(|| {
            for sample in samples.iter() {
                let result = sample.erf();
                ::test::black_box(result);
            }
        });
    }
}

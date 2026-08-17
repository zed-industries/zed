extern crate bytes;

use self::bytes::BytesMut;
use num::traits::Bounded;
use postgres_types::{FromSql, IsNull, ToSql, Type};

use std::error::Error;
use std::fmt;

use super::GenericDecimal;
use convert::TryToConvertFrom;
use fraction::GenericFraction;
use generic::GenericInteger;

use fraction::postgres_support::{fraction_to_sql_buf, read_i16, PG_MAX_PRECISION};

impl<'a, T, P> FromSql<'a> for GenericDecimal<T, P>
where
    T: Clone + GenericInteger + From<u16>,
    P: Copy + GenericInteger + Into<usize> + Bounded + TryToConvertFrom<i16> + fmt::Display,
{
    fn from_sql(ty: &Type, raw: &[u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        if raw.len() < 8 {
            return Err("unexpected data package from the database".into());
        }

        let scale: i16 = read_i16(&raw[6..8])?;
        let precision = if let Some(precision) = P::try_to_convert_from(scale) {
            precision
        } else {
            return Err(format!(
                "{} {};\n{} {};\n {} {};\n{} {}",
                r#"The precision of the source is too big: "#,
                scale,
                r#" your decimal type supports up to "#,
                P::max_value(),
                r#"you may increase the precision type size up to "usize", which is "#,
                usize::max_value(),
                r#"PostgreSQL supports precision up to "#,
                PG_MAX_PRECISION
            )
            .into());
        };

        Ok(GenericDecimal(
            <GenericFraction<T> as FromSql>::from_sql(ty, raw)?,
            precision,
        ))
    }

    accepts!(NUMERIC);
}

impl<T, P> ToSql for GenericDecimal<T, P>
where
    T: Clone + GenericInteger + From<u8> + fmt::Debug,
    P: Copy + GenericInteger + Into<usize> + fmt::Debug,
{
    fn to_sql(
        &self,
        ty: &Type,
        buf: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        match *self {
            GenericDecimal(ref fraction, precision) => {
                fraction_to_sql_buf(fraction, ty, buf, precision.into())
            }
        }
    }

    accepts!(NUMERIC);

    to_sql_checked!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use {One, Zero};

    type Decimal = GenericDecimal<u128, u16>;

    const NUMERIC_OID: u32 = 1700;

    fn get_tests() -> Vec<(Decimal, &'static [u8])> {
        vec![
            (
                Decimal::from_str("-12345678901234").unwrap(),
                &[0, 4, 0, 3, 64, 0, 0, 0, 0, 12, 13, 128, 30, 210, 4, 210],
            ),
            (
                Decimal::from_str("-12345678").unwrap(),
                &[0, 2, 0, 1, 64, 0, 0, 0, 4, 210, 22, 46],
            ),
            (
                Decimal::from_str("-1000000000.0000000001").unwrap(),
                &[
                    0, 6, 0, 2, 64, 0, 0, 10, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 100,
                ],
            ),
            (
                Decimal::from_str("-1000000000").unwrap(),
                &[0, 1, 0, 2, 64, 0, 0, 0, 0, 10],
            ),
            (
                Decimal::from_str("-1234").unwrap(),
                &[0, 1, 0, 0, 64, 0, 0, 0, 4, 210],
            ),
            (
                Decimal::from_str("-256").unwrap(),
                &[0, 1, 0, 0, 64, 0, 0, 0, 1, 0],
            ),
            (
                Decimal::from_str("-42").unwrap(),
                &[0, 1, 0, 0, 64, 0, 0, 0, 0, 42],
            ),
            (
                Decimal::from_str("-1").unwrap(),
                &[0, 1, 0, 0, 64, 0, 0, 0, 0, 1],
            ),
            (
                Decimal::from_str("-0.5").unwrap(),
                &[0, 1, 255, 255, 64, 0, 0, 1, 19, 136],
            ),
            (
                Decimal::from_str("-0.1").unwrap(),
                &[0, 1, 255, 255, 64, 0, 0, 1, 3, 232],
            ),
            (
                Decimal::from_str("-0.66").unwrap(),
                &[0, 1, 255, 255, 64, 0, 0, 2, 25, 200],
            ),
            (
                Decimal::from_str("-0.12345678901234").unwrap(),
                &[0, 4, 255, 255, 64, 0, 0, 14, 4, 210, 22, 46, 35, 52, 13, 72],
            ),
            (
                Decimal::from_str("-0.01").unwrap(),
                &[0, 1, 255, 255, 64, 0, 0, 2, 0, 100],
            ),
            (
                Decimal::from_str("-0.2404").unwrap(),
                &[0, 1, 255, 255, 64, 0, 0, 4, 9, 100],
            ),
            (
                Decimal::from_str("-0.000000001").unwrap(),
                &[0, 1, 255, 253, 64, 0, 0, 9, 3, 232],
            ),
            (
                Decimal::from_str("-12345678901234.12345678901234").unwrap(),
                &[
                    0, 8, 0, 3, 64, 0, 0, 14, 0, 12, 13, 128, 30, 210, 4, 210, 4, 210, 22, 46, 35,
                    52, 13, 72,
                ],
            ),
            (Decimal::zero(), &[0, 0, 0, 0, 0, 0, 0, 0]),
            (Decimal::nan(), &[0, 0, 0, 0, 192, 0, 0, 0]),
            (
                Decimal::from_str("12345678901234.12345678901234").unwrap(),
                &[
                    0, 8, 0, 3, 0, 0, 0, 14, 0, 12, 13, 128, 30, 210, 4, 210, 4, 210, 22, 46, 35,
                    52, 13, 72,
                ],
            ),
            (
                Decimal::from("0.000000001"),
                &[0, 1, 255, 253, 0, 0, 0, 9, 3, 232],
            ),
            (
                Decimal::from(0.2404f64),
                &[0, 1, 255, 255, 0, 0, 0, 4, 9, 100],
            ),
            (
                Decimal::from(0.01f32),
                &[0, 1, 255, 255, 0, 0, 0, 2, 0, 100],
            ),
            (
                Decimal::from(0.66f32),
                &[0, 1, 255, 255, 0, 0, 0, 2, 25, 200],
            ),
            (
                Decimal::from_str("1000000.0000000000000001").unwrap(),
                &[
                    0, 6, 0, 1, 0, 0, 0, 16, 0, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
                ],
            ),
            (
                Decimal::from_str("0.12345678901234").unwrap(),
                &[0, 4, 255, 255, 0, 0, 0, 14, 4, 210, 22, 46, 35, 52, 13, 72],
            ),
            (Decimal::from(0.1f32), &[0, 1, 255, 255, 0, 0, 0, 1, 3, 232]),
            (
                Decimal::from(0.5f32),
                &[0, 1, 255, 255, 0, 0, 0, 1, 19, 136],
            ),
            (
                Decimal::from(0.55f32).set_precision(1),
                &[0, 1, 255, 255, 0, 0, 0, 1, 19, 136],
            ),
            (Decimal::one(), &[0, 1, 0, 0, 0, 0, 0, 0, 0, 1]),
            (Decimal::from(42u8), &[0, 1, 0, 0, 0, 0, 0, 0, 0, 42]),
            (Decimal::from(256u16), &[0, 1, 0, 0, 0, 0, 0, 0, 1, 0]),
            (Decimal::from(1234u16), &[0, 1, 0, 0, 0, 0, 0, 0, 4, 210]),
            (
                Decimal::from(1000000000u64),
                &[0, 1, 0, 2, 0, 0, 0, 0, 0, 10],
            ),
            (
                Decimal::from("1000000000.0000000001"),
                &[
                    0, 6, 0, 2, 0, 0, 0, 10, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 100,
                ],
            ),
            (
                Decimal::from(12345678u32),
                &[0, 2, 0, 1, 0, 0, 0, 0, 4, 210, 22, 46],
            ),
            (
                Decimal::from(12345678901234u64),
                &[0, 4, 0, 3, 0, 0, 0, 0, 0, 12, 13, 128, 30, 210, 4, 210],
            ),
            (
                Decimal::from("0.33333333333333333333"),
                &[
                    0, 5, 255, 255, 0, 0, 0, 20, 13, 5, 13, 5, 13, 5, 13, 5, 13, 5,
                ],
            ),
        ]
    }

    #[test]
    fn test_from_sql() {
        let t = Type::from_oid(NUMERIC_OID).unwrap();
        for ref test in &get_tests() {
            assert_eq!(
                test.0,
                <Decimal as FromSql>::from_sql(&t, test.1).ok().unwrap()
            )
        }
    }

    #[test]
    fn test_to_sql() {
        let t = Type::from_oid(NUMERIC_OID).unwrap();
        let mut buf = BytesMut::with_capacity(1024);

        for ref test in &get_tests() {
            buf.clear();
            let res = <Decimal as ToSql>::to_sql(&test.0, &t, &mut buf)
                .ok()
                .unwrap();

            match res {
                IsNull::Yes => assert!(false),
                IsNull::No => assert!(true),
            };

            assert_eq!(&buf, &test.1);
        }
    }
}

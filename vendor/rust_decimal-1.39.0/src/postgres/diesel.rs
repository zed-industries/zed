use crate::postgres::common::*;
use crate::Decimal;
use diesel::{
    deserialize::{self, FromSql},
    pg::data_types::PgNumeric,
    pg::Pg,
    serialize::{self, Output, ToSql},
    sql_types::Numeric,
};
use std::error;

impl<'a> TryFrom<&'a PgNumeric> for Decimal {
    type Error = Box<dyn error::Error + Send + Sync>;

    fn try_from(numeric: &'a PgNumeric) -> deserialize::Result<Self> {
        let (neg, weight, scale, digits) = match *numeric {
            PgNumeric::Positive {
                weight,
                scale,
                ref digits,
            } => (false, weight, scale, digits),
            PgNumeric::Negative {
                weight,
                scale,
                ref digits,
            } => (true, weight, scale, digits),
            PgNumeric::NaN => return Err(Box::from("NaN is not supported in Decimal")),
        };

        let Some(result) = Self::checked_from_postgres(PostgresDecimal {
            neg,
            weight,
            scale,
            digits: digits.iter().copied().map(|v| v.try_into().unwrap()),
        }) else {
            return Err(Box::new(crate::error::Error::ExceedsMaximumPossibleValue));
        };
        Ok(result)
    }
}

impl TryFrom<PgNumeric> for Decimal {
    type Error = Box<dyn error::Error + Send + Sync>;

    fn try_from(numeric: PgNumeric) -> deserialize::Result<Self> {
        (&numeric).try_into()
    }
}

impl<'a> From<&'a Decimal> for PgNumeric {
    fn from(decimal: &'a Decimal) -> Self {
        let PostgresDecimal {
            neg,
            weight,
            scale,
            digits,
        } = decimal.to_postgres();

        if neg {
            PgNumeric::Negative { digits, scale, weight }
        } else {
            PgNumeric::Positive { digits, scale, weight }
        }
    }
}

impl From<Decimal> for PgNumeric {
    fn from(decimal: Decimal) -> Self {
        (&decimal).into()
    }
}

#[cfg(feature = "diesel")]
impl ToSql<Numeric, Pg> for Decimal {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let numeric = PgNumeric::from(self);
        ToSql::<Numeric, Pg>::to_sql(&numeric, &mut out.reborrow())
    }
}

#[cfg(feature = "diesel")]
impl FromSql<Numeric, Pg> for Decimal {
    fn from_sql(numeric: diesel::pg::PgValue) -> deserialize::Result<Self> {
        PgNumeric::from_sql(numeric)?.try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::str::FromStr;

    #[test]
    fn test_unnecessary_zeroes() {
        fn extract(value: &str) -> Decimal {
            Decimal::from_str(value).unwrap()
        }

        let tests = &[
            ("0.000001660"),
            ("41.120255926293000"),
            ("0.5538973300"),
            ("08883.55986854293100"),
            ("0.0000_0000_0016_6000_00"),
            ("0.00000166650000"),
            ("1666500000000"),
            ("1666500000000.0000054500"),
            ("8944.000000000000"),
        ];

        for &value in tests {
            let value = extract(value);
            let pg = PgNumeric::from(value);
            let dec = Decimal::try_from(pg).unwrap();
            assert_eq!(dec, value);
        }
    }

    #[test]
    fn decimal_to_pgnumeric_converts_digits_to_base_10000() {
        let decimal = Decimal::from_str("1").unwrap();
        let expected = PgNumeric::Positive {
            weight: 0,
            scale: 0,
            digits: vec![1],
        };
        assert_eq!(expected, decimal.into());

        let decimal = Decimal::from_str("10").unwrap();
        let expected = PgNumeric::Positive {
            weight: 0,
            scale: 0,
            digits: vec![10],
        };
        assert_eq!(expected, decimal.into());

        let decimal = Decimal::from_str("10000").unwrap();
        let expected = PgNumeric::Positive {
            weight: 1,
            scale: 0,
            digits: vec![1, 0],
        };
        assert_eq!(expected, decimal.into());

        let decimal = Decimal::from_str("10001").unwrap();
        let expected = PgNumeric::Positive {
            weight: 1,
            scale: 0,
            digits: vec![1, 1],
        };
        assert_eq!(expected, decimal.into());

        let decimal = Decimal::from_str("100000000").unwrap();
        let expected = PgNumeric::Positive {
            weight: 2,
            scale: 0,
            digits: vec![1, 0, 0],
        };
        assert_eq!(expected, decimal.into());
    }

    #[test]
    fn decimal_to_pg_numeric_properly_adjusts_scale() {
        let decimal = Decimal::from_str("1").unwrap();
        let expected = PgNumeric::Positive {
            weight: 0,
            scale: 0,
            digits: vec![1],
        };
        assert_eq!(expected, decimal.into());

        let decimal = Decimal::from_str("1.0").unwrap();
        let expected = PgNumeric::Positive {
            weight: 0,
            scale: 1,
            digits: vec![1],
        };
        assert_eq!(expected, decimal.into());

        let decimal = Decimal::from_str("1.1").unwrap();
        let expected = PgNumeric::Positive {
            weight: 0,
            scale: 1,
            digits: vec![1, 1000],
        };
        assert_eq!(expected, decimal.into());

        let decimal = Decimal::from_str("1.10").unwrap();
        let expected = PgNumeric::Positive {
            weight: 0,
            scale: 2,
            digits: vec![1, 1000],
        };
        assert_eq!(expected, decimal.into());

        let decimal = Decimal::from_str("100000000.0001").unwrap();
        let expected = PgNumeric::Positive {
            weight: 2,
            scale: 4,
            digits: vec![1, 0, 0, 1],
        };
        assert_eq!(expected, decimal.into());

        let decimal = Decimal::from_str("0.1").unwrap();
        let expected = PgNumeric::Positive {
            weight: -1,
            scale: 1,
            digits: vec![1000],
        };
        assert_eq!(expected, decimal.into());
    }

    #[test]
    fn decimal_to_pg_numeric_retains_sign() {
        let decimal = Decimal::from_str("123.456").unwrap();
        let expected = PgNumeric::Positive {
            weight: 0,
            scale: 3,
            digits: vec![123, 4560],
        };
        assert_eq!(expected, decimal.into());

        let decimal = Decimal::from_str("-123.456").unwrap();
        let expected = PgNumeric::Negative {
            weight: 0,
            scale: 3,
            digits: vec![123, 4560],
        };
        assert_eq!(expected, decimal.into());
    }

    #[test]
    fn pg_numeric_to_decimal_works() {
        let expected = Decimal::from_str("50").unwrap();
        let pg_numeric = PgNumeric::Positive {
            weight: 0,
            scale: 0,
            digits: vec![50],
        };
        let res: Decimal = pg_numeric.try_into().unwrap();
        assert_eq!(res, expected);
        let expected = Decimal::from_str("123.456").unwrap();
        let pg_numeric = PgNumeric::Positive {
            weight: 0,
            scale: 3,
            digits: vec![123, 4560],
        };
        let res: Decimal = pg_numeric.try_into().unwrap();
        assert_eq!(res, expected);

        let expected = Decimal::from_str("-56.78").unwrap();
        let pg_numeric = PgNumeric::Negative {
            weight: 0,
            scale: 2,
            digits: vec![56, 7800],
        };
        let res: Decimal = pg_numeric.try_into().unwrap();
        assert_eq!(res, expected);

        // Verify no trailing zeroes are lost.

        let expected = Decimal::from_str("1.100").unwrap();
        let pg_numeric = PgNumeric::Positive {
            weight: 0,
            scale: 3,
            digits: vec![1, 1000],
        };
        let res: Decimal = pg_numeric.try_into().unwrap();
        assert_eq!(res.to_string(), expected.to_string());

        // To represent 5.00, Postgres can return either [5, 0] as the list of digits.
        let expected = Decimal::from_str("5.00").unwrap();
        let pg_numeric = PgNumeric::Positive {
            weight: 0,
            scale: 2,

            digits: vec![5, 0],
        };
        let res: Decimal = pg_numeric.try_into().unwrap();
        assert_eq!(res.to_string(), expected.to_string());

        // To represent 5.00, Postgres can return [5] as the list of digits.
        let expected = Decimal::from_str("5.00").unwrap();
        let pg_numeric = PgNumeric::Positive {
            weight: 0,
            scale: 2,
            digits: vec![5],
        };
        let res: Decimal = pg_numeric.try_into().unwrap();
        assert_eq!(res.to_string(), expected.to_string());

        let expected = Decimal::from_str("3.1415926535897932384626433833").unwrap();
        let pg_numeric = PgNumeric::Positive {
            weight: 0,
            scale: 30,
            digits: vec![3, 1415, 9265, 3589, 7932, 3846, 2643, 3832, 7950, 2800],
        };
        let res: Decimal = pg_numeric.try_into().unwrap();
        assert_eq!(res.to_string(), expected.to_string());

        let expected = Decimal::from_str("3.1415926535897932384626433833").unwrap();
        let pg_numeric = PgNumeric::Positive {
            weight: 0,
            scale: 34,
            digits: vec![3, 1415, 9265, 3589, 7932, 3846, 2643, 3832, 7950, 2800],
        };

        let res: Decimal = pg_numeric.try_into().unwrap();
        assert_eq!(res.to_string(), expected.to_string());

        let expected = Decimal::from_str("1.2345678901234567890123456790").unwrap();
        let pg_numeric = PgNumeric::Positive {
            weight: 0,
            scale: 34,
            digits: vec![1, 2345, 6789, 0123, 4567, 8901, 2345, 6789, 5000, 0],
        };

        let res: Decimal = pg_numeric.try_into().unwrap();
        assert_eq!(res.to_string(), expected.to_string());
    }
}

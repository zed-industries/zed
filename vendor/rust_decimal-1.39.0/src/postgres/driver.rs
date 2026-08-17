use crate::error::Error;
use crate::postgres::common::*;
use crate::Decimal;
use bytes::{BufMut, BytesMut};
use postgres_types::{to_sql_checked, FromSql, IsNull, ToSql, Type};
use std::io::{Cursor, Read};

// These are from numeric.c in the PostgreSQL source code
const NUMERIC_NAN: u16 = 0xC000;
const NUMERIC_PINF: u16 = 0xD000;
const NUMERIC_NINF: u16 = 0xF000;
const NUMERIC_SPECIAL: u16 = 0xC000;

fn read_two_bytes(cursor: &mut Cursor<&[u8]>) -> std::io::Result<[u8; 2]> {
    let mut result = [0; 2];
    cursor.read_exact(&mut result)?;
    Ok(result)
}

impl<'a> FromSql<'a> for Decimal {
    // Decimals are represented as follows:
    // Header:
    //  u16 numGroups
    //  i16 weightFirstGroup (10000^weight)
    //  u16 sign (0x0000 = positive, 0x4000 = negative, 0xC000 = NaN)
    //  i16 dscale. Number of digits (in base 10) to print after decimal separator
    //
    //  Pseudo code :
    //  const Decimals [
    //          0.0000000000000000000000000001,
    //          0.000000000000000000000001,
    //          0.00000000000000000001,
    //          0.0000000000000001,
    //          0.000000000001,
    //          0.00000001,
    //          0.0001,
    //          1,
    //          10000,
    //          100000000,
    //          1000000000000,
    //          10000000000000000,
    //          100000000000000000000,
    //          1000000000000000000000000,
    //          10000000000000000000000000000
    //  ]
    //  overflow = false
    //  result = 0
    //  for i = 0, weight = weightFirstGroup + 7; i < numGroups; i++, weight--
    //    group = read.u16
    //    if weight < 0 or weight > MaxNum
    //       overflow = true
    //    else
    //       result += Decimals[weight] * group
    //  sign == 0x4000 ? -result : result

    // So if we were to take the number: 3950.123456
    //
    //  Stored on Disk:
    //    00 03 00 00 00 00 00 06 0F 6E 04 D2 15 E0
    //
    //  Number of groups: 00 03
    //  Weight of first group: 00 00
    //  Sign: 00 00
    //  DScale: 00 06
    //
    // 0F 6E = 3950
    //   result = result + 3950 * 1;
    // 04 D2 = 1234
    //   result = result + 1234 * 0.0001;
    // 15 E0 = 5600
    //   result = result + 5600 * 0.00000001;
    //

    fn from_sql(_: &Type, raw: &[u8]) -> Result<Decimal, Box<dyn std::error::Error + 'static + Sync + Send>> {
        let mut raw = Cursor::new(raw);
        let num_groups = u16::from_be_bytes(read_two_bytes(&mut raw)?);
        let weight = i16::from_be_bytes(read_two_bytes(&mut raw)?); // 10000^weight
                                                                    // Sign: 0x0000 = positive, 0x4000 = negative, 0xC000 = NaN
        let sign = u16::from_be_bytes(read_two_bytes(&mut raw)?);

        if (sign & NUMERIC_SPECIAL) == NUMERIC_SPECIAL {
            let special = match sign {
                NUMERIC_NAN => "NaN",
                NUMERIC_PINF => "Infinity",
                NUMERIC_NINF => "-Infinity",
                // This shouldn't be hit unless postgres adds a new special numeric type in the
                // future
                _ => "unknown special numeric",
            };

            return Err(Box::new(Error::ConversionTo(special.to_string())));
        }

        // Number of digits (in base 10) to print after decimal separator
        let scale = u16::from_be_bytes(read_two_bytes(&mut raw)?);

        // Read all of the groups
        let mut groups = Vec::new();
        for _ in 0..num_groups as usize {
            groups.push(u16::from_be_bytes(read_two_bytes(&mut raw)?));
        }

        let Some(result) = Self::checked_from_postgres(PostgresDecimal {
            neg: sign == 0x4000,
            weight,
            scale,
            digits: groups.into_iter(),
        }) else {
            return Err(Box::new(crate::error::Error::ExceedsMaximumPossibleValue));
        };
        Ok(result)
    }

    fn accepts(ty: &Type) -> bool {
        matches!(*ty, Type::NUMERIC)
    }
}

impl ToSql for Decimal {
    fn to_sql(
        &self,
        _: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn std::error::Error + 'static + Sync + Send>> {
        let PostgresDecimal {
            neg,
            weight,
            scale,
            digits,
        } = self.to_postgres();

        let num_digits = digits.len();

        // Reserve bytes
        out.reserve(8 + num_digits * 2);

        // Number of groups
        out.put_u16(num_digits.try_into().unwrap());
        // Weight of first group
        out.put_i16(weight);
        // Sign
        out.put_u16(if neg { 0x4000 } else { 0x0000 });
        // DScale
        out.put_u16(scale);
        // Now process the number
        for digit in digits[0..num_digits].iter() {
            out.put_i16(*digit);
        }

        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool {
        matches!(*ty, Type::NUMERIC)
    }

    to_sql_checked!();
}

#[cfg(test)]
mod test {
    use super::*;
    use ::postgres::{Client, NoTls};
    use core::str::FromStr;

    /// Gets the URL for connecting to PostgreSQL for testing. Set the POSTGRES_URL
    /// environment variable to change from the default of "postgres://postgres@localhost".
    fn get_postgres_url() -> String {
        if let Ok(url) = std::env::var("POSTGRES_URL") {
            return url;
        }
        "postgres://postgres@localhost".to_string()
    }

    pub static TEST_DECIMALS: &[(u32, u32, &str, &str)] = &[
        // precision, scale, sent, expected
        (35, 6, "3950.123456", "3950.123456"),
        (35, 2, "3950.123456", "3950.12"),
        (35, 2, "3950.1256", "3950.13"),
        (10, 2, "3950.123456", "3950.12"),
        (35, 6, "3950", "3950.000000"),
        (4, 0, "3950", "3950"),
        (35, 6, "0.1", "0.100000"),
        (35, 6, "0.01", "0.010000"),
        (35, 6, "0.001", "0.001000"),
        (35, 6, "0.0001", "0.000100"),
        (35, 6, "0.00001", "0.000010"),
        (35, 6, "0.000001", "0.000001"),
        (35, 6, "1", "1.000000"),
        (35, 6, "-100", "-100.000000"),
        (35, 6, "-123.456", "-123.456000"),
        (35, 6, "119996.25", "119996.250000"),
        (35, 6, "1000000", "1000000.000000"),
        (35, 6, "9999999.99999", "9999999.999990"),
        (35, 6, "12340.56789", "12340.567890"),
        // Scale is only 28 since that is the maximum we can represent.
        (65, 30, "1.2", "1.2000000000000000000000000000"),
        // Pi - rounded at scale 28
        (
            65,
            30,
            "3.141592653589793238462643383279",
            "3.1415926535897932384626433833",
        ),
        (
            65,
            34,
            "3.1415926535897932384626433832795028",
            "3.1415926535897932384626433833",
        ),
        // Unrounded number
        (
            65,
            34,
            "1.234567890123456789012345678950000",
            "1.2345678901234567890123456790",
        ),
        (
            65,
            34, // No rounding due to 49999 after significant digits
            "1.234567890123456789012345678949999",
            "1.2345678901234567890123456789",
        ),
        // 0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF (96 bit)
        (35, 0, "79228162514264337593543950335", "79228162514264337593543950335"),
        // 0x0FFF_FFFF_FFFF_FFFF_FFFF_FFFF (95 bit)
        (35, 1, "4951760157141521099596496895", "4951760157141521099596496895.0"),
        // 0x1000_0000_0000_0000_0000_0000
        (35, 1, "4951760157141521099596496896", "4951760157141521099596496896.0"),
        (35, 6, "18446744073709551615", "18446744073709551615.000000"),
        (35, 6, "-18446744073709551615", "-18446744073709551615.000000"),
        (35, 6, "0.10001", "0.100010"),
        (35, 6, "0.12345", "0.123450"),
    ];

    #[test]
    fn test_null() {
        let mut client = match Client::connect(&get_postgres_url(), NoTls) {
            Ok(x) => x,
            Err(err) => panic!("{:#?}", err),
        };

        // Test NULL
        let result: Option<Decimal> = match client.query("SELECT NULL::numeric", &[]) {
            Ok(x) => x.first().unwrap().get(0),
            Err(err) => panic!("{:#?}", err),
        };
        assert_eq!(None, result);
    }

    #[tokio::test]
    #[cfg(feature = "tokio-pg")]
    async fn async_test_null() {
        use futures::future::FutureExt;
        use tokio_postgres::connect;

        let (client, connection) = connect(&get_postgres_url(), NoTls).await.unwrap();
        let connection = connection.map(|e| e.unwrap());
        tokio::spawn(connection);

        let statement = client.prepare("SELECT NULL::numeric").await.unwrap();
        let rows = client.query(&statement, &[]).await.unwrap();
        let result: Option<Decimal> = rows.first().unwrap().get(0);

        assert_eq!(None, result);
    }

    #[test]
    fn read_very_small_numeric_type() {
        let mut client = match Client::connect(&get_postgres_url(), NoTls) {
            Ok(x) => x,
            Err(err) => panic!("{:#?}", err),
        };
        let result: Decimal = match client.query("SELECT 1e-130::NUMERIC(130, 0)", &[]) {
            Ok(x) => x.first().unwrap().get(0),
            Err(err) => panic!("error - {:#?}", err),
        };
        // We compare this to zero since it is so small that it is effectively zero
        assert_eq!(Decimal::ZERO, result);
    }

    #[test]
    fn read_small_unconstrained_numeric_type() {
        let mut client = match Client::connect(&get_postgres_url(), NoTls) {
            Ok(x) => x,
            Err(err) => panic!("{:#?}", err),
        };
        let result: Decimal = match client.query("SELECT 0.100000000000000000000000000001::NUMERIC", &[]) {
            Ok(x) => x.first().unwrap().get(0),
            Err(err) => panic!("error - {:#?}", err),
        };

        // This gets rounded to 28 decimal places. In the future we may want to introduce a global feature which
        // prevents rounding.
        assert_eq!(result.to_string(), "0.1000000000000000000000000000");
        assert_eq!(result.scale(), 28);
    }

    #[test]
    fn read_small_unconstrained_numeric_type_addition() {
        let mut client = match Client::connect(&get_postgres_url(), NoTls) {
            Ok(x) => x,
            Err(err) => panic!("{:#?}", err),
        };
        let (a, b): (Decimal, Decimal) = match client.query(
            "SELECT 0.100000000000000000000000000001::NUMERIC, 0.00000000000014780214::NUMERIC",
            &[],
        ) {
            Ok(x) => {
                let row = x.first().unwrap();
                (row.get(0), row.get(1))
            }
            Err(err) => panic!("error - {:#?}", err),
        };

        assert_eq!(a + b, Decimal::from_str("0.1000000000001478021400000000").unwrap());
    }

    #[test]
    fn read_numeric_type() {
        let mut client = match Client::connect(&get_postgres_url(), NoTls) {
            Ok(x) => x,
            Err(err) => panic!("{:#?}", err),
        };
        for &(precision, scale, sent, expected) in TEST_DECIMALS.iter() {
            let result: Decimal =
                match client.query(&*format!("SELECT {}::NUMERIC({}, {})", sent, precision, scale), &[]) {
                    Ok(x) => x.first().unwrap().get(0),
                    Err(err) => panic!("SELECT {}::NUMERIC({}, {}), error - {:#?}", sent, precision, scale, err),
                };
            assert_eq!(
                expected,
                result.to_string(),
                "NUMERIC({}, {}) sent: {}",
                precision,
                scale,
                sent
            );
        }
    }

    #[tokio::test]
    #[cfg(feature = "tokio-pg")]
    async fn async_read_numeric_type() {
        use futures::future::FutureExt;
        use tokio_postgres::connect;

        let (client, connection) = connect(&get_postgres_url(), NoTls).await.unwrap();
        let connection = connection.map(|e| e.unwrap());
        tokio::spawn(connection);
        for &(precision, scale, sent, expected) in TEST_DECIMALS.iter() {
            let statement = client
                .prepare(&format!("SELECT {}::NUMERIC({}, {})", sent, precision, scale))
                .await
                .unwrap();
            let rows = client.query(&statement, &[]).await.unwrap();
            let result: Decimal = rows.first().unwrap().get(0);

            assert_eq!(expected, result.to_string(), "NUMERIC({}, {})", precision, scale);
        }
    }

    #[test]
    fn write_numeric_type() {
        let mut client = match Client::connect(&get_postgres_url(), NoTls) {
            Ok(x) => x,
            Err(err) => panic!("{:#?}", err),
        };
        for &(precision, scale, sent, expected) in TEST_DECIMALS.iter() {
            let number = Decimal::from_str(sent).unwrap();
            let result: Decimal =
                match client.query(&*format!("SELECT $1::NUMERIC({}, {})", precision, scale), &[&number]) {
                    Ok(x) => x.first().unwrap().get(0),
                    Err(err) => panic!("{:#?}", err),
                };
            assert_eq!(expected, result.to_string(), "NUMERIC({}, {})", precision, scale);
        }
    }

    #[tokio::test]
    #[cfg(feature = "tokio-pg")]
    async fn async_write_numeric_type() {
        use futures::future::FutureExt;
        use tokio_postgres::connect;

        let (client, connection) = connect(&get_postgres_url(), NoTls).await.unwrap();
        let connection = connection.map(|e| e.unwrap());
        tokio::spawn(connection);

        for &(precision, scale, sent, expected) in TEST_DECIMALS.iter() {
            let statement = client
                .prepare(&format!("SELECT $1::NUMERIC({}, {})", precision, scale))
                .await
                .unwrap();
            let number = Decimal::from_str(sent).unwrap();
            let rows = client.query(&statement, &[&number]).await.unwrap();
            let result: Decimal = rows.first().unwrap().get(0);

            assert_eq!(expected, result.to_string(), "NUMERIC({}, {})", precision, scale);
        }
    }

    #[test]
    fn numeric_overflow() {
        let tests = [(4, 4, "3950.1234")];
        let mut client = match Client::connect(&get_postgres_url(), NoTls) {
            Ok(x) => x,
            Err(err) => panic!("{:#?}", err),
        };
        for &(precision, scale, sent) in tests.iter() {
            match client.query(&*format!("SELECT {}::NUMERIC({}, {})", sent, precision, scale), &[]) {
                Ok(_) => panic!(
                    "Expected numeric overflow for {}::NUMERIC({}, {})",
                    sent, precision, scale
                ),
                Err(err) => {
                    assert_eq!("22003", err.code().unwrap().code(), "Unexpected error code");
                }
            };
        }
    }

    #[tokio::test]
    #[cfg(feature = "tokio-pg")]
    async fn async_numeric_overflow() {
        use futures::future::FutureExt;
        use tokio_postgres::connect;

        let tests = [(4, 4, "3950.1234")];
        let (client, connection) = connect(&get_postgres_url(), NoTls).await.unwrap();
        let connection = connection.map(|e| e.unwrap());
        tokio::spawn(connection);

        for &(precision, scale, sent) in tests.iter() {
            let statement = client
                .prepare(&format!("SELECT {}::NUMERIC({}, {})", sent, precision, scale))
                .await
                .unwrap();

            match client.query(&statement, &[]).await {
                Ok(_) => panic!(
                    "Expected numeric overflow for {}::NUMERIC({}, {})",
                    sent, precision, scale
                ),
                Err(err) => assert_eq!("22003", err.code().unwrap().code(), "Unexpected error code"),
            }
        }
    }

    #[test]
    fn numeric_overflow_from_sql() {
        let close_to_overflow = Decimal::from_sql(
            &Type::NUMERIC,
            &[0x00, 0x01, 0x00, 0x07, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01],
        );
        assert!(close_to_overflow.is_ok());
        assert_eq!(close_to_overflow.unwrap().to_string(), "10000000000000000000000000000");
        let overflow = Decimal::from_sql(
            &Type::NUMERIC,
            &[0x00, 0x01, 0x00, 0x07, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0a],
        );
        assert!(overflow.is_err());
        assert_eq!(
            overflow.unwrap_err().to_string(),
            crate::error::Error::ExceedsMaximumPossibleValue.to_string()
        );
    }
}

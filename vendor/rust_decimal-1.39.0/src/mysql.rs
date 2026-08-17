use crate::Decimal;
use diesel::{
    deserialize::{self, FromSql},
    mysql::Mysql,
    serialize::{self, IsNull, Output, ToSql},
    sql_types::Numeric,
};
use std::io::Write;
use std::str::FromStr;

#[cfg(feature = "diesel")]
impl ToSql<Numeric, Mysql> for Decimal {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Mysql>) -> serialize::Result {
        write!(out, "{}", *self).map(|_| IsNull::No).map_err(|e| e.into())
    }
}

#[cfg(feature = "diesel")]
impl FromSql<Numeric, Mysql> for Decimal {
    fn from_sql(numeric: diesel::mysql::MysqlValue) -> deserialize::Result<Self> {
        // From what I can ascertain, MySQL simply reads from a string format for the Decimal type.
        // Explicitly, it looks like it is length followed by the string. Regardless, we can leverage
        // internal types.
        let s = std::str::from_utf8(numeric.as_bytes())?;
        Decimal::from_str(s).map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Test {
        value: Decimal,
    }

    struct NullableTest {
        value: Option<Decimal>,
    }

    pub static TEST_DECIMALS: &[(u32, u32, &str, &str)] = &[
        // precision, scale, sent, expected
        (1, 0, "1", "1"),
        (6, 2, "1", "1.00"),
        (6, 2, "9999.99", "9999.99"),
        (35, 6, "3950.123456", "3950.123456"),
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
    ];

    /// Gets the URL for connecting to MySQL for testing. Set the MYSQL_URL
    /// environment variable to change from the default of "mysql://root@localhost/mysql".
    fn get_mysql_url() -> String {
        if let Ok(url) = std::env::var("MYSQL_URL") {
            return url;
        }
        "mysql://root@127.0.0.1/mysql".to_string()
    }

    #[cfg(feature = "diesel")]
    mod diesel_tests {
        use super::*;
        use diesel::deserialize::QueryableByName;
        use diesel::prelude::*;
        use diesel::row::NamedRow;
        use diesel::sql_query;
        use diesel::sql_types::Text;

        impl QueryableByName<Mysql> for Test {
            fn build<'a>(row: &impl NamedRow<'a, Mysql>) -> deserialize::Result<Self> {
                let value = NamedRow::get(row, "value")?;
                Ok(Test { value })
            }
        }

        impl QueryableByName<Mysql> for NullableTest {
            fn build<'a>(row: &impl NamedRow<'a, Mysql>) -> deserialize::Result<Self> {
                let value = NamedRow::get(row, "value")?;
                Ok(NullableTest { value })
            }
        }

        #[test]
        fn test_null() {
            let mut connection = diesel::MysqlConnection::establish(&get_mysql_url()).expect("Establish connection");

            // Test NULL
            let items: Vec<NullableTest> = sql_query("SELECT CAST(NULL AS DECIMAL) AS value")
                .load(&mut connection)
                .expect("Unable to query value");
            let result = items.first().unwrap().value;
            assert_eq!(None, result);
        }

        #[test]
        fn read_numeric_type() {
            let mut connection = diesel::MysqlConnection::establish(&get_mysql_url()).expect("Establish connection");
            for &(precision, scale, sent, expected) in TEST_DECIMALS.iter() {
                let items: Vec<Test> = sql_query(format!(
                    "SELECT CAST('{}' AS DECIMAL({}, {})) AS value",
                    sent, precision, scale
                ))
                .load(&mut connection)
                .expect("Unable to query value");
                assert_eq!(
                    expected,
                    items.first().unwrap().value.to_string(),
                    "DECIMAL({}, {}) sent: {}",
                    precision,
                    scale,
                    sent
                );
            }
        }

        #[test]
        fn write_numeric_type() {
            let mut connection = diesel::MysqlConnection::establish(&get_mysql_url()).expect("Establish connection");
            for &(precision, scale, sent, expected) in TEST_DECIMALS.iter() {
                let items: Vec<Test> =
                    sql_query(format!("SELECT CAST(? AS DECIMAL({}, {})) AS value", precision, scale))
                        .bind::<Text, _>(sent)
                        .load(&mut connection)
                        .expect("Unable to query value");
                assert_eq!(
                    expected,
                    items.first().unwrap().value.to_string(),
                    "DECIMAL({}, {}) sent: {}",
                    precision,
                    scale,
                    sent
                );
            }
        }
    }
}

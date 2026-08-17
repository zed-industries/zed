extern crate time_ as time;

use std::net::SocketAddr;
use std::ops::Bound;
use std::str::FromStr;

use sqlx::postgres::types::{Oid, PgCiText, PgInterval, PgMoney, PgRange};
use sqlx::postgres::Postgres;
use sqlx_test::{new, test_decode_type, test_prepared_type, test_type};

use sqlx_core::executor::Executor;
use sqlx_core::types::Text;

test_type!(null<Option<i16>>(Postgres,
    "NULL::int2" == None::<i16>
));

test_type!(null_vec<Vec<Option<i16>>>(Postgres,
    "array[10,NULL,50]::int2[]" == vec![Some(10_i16), None, Some(50)],
));

test_type!(null_array<[Option<i16>; 3]>(Postgres,
    "array[10,NULL,50]::int2[]" == vec![Some(10_i16), None, Some(50)],
));

test_type!(bool<bool>(Postgres,
    "false::boolean" == false,
    "true::boolean" == true
));

test_type!(bool_vec<Vec<bool>>(Postgres,
    "array[true,false,true]::bool[]" == vec![true, false, true],
));

test_type!(bool_array<[bool; 3]>(Postgres,
    "array[true,false,true]::bool[]" == vec![true, false, true],
));

test_type!(byte_vec<Vec<u8>>(Postgres,
    "E'\\\\xDEADBEEF'::bytea"
        == vec![0xDE_u8, 0xAD, 0xBE, 0xEF],
    "E'\\\\x'::bytea"
        == Vec::<u8>::new(),
    "E'\\\\x0000000052'::bytea"
        == vec![0_u8, 0, 0, 0, 0x52]
));

// BYTEA cannot be decoded by-reference from a simple query as postgres sends it as hex
test_prepared_type!(byte_slice<&[u8]>(Postgres,
    "E'\\\\xDEADBEEF'::bytea"
        == &[0xDE_u8, 0xAD, 0xBE, 0xEF][..],
    "E'\\\\x0000000052'::bytea"
        == &[0_u8, 0, 0, 0, 0x52][..]
));

test_type!(byte_array_empty<[u8; 0]>(Postgres,
    "E'\\\\x'::bytea" == [0_u8; 0],
));

test_type!(byte_array<[u8; 4]>(Postgres,
    "E'\\\\xDEADBEEF'::bytea" == [0xDE_u8, 0xAD, 0xBE, 0xEF],
));

test_type!(str<&str>(Postgres,
    "'this is foo'" == "this is foo",
    "''" == "",
    "'identifier'::name" == "identifier",
    "'five'::char(4)" == "five",
    "'more text'::varchar" == "more text",
    "'case insensitive searching'::citext" == "case insensitive searching",
));

test_type!(string<String>(Postgres,
    "'this is foo'" == format!("this is foo"),
));

test_type!(string_vec<Vec<String>>(Postgres,
    "array['one','two','three']::text[]"
        == vec!["one","two","three"],

    "array['', '\"']::text[]"
        == vec!["", "\""],

    "array['Hello, World', '', 'Goodbye']::text[]"
        == vec!["Hello, World", "", "Goodbye"],
));

test_type!(string_array<[String; 3]>(Postgres,
    "array['one','two','three']::text[]" == ["one","two","three"],
));

test_type!(i8(
    Postgres,
    "0::\"char\"" == 0_i8,
    "120::\"char\"" == 120_i8,
));

test_type!(Oid(Postgres, "325235::oid" == Oid(325235),));

test_type!(i16(
    Postgres,
    "-2144::smallint" == -2144_i16,
    "821::smallint" == 821_i16,
));

test_type!(i32(
    Postgres,
    "94101::int" == 94101_i32,
    "-5101::int" == -5101_i32
));

test_type!(i32_vec<Vec<i32>>(Postgres,
    "'{5,10,50,100}'::int[]" == vec![5_i32, 10, 50, 100],
    "'{1050}'::int[]" == vec![1050_i32],
    "'{}'::int[]" == Vec::<i32>::new(),
    "'{1,3,-5}'::int[]" == vec![1_i32, 3, -5]
));

test_type!(i32_array_empty<[i32; 0]>(Postgres,
    "'{}'::int[]" == [0_i32; 0],
));

test_type!(i32_array<[i32; 4]>(Postgres,
    "'{5,10,50,100}'::int[]" == [5_i32, 10, 50, 100],
));

test_type!(i64(Postgres, "9358295312::bigint" == 9358295312_i64));

test_type!(f32(Postgres, "9419.122::real" == 9419.122_f32));

test_type!(f64(
    Postgres,
    "939399419.1225182::double precision" == 939399419.1225182_f64
));

test_type!(f64_vec<Vec<f64>>(Postgres,
    "'{939399419.1225182,-12.0}'::float8[]" == vec![939399419.1225182_f64, -12.0]
));

test_decode_type!(bool_tuple<(bool,)>(Postgres, "row(true)" == (true,)));

test_decode_type!(num_tuple<(i32, i64, f64,)>(Postgres, "row(10,515::int8,3.124::float8)" == (10,515,3.124)));

test_decode_type!(empty_tuple<()>(Postgres, "row()" == ()));

test_decode_type!(string_tuple<(String, String, String)>(Postgres,
    "row('one','two','three')"
        == ("one".to_string(), "two".to_string(), "three".to_string()),

    "row('', '\"', '\"\"\"\"\"\"')"
        == ("".to_string(), "\"".to_string(), "\"\"\"\"\"\"".to_string()),

    "row('Hello, World', '', 'Goodbye')"
        == ("Hello, World".to_string(), "".to_string(), "Goodbye".to_string())
));

#[cfg(feature = "uuid")]
test_type!(uuid<sqlx::types::Uuid>(Postgres,
    "'b731678f-636f-4135-bc6f-19440c13bd19'::uuid"
        == sqlx::types::Uuid::parse_str("b731678f-636f-4135-bc6f-19440c13bd19").unwrap(),
    "'00000000-0000-0000-0000-000000000000'::uuid"
        == sqlx::types::Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap()
));

#[cfg(feature = "uuid")]
test_type!(uuid_vec<Vec<sqlx::types::Uuid>>(Postgres,
    "'{b731678f-636f-4135-bc6f-19440c13bd19,00000000-0000-0000-0000-000000000000}'::uuid[]"
        == vec![
           sqlx::types::Uuid::parse_str("b731678f-636f-4135-bc6f-19440c13bd19").unwrap(),
           sqlx::types::Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap()
        ]
));

#[cfg(feature = "ipnet")]
test_type!(ipnet<sqlx::types::ipnet::IpNet>(Postgres,
    "'127.0.0.1'::inet"
        == "127.0.0.1/32"
            .parse::<sqlx::types::ipnet::IpNet>()
            .unwrap(),
    "'8.8.8.8/24'::inet"
        == "8.8.8.8/24"
            .parse::<sqlx::types::ipnet::IpNet>()
            .unwrap(),
    "'10.1.1/24'::inet"
        == "10.1.1.0/24"
            .parse::<sqlx::types::ipnet::IpNet>()
            .unwrap(),
    "'::ffff:1.2.3.0'::inet"
        == "::ffff:1.2.3.0/128"
            .parse::<sqlx::types::ipnet::IpNet>()
            .unwrap(),
    "'2001:4f8:3:ba::/64'::inet"
        == "2001:4f8:3:ba::/64"
            .parse::<sqlx::types::ipnet::IpNet>()
            .unwrap(),
    "'192.168'::cidr"
        == "192.168.0.0/24"
            .parse::<sqlx::types::ipnet::IpNet>()
            .unwrap(),
    "'::ffff:1.2.3.0/120'::cidr"
        == "::ffff:1.2.3.0/120"
            .parse::<sqlx::types::ipnet::IpNet>()
            .unwrap(),
));

#[cfg(feature = "ipnetwork")]
test_type!(ipnetwork<sqlx::types::ipnetwork::IpNetwork>(Postgres,
    "'127.0.0.1'::inet"
        == "127.0.0.1"
            .parse::<sqlx::types::ipnetwork::IpNetwork>()
            .unwrap(),
    "'8.8.8.8/24'::inet"
        == "8.8.8.8/24"
            .parse::<sqlx::types::ipnetwork::IpNetwork>()
            .unwrap(),
    "'::ffff:1.2.3.0'::inet"
        == "::ffff:1.2.3.0"
            .parse::<sqlx::types::ipnetwork::IpNetwork>()
            .unwrap(),
    "'2001:4f8:3:ba::/64'::inet"
        == "2001:4f8:3:ba::/64"
            .parse::<sqlx::types::ipnetwork::IpNetwork>()
            .unwrap(),
    "'192.168'::cidr"
        == "192.168.0.0/24"
            .parse::<sqlx::types::ipnetwork::IpNetwork>()
            .unwrap(),
    "'::ffff:1.2.3.0/120'::cidr"
        == "::ffff:1.2.3.0/120"
            .parse::<sqlx::types::ipnetwork::IpNetwork>()
            .unwrap(),
));

#[cfg(feature = "mac_address")]
test_type!(mac_address<sqlx::types::mac_address::MacAddress>(Postgres,
    "'00:01:02:03:04:05'::macaddr"
        == "00:01:02:03:04:05"
            .parse::<sqlx::types::mac_address::MacAddress>()
            .unwrap()
));

#[cfg(feature = "bit-vec")]
test_type!(bitvec<sqlx::types::BitVec>(
    Postgres,
    // A full byte VARBIT
    "B'01101001'" == sqlx::types::BitVec::from_bytes(&[0b0110_1001]),
    // A VARBIT value missing five bits from a byte
    "B'110'" == {
        let mut bit_vec = sqlx::types::BitVec::with_capacity(4);
        bit_vec.push(true);
        bit_vec.push(true);
        bit_vec.push(false);
        bit_vec
    },
    // A BIT value
    "B'01101'::bit(5)" == {
        let mut bit_vec = sqlx::types::BitVec::with_capacity(5);
        bit_vec.push(false);
        bit_vec.push(true);
        bit_vec.push(true);
        bit_vec.push(false);
        bit_vec.push(true);
        bit_vec
    },
));

#[cfg(feature = "ipnet")]
test_type!(ipnet_vec<Vec<sqlx::types::ipnet::IpNet>>(Postgres,
    "'{127.0.0.1,8.8.8.8/24}'::inet[]"
        == vec![
           "127.0.0.1/32".parse::<sqlx::types::ipnet::IpNet>().unwrap(),
           "8.8.8.8/24".parse::<sqlx::types::ipnet::IpNet>().unwrap()
        ]
));

#[cfg(feature = "ipnetwork")]
test_type!(ipnetwork_vec<Vec<sqlx::types::ipnetwork::IpNetwork>>(Postgres,
    "'{127.0.0.1,8.8.8.8/24}'::inet[]"
        == vec![
           "127.0.0.1".parse::<sqlx::types::ipnetwork::IpNetwork>().unwrap(),
           "8.8.8.8/24".parse::<sqlx::types::ipnetwork::IpNetwork>().unwrap()
        ]
));

#[cfg(feature = "mac_address")]
test_type!(mac_address_vec<Vec<sqlx::types::mac_address::MacAddress>>(Postgres,
    "'{01:02:03:04:05:06,FF:FF:FF:FF:FF:FF}'::macaddr[]"
        == vec![
           "01:02:03:04:05:06".parse::<sqlx::types::mac_address::MacAddress>().unwrap(),
           "FF:FF:FF:FF:FF:FF".parse::<sqlx::types::mac_address::MacAddress>().unwrap()
        ]
));

#[cfg(feature = "chrono")]
mod chrono {
    use super::*;
    use sqlx::types::chrono::{
        DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc,
    };

    type PgTimeTz = sqlx::postgres::types::PgTimeTz<NaiveTime, FixedOffset>;

    test_type!(chrono_date<NaiveDate>(Postgres,
        "DATE '2001-01-05'" == NaiveDate::from_ymd_opt(2001, 1, 5).unwrap(),
        "DATE '2050-11-23'" == NaiveDate::from_ymd_opt(2050, 11, 23).unwrap()
    ));

    test_type!(chrono_time<NaiveTime>(Postgres,
        "TIME '05:10:20.115100'" == NaiveTime::from_hms_micro_opt(5, 10, 20, 115100).unwrap()
    ));

    test_type!(chrono_date_time<NaiveDateTime>(Postgres,
        "'2019-01-02 05:10:20'::timestamp" == NaiveDate::from_ymd_opt(2019, 1, 2).unwrap().and_hms_opt(5, 10, 20).unwrap()
    ));

    test_type!(chrono_date_time_vec<Vec<NaiveDateTime>>(Postgres,
        "array['2019-01-02 05:10:20']::timestamp[]"
            == vec![NaiveDate::from_ymd_opt(2019, 1, 2).unwrap().and_hms_opt(5, 10, 20).unwrap()]
    ));

    test_type!(chrono_date_time_tz_utc<DateTime::<Utc>>(Postgres,
        "TIMESTAMPTZ '2019-01-02 05:10:20.115100'"
            == Utc.from_utc_datetime(
                &NaiveDate::from_ymd_opt(2019, 1, 2).unwrap().and_hms_micro_opt(5, 10, 20, 115100).unwrap(),
            )
    ));

    test_type!(chrono_date_time_tz<DateTime::<FixedOffset>>(Postgres,
        "TIMESTAMPTZ '2019-01-02 05:10:20.115100+06:30'"
            == FixedOffset::east_opt(60 * 60 * 6 + 1800).unwrap().ymd(2019, 1, 2).and_hms_micro_opt(5, 10, 20, 115100).unwrap()
    ));

    test_type!(chrono_date_time_tz_vec<Vec<DateTime::<Utc>>>(Postgres,
        "array['2019-01-02 05:10:20.115100']::timestamptz[]"
            == vec![
                Utc.from_utc_datetime(
                    &NaiveDate::from_ymd_opt(2019, 1, 2).unwrap().and_hms_micro_opt(5, 10, 20, 115100).unwrap(),
                )
            ]
    ));

    test_type!(chrono_time_tz<PgTimeTz>(Postgres,
        "TIMETZ '05:10:20.115100+00'" == PgTimeTz { time: NaiveTime::from_hms_micro_opt(5, 10, 20, 115100).unwrap(), offset: FixedOffset::east_opt(0).unwrap() },
        "TIMETZ '05:10:20.115100+06:30'" == PgTimeTz { time: NaiveTime::from_hms_micro_opt(5, 10, 20, 115100).unwrap(), offset: FixedOffset::east_opt(60 * 60 * 6 + 1800).unwrap() },
        "TIMETZ '05:10:20.115100-05'" == PgTimeTz { time: NaiveTime::from_hms_micro_opt(5, 10, 20, 115100).unwrap(), offset: FixedOffset::west_opt(60 * 60 * 5).unwrap() },
        "TIMETZ '05:10:20+02'" == PgTimeTz { time: NaiveTime::from_hms_opt(5, 10, 20).unwrap(), offset: FixedOffset::east_opt(60 * 60 * 2 ).unwrap() }
    ));
}

#[cfg(feature = "time")]
mod time_tests {
    use super::*;
    use sqlx::types::time::{Date, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset};
    use time::macros::{date, time};

    type PgTimeTz = sqlx::postgres::types::PgTimeTz<Time, UtcOffset>;

    test_type!(time_date<Date>(
        Postgres,
        "DATE '2001-01-05'" == date!(2001 - 1 - 5),
        "DATE '2050-11-23'" == date!(2050 - 11 - 23)
    ));

    test_type!(time_time<Time>(
        Postgres,
        "TIME '05:10:20.115100'" == time!(5:10:20.115100),
        "TIME '05:10:20'" == time!(5:10:20)
    ));

    test_type!(time_date_time<PrimitiveDateTime>(
        Postgres,
        "TIMESTAMP '2019-01-02 05:10:20'" == date!(2019 - 1 - 2).with_time(time!(5:10:20)),
        "TIMESTAMP '2019-01-02 05:10:20.1151'" == date!(2019 - 1 - 2).with_time(time!(5:10:20.115100))
    ));

    test_type!(time_timestamp<OffsetDateTime>(
        Postgres,
        "TIMESTAMPTZ '2019-01-02 05:10:20.115100'"
            == date!(2019 - 1 - 2)
                .with_time(time!(5:10:20.115100))
                .assume_utc()
    ));

    test_prepared_type!(time_time_tz<PgTimeTz>(Postgres,
        "TIMETZ '05:10:20.115100+00'" == PgTimeTz { time: time!(5:10:20.115100), offset: UtcOffset::from_whole_seconds(0).unwrap() },
        "TIMETZ '05:10:20.115100+00'" == PgTimeTz { time: time!(5:10:20.115100), offset: UtcOffset::from_whole_seconds(0).unwrap() },
        "TIMETZ '05:10:20.115100+06:30'" == PgTimeTz { time: time!(5:10:20.115100), offset: UtcOffset::from_whole_seconds(60 * 60 * 6 + 1800).unwrap() },
        "TIMETZ '05:10:20.115100-05'" == PgTimeTz { time: time!(5:10:20.115100), offset: UtcOffset::from_whole_seconds(-(60 * 60 * 5)).unwrap() },
        "TIMETZ '05:10:20+02'" == PgTimeTz { time: time!(5:10:20), offset: UtcOffset::from_whole_seconds(60 * 60 * 2 ).unwrap() }
    ));
}

#[cfg(feature = "json")]
mod json {
    use super::*;
    use serde_json::value::RawValue as JsonRawValue;
    use serde_json::{json, Value as JsonValue};
    use sqlx::postgres::PgRow;
    use sqlx::types::Json;
    use sqlx::{Executor, Row};
    use sqlx_test::new;

    // When testing JSON, coerce to JSONB for `=` comparison as `JSON = JSON` is not
    // supported in PostgreSQL

    test_type!(json<JsonValue>(
        Postgres,
        "SELECT ({0}::jsonb is not distinct from $1::jsonb)::int4, {0} as _2, $2 as _3",
        "'\"Hello, World\"'::json" == json!("Hello, World"),
        "'\"😎\"'::json" == json!("😎"),
        "'\"🙋‍♀️\"'::json" == json!("🙋‍♀️"),
        "'[\"Hello\", \"World!\"]'::json" == json!(["Hello", "World!"])
    ));

    test_type!(json_vec<Vec<JsonValue>>(
        Postgres,
        "SELECT ({0}::jsonb[] is not distinct from $1::jsonb[])::int4, {0} as _2, $2 as _3",
        "array['\"😎\"'::json, '\"🙋‍♀️\"'::json]::json[]" == vec![json!("😎"), json!("🙋‍♀️")],
    ));

    test_type!(json_array<[JsonValue; 2]>(
        Postgres,
        "SELECT ({0}::jsonb[] is not distinct from $1::jsonb[])::int4, {0} as _2, $2 as _3",
        "array['\"😎\"'::json, '\"🙋‍♀️\"'::json]::json[]" == [json!("😎"), json!("🙋‍♀️")],
    ));

    test_type!(jsonb<JsonValue>(
        Postgres,
        "'\"Hello, World\"'::jsonb" == json!("Hello, World"),
        "'\"😎\"'::jsonb" == json!("😎"),
        "'\"🙋‍♀️\"'::jsonb" == json!("🙋‍♀️"),
        "'[\"Hello\", \"World!\"]'::jsonb" == json!(["Hello", "World!"])
    ));

    test_type!(jsonb_array<Vec<JsonValue>>(
        Postgres,
        "array['\"😎\"'::jsonb, '\"🙋‍♀️\"'::jsonb]::jsonb[]" == vec![json!("😎"), json!("🙋‍♀️")],
    ));

    #[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq)]
    struct Friend {
        name: String,
        age: u32,
    }

    test_type!(json_struct<Json<Friend>>(Postgres,
        "'{\"name\":\"Joe\",\"age\":33}'::jsonb" == Json(Friend { name: "Joe".to_string(), age: 33 })
    ));

    test_type!(json_struct_vec<Vec<Json<Friend>>>(Postgres,
        "array['{\"name\":\"Joe\",\"age\":33}','{\"name\":\"Bob\",\"age\":22}']::jsonb[]"
            == vec![
                Json(Friend { name: "Joe".to_string(), age: 33 }),
                Json(Friend { name: "Bob".to_string(), age: 22 }),
            ]
    ));

    #[sqlx_macros::test]
    async fn test_json_raw_value() -> anyhow::Result<()> {
        let mut conn = new::<Postgres>().await?;

        // unprepared, text API
        let row: PgRow = conn
            .fetch_one("SELECT '{\"hello\": \"world\"}'::jsonb")
            .await?;

        let value: &JsonRawValue = row.try_get(0)?;

        assert_eq!(value.get(), "{\"hello\": \"world\"}");

        // prepared, binary API
        let row: PgRow = conn
            .fetch_one(sqlx::query("SELECT '{\"hello\": \"world\"}'::jsonb"))
            .await?;

        let value: &JsonRawValue = row.try_get(0)?;

        assert_eq!(value.get(), "{\"hello\": \"world\"}");

        Ok(())
    }
}

#[cfg(feature = "bigdecimal")]
test_type!(bigdecimal<sqlx::types::BigDecimal>(Postgres,

    // https://github.com/launchbadge/sqlx/issues/283
    "0::numeric" == "0".parse::<sqlx::types::BigDecimal>().unwrap(),

    "1::numeric" == "1".parse::<sqlx::types::BigDecimal>().unwrap(),
    "10000::numeric" == "10000".parse::<sqlx::types::BigDecimal>().unwrap(),
    "0.1::numeric" == "0.1".parse::<sqlx::types::BigDecimal>().unwrap(),
    "0.01::numeric" == "0.01".parse::<sqlx::types::BigDecimal>().unwrap(),
    "0.012::numeric" == "0.012".parse::<sqlx::types::BigDecimal>().unwrap(),
    "0.0123::numeric" == "0.0123".parse::<sqlx::types::BigDecimal>().unwrap(),
    "0.01234::numeric" == "0.01234".parse::<sqlx::types::BigDecimal>().unwrap(),
    "0.012345::numeric" == "0.012345".parse::<sqlx::types::BigDecimal>().unwrap(),
    "0.0123456::numeric" == "0.0123456".parse::<sqlx::types::BigDecimal>().unwrap(),
    "0.01234567::numeric" == "0.01234567".parse::<sqlx::types::BigDecimal>().unwrap(),
    "0.012345678::numeric" == "0.012345678".parse::<sqlx::types::BigDecimal>().unwrap(),
    "0.0123456789::numeric" == "0.0123456789".parse::<sqlx::types::BigDecimal>().unwrap(),
    "0.002::numeric" == "0.002".parse::<sqlx::types::BigDecimal>().unwrap(),
    "0.0002::numeric" == "0.0002".parse::<sqlx::types::BigDecimal>().unwrap(),
    "0.00002::numeric" == "0.00002".parse::<sqlx::types::BigDecimal>().unwrap(),
    "0.000002::numeric" == "0.000002".parse::<sqlx::types::BigDecimal>().unwrap(),
    "0.0000002::numeric" == "0.0000002".parse::<sqlx::types::BigDecimal>().unwrap(),
    "0.00000002::numeric" == "0.00000002".parse::<sqlx::types::BigDecimal>().unwrap(),
    "12.34::numeric" == "12.34".parse::<sqlx::types::BigDecimal>().unwrap(),
    "12345.6789::numeric" == "12345.6789".parse::<sqlx::types::BigDecimal>().unwrap(),
));

#[cfg(feature = "bigdecimal")]
test_type!(numrange_bigdecimal<PgRange<sqlx::types::BigDecimal>>(Postgres,
    "'(1.3,2.4)'::numrange" == PgRange::from(
        (Bound::Excluded("1.3".parse::<sqlx::types::BigDecimal>().unwrap()),
         Bound::Excluded("2.4".parse::<sqlx::types::BigDecimal>().unwrap())))
));

#[cfg(any(postgres_14, postgres_15))]
test_type!(cube<sqlx::postgres::types::PgCube>(Postgres,
    "cube(2)" == sqlx::postgres::types::PgCube::Point(2.),
    "cube(2.1)" == sqlx::postgres::types::PgCube::Point(2.1),
    "cube(2,3)" == sqlx::postgres::types::PgCube::OneDimensionInterval(2., 3.),
    "cube(2.2,-3.4)" == sqlx::postgres::types::PgCube::OneDimensionInterval(2.2, -3.4),
    "cube(array[2,3])" == sqlx::postgres::types::PgCube::ZeroVolume(vec![2., 3.]),
    "cube(array[2,3],array[4,5])" == sqlx::postgres::types::PgCube::MultiDimension(vec![vec![2.,3.],vec![4.,5.]]),
    "cube(array[2,3,4],array[4,5,6])" == sqlx::postgres::types::PgCube::MultiDimension(vec![vec![2.,3.,4.],vec![4.,5.,6.]]),
));

#[cfg(any(postgres_14, postgres_15))]
test_type!(_cube<Vec<sqlx::postgres::types::PgCube>>(Postgres,
    "array[cube(2),cube(2)]" == vec![sqlx::postgres::types::PgCube::Point(2.), sqlx::postgres::types::PgCube::Point(2.)],
    "array[cube(2.2,-3.4)]" == vec![sqlx::postgres::types::PgCube::OneDimensionInterval(2.2, -3.4)],
));

#[cfg(any(postgres_12, postgres_13, postgres_14, postgres_15))]
test_type!(point<sqlx::postgres::types::PgPoint>(Postgres,
    "point(2.2,-3.4)" ~= sqlx::postgres::types::PgPoint { x: 2.2, y:-3.4 },
));

#[cfg(any(postgres_12, postgres_13, postgres_14, postgres_15))]
test_type!(_point<Vec<sqlx::postgres::types::PgPoint>>(Postgres,
    "array[point(2,3),point(2.1,3.4)]" @= vec![sqlx::postgres::types::PgPoint { x:2., y: 3. }, sqlx::postgres::types::PgPoint { x:2.1, y: 3.4 }],
    "array[point(2.2,-3.4)]" @= vec![sqlx::postgres::types::PgPoint { x: 2.2, y: -3.4 }],
));

#[cfg(any(postgres_12, postgres_13, postgres_14, postgres_15))]
test_type!(line<sqlx::postgres::types::PgLine>(Postgres,
    "line('{1.1, -2.2, 3.3}')" == sqlx::postgres::types::PgLine { a: 1.1, b:-2.2, c: 3.3 },
    "line('((0.0, 0.0), (1.0,1.0))')" == sqlx::postgres::types::PgLine { a: 1., b: -1., c: 0. },
));

#[cfg(any(postgres_12, postgres_13, postgres_14, postgres_15))]
test_type!(lseg<sqlx::postgres::types::PgLSeg>(Postgres,
    "lseg('((1.0, 2.0), (3.0,4.0))')" == sqlx::postgres::types::PgLSeg { start_x: 1., start_y: 2., end_x: 3. , end_y: 4.},
));

#[cfg(any(postgres_12, postgres_13, postgres_14, postgres_15))]
test_type!(box<sqlx::postgres::types::PgBox>(Postgres,
    "box('((1.0, 2.0), (3.0,4.0))')" == sqlx::postgres::types::PgBox { upper_right_x: 3., upper_right_y: 4., lower_left_x: 1. , lower_left_y: 2.},
));

#[cfg(any(postgres_12, postgres_13, postgres_14, postgres_15))]
test_type!(_box<Vec<sqlx::postgres::types::PgBox>>(Postgres,
    "array[box('1,2,3,4'),box('((1.1, 2.2), (3.3, 4.4))')]" @= vec![sqlx::postgres::types::PgBox { upper_right_x: 3., upper_right_y: 4., lower_left_x: 1., lower_left_y: 2. }, sqlx::postgres::types::PgBox { upper_right_x: 3.3, upper_right_y: 4.4, lower_left_x: 1.1, lower_left_y: 2.2 }],
));

#[cfg(any(postgres_12, postgres_13, postgres_14, postgres_15))]
test_type!(path<sqlx::postgres::types::PgPath>(Postgres,
    "path('((1.0, 2.0), (3.0,4.0))')" == sqlx::postgres::types::PgPath { closed: true, points: vec![ sqlx::postgres::types::PgPoint { x: 1., y: 2. }, sqlx::postgres::types::PgPoint { x: 3. , y: 4. } ]},
    "path('[(1.0, 2.0), (3.0,4.0)]')" == sqlx::postgres::types::PgPath { closed: false, points: vec![ sqlx::postgres::types::PgPoint { x: 1., y: 2. }, sqlx::postgres::types::PgPoint { x: 3. , y: 4. } ]},
));

#[cfg(any(postgres_12, postgres_13, postgres_14, postgres_15))]
test_type!(polygon<sqlx::postgres::types::PgPolygon>(Postgres,
    "polygon('((-2,-3),(-1,-3),(-1,-1),(1,1),(1,3),(2,3),(2,-3),(1,-3),(1,0),(-1,0),(-1,-2),(-2,-2))')" ~= sqlx::postgres::types::PgPolygon {  points: vec![
            sqlx::postgres::types::PgPoint { x: -2., y: -3. }, sqlx::postgres::types::PgPoint { x: -1., y: -3. }, sqlx::postgres::types::PgPoint { x: -1., y: -1. }, sqlx::postgres::types::PgPoint { x: 1., y: 1. },
            sqlx::postgres::types::PgPoint { x: 1., y: 3. },   sqlx::postgres::types::PgPoint { x: 2., y: 3. },   sqlx::postgres::types::PgPoint { x: 2., y: -3. },  sqlx::postgres::types::PgPoint { x: 1., y: -3. },
            sqlx::postgres::types::PgPoint { x: 1., y: 0. },   sqlx::postgres::types::PgPoint { x: -1., y: 0. },  sqlx::postgres::types::PgPoint { x: -1., y: -2. }, sqlx::postgres::types::PgPoint { x: -2., y: -2. },
    ]},
));

#[cfg(any(postgres_12, postgres_13, postgres_14, postgres_15))]
test_type!(circle<sqlx::postgres::types::PgCircle>(Postgres,
    "circle('<(1.1, -2.2), 3.3>')" ~= sqlx::postgres::types::PgCircle { x: 1.1, y:-2.2, radius: 3.3 },
    "circle('((1.1, -2.2), 3.3)')" ~= sqlx::postgres::types::PgCircle { x: 1.1, y:-2.2, radius: 3.3 },
    "circle('(1.1, -2.2), 3.3')" ~= sqlx::postgres::types::PgCircle { x: 1.1, y:-2.2, radius: 3.3 },
    "circle('1.1, -2.2, 3.3')" ~= sqlx::postgres::types::PgCircle { x: 1.1, y:-2.2, radius: 3.3 },
));

#[cfg(feature = "rust_decimal")]
test_type!(decimal<sqlx::types::Decimal>(Postgres,
    "0::numeric" == sqlx::types::Decimal::from_str("0").unwrap(),
    "1::numeric" == sqlx::types::Decimal::from_str("1").unwrap(),
    "10000::numeric" == sqlx::types::Decimal::from_str("10000").unwrap(),
    "0.1::numeric" == sqlx::types::Decimal::from_str("0.1").unwrap(),
    "0.01234::numeric" == sqlx::types::Decimal::from_str("0.01234").unwrap(),
    "12.34::numeric" == sqlx::types::Decimal::from_str("12.34").unwrap(),
    "12345.6789::numeric" == sqlx::types::Decimal::from_str("12345.6789").unwrap(),
    // https://github.com/launchbadge/sqlx/issues/666#issuecomment-683872154
    "17.905625985174584660842500258::numeric" == sqlx::types::Decimal::from_str("17.905625985174584660842500258").unwrap(),
    "-17.905625985174584660842500258::numeric" == sqlx::types::Decimal::from_str("-17.905625985174584660842500258").unwrap(),
));

#[cfg(feature = "rust_decimal")]
test_type!(numrange_decimal<PgRange<sqlx::types::Decimal>>(Postgres,
    "'(1.3,2.4)'::numrange" == PgRange::from(
        (Bound::Excluded(sqlx::types::Decimal::from_str("1.3").unwrap()),
         Bound::Excluded(sqlx::types::Decimal::from_str("2.4").unwrap()))),
));

const EXC2: Bound<i32> = Bound::Excluded(2);
const EXC3: Bound<i32> = Bound::Excluded(3);
const INC1: Bound<i32> = Bound::Included(1);
const INC2: Bound<i32> = Bound::Included(2);
const UNB: Bound<i32> = Bound::Unbounded;

test_type!(int4range<PgRange<i32>>(Postgres,
    "'(,)'::int4range" == PgRange::from((UNB, UNB)),
    "'(,]'::int4range" == PgRange::from((UNB, UNB)),
    "'(,2)'::int4range" == PgRange::from((UNB, EXC2)),
    "'(,2]'::int4range" == PgRange::from((UNB, EXC3)),
    "'(1,)'::int4range" == PgRange::from((INC2, UNB)),
    "'(1,]'::int4range" == PgRange::from((INC2, UNB)),
    "'(1,2]'::int4range" == PgRange::from((INC2, EXC3)),
    "'[,)'::int4range" == PgRange::from((UNB, UNB)),
    "'[,]'::int4range" == PgRange::from((UNB, UNB)),
    "'[,2)'::int4range" == PgRange::from((UNB, EXC2)),
    "'[,2]'::int4range" == PgRange::from((UNB, EXC3)),
    "'[1,)'::int4range" == PgRange::from((INC1, UNB)),
    "'[1,]'::int4range" == PgRange::from((INC1, UNB)),
    "'[1,2)'::int4range" == PgRange::from((INC1, EXC2)),
    "'[1,2]'::int4range" == PgRange::from((INC1, EXC3)),
));

test_prepared_type!(interval<PgInterval>(
    Postgres,
    "INTERVAL '1h'"
        == PgInterval {
            months: 0,
            days: 0,
            microseconds: 3_600_000_000
        },
    "INTERVAL '-1 hours'"
        == PgInterval {
            months: 0,
            days: 0,
            microseconds: -3_600_000_000
        },
    "INTERVAL '3 months 12 days 1h 15 minutes 10 second '"
        == PgInterval {
            months: 3,
            days: 12,
            microseconds: (3_600 + 15 * 60 + 10) * 1_000_000
        },
    "INTERVAL '03:10:20.116100'"
        == PgInterval {
            months: 0,
            days: 0,
            microseconds: (3 * 3_600 + 10 * 60 + 20) * 1_000_000 + 116100
        },
));

test_prepared_type!(money<PgMoney>(Postgres, "123.45::money" == PgMoney(12345)));

test_prepared_type!(money_vec<Vec<PgMoney>>(Postgres,
    "array[123.45,420.00,666.66]::money[]" == vec![PgMoney(12345), PgMoney(42000), PgMoney(66666)],
));

test_prepared_type!(citext_array<Vec<PgCiText>>(Postgres,
    "array['one','two','three']::citext[]" == vec![
        PgCiText("one".to_string()),
        PgCiText("two".to_string()),
        PgCiText("three".to_string()),
    ],
));

// FIXME: needed to disable `ltree` tests in version that don't have a binary format for it
// but `PgLTree` should just fall back to text format
#[cfg(any(postgres_14, postgres_15))]
test_type!(ltree<sqlx::postgres::types::PgLTree>(Postgres,
    "'Foo.Bar.Baz.Quux'::ltree" == sqlx::postgres::types::PgLTree::from_str("Foo.Bar.Baz.Quux").unwrap(),
    "'Alpha.Beta.Delta.Gamma'::ltree" == sqlx::postgres::types::PgLTree::try_from_iter(["Alpha", "Beta", "Delta", "Gamma"]).unwrap(),
));

// FIXME: needed to disable `ltree` tests in version that don't have a binary format for it
// but `PgLTree` should just fall back to text format
#[cfg(any(postgres_14, postgres_15))]
test_type!(ltree_vec<Vec<sqlx::postgres::types::PgLTree>>(Postgres,
    "array['Foo.Bar.Baz.Quux', 'Alpha.Beta.Delta.Gamma']::ltree[]" ==
        vec![
            sqlx::postgres::types::PgLTree::from_str("Foo.Bar.Baz.Quux").unwrap(),
            sqlx::postgres::types::PgLTree::try_from_iter(["Alpha", "Beta", "Delta", "Gamma"]).unwrap()
        ]
));

#[sqlx_macros::test]
async fn test_text_adapter() -> anyhow::Result<()> {
    #[derive(sqlx::FromRow, Debug, PartialEq, Eq)]
    struct Login {
        user_id: i32,
        socket_addr: Text<SocketAddr>,
        #[cfg(feature = "time")]
        login_at: time::OffsetDateTime,
    }

    let mut conn = new::<Postgres>().await?;

    conn.execute(
        r#"
CREATE TEMPORARY TABLE user_login (
    user_id INT PRIMARY KEY,
    socket_addr TEXT NOT NULL,
    login_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
    "#,
    )
    .await?;

    let user_id = 1234;
    let socket_addr: SocketAddr = "198.51.100.47:31790".parse().unwrap();

    sqlx::query("INSERT INTO user_login (user_id, socket_addr) VALUES ($1, $2)")
        .bind(user_id)
        .bind(Text(socket_addr))
        .execute(&mut conn)
        .await?;

    let last_login: Login =
        sqlx::query_as("SELECT * FROM user_login ORDER BY login_at DESC LIMIT 1")
            .fetch_one(&mut conn)
            .await?;

    assert_eq!(last_login.user_id, user_id);
    assert_eq!(*last_login.socket_addr, socket_addr);

    Ok(())
}

#![allow(clippy::unnecessary_to_owned)]
#![allow(clippy::unnecessary_cast)]

#[test]
fn structured_log() {
    use std::sync::{Arc, Mutex};

    use glib::{gstr, prelude::*, GString, LogField, LogLevel};

    let log = Arc::new(Mutex::new(Vec::new()));
    {
        let log = log.clone();
        // can only be called once per test file
        glib::log_set_writer_func(move |level, fields| {
            let fields = fields
                .iter()
                .map(|f| {
                    let value = if let Some(data) = f.user_data() {
                        assert!(f.value_str().is_none());
                        format!("USERDATA: {data}")
                    } else {
                        f.value_str().unwrap().to_owned()
                    };
                    (f.key().to_owned(), value)
                })
                .collect::<Vec<_>>();
            log.lock().unwrap().push((level, fields));
            glib::LogWriterOutput::Handled
        });
    }
    glib::log_structured!(
        "test",
        LogLevel::Message,
        {
            "MY_META" => "abc";
            "MESSAGE" => "normal with meta";
            "MY_META2" => "def";
        }
    );

    glib::log_structured!(
        None,
        LogLevel::Message,
        {
            "MY_META" => "abc";
            "MESSAGE" => "formatted with meta: {} {}", 123, 456.0;
            "MY_META2" => "def{}", "ghi";
            "EMPTY" => b"";
            GString::from("MY_META3") => b"bstring".to_owned();
        }
    );
    glib::log_structured_array(
        LogLevel::Warning,
        &[
            LogField::new(
                gstr!("MESSAGE_ID"),
                "1e45a69523d3460680e2721d3072408f".as_bytes(),
            ),
            LogField::new(gstr!("PRIORITY"), "4".as_bytes()),
            LogField::new(gstr!("MESSAGE"), "from array".as_bytes()),
            LogField::new_user_data(gstr!("SOMEDATA"), 12345),
        ],
    );
    let dict = glib::VariantDict::new(None);
    dict.insert_value(
        "MESSAGE_ID",
        &"9e093d0fac2f4d50838a649796ab154b".to_variant(),
    );
    dict.insert_value("RELEASE", &"true".to_variant());
    dict.insert_value("MY_BYTES", &"123".as_bytes().to_variant());
    dict.insert_value("MESSAGE", &"from variant".to_variant());
    glib::log_variant(Some("test"), LogLevel::Debug, &dict.end());

    let log = std::mem::take(&mut *log.lock().unwrap());
    let log = log
        .iter()
        .map(|(l, v)| {
            (
                *l,
                v.iter()
                    .map(|(k, v)| (k.as_str(), v.as_str()))
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    let path = if cfg!(windows) {
        "glib\\tests\\structured_log.rs"
    } else {
        "glib/tests/structured_log.rs"
    };

    assert_eq!(
        log[0],
        (
            LogLevel::Message,
            vec![
                ("PRIORITY", "5" as &str),
                ("CODE_FILE", path as &str),
                ("CODE_LINE", "31" as &str),
                ("CODE_FUNC", "structured_log::structured_log" as &str),
                ("MY_META", "abc"),
                ("MESSAGE", "normal with meta"),
                ("MY_META2", "def"),
                ("GLIB_DOMAIN", "test"),
            ]
        ),
    );
    assert_eq!(
        log[1],
        (
            LogLevel::Message,
            vec![
                ("PRIORITY", "5" as &str),
                ("CODE_FILE", path as &str),
                ("CODE_LINE", "41" as &str),
                ("CODE_FUNC", "structured_log::structured_log" as &str),
                ("MY_META", "abc"),
                ("MESSAGE", "formatted with meta: 123 456"),
                ("MY_META2", "defghi"),
                ("EMPTY", ""),
                ("MY_META3", "bstring"),
            ]
        )
    );
    assert_eq!(
        log[2],
        (
            LogLevel::Warning,
            vec![
                ("MESSAGE_ID", "1e45a69523d3460680e2721d3072408f" as &str),
                ("PRIORITY", "4"),
                ("MESSAGE", "from array"),
                ("SOMEDATA", "USERDATA: 12345"),
            ]
        )
    );
    assert_eq!(
        log[3],
        (
            LogLevel::Debug,
            vec![
                ("PRIORITY", "7" as &str),
                ("GLIB_DOMAIN", "test"),
                ("MESSAGE_ID", "9e093d0fac2f4d50838a649796ab154b"),
                ("MY_BYTES", "123"),
                ("RELEASE", "true"),
                ("MESSAGE", "from variant"),
            ]
        )
    );
}

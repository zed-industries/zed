use crate::io::PgBufMutExt;
use crate::io::{BufMutExt, ProtocolEncode};

// To begin a session, a frontend opens a connection to the server and sends a startup message.
// This message includes the names of the user and of the database the user wants to connect to;
// it also identifies the particular protocol version to be used.

// Optionally, the startup message can include additional settings for run-time parameters.

pub struct Startup<'a> {
    /// The database user name to connect as. Required; there is no default.
    pub username: Option<&'a str>,

    /// The database to connect to. Defaults to the user name.
    pub database: Option<&'a str>,

    /// Additional start-up params.
    /// <https://www.postgresql.org/docs/devel/runtime-config-client.html>
    pub params: &'a [(&'a str, &'a str)],
}

// Startup cannot impl FrontendMessage because it doesn't have a format code.
impl ProtocolEncode<'_> for Startup<'_> {
    fn encode_with(&self, buf: &mut Vec<u8>, _context: ()) -> Result<(), crate::Error> {
        buf.reserve(120);

        buf.put_length_prefixed(|buf| {
            // The protocol version number. The most significant 16 bits are the
            // major version number (3 for the protocol described here). The least
            // significant 16 bits are the minor version number (0
            // for the protocol described here)
            buf.extend(&196_608_i32.to_be_bytes());

            if let Some(username) = self.username {
                // The database user name to connect as.
                encode_startup_param(buf, "user", username);
            }

            if let Some(database) = self.database {
                // The database to connect to. Defaults to the user name.
                encode_startup_param(buf, "database", database);
            }

            for (name, value) in self.params {
                encode_startup_param(buf, name, value);
            }

            // A zero byte is required as a terminator
            // after the last name/value pair.
            buf.push(0);

            Ok(())
        })
    }
}

#[inline]
fn encode_startup_param(buf: &mut Vec<u8>, name: &str, value: &str) {
    buf.put_str_nul(name);
    buf.put_str_nul(value);
}

#[test]
fn test_encode_startup() {
    const EXPECTED: &[u8] = b"\0\0\0)\0\x03\0\0user\0postgres\0database\0postgres\0\0";

    let mut buf = Vec::new();
    let m = Startup {
        username: Some("postgres"),
        database: Some("postgres"),
        params: &[],
    };

    m.encode(&mut buf).unwrap();

    assert_eq!(buf, EXPECTED);
}

#[cfg(all(test, not(debug_assertions)))]
#[bench]
fn bench_encode_startup(b: &mut test::Bencher) {
    use test::black_box;

    let mut buf = Vec::with_capacity(128);

    b.iter(|| {
        buf.clear();

        black_box(Startup {
            username: Some("postgres"),
            database: Some("postgres"),
            params: &[],
        })
        .encode(&mut buf);
    });
}

use std::{
    future::Future,
    io,
    sync::{Arc, Mutex},
};

use serde::{de::DeserializeOwned, Deserialize};
use tracing_subscriber::prelude::*;
use tracing_subscriber::{filter::Targets, fmt::MakeWriter};

#[derive(Deserialize, Eq, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub(crate) struct TracingEvent<T> {
    pub(crate) fields: T,
    pub(crate) target: String,
    pub(crate) level: String,
}

/// Run an async closure and capture the tracing output it produces.
pub(crate) async fn capture_tracing<T, F, Fut>(f: F) -> Vec<TracingEvent<T>>
where
    F: Fn() -> Fut,
    Fut: Future,
    T: DeserializeOwned,
{
    let (make_writer, handle) = TestMakeWriter::new();

    let subscriber = tracing_subscriber::registry().with(
        tracing_subscriber::fmt::layer()
            .with_writer(make_writer)
            .with_target(true)
            .without_time()
            .with_ansi(false)
            .json()
            .flatten_event(false)
            .with_filter("axum=trace".parse::<Targets>().unwrap()),
    );

    let guard = tracing::subscriber::set_default(subscriber);

    f().await;

    drop(guard);

    handle
        .take()
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect()
}

struct TestMakeWriter {
    write: Arc<Mutex<Option<Vec<u8>>>>,
}

impl TestMakeWriter {
    fn new() -> (Self, Handle) {
        let write = Arc::new(Mutex::new(Some(Vec::<u8>::new())));

        (
            Self {
                write: write.clone(),
            },
            Handle { write },
        )
    }
}

impl<'a> MakeWriter<'a> for TestMakeWriter {
    type Writer = Writer<'a>;

    fn make_writer(&'a self) -> Self::Writer {
        Writer(self)
    }
}

struct Writer<'a>(&'a TestMakeWriter);

impl<'a> io::Write for Writer<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match &mut *self.0.write.lock().unwrap() {
            Some(vec) => {
                let len = buf.len();
                vec.extend(buf);
                Ok(len)
            }
            None => Err(io::Error::new(
                io::ErrorKind::Other,
                "inner writer has been taken",
            )),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

struct Handle {
    write: Arc<Mutex<Option<Vec<u8>>>>,
}

impl Handle {
    fn take(self) -> String {
        let vec = self.write.lock().unwrap().take().unwrap();
        String::from_utf8(vec).unwrap()
    }
}

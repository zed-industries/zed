//! Benchmarks for write performance.
use criterion::{BatchSize, Criterion};
use std::{
    hint,
    io::{self, Read, Write},
    time::{Duration, Instant},
};
use tungstenite::{Message, WebSocket};

const MOCK_WRITE_LEN: usize = 8 * 1024 * 1024;

/// `Write` impl that simulates slowish writes and slow flushes.
///
/// Each `write` can buffer up to 8 MiB before flushing but takes an additional **~80ns**
/// to simulate stuff going on in the underlying stream.
/// Each `flush` takes **~8µs** to simulate flush io.
struct MockWrite(Vec<u8>);

impl Read for MockWrite {
    fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
        Err(io::Error::new(io::ErrorKind::WouldBlock, "reads not supported"))
    }
}
impl Write for MockWrite {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if self.0.len() + buf.len() > MOCK_WRITE_LEN {
            self.flush()?;
        }
        // simulate io
        spin(Duration::from_nanos(80));
        self.0.extend(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        if !self.0.is_empty() {
            // simulate io
            spin(Duration::from_micros(8));
            self.0.clear();
        }
        Ok(())
    }
}

fn spin(duration: Duration) {
    let a = Instant::now();
    while a.elapsed() < duration {
        hint::spin_loop();
    }
}

fn benchmark(c: &mut Criterion) {
    // Writes 100k small json text messages then flushes
    c.bench_function("write 100k small texts then flush", |b| {
        let mut ws = WebSocket::from_raw_socket(
            MockWrite(Vec::with_capacity(MOCK_WRITE_LEN)),
            tungstenite::protocol::Role::Server,
            None,
        );

        b.iter_batched(
            || (0..100_000).map(|i| Message::Text(format!("{{\"id\":{i}}}"))),
            |batch| {
                for msg in batch {
                    ws.write(msg).unwrap();
                }
                ws.flush().unwrap();
            },
            BatchSize::SmallInput,
        )
    });
}

criterion::criterion_group!(write_benches, benchmark);
criterion::criterion_main!(write_benches);

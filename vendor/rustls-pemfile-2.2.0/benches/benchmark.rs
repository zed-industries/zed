use std::io::BufReader;

use bencher::{benchmark_group, benchmark_main, Bencher};

fn criterion_benchmark(c: &mut Bencher) {
    c.iter(|| {
        let data = include_bytes!("../tests/data/certificate.chain.pem");
        let mut reader = BufReader::new(&data[..]);
        assert_eq!(
            rustls_pemfile::certs(&mut reader)
                .collect::<Result<Vec<_>, _>>()
                .unwrap()
                .len(),
            3
        );
    });
}

benchmark_group!(benches, criterion_benchmark);
benchmark_main!(benches);

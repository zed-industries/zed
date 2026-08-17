// This program does assorted benchmarking of rustls.
//
// Note: we don't use any of the standard 'cargo bench', 'test::Bencher',
// etc. because it's unstable at the time of writing.

use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::Arc;
use std::time::{Duration, Instant};

use rustls::client::Resumption;
use rustls::server::{
    AllowAnyAuthenticatedClient, NoClientAuth, NoServerSessionStorage, ServerSessionMemoryCache,
};
use rustls::RootCertStore;
use rustls::Ticketer;
use rustls::{ClientConfig, ClientConnection};
use rustls::{ConnectionCommon, SideData};
use rustls::{ServerConfig, ServerConnection};

fn duration_nanos(d: Duration) -> f64 {
    (d.as_secs() as f64) + f64::from(d.subsec_nanos()) / 1e9
}

fn _bench<Fsetup, Ftest, S>(count: usize, name: &'static str, f_setup: Fsetup, f_test: Ftest)
where
    Fsetup: Fn() -> S,
    Ftest: Fn(S),
{
    let mut times = Vec::new();

    for _ in 0..count {
        let state = f_setup();
        let start = Instant::now();
        f_test(state);
        times.push(duration_nanos(Instant::now().duration_since(start)));
    }

    println!("{}", name);
    println!("{:?}", times);
}

fn time<F>(mut f: F) -> f64
where
    F: FnMut(),
{
    let start = Instant::now();
    f();
    let end = Instant::now();
    duration_nanos(end.duration_since(start))
}

fn transfer<L, R, LS, RS>(left: &mut L, right: &mut R, expect_data: Option<usize>) -> f64
where
    L: DerefMut + Deref<Target = ConnectionCommon<LS>>,
    R: DerefMut + Deref<Target = ConnectionCommon<RS>>,
    LS: SideData,
    RS: SideData,
{
    let mut tls_buf = [0u8; 262144];
    let mut read_time = 0f64;
    let mut data_left = expect_data;
    let mut data_buf = [0u8; 8192];

    loop {
        let mut sz = 0;

        while left.wants_write() {
            let written = left
                .write_tls(&mut tls_buf[sz..].as_mut())
                .unwrap();
            if written == 0 {
                break;
            }

            sz += written;
        }

        if sz == 0 {
            return read_time;
        }

        let mut offs = 0;
        loop {
            let start = Instant::now();
            match right.read_tls(&mut tls_buf[offs..sz].as_ref()) {
                Ok(read) => {
                    right.process_new_packets().unwrap();
                    offs += read;
                }
                Err(err) => {
                    panic!("error on transfer {}..{}: {}", offs, sz, err);
                }
            }

            if let Some(left) = &mut data_left {
                loop {
                    let sz = match right.reader().read(&mut data_buf) {
                        Ok(sz) => sz,
                        Err(err) if err.kind() == io::ErrorKind::WouldBlock => break,
                        Err(err) => panic!("failed to read data: {}", err),
                    };

                    *left -= sz;
                    if *left == 0 {
                        break;
                    }
                }
            }

            let end = Instant::now();
            read_time += duration_nanos(end.duration_since(start));
            if sz == offs {
                break;
            }
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum ClientAuth {
    No,
    Yes,
}

#[derive(PartialEq, Clone, Copy)]
enum ResumptionParam {
    No,
    SessionID,
    Tickets,
}

impl ResumptionParam {
    fn label(&self) -> &'static str {
        match *self {
            Self::No => "no-resume",
            Self::SessionID => "sessionid",
            Self::Tickets => "tickets",
        }
    }
}

// copied from tests/api.rs
#[derive(PartialEq, Clone, Copy, Debug)]
enum KeyType {
    Rsa,
    Ecdsa,
    Ed25519,
}

struct BenchmarkParam {
    key_type: KeyType,
    ciphersuite: rustls::SupportedCipherSuite,
    version: &'static rustls::SupportedProtocolVersion,
}

impl BenchmarkParam {
    const fn new(
        key_type: KeyType,
        ciphersuite: rustls::SupportedCipherSuite,
        version: &'static rustls::SupportedProtocolVersion,
    ) -> Self {
        Self {
            key_type,
            ciphersuite,
            version,
        }
    }
}

static ALL_BENCHMARKS: &[BenchmarkParam] = &[
    #[cfg(feature = "tls12")]
    BenchmarkParam::new(
        KeyType::Rsa,
        rustls::cipher_suite::TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256,
        &rustls::version::TLS12,
    ),
    #[cfg(feature = "tls12")]
    BenchmarkParam::new(
        KeyType::Ecdsa,
        rustls::cipher_suite::TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256,
        &rustls::version::TLS12,
    ),
    #[cfg(feature = "tls12")]
    BenchmarkParam::new(
        KeyType::Rsa,
        rustls::cipher_suite::TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256,
        &rustls::version::TLS12,
    ),
    #[cfg(feature = "tls12")]
    BenchmarkParam::new(
        KeyType::Rsa,
        rustls::cipher_suite::TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256,
        &rustls::version::TLS12,
    ),
    #[cfg(feature = "tls12")]
    BenchmarkParam::new(
        KeyType::Rsa,
        rustls::cipher_suite::TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384,
        &rustls::version::TLS12,
    ),
    #[cfg(feature = "tls12")]
    BenchmarkParam::new(
        KeyType::Ecdsa,
        rustls::cipher_suite::TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256,
        &rustls::version::TLS12,
    ),
    #[cfg(feature = "tls12")]
    BenchmarkParam::new(
        KeyType::Ecdsa,
        rustls::cipher_suite::TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384,
        &rustls::version::TLS12,
    ),
    BenchmarkParam::new(
        KeyType::Rsa,
        rustls::cipher_suite::TLS13_CHACHA20_POLY1305_SHA256,
        &rustls::version::TLS13,
    ),
    BenchmarkParam::new(
        KeyType::Rsa,
        rustls::cipher_suite::TLS13_AES_256_GCM_SHA384,
        &rustls::version::TLS13,
    ),
    BenchmarkParam::new(
        KeyType::Rsa,
        rustls::cipher_suite::TLS13_AES_128_GCM_SHA256,
        &rustls::version::TLS13,
    ),
    BenchmarkParam::new(
        KeyType::Ecdsa,
        rustls::cipher_suite::TLS13_AES_128_GCM_SHA256,
        &rustls::version::TLS13,
    ),
    BenchmarkParam::new(
        KeyType::Ed25519,
        rustls::cipher_suite::TLS13_AES_128_GCM_SHA256,
        &rustls::version::TLS13,
    ),
];

impl KeyType {
    fn path_for(&self, part: &str) -> String {
        match self {
            Self::Rsa => format!("test-ca/rsa/{}", part),
            Self::Ecdsa => format!("test-ca/ecdsa/{}", part),
            Self::Ed25519 => format!("test-ca/eddsa/{}", part),
        }
    }

    fn get_chain(&self) -> Vec<rustls::Certificate> {
        rustls_pemfile::certs(&mut io::BufReader::new(
            fs::File::open(self.path_for("end.fullchain")).unwrap(),
        ))
        .unwrap()
        .iter()
        .map(|v| rustls::Certificate(v.clone()))
        .collect()
    }

    fn get_key(&self) -> rustls::PrivateKey {
        rustls::PrivateKey(
            rustls_pemfile::pkcs8_private_keys(&mut io::BufReader::new(
                fs::File::open(self.path_for("end.key")).unwrap(),
            ))
            .unwrap()[0]
                .clone(),
        )
    }

    fn get_client_chain(&self) -> Vec<rustls::Certificate> {
        rustls_pemfile::certs(&mut io::BufReader::new(
            fs::File::open(self.path_for("client.fullchain")).unwrap(),
        ))
        .unwrap()
        .iter()
        .map(|v| rustls::Certificate(v.clone()))
        .collect()
    }

    fn get_client_key(&self) -> rustls::PrivateKey {
        rustls::PrivateKey(
            rustls_pemfile::pkcs8_private_keys(&mut io::BufReader::new(
                fs::File::open(self.path_for("client.key")).unwrap(),
            ))
            .unwrap()[0]
                .clone(),
        )
    }
}

fn make_server_config(
    params: &BenchmarkParam,
    client_auth: ClientAuth,
    resume: ResumptionParam,
    max_fragment_size: Option<usize>,
) -> ServerConfig {
    let client_auth = match client_auth {
        ClientAuth::Yes => {
            let roots = params.key_type.get_chain();
            let mut client_auth_roots = RootCertStore::empty();
            for root in roots {
                client_auth_roots.add(&root).unwrap();
            }
            Arc::new(AllowAnyAuthenticatedClient::new(client_auth_roots))
        }
        ClientAuth::No => NoClientAuth::boxed(),
    };

    let mut cfg = ServerConfig::builder()
        .with_safe_default_cipher_suites()
        .with_safe_default_kx_groups()
        .with_protocol_versions(&[params.version])
        .unwrap()
        .with_client_cert_verifier(client_auth)
        .with_single_cert(params.key_type.get_chain(), params.key_type.get_key())
        .expect("bad certs/private key?");

    if resume == ResumptionParam::SessionID {
        cfg.session_storage = ServerSessionMemoryCache::new(128);
    } else if resume == ResumptionParam::Tickets {
        cfg.ticketer = Ticketer::new().unwrap();
    } else {
        cfg.session_storage = Arc::new(NoServerSessionStorage {});
    }

    cfg.max_fragment_size = max_fragment_size;
    cfg
}

fn make_client_config(
    params: &BenchmarkParam,
    clientauth: ClientAuth,
    resume: ResumptionParam,
) -> ClientConfig {
    let mut root_store = RootCertStore::empty();
    let mut rootbuf =
        io::BufReader::new(fs::File::open(params.key_type.path_for("ca.cert")).unwrap());
    root_store.add_parsable_certificates(&rustls_pemfile::certs(&mut rootbuf).unwrap());

    let cfg = ClientConfig::builder()
        .with_cipher_suites(&[params.ciphersuite])
        .with_safe_default_kx_groups()
        .with_protocol_versions(&[params.version])
        .unwrap()
        .with_root_certificates(root_store);

    let mut cfg = if clientauth == ClientAuth::Yes {
        cfg.with_client_auth_cert(
            params.key_type.get_client_chain(),
            params.key_type.get_client_key(),
        )
        .unwrap()
    } else {
        cfg.with_no_client_auth()
    };

    if resume != ResumptionParam::No {
        cfg.resumption = Resumption::in_memory_sessions(128);
    } else {
        cfg.resumption = Resumption::disabled();
    }

    cfg
}

fn apply_work_multiplier(work: u64) -> u64 {
    let mul = match env::var("BENCH_MULTIPLIER") {
        Ok(val) => val
            .parse::<f64>()
            .expect("invalid BENCH_MULTIPLIER value"),
        Err(_) => 1.,
    };

    ((work as f64) * mul).round() as u64
}

fn bench_handshake(params: &BenchmarkParam, clientauth: ClientAuth, resume: ResumptionParam) {
    let client_config = Arc::new(make_client_config(params, clientauth, resume));
    let server_config = Arc::new(make_server_config(params, clientauth, resume, None));

    assert!(params.ciphersuite.version() == params.version);

    let rounds = apply_work_multiplier(if resume == ResumptionParam::No {
        512
    } else {
        4096
    });
    let mut client_time = 0f64;
    let mut server_time = 0f64;

    for _ in 0..rounds {
        let server_name = "localhost".try_into().unwrap();
        let mut client = ClientConnection::new(Arc::clone(&client_config), server_name).unwrap();
        let mut server = ServerConnection::new(Arc::clone(&server_config)).unwrap();

        server_time += time(|| {
            transfer(&mut client, &mut server, None);
        });
        client_time += time(|| {
            transfer(&mut server, &mut client, None);
        });
        server_time += time(|| {
            transfer(&mut client, &mut server, None);
        });
        client_time += time(|| {
            transfer(&mut server, &mut client, None);
        });
    }

    println!(
        "handshakes\t{:?}\t{:?}\t{:?}\tclient\t{}\t{}\t{:.2}\thandshake/s",
        params.version,
        params.key_type,
        params.ciphersuite.suite(),
        if clientauth == ClientAuth::Yes {
            "mutual"
        } else {
            "server-auth"
        },
        resume.label(),
        (rounds as f64) / client_time
    );
    println!(
        "handshakes\t{:?}\t{:?}\t{:?}\tserver\t{}\t{}\t{:.2}\thandshake/s",
        params.version,
        params.key_type,
        params.ciphersuite.suite(),
        if clientauth == ClientAuth::Yes {
            "mutual"
        } else {
            "server-auth"
        },
        resume.label(),
        (rounds as f64) / server_time
    );
}

fn do_handshake_step(client: &mut ClientConnection, server: &mut ServerConnection) -> bool {
    if server.is_handshaking() || client.is_handshaking() {
        transfer(client, server, None);
        transfer(server, client, None);
        true
    } else {
        false
    }
}

fn do_handshake(client: &mut ClientConnection, server: &mut ServerConnection) {
    while do_handshake_step(client, server) {}
}

fn bench_bulk(params: &BenchmarkParam, plaintext_size: u64, max_fragment_size: Option<usize>) {
    let client_config = Arc::new(make_client_config(
        params,
        ClientAuth::No,
        ResumptionParam::No,
    ));
    let server_config = Arc::new(make_server_config(
        params,
        ClientAuth::No,
        ResumptionParam::No,
        max_fragment_size,
    ));

    let server_name = "localhost".try_into().unwrap();
    let mut client = ClientConnection::new(client_config, server_name).unwrap();
    client.set_buffer_limit(None);
    let mut server = ServerConnection::new(Arc::clone(&server_config)).unwrap();
    server.set_buffer_limit(None);

    do_handshake(&mut client, &mut server);

    let buf = vec![0u8; plaintext_size as usize];

    let total_data = apply_work_multiplier(if plaintext_size < 8192 {
        64 * 1024 * 1024
    } else {
        1024 * 1024 * 1024
    });
    let rounds = total_data / plaintext_size;
    let mut time_send = 0f64;
    let mut time_recv = 0f64;

    for _ in 0..rounds {
        time_send += time(|| {
            server.writer().write_all(&buf).unwrap();
        });

        time_recv += transfer(&mut server, &mut client, Some(buf.len()));
    }

    let mfs_str = format!(
        "max_fragment_size:{}",
        max_fragment_size
            .map(|v| v.to_string())
            .unwrap_or_else(|| "default".to_string())
    );
    let total_mbs = ((plaintext_size * rounds) as f64) / (1024. * 1024.);
    println!(
        "bulk\t{:?}\t{:?}\t{}\tsend\t{:.2}\tMB/s",
        params.version,
        params.ciphersuite.suite(),
        mfs_str,
        total_mbs / time_send
    );
    println!(
        "bulk\t{:?}\t{:?}\t{}\trecv\t{:.2}\tMB/s",
        params.version,
        params.ciphersuite.suite(),
        mfs_str,
        total_mbs / time_recv
    );
}

fn bench_memory(params: &BenchmarkParam, conn_count: u64) {
    let client_config = Arc::new(make_client_config(
        params,
        ClientAuth::No,
        ResumptionParam::No,
    ));
    let server_config = Arc::new(make_server_config(
        params,
        ClientAuth::No,
        ResumptionParam::No,
        None,
    ));

    // The target here is to end up with conn_count post-handshake
    // server and client sessions.
    let conn_count = (conn_count / 2) as usize;
    let mut servers = Vec::with_capacity(conn_count);
    let mut clients = Vec::with_capacity(conn_count);

    for _i in 0..conn_count {
        servers.push(ServerConnection::new(Arc::clone(&server_config)).unwrap());
        let server_name = "localhost".try_into().unwrap();
        clients.push(ClientConnection::new(Arc::clone(&client_config), server_name).unwrap());
    }

    for _step in 0..5 {
        for (client, server) in clients
            .iter_mut()
            .zip(servers.iter_mut())
        {
            do_handshake_step(client, server);
        }
    }

    for client in clients.iter_mut() {
        client
            .writer()
            .write_all(&[0u8; 1024])
            .unwrap();
    }

    for (client, server) in clients
        .iter_mut()
        .zip(servers.iter_mut())
    {
        transfer(client, server, Some(1024));
    }
}

fn lookup_matching_benches(name: &str) -> Vec<&BenchmarkParam> {
    let r: Vec<&BenchmarkParam> = ALL_BENCHMARKS
        .iter()
        .filter(|params| {
            format!("{:?}", params.ciphersuite.suite()).to_lowercase() == name.to_lowercase()
        })
        .collect();

    if r.is_empty() {
        panic!("unknown suite {:?}", name);
    }

    r
}

fn selected_tests(mut args: env::Args) {
    let mode = args
        .next()
        .expect("first argument must be mode");

    match mode.as_ref() {
        "bulk" => match args.next() {
            Some(suite) => {
                let len = args
                    .next()
                    .map(|arg| {
                        arg.parse::<u64>()
                            .expect("3rd arg must be plaintext size integer")
                    })
                    .unwrap_or(1048576);
                let mfs = args.next().map(|arg| {
                    arg.parse::<usize>()
                        .expect("4th arg must be max_fragment_size integer")
                });
                for param in lookup_matching_benches(&suite).iter() {
                    bench_bulk(param, len, mfs);
                }
            }
            None => {
                panic!("bulk needs ciphersuite argument");
            }
        },

        "handshake" | "handshake-resume" | "handshake-ticket" => match args.next() {
            Some(suite) => {
                let resume = if mode == "handshake" {
                    ResumptionParam::No
                } else if mode == "handshake-resume" {
                    ResumptionParam::SessionID
                } else {
                    ResumptionParam::Tickets
                };

                for param in lookup_matching_benches(&suite).iter() {
                    bench_handshake(param, ClientAuth::No, resume);
                }
            }
            None => {
                panic!("handshake* needs ciphersuite argument");
            }
        },

        "memory" => match args.next() {
            Some(suite) => {
                let count = args
                    .next()
                    .map(|arg| {
                        arg.parse::<u64>()
                            .expect("3rd arg must be connection count integer")
                    })
                    .unwrap_or(1000000);
                for param in lookup_matching_benches(&suite).iter() {
                    bench_memory(param, count);
                }
            }
            None => {
                panic!("memory needs ciphersuite argument");
            }
        },

        _ => {
            panic!("unsupported mode {:?}", mode);
        }
    }
}

fn all_tests() {
    for test in ALL_BENCHMARKS.iter() {
        bench_bulk(test, 1024 * 1024, None);
        bench_bulk(test, 1024 * 1024, Some(10000));
        bench_handshake(test, ClientAuth::No, ResumptionParam::No);
        bench_handshake(test, ClientAuth::Yes, ResumptionParam::No);
        bench_handshake(test, ClientAuth::No, ResumptionParam::SessionID);
        bench_handshake(test, ClientAuth::Yes, ResumptionParam::SessionID);
        bench_handshake(test, ClientAuth::No, ResumptionParam::Tickets);
        bench_handshake(test, ClientAuth::Yes, ResumptionParam::Tickets);
    }
}

fn main() {
    let mut args = env::args();
    if args.len() > 1 {
        args.next();
        selected_tests(args);
    } else {
        all_tests();
    }
}

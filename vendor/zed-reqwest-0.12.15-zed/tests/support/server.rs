#![cfg(not(target_arch = "wasm32"))]
use std::convert::Infallible;
use std::future::Future;
use std::net;
use std::sync::mpsc as std_mpsc;
use std::thread;
use std::time::Duration;

use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::runtime;
use tokio::sync::oneshot;

pub struct Server {
    addr: net::SocketAddr,
    panic_rx: std_mpsc::Receiver<()>,
    events_rx: std_mpsc::Receiver<Event>,
    shutdown_tx: Option<oneshot::Sender<()>>,
}

#[non_exhaustive]
pub enum Event {
    ConnectionClosed,
}

impl Server {
    pub fn addr(&self) -> net::SocketAddr {
        self.addr
    }

    pub fn events(&mut self) -> Vec<Event> {
        let mut events = Vec::new();
        while let Ok(event) = self.events_rx.try_recv() {
            events.push(event);
        }
        events
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }

        if !::std::thread::panicking() {
            self.panic_rx
                .recv_timeout(Duration::from_secs(3))
                .expect("test server should not panic");
        }
    }
}

pub fn http<F, Fut>(func: F) -> Server
where
    F: Fn(http::Request<hyper::body::Incoming>) -> Fut + Clone + Send + 'static,
    Fut: Future<Output = http::Response<reqwest::Body>> + Send + 'static,
{
    http_with_config(func, |_builder| {})
}

type Builder = hyper_util::server::conn::auto::Builder<hyper_util::rt::TokioExecutor>;

pub fn http_with_config<F1, Fut, F2, Bu>(func: F1, apply_config: F2) -> Server
where
    F1: Fn(http::Request<hyper::body::Incoming>) -> Fut + Clone + Send + 'static,
    Fut: Future<Output = http::Response<reqwest::Body>> + Send + 'static,
    F2: FnOnce(&mut Builder) -> Bu + Send + 'static,
{
    // Spawn new runtime in thread to prevent reactor execution context conflict
    let test_name = thread::current().name().unwrap_or("<unknown>").to_string();
    thread::spawn(move || {
        let rt = runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("new rt");
        let listener = rt.block_on(async move {
            tokio::net::TcpListener::bind(&std::net::SocketAddr::from(([127, 0, 0, 1], 0)))
                .await
                .unwrap()
        });
        let addr = listener.local_addr().unwrap();

        let (shutdown_tx, mut shutdown_rx) = oneshot::channel();
        let (panic_tx, panic_rx) = std_mpsc::channel();
        let (events_tx, events_rx) = std_mpsc::channel();
        let tname = format!(
            "test({})-support-server",
            test_name,
        );
        thread::Builder::new()
            .name(tname)
            .spawn(move || {
                rt.block_on(async move {
                    let mut builder =
                        hyper_util::server::conn::auto::Builder::new(hyper_util::rt::TokioExecutor::new());
                    apply_config(&mut builder);

                    loop {
                        tokio::select! {
                            _ = &mut shutdown_rx => {
                                break;
                            }
                            accepted = listener.accept() => {
                                let (io, _) = accepted.expect("accepted");
                                let func = func.clone();
                                let svc = hyper::service::service_fn(move |req| {
                                    let fut = func(req);
                                    async move { Ok::<_, Infallible>(fut.await) }
                                });
                                let builder = builder.clone();
                                let events_tx = events_tx.clone();
                                tokio::spawn(async move {
                                    let _ = builder.serve_connection_with_upgrades(hyper_util::rt::TokioIo::new(io), svc).await;
                                    let _ = events_tx.send(Event::ConnectionClosed);
                                });
                            }
                        }
                    }
                    let _ = panic_tx.send(());
                });
            })
            .expect("thread spawn");
        Server {
            addr,
            panic_rx,
            events_rx,
            shutdown_tx: Some(shutdown_tx),
        }
    })
    .join()
    .unwrap()
}

#[cfg(feature = "http3")]
#[derive(Debug, Default)]
pub struct Http3 {
    addr: Option<std::net::SocketAddr>,
}

#[cfg(feature = "http3")]
impl Http3 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_addr(mut self, addr: std::net::SocketAddr) -> Self {
        self.addr = Some(addr);
        self
    }

    pub fn build<F1, Fut>(self, func: F1) -> Server
    where
        F1: Fn(http::Request<http_body_util::combinators::BoxBody<bytes::Bytes, h3::Error>>) -> Fut
            + Clone
            + Send
            + 'static,
        Fut: Future<Output = http::Response<reqwest::Body>> + Send + 'static,
    {
        use bytes::Buf;
        use http_body_util::BodyExt;
        use quinn::crypto::rustls::QuicServerConfig;
        use std::sync::Arc;

        let addr = self.addr.unwrap_or_else(|| "[::1]:0".parse().unwrap());

        // Spawn new runtime in thread to prevent reactor execution context conflict
        let test_name = thread::current().name().unwrap_or("<unknown>").to_string();
        thread::spawn(move || {
            let rt = runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("new rt");

            let cert = std::fs::read("tests/support/server.cert").unwrap().into();
            let key = std::fs::read("tests/support/server.key").unwrap().try_into().unwrap();

            let mut tls_config = rustls::ServerConfig::builder()
                .with_no_client_auth()
                .with_single_cert(vec![cert], key)
                .unwrap();
            tls_config.max_early_data_size = u32::MAX;
            tls_config.alpn_protocols = vec![b"h3".into()];

            let server_config = quinn::ServerConfig::with_crypto(Arc::new(QuicServerConfig::try_from(tls_config).unwrap()));
            let endpoint = rt.block_on(async move {
                quinn::Endpoint::server(server_config, addr).unwrap()
            });
            let addr = endpoint.local_addr().unwrap();

            let (shutdown_tx, mut shutdown_rx) = oneshot::channel();
            let (panic_tx, panic_rx) = std_mpsc::channel();
            let (events_tx, events_rx) = std_mpsc::channel();
            let tname = format!(
                "test({})-support-server",
                test_name,
            );
            thread::Builder::new()
                .name(tname)
                .spawn(move || {
                    rt.block_on(async move {

                        loop {
                            tokio::select! {
                                _ = &mut shutdown_rx => {
                                    break;
                                }
                                Some(accepted) = endpoint.accept() => {
                                    let conn = accepted.await.expect("accepted");
                                    let mut h3_conn = h3::server::Connection::new(h3_quinn::Connection::new(conn)).await.unwrap();
                                    let events_tx = events_tx.clone();
                                    let func = func.clone();
                                    tokio::spawn(async move {
                                        while let Ok(Some((req, stream))) = h3_conn.accept().await {
                                            let events_tx = events_tx.clone();
                                            let func = func.clone();
                                            tokio::spawn(async move {
                                                let (mut tx, rx) = stream.split();
                                                let body = futures_util::stream::unfold(rx, |mut rx| async move {
                                                    match rx.recv_data().await {
                                                        Ok(Some(mut buf)) => {
                                                            Some((Ok(hyper::body::Frame::data(buf.copy_to_bytes(buf.remaining()))), rx))
                                                        },
                                                        Ok(None) => None,
                                                        Err(err) => {
                                                            Some((Err(err), rx))
                                                        }
                                                    }
                                                });
                                                let body = BodyExt::boxed(http_body_util::StreamBody::new(body));
                                                let resp = func(req.map(move |()| body)).await;
                                                let (parts, mut body) = resp.into_parts();
                                                let resp = http::Response::from_parts(parts, ());
                                                tx.send_response(resp).await.unwrap();

                                                while let Some(Ok(frame)) = body.frame().await {
                                                    if let Ok(data) = frame.into_data() {
                                                        tx.send_data(data).await.unwrap();
                                                    }
                                                }
                                                tx.finish().await.unwrap();
                                                events_tx.send(Event::ConnectionClosed).unwrap();
                                            });
                                        }
                                    });
                                }
                            }
                        }
                        let _ = panic_tx.send(());
                    });
                })
                .expect("thread spawn");
            Server {
                addr,
                panic_rx,
                events_rx,
                shutdown_tx: Some(shutdown_tx),
            }
        })
        .join()
        .unwrap()
    }
}

pub fn low_level_with_response<F>(do_response: F) -> Server
where
    for<'c> F: Fn(&'c [u8], &'c mut TcpStream) -> Box<dyn Future<Output = ()> + Send + 'c>
        + Clone
        + Send
        + 'static,
{
    // Spawn new runtime in thread to prevent reactor execution context conflict
    let test_name = thread::current().name().unwrap_or("<unknown>").to_string();
    thread::spawn(move || {
        let rt = runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("new rt");
        let listener = rt.block_on(async move {
            tokio::net::TcpListener::bind(&std::net::SocketAddr::from(([127, 0, 0, 1], 0)))
                .await
                .unwrap()
        });
        let addr = listener.local_addr().unwrap();

        let (shutdown_tx, mut shutdown_rx) = oneshot::channel();
        let (panic_tx, panic_rx) = std_mpsc::channel();
        let (events_tx, events_rx) = std_mpsc::channel();
        let tname = format!("test({})-support-server", test_name,);
        thread::Builder::new()
            .name(tname)
            .spawn(move || {
                rt.block_on(async move {
                    loop {
                        tokio::select! {
                            _ = &mut shutdown_rx => {
                                break;
                            }
                            accepted = listener.accept() => {
                                let (io, _) = accepted.expect("accepted");
                                let do_response = do_response.clone();
                                let events_tx = events_tx.clone();
                                tokio::spawn(async move {
                                    low_level_server_client(io, do_response).await;
                                    let _ = events_tx.send(Event::ConnectionClosed);
                                });
                            }
                        }
                    }
                    let _ = panic_tx.send(());
                });
            })
            .expect("thread spawn");
        Server {
            addr,
            panic_rx,
            events_rx,
            shutdown_tx: Some(shutdown_tx),
        }
    })
    .join()
    .unwrap()
}

async fn low_level_server_client<F>(mut client_socket: TcpStream, do_response: F)
where
    for<'c> F: Fn(&'c [u8], &'c mut TcpStream) -> Box<dyn Future<Output = ()> + Send + 'c>,
{
    loop {
        let request = low_level_read_http_request(&mut client_socket)
            .await
            .expect("read_http_request failed");
        if request.is_empty() {
            // connection closed by client
            break;
        }

        Box::into_pin(do_response(&request, &mut client_socket)).await;
    }
}

async fn low_level_read_http_request(
    client_socket: &mut TcpStream,
) -> core::result::Result<Vec<u8>, std::io::Error> {
    let mut buf = Vec::new();

    // Read until the delimiter "\r\n\r\n" is found
    loop {
        let mut temp_buffer = [0; 1024];
        let n = client_socket.read(&mut temp_buffer).await?;

        if n == 0 {
            break;
        }

        buf.extend_from_slice(&temp_buffer[..n]);

        if let Some(pos) = buf.windows(4).position(|window| window == b"\r\n\r\n") {
            return Ok(buf.drain(..pos + 4).collect());
        }
    }

    Ok(buf)
}

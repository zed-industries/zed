mod utils {
    use std::collections::VecDeque;
    use std::io::IoSlice;
    use std::pin::Pin;
    use std::task::{Context, Poll};

    use rustls::{
        pki_types::{pem::PemObject, CertificateDer, PrivateKeyDer},
        ClientConfig, RootCertStore, ServerConfig,
    };
    use tokio::io::{self, AsyncRead, AsyncWrite, AsyncWriteExt};

    #[allow(dead_code)]
    pub(crate) fn make_configs() -> (ServerConfig, ClientConfig) {
        // A test root certificate that is the trust anchor for the CHAIN.
        const ROOT: &str = include_str!("certs/root.pem");
        // A server certificate chain that includes both an end-entity server certificate
        // and the intermediate certificate that issued it. The ROOT is configured
        // out-of-band.
        const CHAIN: &str = include_str!("certs/chain.pem");
        // A private key corresponding to the end-entity server certificate in CHAIN.
        const EE_KEY: &str = include_str!("certs/end.key");

        let cert = CertificateDer::pem_slice_iter(CHAIN.as_bytes())
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let key = PrivateKeyDer::from_pem_slice(EE_KEY.as_bytes()).unwrap();
        let sconfig = ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(cert, key)
            .unwrap();

        let mut client_root_cert_store = RootCertStore::empty();
        for root in CertificateDer::pem_slice_iter(ROOT.as_bytes()) {
            client_root_cert_store.add(root.unwrap()).unwrap();
        }

        let cconfig = ClientConfig::builder()
            .with_root_certificates(client_root_cert_store)
            .with_no_client_auth();

        (sconfig, cconfig)
    }

    #[allow(dead_code)]
    pub(crate) async fn write<W: AsyncWrite + Unpin>(
        w: &mut W,
        data: &[u8],
        vectored: bool,
    ) -> io::Result<()> {
        if !vectored {
            return w.write_all(data).await;
        }

        let mut data = data;

        while !data.is_empty() {
            let chunk_size = (data.len() / 4).max(1);
            let vectors = data
                .chunks(chunk_size)
                .map(IoSlice::new)
                .collect::<Vec<_>>();
            let written = w.write_vectored(&vectors).await?;
            data = &data[written..];
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub(crate) const TEST_SERVER_DOMAIN: &str = "foobar.com";

    /// An IO wrapper that never flushes when writing, and always returns pending on first flush.
    ///
    /// This is used to test that rustls always flushes to completion during handshake.
    pub(crate) struct FlushWrapper<S> {
        stream: S,
        buf: VecDeque<Vec<u8>>,
        queued: Vec<u8>,
    }

    impl<S> FlushWrapper<S> {
        #[allow(dead_code)]
        pub(crate) fn new(stream: S) -> Self {
            Self {
                stream,
                buf: VecDeque::new(),
                queued: Vec::new(),
            }
        }
    }

    impl<S: AsyncRead + Unpin> AsyncRead for FlushWrapper<S> {
        fn poll_read(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &mut tokio::io::ReadBuf<'_>,
        ) -> Poll<io::Result<()>> {
            Pin::new(&mut self.get_mut().stream).poll_read(cx, buf)
        }
    }

    impl<S: AsyncWrite + Unpin> FlushWrapper<S> {
        fn poll_flush_inner<F>(
            &mut self,
            cx: &mut Context<'_>,
            flush_inner: F,
        ) -> Poll<Result<(), io::Error>>
        where
            F: FnOnce(Pin<&mut S>, &mut Context<'_>) -> Poll<Result<(), io::Error>>,
        {
            loop {
                let stream = Pin::new(&mut self.stream);
                if !self.queued.is_empty() {
                    // write out the queued data
                    let n = std::task::ready!(stream.poll_write(cx, &self.queued))?;
                    self.queued = self.queued[n..].to_vec();
                } else if let Some(buf) = self.buf.pop_front() {
                    // queue the flush, but don't trigger the write immediately.
                    self.queued = buf;
                    cx.waker().wake_by_ref();
                    return Poll::Pending;
                } else {
                    // nothing more to flush to the inner stream, flush the inner stream instead.
                    return flush_inner(stream, cx);
                }
            }
        }
    }

    impl<S: AsyncWrite + Unpin> AsyncWrite for FlushWrapper<S> {
        fn poll_write(
            self: Pin<&mut Self>,
            _: &mut Context<'_>,
            buf: &[u8],
        ) -> Poll<Result<usize, io::Error>> {
            self.get_mut().buf.push_back(buf.to_vec());
            Poll::Ready(Ok(buf.len()))
        }

        fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
            self.get_mut()
                .poll_flush_inner(cx, |s, cx| s.poll_flush(cx))
        }

        fn poll_shutdown(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
        ) -> Poll<Result<(), io::Error>> {
            self.get_mut()
                .poll_flush_inner(cx, |s, cx| s.poll_shutdown(cx))
        }
    }
}

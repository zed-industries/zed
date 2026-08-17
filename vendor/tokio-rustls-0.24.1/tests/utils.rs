mod utils {
    use std::io::{BufReader, Cursor};
    use std::sync::Arc;

    use rustls::{ClientConfig, OwnedTrustAnchor, PrivateKey, RootCertStore, ServerConfig};
    use rustls_pemfile::{certs, rsa_private_keys};

    #[allow(dead_code)]
    pub fn make_configs() -> (Arc<ServerConfig>, Arc<ClientConfig>) {
        const CERT: &str = include_str!("end.cert");
        const CHAIN: &str = include_str!("end.chain");
        const RSA: &str = include_str!("end.rsa");

        let cert = certs(&mut BufReader::new(Cursor::new(CERT)))
            .unwrap()
            .drain(..)
            .map(rustls::Certificate)
            .collect();
        let mut keys = rsa_private_keys(&mut BufReader::new(Cursor::new(RSA))).unwrap();
        let mut keys = keys.drain(..).map(PrivateKey);
        let sconfig = ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(cert, keys.next().unwrap())
            .unwrap();

        let mut client_root_cert_store = RootCertStore::empty();
        let mut chain = BufReader::new(Cursor::new(CHAIN));
        let certs = certs(&mut chain).unwrap();
        client_root_cert_store.add_server_trust_anchors(certs.iter().map(|cert| {
            let ta = webpki::TrustAnchor::try_from_cert_der(&cert[..]).unwrap();
            OwnedTrustAnchor::from_subject_spki_name_constraints(
                ta.subject,
                ta.spki,
                ta.name_constraints,
            )
        }));

        let cconfig = ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(client_root_cert_store)
            .with_no_client_auth();

        (Arc::new(sconfig), Arc::new(cconfig))
    }
}

use std::io::BufReader;
use std::iter;

#[test]
fn test_rsa_private_keys() {
    let data = include_bytes!("data/zen2.pem");
    let mut reader = BufReader::new(&data[..]);

    assert_eq!(
        rustls_pemfile::rsa_private_keys(&mut reader).unwrap().len(),
        2
    );
}

#[test]
fn test_certs() {
    let data = include_bytes!("data/certificate.chain.pem");
    let mut reader = BufReader::new(&data[..]);

    assert_eq!(rustls_pemfile::certs(&mut reader).unwrap().len(), 3);
}

#[test]
fn test_certs_with_binary() {
    let data = include_bytes!("data/gunk.pem");
    let mut reader = BufReader::new(&data[..]);
    assert_eq!(rustls_pemfile::certs(&mut reader).unwrap().len(), 2);
}

#[test]
fn test_crls() {
    let data = include_bytes!("data/crl.pem");
    let mut reader = BufReader::new(&data[..]);
    assert_eq!(rustls_pemfile::crls(&mut reader).unwrap().len(), 1);
}

#[test]
fn test_pkcs8() {
    let data = include_bytes!("data/zen.pem");
    let mut reader = BufReader::new(&data[..]);

    assert_eq!(
        rustls_pemfile::pkcs8_private_keys(&mut reader)
            .unwrap()
            .len(),
        2
    );
}

#[test]
fn test_sec1() {
    let data = include_bytes!("data/nistp256key.pem");
    let mut reader = BufReader::new(&data[..]);

    let items = rustls_pemfile::read_all(&mut reader).unwrap();
    assert_eq!(items.len(), 1);
    assert!(matches!(items[0], rustls_pemfile::Item::ECKey(_)));
}

#[test]
fn smoketest_iterate() {
    let data = include_bytes!("data/zen2.pem");
    let mut reader = BufReader::new(&data[..]);

    let mut count = 0;

    for item in iter::from_fn(|| rustls_pemfile::read_one(&mut reader).transpose()) {
        println!("item {:?}", item);
        count += 1;
    }

    assert_eq!(count, 16);
}

#[test]
fn test_sec1_vs_pkcs8() {
    {
        let data = include_bytes!("data/nistp256key.pem");
        let mut reader = BufReader::new(&data[..]);

        let items = rustls_pemfile::read_all(&mut reader).unwrap();
        assert!(matches!(items[0], rustls_pemfile::Item::ECKey(_)));
        println!("sec1 {:?}", items);
    }
    {
        let data = include_bytes!("data/nistp256key.pkcs8.pem");
        let mut reader = BufReader::new(&data[..]);

        let items = rustls_pemfile::read_all(&mut reader).unwrap();
        assert!(matches!(items[0], rustls_pemfile::Item::PKCS8Key(_)));
        println!("p8 {:?}", items);
    }
}

#[test]
fn parse_in_order() {
    let data = include_bytes!("data/zen.pem");
    let mut reader = BufReader::new(&data[..]);

    let items = rustls_pemfile::read_all(&mut reader).unwrap();
    assert_eq!(items.len(), 9);
    assert!(matches!(items[0], rustls_pemfile::Item::X509Certificate(_)));
    assert!(matches!(items[1], rustls_pemfile::Item::X509Certificate(_)));
    assert!(matches!(items[2], rustls_pemfile::Item::X509Certificate(_)));
    assert!(matches!(items[3], rustls_pemfile::Item::X509Certificate(_)));
    assert!(matches!(items[4], rustls_pemfile::Item::ECKey(_)));
    assert!(matches!(items[5], rustls_pemfile::Item::PKCS8Key(_)));
    assert!(matches!(items[6], rustls_pemfile::Item::RSAKey(_)));
    assert!(matches!(items[7], rustls_pemfile::Item::PKCS8Key(_)));
    assert!(matches!(items[8], rustls_pemfile::Item::Crl(_)));
}

#[test]
fn different_line_endings() {
    let data = include_bytes!("data/mixed-line-endings.crt");

    // Ensure non-LF line endings are not lost by mistake, causing the test
    // to silently regress.
    let mut contained_unix_ending = false;
    let mut contained_other_ending = false;
    for byte in data.iter().copied() {
        if contained_other_ending && contained_unix_ending {
            break;
        }

        if byte == b'\n' {
            contained_unix_ending = true;
        } else if byte == b'\r' {
            contained_other_ending = true;
        }
    }
    assert!(contained_unix_ending);
    assert!(contained_other_ending);

    let mut reader = BufReader::new(&data[..]);

    let items = rustls_pemfile::read_all(&mut reader).unwrap();

    assert_eq!(items.len(), 4);
    for cert in items {
        assert!(matches!(cert, rustls_pemfile::Item::X509Certificate(_)));
    }
}

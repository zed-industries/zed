use std::io::BufReader;
use std::iter;

#[test]
fn test_rsa_private_keys() {
    let data = include_bytes!("data/zen2.pem");
    let mut reader = BufReader::new(&data[..]);

    assert_eq!(
        rustls_pemfile::rsa_private_keys(&mut reader)
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .len(),
        2
    );
}

#[test]
fn private_key() {
    let data = include_bytes!("data/zen2.pem");
    let mut reader = BufReader::new(&data[..]);
    rustls_pemfile::private_key(&mut reader).unwrap().unwrap();

    let data = include_bytes!("data/certificate.chain.pem");
    let mut reader = BufReader::new(&data[..]);
    assert!(rustls_pemfile::private_key(&mut reader).unwrap().is_none());
}

#[test]
fn public_keys() {
    let data = include_bytes!("data/spki.pem");
    let mut reader = BufReader::new(&data[..]);
    assert_eq!(
        rustls_pemfile::public_keys(&mut reader)
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .len(),
        1
    );

    let data = include_bytes!("data/zen2.pem");
    let mut reader = BufReader::new(&data[..]);
    assert_eq!(
        rustls_pemfile::public_keys(&mut reader)
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .len(),
        2
    );

    let data = include_bytes!("data/certificate.chain.pem");
    let mut reader = BufReader::new(&data[..]);
    assert_eq!(
        rustls_pemfile::public_keys(&mut reader)
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .len(),
        0
    );
}

#[test]
fn test_csr() {
    let data = include_bytes!("data/csr.pem");
    let mut reader = BufReader::new(&data[..]);
    rustls_pemfile::csr(&mut reader).unwrap().unwrap();

    let data = include_bytes!("data/certificate.chain.pem");
    let mut reader = BufReader::new(&data[..]);
    assert!(rustls_pemfile::private_key(&mut reader).unwrap().is_none());
}

#[test]
fn test_certs() {
    let data = include_bytes!("data/certificate.chain.pem");
    let mut reader = BufReader::new(&data[..]);

    assert_eq!(
        rustls_pemfile::certs(&mut reader)
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .len(),
        3
    );
}

#[test]
fn test_certs_with_binary() {
    let data = include_bytes!("data/gunk.pem");
    let mut reader = BufReader::new(&data[..]);
    assert_eq!(
        rustls_pemfile::certs(&mut reader)
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .len(),
        2
    );
}

#[test]
fn test_crls() {
    let data = include_bytes!("data/crl.pem");
    let mut reader = BufReader::new(&data[..]);
    assert_eq!(
        rustls_pemfile::crls(&mut reader)
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .len(),
        1
    );
}

#[test]
fn test_pkcs8() {
    let data = include_bytes!("data/zen.pem");
    let mut reader = BufReader::new(&data[..]);

    assert_eq!(
        rustls_pemfile::pkcs8_private_keys(&mut reader)
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .len(),
        2
    );
}

#[test]
fn test_sec1() {
    let data = include_bytes!("data/nistp256key.pem");
    let mut reader = BufReader::new(&data[..]);

    let items = rustls_pemfile::read_all(&mut reader)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(items.len(), 1);
    assert!(matches!(items[0], rustls_pemfile::Item::Sec1Key(_)));
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

    assert_eq!(count, 18);
}

#[test]
fn test_sec1_vs_pkcs8() {
    {
        let data = include_bytes!("data/nistp256key.pem");
        let mut reader = BufReader::new(&data[..]);

        let items = rustls_pemfile::read_all(&mut reader)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert!(matches!(items[0], rustls_pemfile::Item::Sec1Key(_)));
        println!("sec1 {:?}", items);
    }
    {
        let data = include_bytes!("data/nistp256key.pkcs8.pem");
        let mut reader = BufReader::new(&data[..]);

        let items = rustls_pemfile::read_all(&mut reader)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert!(matches!(items[0], rustls_pemfile::Item::Pkcs8Key(_)));
        println!("p8 {:?}", items);
    }
}

#[test]
fn parse_in_order() {
    let data = include_bytes!("data/zen.pem");
    let mut reader = BufReader::new(&data[..]);

    let items = rustls_pemfile::read_all(&mut reader)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(items.len(), 10);
    assert!(matches!(items[0], rustls_pemfile::Item::X509Certificate(_)));
    assert!(matches!(items[1], rustls_pemfile::Item::X509Certificate(_)));
    assert!(matches!(items[2], rustls_pemfile::Item::X509Certificate(_)));
    assert!(matches!(items[3], rustls_pemfile::Item::X509Certificate(_)));
    assert!(matches!(items[4], rustls_pemfile::Item::Sec1Key(_)));
    assert!(matches!(items[5], rustls_pemfile::Item::Pkcs8Key(_)));
    assert!(matches!(items[6], rustls_pemfile::Item::Pkcs1Key(_)));
    assert!(matches!(items[7], rustls_pemfile::Item::Pkcs8Key(_)));
    assert!(matches!(items[8], rustls_pemfile::Item::Crl(_)));
    assert!(matches!(items[9], rustls_pemfile::Item::Csr(_)));
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

    let items = rustls_pemfile::read_all(&mut reader)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    assert_eq!(items.len(), 4);
    for cert in items {
        assert!(matches!(cert, rustls_pemfile::Item::X509Certificate(_)));
    }
}

#[test]
fn whitespace_prefix() {
    let items = rustls_pemfile::read_all(&mut BufReader::new(
        &include_bytes!("data/whitespace-prefix.crt")[..],
    ))
    .collect::<Result<Vec<_>, _>>()
    .unwrap();

    assert_eq!(items.len(), 1);
    assert!(matches!(items[0], rustls_pemfile::Item::X509Certificate(_)));
}

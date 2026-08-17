//! PEM decoding tests

#[test]
fn pkcs1_example() {
    let pem = include_bytes!("examples/pkcs1.pem");
    let mut buf = [0u8; 2048];
    let (label, decoded) = pem_rfc7468::decode(pem, &mut buf).unwrap();
    assert_eq!(label, "RSA PRIVATE KEY");
    assert_eq!(decoded, include_bytes!("examples/pkcs1.der"));
}

#[test]
fn binary_example() {
    let der = include_bytes!("examples/pkcs1.der");
    let mut buf = [0u8; 2048];
    match pem_rfc7468::decode(der, &mut buf) {
        Err(pem_rfc7468::Error::Preamble) => (),
        _ => panic!("Expected Preamble error"),
    }
}

#[test]
fn pkcs1_example_with_preceeding_junk() {
    let pem = include_bytes!("examples/pkcs1_with_preceeding_junk.pem");
    let mut buf = [0u8; 2048];
    let (label, decoded) = pem_rfc7468::decode(pem, &mut buf).unwrap();
    assert_eq!(label, "RSA PRIVATE KEY");
    assert_eq!(decoded, include_bytes!("examples/pkcs1.der"));
}

#[test]
fn pkcs1_enc_example() {
    let pem = include_bytes!("examples/ssh_rsa_pem_password.pem");
    let mut buf = [0u8; 2048];
    let result = pem_rfc7468::decode(pem, &mut buf);
    assert_eq!(result, Err(pem_rfc7468::Error::HeaderDisallowed));

    let label = pem_rfc7468::decode_label(pem).unwrap();
    assert_eq!(label, "RSA PRIVATE KEY");
}

#[test]
#[cfg(feature = "alloc")]
fn pkcs1_enc_example_with_vec() {
    let pem = include_bytes!("examples/ssh_rsa_pem_password.pem");
    let result = pem_rfc7468::decode_vec(pem);
    assert_eq!(result, Err(pem_rfc7468::Error::HeaderDisallowed));
}

#[test]
fn header_of_length_64() {
    let pem = include_bytes!("examples/chosen_header.pem");
    let mut buf = [0u8; 2048];
    let result = pem_rfc7468::decode(pem, &mut buf);
    assert_eq!(result, Err(pem_rfc7468::Error::HeaderDisallowed));

    let label = pem_rfc7468::decode_label(pem).unwrap();
    assert_eq!(label, "RSA PRIVATE KEY");
}

#[test]
#[cfg(feature = "alloc")]
fn header_of_length_64_with_vec() {
    let pem = include_bytes!("examples/chosen_header.pem");
    match pem_rfc7468::decode_vec(pem) {
        Err(pem_rfc7468::Error::HeaderDisallowed) => (),
        res => panic!("Expected HeaderDisallowed error; Found {:?}", res),
    }
}

#[test]
fn pkcs8_example() {
    let pem = include_bytes!("examples/pkcs8.pem");
    let mut buf = [0u8; 2048];
    let (label, decoded) = pem_rfc7468::decode(pem, &mut buf).unwrap();
    assert_eq!(label, "PRIVATE KEY");
    assert_eq!(decoded, include_bytes!("examples/pkcs8.der"));
}

#[test]
fn pkcs8_enc_example() {
    let pem = include_bytes!("examples/pkcs8-enc.pem");
    let mut buf = [0u8; 2048];
    let (label, decoded) = pem_rfc7468::decode(pem, &mut buf).unwrap();
    assert_eq!(label, "ENCRYPTED PRIVATE KEY");
    assert_eq!(decoded, include_bytes!("examples/pkcs8-enc.der"));
}

#[test]
#[cfg(feature = "alloc")]
fn pkcs1_example_with_vec() {
    let pem = include_bytes!("examples/pkcs1.pem");
    let (label, decoded) = pem_rfc7468::decode_vec(pem).unwrap();
    assert_eq!(label, "RSA PRIVATE KEY");
    assert_eq!(decoded, include_bytes!("examples/pkcs1.der"));
}

#[test]
#[cfg(feature = "alloc")]
fn pkcs8_enc_example_with_vec() {
    let pem = include_bytes!("examples/pkcs8-enc.pem");
    let (label, decoded) = pem_rfc7468::decode_vec(pem).unwrap();
    assert_eq!(label, "ENCRYPTED PRIVATE KEY");
    assert_eq!(decoded, include_bytes!("examples/pkcs8-enc.der"));
}

#[test]
fn ed25519_example() {
    let pem = include_bytes!("examples/ed25519_id.pem");
    let label = pem_rfc7468::decode_label(pem).unwrap();
    assert_eq!(label, "ED25519 CERT");
}

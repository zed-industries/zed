// simple but prevent regression - see https://github.com/RustCrypto/RSA/issues/329
#[cfg(feature = "pem")]
#[test]
fn signature_stringify() {
    use pkcs8::DecodePrivateKey;
    use signature::Signer;

    use rsa::pkcs1v15::SigningKey;
    use rsa::RsaPrivateKey;

    let pem = include_str!("examples/pkcs8/rsa2048-priv.pem");
    let private_key = RsaPrivateKey::from_pkcs8_pem(pem).unwrap();
    let signing_key = SigningKey::<sha2::Sha256>::new(private_key);

    let bytes: &[u8] = b"rsa4096"; // HACK - the criterion is that the signature has leading zeros.
    let signature = signing_key.sign(bytes);

    let expected = "029E365B60971D5A499FF5E1C288B954D3A5DCF52482CEE46DB90DC860B725A8D6CA031146FA156E9F17579BE6122FFB11DAC35E59B2193D75F7B31CE1442DDE7F4FF7885AD5D6080266E9A33BB4CEC93FCC2B6B885457A0ABF19E2DAA00876F694B37F535F119925CCCF9A17B90AE6CF39F07D7FEFBEECDF1B344C14B728196DDD154230BADDEDA5A7EFF373F6CD3EF6D41789572A7A068E3A252D3B7D5D706C6170D8CFDB48C8E738A4B3BFEA3E15716805E376EBD99EA09C6E82F3CFA13CEB23CD289E8F95C27F489ADC05AAACE8A9276EE7CED3B7A5C7264F0D34FF18CEDC3E91D667FCF9992A8CFDE8562F65FDDE1E06595C27E0F82063839A358C927B2";
    assert_eq!(format!("{}", signature), expected);
    assert_eq!(format!("{:x}", signature), expected.to_lowercase());
    assert_eq!(format!("{:X}", signature), expected);
    assert_eq!(signature.to_string(), expected);
}

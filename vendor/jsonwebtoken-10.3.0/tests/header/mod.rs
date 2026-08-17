use base64::{Engine, engine::general_purpose::STANDARD};
use wasm_bindgen_test::wasm_bindgen_test;

use jsonwebtoken::Header;

static CERT_CHAIN: [&str; 3] = include!("cert_chain.json");

#[test]
#[wasm_bindgen_test]
fn x5c_der_empty_chain() {
    let header = Header { x5c: None, ..Default::default() };
    assert_eq!(header.x5c_der().unwrap(), None);

    let header = Header { x5c: Some(Vec::new()), ..Default::default() };
    assert_eq!(header.x5c_der().unwrap(), Some(Vec::new()));
}

#[test]
#[wasm_bindgen_test]
fn x5c_der_valid_chain() {
    let der_chain: Vec<Vec<u8>> =
        CERT_CHAIN.iter().map(|x| STANDARD.decode(x)).collect::<Result<_, _>>().unwrap();

    let x5c = Some(CERT_CHAIN.iter().map(ToString::to_string).collect());
    let header = Header { x5c, ..Default::default() };

    assert_eq!(header.x5c_der().unwrap(), Some(der_chain));
}

#[test]
#[wasm_bindgen_test]
fn x5c_der_invalid_chain() {
    let mut x5c: Vec<_> = CERT_CHAIN.iter().map(ToString::to_string).collect();
    x5c.push("invalid base64 data".to_string());

    let x5c = Some(x5c);
    let header = Header { x5c, ..Default::default() };

    assert!(header.x5c_der().is_err());
}

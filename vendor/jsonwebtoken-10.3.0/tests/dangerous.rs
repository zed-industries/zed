use jsonwebtoken::{Algorithm, TokenData, dangerous::insecure_decode};
use wasm_bindgen_test::wasm_bindgen_test;

#[derive(Debug, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    sub: String,
    aud: Vec<String>,
    iat: i64,
    exp: i64,
}

#[test]
#[wasm_bindgen_test]
fn dangerous_insecure_decode_valid_jwt() {
    let token = "eyJhbGciOiJSUzI1NiIsImtpZCI6IkRReWk2eEFmVVRPWmhJV2R5VWtKZTBFMUJmM1VXV05QIiwidHlwIjoiSldUIn0.eyJhdWQiOlsianNvbndlYnRva2VudGVzdCJdLCJleHAiOjE3NTk4MjYyMTcsImlhdCI6MTc1OTgyNTkxNywic3ViIjoic3BpZmZlOi8vZXhhbXBsZS5vcmcvdGVzdHNlcnZpY2UifQ.1qr1zmMM1hmF-sDZupGc7sT2zGQxl1hFfaUKFWz3UGUeJfUweZfFymGR4jIOJb9ywXmfaafGQbNypaHILPWpeXT8RB7GZ7APu09ZPFvLiKBqagCVWgwhXc30giYPfTq5iNct1ejdYgB1wLxtnrsDRoD_k3EMkB58pDz4H5ZFXc_3xB9TLGw2UdaZ7AloV1yFV6OC5PdleSKchb9E_WaBlbZWLjQNSLhN-YhCRLJ4K59lmL_Z2rnR2812kan8xicyxJAzZ6k0y6K8tpKxUhT--THz2ikUk_olOwDIMfjYe9xmAk-PVvIGwHUVR6fMYv74vhdpwVJACkI2U7HVUhRFkg";

    let TokenData { header, claims } = insecure_decode::<Claims>(token).unwrap();

    assert_eq!(Algorithm::RS256, header.alg);
    assert_eq!("DQyi6xAfUTOZhIWdyUkJe0E1Bf3UWWNP".to_string(), header.kid.unwrap());
    assert_eq!(Some("JWT".to_string()), header.typ);

    assert_eq!(vec!["jsonwebtokentest"], claims.aud);
    assert_eq!("spiffe://example.org/testservice", claims.sub);
    assert_eq!(1759825917, claims.iat);
    assert_eq!(1759826217, claims.exp);
}

#[test]
#[wasm_bindgen_test]
fn dangerous_insecure_decode_invalid_sig() {
    let token = "eyJhbGciOiJSUzI1NiIsImtpZCI6IkRReWk2eEFmVVRPWmhJV2R5VWtKZTBFMUJmM1VXV05QIiwidHlwIjoiSldUIn0.eyJhdWQiOlsianNvbndlYnRva2VudGVzdCJdLCJleHAiOjE3NTk4MjYyMTcsImlhdCI6MTc1OTgyNTkxNywic3ViIjoic3BpZmZlOi8vZXhhbXBsZS5vcmcvdGVzdHNlcnZpY2UifQ.sig";

    let TokenData { header, claims } = insecure_decode::<Claims>(token).unwrap();

    assert_eq!(Algorithm::RS256, header.alg);
    assert_eq!("DQyi6xAfUTOZhIWdyUkJe0E1Bf3UWWNP".to_string(), header.kid.unwrap());
    assert_eq!(Some("JWT".to_string()), header.typ);

    assert_eq!(vec!["jsonwebtokentest"], claims.aud);
    assert_eq!("spiffe://example.org/testservice", claims.sub);
    assert_eq!(1759825917, claims.iat);
    assert_eq!(1759826217, claims.exp);
}

#[test]
#[wasm_bindgen_test]
fn dangerous_insecure_decode_invalid_header() {
    let token = "badz.eyJhdWQiOlsianNvbndlYnRva2VudGVzdCJdLCJleHAiOjE3NTk4MjYyMTcsImlhdCI6MTc1OTgyNTkxNywic3ViIjoic3BpZmZlOi8vZXhhbXBsZS5vcmcvdGVzdHNlcnZpY2UifQ.sig";

    insecure_decode::<Claims>(token).unwrap_err();
}

#[test]
#[wasm_bindgen_test]
fn dangerous_insecure_decode_invalid_claims() {
    let token = "eyJhbGciOiJSUzI1NiIsImtpZCI6IkRReWk2eEFmVVRPWmhJV2R5VWtKZTBFMUJmM1VXV05QIiwidHlwIjoiSldUIn0.badz.sig";

    insecure_decode::<Claims>(token).unwrap_err();
}

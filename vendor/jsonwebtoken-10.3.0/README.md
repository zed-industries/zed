# jsonwebtoken

[API documentation on docs.rs](https://docs.rs/jsonwebtoken/)

See [JSON Web Tokens](https://en.wikipedia.org/wiki/JSON_Web_Token) for more information on what JSON Web Tokens are.

## Installation
Add the following to Cargo.toml:

```toml
# You will have to select either `aws_lc_rs` or `rust_crypto` as backend if you're not using your own
jsonwebtoken = { version = "10", features = ["aws_lc_rs"] }
# If you do not need pem decoding, you can disable the default feature `use_pem` that way:
# jsonwebtoken = {version = "10", default-features = false, features = ["aws_lc_rs"] }
serde = {version = "1.0", features = ["derive"] }
```

Two crypto backends are available via features, `aws_lc_rs` and `rust_crypto`, at most one of which must be enabled. If you select neither feature, you need to provide your own `CryptoProvider`.

For examples of how to implement a `CryptoProvider`, see
- [arckoor/jsonwebtoken-botan](https://github.com/arckoor/jsonwebtoken-botan)

The minimum required Rust version (MSRV) is specified in the `rust-version` field in this project's [Cargo.toml](Cargo.toml).

## Algorithms
This library currently supports the following:

- HS256
- HS384
- HS512
- RS256
- RS384
- RS512
- PS256
- PS384
- PS512
- ES256
- ES384
- EdDSA


## How to use
Complete examples are available in the examples directory: a basic one and one with a custom header.

In terms of imports and structs:
```rust
use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}
```

### Claims
The claims fields which can be validated. (see [validation](#validation))

```rust
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String,         // Optional. Audience
    exp: usize,          // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize,          // Optional. Issued at (as UTC timestamp)
    iss: String,         // Optional. Issuer
    nbf: usize,          // Optional. Not Before (as UTC timestamp)
    sub: String,         // Optional. Subject (whom token refers to)
}
```

### Header
The default algorithm is HS256, which uses a shared secret.

```rust
let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret("secret".as_ref()))?;
```

#### Custom headers & changing algorithm
All the parameters from the RFC are supported but the default header only has `typ` and `alg` set.
If you want to set the `kid` parameter or change the algorithm for example:

```rust
let mut header = Header::new(Algorithm::HS512);
header.kid = Some("blabla".to_owned());

let mut extras = HashMap::with_capacity(1);
extras.insert("custom".to_string(), "header".to_string());
header.extras = Some(extras);

let token = encode(&header, &my_claims, &EncodingKey::from_secret("secret".as_ref()))?;
```
Look at `examples/custom_header.rs` for a full working example.

### Encoding

```rust
// HS256
let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret("secret".as_ref()))?;
// RSA
let token = encode(&Header::new(Algorithm::RS256), &my_claims, &EncodingKey::from_rsa_pem(include_bytes!("privkey.pem"))?)?;
```
Encoding a JWT takes 3 parameters:

- a header: the `Header` struct
- some claims: your own struct
- a key/secret

When using HS256, HS384, or HS512, the key is always a shared secret like in the example above. When using
RSA/EC, the key should always be the content of the private key in PEM or DER format.

If your key is in PEM format, it is better performance wise to generate the `EncodingKey` once in a `lazy_static` or
something similar and reuse it.

### Encoding and decoding JWS

JWS is handled the same way as JWT, but using `jsonwebtoken::jws::encode` and `jsonwebtoken::jws::decode`:

```rust
use jsonwebtoken::jws::{encode, decode};

let encoded = encode(&Header::default(), &my_claims, &EncodingKey::from_secret("secret".as_ref()))?;
my_claims = decode(&encoded, &DecodingKey::from_secret("secret".as_ref()), &Validation::default())?.claims;
```

`jsonwebtoken::jws::encode` returns a `Jws<C>` struct which can be placed in other structs or serialized/deserialized from JSON directly.

The generic parameter in `Jws<C>` indicates the claims type and prevents accidentally encoding or decoding the wrong claims type
when the Jws is nested in another struct.

### JWK Thumbprints

If you have a JWK object, you can generate a thumbprint like

```
let tp = my_jwk.thumbprint(&jsonwebtoken::DIGEST_SHA256);
```

### Decoding

```rust
// `token` is a struct with 2 fields: `header` and `claims` where `claims` is your own struct.
let token = decode::<Claims>(&token, &DecodingKey::from_secret("secret".as_ref()), &Validation::default())?;
```
`decode` can result in errors for a variety of reasons:

- the token or its signature is invalid
- the token had invalid base64
- validation of at least one reserved claim failed

As with encoding, when using HS256, HS384, or HS512, the key is always a shared secret like in the example above. When using
RSA/EC, the key should always be the content of the public key in PEM (or certificate in this case) or DER format.

In some cases, for example if you don't know the algorithm used or need to grab the `kid`, you can choose to decode only the header:

```rust
let header = decode_header(&token)?;
```

This does not perform any signature verification or validate the token claims.

You can also decode a token using the public key components of a RSA key in base64 format.
The main use-case is for JWK where your public key is in a JSON format like so:

```json
{
   "kty":"RSA",
   "e":"AQAB",
   "kid":"6a7a119f-0876-4f7e-8d0f-bf3ea1391dd8",
   "n":"yRE6rHuNR0QbHO3H3Kt2pOKGVhQqGZXInOduQNxXzuKlvQTLUTv4l4sggh5_CYYi_cvI-SXVT9kPWSKXxJXBXd_4LkvcPuUakBoAkfh-eiFVMh2VrUyWyj3MFl0HTVF9KwRXLAcwkREiS3npThHRyIxuy0ZMeZfxVL5arMhw1SRELB8HoGfG_AtH89BIE9jDBHZ9dLelK9a184zAf8LwoPLxvJb3Il5nncqPcSfKDDodMFBIMc4lQzDKL5gvmiXLXB1AGLm8KBjfE8s3L5xqi-yUod-j8MtvIj812dkS4QMiRVN_by2h3ZY8LYVGrqZXZTcgn2ujn8uKjXLZVD5TdQ"
}
```

```rust
// `token` is a struct with 2 fields: `header` and `claims` where `claims` is your own struct.
let token = decode::<Claims>(&token, &DecodingKey::from_rsa_components(jwk["n"], jwk["e"]), &Validation::new(Algorithm::RS256))?;
```

If your key is in PEM format, it is better performance wise to generate the `DecodingKey` once in a `lazy_static` or
something similar and reuse it.

### Convert SEC1 private key to PKCS8
`jsonwebtoken` currently only supports PKCS8 format for private EC keys. If your key has `BEGIN EC PRIVATE KEY` at the top,
this is a SEC1 type and can be converted to PKCS8 like so:

```bash
openssl pkcs8 -topk8 -nocrypt -in sec1.pem -out pkcs8.pem
```


## Validation
This library automatically validates the `exp` claim, and `nbf` is validated if present. You can also validate the `sub`, `iss`, and `aud` but
those require setting the expected values in the `Validation` struct. In the case of `aud`, if there is a value set in the token but
not in the `Validation`, the token will be rejected.

Validation is only made on present fields in the claims. It is possible to define the required claims, hence verifying that a JWT has a value for each of these claims before it is considered for validation. The required claims can be set in the `Validation` struct. The default option requires the `exp` claim to be present.

Since validating time fields is always a bit tricky due to clock skew,
you can add some leeway to the `iat`, `exp`, and `nbf` validation by setting the `leeway` field.

Last but not least, you will need to set the algorithm(s) allowed for this token if you are not using `HS256`.

Look at `examples/validation.rs` for a full working example.

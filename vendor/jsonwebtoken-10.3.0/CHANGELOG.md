# Changelog

## 10.3.0 (2026-01-27)

- Export everything needed to define your own CryptoProvider
- Fix type confusion with exp/nbf when not required

## 10.2.0 (2025-11-06)

- Remove `Clone` bound from decode functions

## 10.1.0 (2025-10-18)

- add `dangerous::insecure_decode`
- Implement TryFrom &Jwk for DecodingKey

## 10.0.0 (2025-09-29)

- BREAKING: now using traits for crypto backends, you have to choose between `aws_lc_rs` and `rust_crypto`
- Add `Clone` bound to `decode`
- Support decoding byte slices
- Support JWS

## 9.3.1 (2024-02-06)

- Update base64

## 9.3.0 (2024-03-12)

- Add `Validation.reject_tokens_expiring_in_less_than`, the opposite of leeway

## 9.2.0 (2023-12-01)

- Add an option to not validate `aud` in the Validation struct
- Get the current timestamp in wasm without using std
- Update ring to 0.17

## 9.1.0 (2023-10-21)

- Supports deserialization of unsupported algorithms for JWKs


## 9.0.0 (2023-10-16)

- Update ring
- Rejects JWTs containing audiences when the Validation doesn't contain any

## 8.3.0 (2023-03-15)

- Update base64
- Implement Clone for TokenData<T> if T impls Clone


## 8.2.0 (2022-12-03)

- Add DecodingKey::from_jwk
- Can now use PEM certificates if you have the `use_pem` feature enabled


## 8.1.1 (2022-06-17)

- Fix invalid field name on OctetKeyParameters

## 8.1.0 (2022-04-12)

- Make optional fields in the spec really optional
- Implements `Hash` for `Header`

## 8.0.1 (2022-02-03)

- Fix documentation of leeway


## 8.0.0 (2022-02-02)
 
- Add EdDSA algorithm
- `sign`/`verify` now takes a `&[u8]` instead of `&str` to be more flexible
- `DecodingKey` now own its data
- Remove deprecated `dangerous_unsafe_decode`
- `Validation::iss` is now a `HashSet` instead of a single value
- `decode` will now error if `Validation::algorithms` is empty
- Add JWKs types for easy interop with various Oauth provider, see `examples/auth0.rs` for an example
- Removed `decode_*` functions in favour of using the `Validation` struct
- Allow float values for `exp` and `nbf`, yes it's in the spec... floats will be rounded and converted to u64
- Error now implements Clone/Eq
- Change default leeway from 0s to 60s
- Add `Validation::require_spec_claims` to validate presence of the spec claims
- Add default feature for pem decoding named `use_pem` that can be disabled to avoid 2 dependencies

## 7.2.0 (2020-06-30)

- Add `dangerous_insecure_decode` to replace `dangerous_unsafe_decode`, which is now deprecated
- Add `dangerous_insecure_decode_with_validation`

## 7.1.2 (2020-06-16)

- Derive `Hash` for `Header` and `Algorithm`

## 7.1.1 (2020-06-09)

- Update dependencies

## 7.1.0 (2020-03-01)

- Add `into_static` to `DecodingKey` for easier re-use

# 7.0.0 (2020-01-28)

- Add support for PS256, PS384 and PS512
- Add support for verifying with modulus/exponent components for RSA
- Update to 2018 edition
- Changed aud field type in Validation to `Option<HashSet<String>>`.  Audience 
  validation now tests for "any-of-these" audience membership.
- Add support for keys in PEM format
- Add EncodingKey/DecodingKey API to improve performance and UX

## 6.0.1 (2019-05-10)

- Fix Algorithm mapping in FromStr for RSA

## 6.0.0 (2019-04-21)

- Update Ring to 0.14
- Remove `iat` check to match the JWT spec
- Add ES256 and ES384 signing decoding

## 5.0.1 (2018-09-10)

- Add implementation of FromStr for Algorithm

## 5.0.0 (2018-08-13)

- Update ring
- Change error handling to be based on simple struct/enum rather than error-chain
- Fix validations not being called properly in some cases
- Default validation is not checking `iat` and `nbf` anymore

## 4.0.1 (2018-03-19)

- Add method to decode a token without signature verification

## 4.0.0 (2017-11-22)

### Breaking changes

- Make it mandatory to specify the algorithm in `decode`

## 3.0.0 (2017-09-08)

### Breaking changes
- Remove `validate_signature` from `Validation`, use `decode_header` instead if you don't know the alg used
- Make `typ` optional in header, some providers apparently don't use it

### Others

- Update ring & error-chain
- Fix documentation about `leeway` being in seconds and not milliseconds
- Add `decode_header` to only decode the header: replaces the use case of `validate_signature`

## 2.0.3 (2017-07-18)

- Make `TokenData` public

## 2.0.2 (2017-06-24)

- Update ring & chrono

## 2.0.1 (2017-05-09)

- Update ring

## 2.0.0 (2017-04-23)

- Use Serde instead of rustc_serialize
- Add RSA support
- API overhaul, see README for new usage
- Add validation
- Update all dependencies

## Previous

- 1.1.7: update ring
- 1.1.6: update ring
- 1.1.5: update ring version
- 1.1.4: use ring instead of rust-crypto
- 1.1.3: Make sign and verify public
- 1.1.2: Update rust-crypto to 0.2.35
- 1.1.1: Don't serialize empty fields in header
- 1.1.0: Impl Error for jsonwebtoken errors
- 1.0: Initial release

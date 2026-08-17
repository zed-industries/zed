// Copyright 2015 The Chromium Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef BSSL_PKI_SIGNATURE_ALGORITHM_H_
#define BSSL_PKI_SIGNATURE_ALGORITHM_H_

#include <stdint.h>

#include <optional>

#include <openssl/base.h>
#include <openssl/evp.h>

BSSL_NAMESPACE_BEGIN

namespace der {
class Input;
}  // namespace der

// The digest algorithm used within a signature.
enum class DigestAlgorithm {
  Md2,
  Md4,
  Md5,
  Sha1,
  Sha256,
  Sha384,
  Sha512,
};

// The signature algorithm used within a certificate.
enum class SignatureAlgorithm {
  kRsaPkcs1Sha1,
  kRsaPkcs1Sha256,
  kRsaPkcs1Sha384,
  kRsaPkcs1Sha512,
  kEcdsaSha1,
  kEcdsaSha256,
  kEcdsaSha384,
  kEcdsaSha512,
  // These RSA-PSS constants match RFC 8446 and refer to RSASSA-PSS with MGF-1,
  // using the specified hash as both the signature and MGF-1 hash, and the hash
  // length as the salt length.
  kRsaPssSha256,
  kRsaPssSha384,
  kRsaPssSha512,
  kMaxValue = kRsaPssSha512,
};

// Parses AlgorithmIdentifier as defined by RFC 5280 section 4.1.1.2:
//
//     AlgorithmIdentifier  ::=  SEQUENCE  {
//          algorithm               OBJECT IDENTIFIER,
//          parameters              ANY DEFINED BY algorithm OPTIONAL  }
[[nodiscard]] OPENSSL_EXPORT bool ParseAlgorithmIdentifier(
    der::Input input, der::Input *algorithm, der::Input *parameters);

// Parses a HashAlgorithm as defined by RFC 5912:
//
//     HashAlgorithm  ::=  AlgorithmIdentifier{DIGEST-ALGORITHM,
//                             {HashAlgorithms}}
//
//     HashAlgorithms DIGEST-ALGORITHM ::=  {
//         { IDENTIFIER id-sha1 PARAMS TYPE NULL ARE preferredPresent } |
//         { IDENTIFIER id-sha224 PARAMS TYPE NULL ARE preferredPresent } |
//         { IDENTIFIER id-sha256 PARAMS TYPE NULL ARE preferredPresent } |
//         { IDENTIFIER id-sha384 PARAMS TYPE NULL ARE preferredPresent } |
//         { IDENTIFIER id-sha512 PARAMS TYPE NULL ARE preferredPresent }
//     }
[[nodiscard]] bool ParseHashAlgorithm(der::Input input, DigestAlgorithm *out);

// Parses an AlgorithmIdentifier into a signature algorithm and returns it, or
// returns `std::nullopt` if `algorithm_identifier` either cannot be parsed or
// is not a recognized signature algorithm.
OPENSSL_EXPORT std::optional<SignatureAlgorithm> ParseSignatureAlgorithm(
    der::Input algorithm_identifier);

// Returns the hash to be used with the tls-server-end-point channel binding
// (RFC 5929) or `std::nullopt`, if not supported for this signature algorithm.
OPENSSL_EXPORT std::optional<DigestAlgorithm>
GetTlsServerEndpointDigestAlgorithm(SignatureAlgorithm alg);

BSSL_NAMESPACE_END

#endif  // BSSL_PKI_SIGNATURE_ALGORITHM_H_

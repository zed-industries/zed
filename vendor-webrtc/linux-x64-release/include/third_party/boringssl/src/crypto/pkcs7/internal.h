// Copyright 2017 The BoringSSL Authors
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

#ifndef OPENSSL_HEADER_CRYPTO_PKCS7_INTERNAL_H
#define OPENSSL_HEADER_CRYPTO_PKCS7_INTERNAL_H

#include <openssl/base.h>

#if defined(__cplusplus)
extern "C" {
#endif


// pkcs7_parse_header reads the non-certificate/non-CRL prefix of a PKCS#7
// SignedData blob from |cbs| and sets |*out| to point to the rest of the
// input. If the input is in BER format, then |*der_bytes| will be set to a
// pointer that needs to be freed by the caller once they have finished
// processing |*out| (which will be pointing into |*der_bytes|).
//
// It returns one on success or zero on error. On error, |*der_bytes| is
// NULL.
int pkcs7_parse_header(uint8_t **der_bytes, CBS *out, CBS *cbs);

// pkcs7_add_signed_data writes a PKCS#7, SignedData structure to |out|. While
// doing so it makes callbacks to let the caller fill in parts of the structure.
// All callbacks are ignored if NULL and return one on success or zero on error.
//
//   signed_data_version: version number of the SignedData structure. In PKCS#7,
//       it is always 1. In CMS, it depends on the features used.
//   digest_algos_cb: may write AlgorithmIdentifiers into the given CBB, which
//       is a SET of digest algorithms.
//   cert_crl_cb: may write the |certificates| or |crls| fields.
//       (See https://datatracker.ietf.org/doc/html/rfc2315#section-9.1)
//   signer_infos_cb: may write the contents of the |signerInfos| field.
//       (See https://datatracker.ietf.org/doc/html/rfc2315#section-9.1)
//
// pkcs7_add_signed_data returns one on success or zero on error.
int pkcs7_add_signed_data(CBB *out, uint64_t signed_data_version,
                          int (*digest_algos_cb)(CBB *out, void *arg),
                          int (*cert_crl_cb)(CBB *out, void *arg),
                          int (*signer_infos_cb)(CBB *out, void *arg),
                          void *arg);

// pkcs7_add_external_signature writes a PKCS#7 or CMS SignedData structure to
// |out|, containing an external (i.e. the contents are not included) signature,
// using |sign_cert| and |key| to sign the contents of |data| with |md|. If
// |use_key_id| is true (CMS-only), the SignerInfo specifies the signer with key
// identifier. Otherwise, it uses issuer and serial number (PKCS#7 or CMS v1).
// The SignedData will have no embedded certificates and no attributes.
//
// Note: CMS v1 and PKCS#7 v1.5 are not completely compatible, but they overlap
// in all cases implemented by this function.
int pkcs7_add_external_signature(CBB *out, X509 *sign_cert, EVP_PKEY *key,
                                 const EVP_MD *md, BIO *data, bool use_key_id);


#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_CRYPTO_PKCS7_INTERNAL_H

// Copyright 2025 The BoringSSL Authors
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

#ifndef OPENSSL_HEADER_CMS_H
#define OPENSSL_HEADER_CMS_H

#include <openssl/base.h>   // IWYU pragma: export

#include <openssl/stack.h>

#if defined(__cplusplus)
extern "C" {
#endif


// CMS.
//
// This library contains functions for implementing a small subset of OpenSSL's
// API for CMS (RFC 5652). A general CMS implementation, notably one suitable
// for S/MIME, is out of scope for BoringSSL.
//
// As this library is intentionally not a general CMS implementation, BoringSSL
// continues to define |OPENSSL_NO_CMS|, so that most callers turn off their
// general-purpose CMS code. In callers that are compatible with this subset,
// the |BORINGSSL_NO_NO_CMS| build option can be used to suppress
// |OPENSSL_NO_CMS|.

DECLARE_STACK_OF(X509)

// CMS_* are flags that can be passed to functions in this library. Their
// interpretation is specified in the corresponding functinos.
#define CMS_NOCERTS 0x2
#define CMS_DETACHED 0x40
#define CMS_BINARY 0x80
#define CMS_NOATTR 0x100
#define CMS_NOSMIMECAP 0x200
#define CMS_STREAM 0x1000
#define CMS_PARTIAL 0x4000
#define CMS_USE_KEYID 0x10000
#define CMS_NO_SIGNING_TIME 0x400000

// CMS_sign returns a newly-allocated |CMS_ContentInfo| structure for building a
// SignedData (RFC 5652), or NULL on error.
//
// |certs| must be NULL or zero length. BoringSSL does not support embedding
// certificates in SignedData.
//
// |flags| must contain |CMS_DETACHED|, which indicates an external signature.
// BoringSSL only supports generating external signatures and does not support
// embedding encapsulated content directly in a SignedData.
//
// If |pkey| is non-NULL, |CMS_add1_signer| is automatically called with
// |signcert|, |pkey|, a default hash of SHA-256, and |flags|. |flags| will then
// additionally be interpreted as in |CMS_add1_signer|.
//
// If |CMS_PARTIAL| or |CMS_STREAM| is set in |flags|, the object will be left
// incomplete. |data| will then be ignored and should be NULL. The caller can
// then continue configuring it and finalizing it with |CMS_final|. Otherwise,
// the object will be finalized with |data| and |flags| passed to |CMS_final|.
OPENSSL_EXPORT CMS_ContentInfo *CMS_sign(X509 *signcert, EVP_PKEY *pkey,
                                         STACK_OF(X509) *certs, BIO *data,
                                         uint32_t flags);

// CMS_ContentInfo_free releases memory associated with |cms|.
OPENSSL_EXPORT void CMS_ContentInfo_free(CMS_ContentInfo *cms);

// CMS_add1_signer adds a signer to |cms|, which must be a SignedData created by
// |CMS_sign|, with the |CMS_PARTIAL| flag set. The signer will use |signcert|,
// |pkey|, and |md| for the signing certificate, private key, and digest
// algorithm, respectively. It returns a non-NULL pointer to the signer on
// success, and NULL on error. The signer is owned by |cms| and should not be
// released by the caller.
//
// |flags| is interpreted as follows:
//
// - |CMS_PARTIAL| must not be set. BoringSSL does not support configuring a
//   signer in multiple steps.
//
// - |CMS_NOCERTS| must be set. BoringSSL does not support embedding
//   certificates in SignedData.
//
// - |CMS_NOATTR| must be set. BoringSSL does not support attributes in
//   SignedData.
//
// - If |CMS_USE_KEYID| is set, SignerInfos will be identified by subject key
//   identifier instead of issuer and serial number. |signcert| must then have
//   the subject key identifier extension.
//
// BoringSSL currently only supports one signer per |CMS_ContentInfo|.
// Subsequent calls will fail. Additionally, only RSA keys are currently
// supported for |pkey|.
OPENSSL_EXPORT CMS_SignerInfo *CMS_add1_signer(CMS_ContentInfo *cms,
                                               X509 *signcert, EVP_PKEY *pkey,
                                               const EVP_MD *md,
                                               uint32_t flags);

// CMS_final finalizes constructing |cms|, which must have been initialized with
// the |CMS_PARTIAL| flag. |data| is read, until EOF, as the data to be
// processed by CMS. It returns one on success and zero on error.
//
// |CMS_BINARY| must be set in |flags|. BoringSSL does not support translating
// inputs according to S/MIME.
//
// |dcont| must be NULL. What a non-NULL |dcont| does is not clearly documented
// by OpenSSL, and there are no tests to demonstrate its behavior.
OPENSSL_EXPORT int CMS_final(CMS_ContentInfo *cms, BIO *data, BIO *dcont,
                             uint32_t flags);

// i2d_CMS_bio encodes |cms| as a DER-encoded ContentInfo structure (RFC 5652).
// It returns one on success and zero on failure.
OPENSSL_EXPORT int i2d_CMS_bio(BIO *out, CMS_ContentInfo *cms);

// i2d_CMS_bio_stream calls |i2d_CMS_bio|. |in| must be NULL and |flags| must
// not contain |CMS_STREAM|. BoringSSL does not support any streaming modes for
// CMS.
OPENSSL_EXPORT int i2d_CMS_bio_stream(BIO *out, CMS_ContentInfo *cms, BIO *in,
                                      int flags);


#if defined(__cplusplus)
}  // extern C

extern "C++" {
BSSL_NAMESPACE_BEGIN

BORINGSSL_MAKE_DELETER(CMS_ContentInfo, CMS_ContentInfo_free)

BSSL_NAMESPACE_END
}  // extern C++
#endif

#define CMS_R_CERTIFICATE_HAS_NO_KEYID 100
#define CMS_R_PRIVATE_KEY_DOES_NOT_MATCH_CERTIFICATE 101

#endif  // OPENSSL_HEADER_CMS_H

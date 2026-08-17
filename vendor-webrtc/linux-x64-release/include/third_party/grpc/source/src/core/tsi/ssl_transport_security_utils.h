//
//
// Copyright 2022 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
//

#ifndef GRPC_SRC_CORE_TSI_SSL_TRANSPORT_SECURITY_UTILS_H
#define GRPC_SRC_CORE_TSI_SSL_TRANSPORT_SECURITY_UTILS_H

#include <grpc/grpc_security_constants.h>
#include <grpc/support/port_platform.h>
#include <openssl/evp.h>
#include <openssl/x509.h>

#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "src/core/tsi/ssl/key_logging/ssl_key_logging.h"
#include "src/core/tsi/transport_security_interface.h"

namespace grpc_core {

// Converts an SSL error status code to a readable string.
//
// error: the SSL error status code.
//
// return: the corresponding status string.
const char* SslErrorString(int error);

// Logs the SSL error stack.
void LogSslErrorStack(void);

// Performs an SSL_write and handle errors.
//
// ssl: the SSL object to write to.
// unprotected_bytes: the buffer containing the bytes for writing to |ssl|.
// unprotected_bytes_size: the size of the buffer |unprotected_bytes|.
//
// return: TSI_OK if the write operation succeeds or corresponding TSI errors.
tsi_result DoSslWrite(SSL* ssl, unsigned char* unprotected_bytes,
                      size_t unprotected_bytes_size);

// Performs an SSL_read and handle errors.
//
// ssl: the SSL object to read from.
// unprotected_bytes: the buffer to which this function will populate the read
//                    result from |ssl|.
// unprotected_bytes_size: the maximum size of the buffer |unprotected_bytes|.
//                         This will be populated with the size of the bytes
//                         read from |ssl| if this function returns TSI_OK.
//
// return: TSI_OK if the write operation succeeds or corresponding TSI errors.
tsi_result DoSslRead(SSL* ssl, unsigned char* unprotected_bytes,
                     size_t* unprotected_bytes_size);

// Builds a maximum-size TLS frame if there is enough (|buffer_offset| +
// |unprotected_bytes_size| >= |buffers_size|) data. Otherwise it copies the
// |unprotected_bytes| into |buffer| and returns TSI_OK.
//
// unprotected_bytes: pointing to the buffer containing the plaintext to be
//                    protected.
// buffer_size: the size of |buffer|. If |buffer_offset| equals |buffer_size|,
//              then we have enough data to create a TLS frame.
// buffer_offset: the offset of |buffer|. The data in |buffer| up to
//                |buffer_offset| are valid data. This will be updated whenever
//                new data are copied into |buffer|.
// buffer: the buffer holding the data that has not been sent for protecting.
// ssl: the |SSL| object that protects the data.
// network_io: the |BIO| object associated with |ssl|.
// unprotected_bytes_size: the size of the unprotected plaintext. This will be
//                         populated with the size of data that is consumed by
//                         this function. Caller can use this to see the size of
//                         unconsumed data in |unprotected_bytes|.
// protected_output_frames: the TLS frames built out of the plaintext.
// protected_output_frames_size: the size of the TLS frames built.
//
// return: TSI_OK if either successfully created a TSI frame or copied the
//         |unprotected_data| into |buffer|. Returns corresponding TSI errors
//         otherwise.
tsi_result SslProtectorProtect(const unsigned char* unprotected_bytes,
                               const size_t buffer_size, size_t& buffer_offset,
                               unsigned char* buffer, SSL* ssl, BIO* network_io,
                               std::size_t* unprotected_bytes_size,
                               unsigned char* protected_output_frames,
                               size_t* protected_output_frames_size);

// Builds a TLS frame out of the remaining plaintext bytes that's left in
// buffer. Populates the size of the remaining TLS frame to
// |still_pending_size|.
//
// buffer_size: the size of |buffer|. If |buffer_offset| equals |buffer_size|,
//              then we have enough data to create a TLS frame.
// buffer_offset: the offset of |buffer|. The data in |buffer| up to
//                |buffer_offset| are valid data. This will be updated whenever
//                new data are copied into |buffer|.
// buffer: the buffer holding the data that has not been sent for protecting.
// ssl: the |SSL| object that protects the data.
// network_io: the |BIO| object associated with |ssl|.
// protected_output_frames: the TLS frames built out of the plaintext.
// protected_output_frames_size: the size of the TLS frames built.
// still_pending_size: the size of the bytes that remains in |network_io|.
//
// return: TSI_OK if successfully created a TSI frame. Returns corresponding TSI
//         errors otherwise.
tsi_result SslProtectorProtectFlush(size_t& buffer_offset,
                                    unsigned char* buffer, SSL* ssl,
                                    BIO* network_io,
                                    unsigned char* protected_output_frames,
                                    size_t* protected_output_frames_size,
                                    size_t* still_pending_size);

// Extracts the plaintext from a TLS frame.
//
// protected_frames_bytes: the TLS frame to extract plaintext from.
// ssl: the |SSL| object that protects the data.
// network_io: the |BIO| object associated with |ssl|.
// unprotected_bytes_size: the size of the unprotected plaintext. This will be
//                         populated with the size of data that is consumed by
//                         this function. Caller can use this to see the size of
//                         unconsumed data in |unprotected_bytes|.
// protected_output_frames: the TLS frames built out of the plaintext.
// protected_output_frames_size: the size of the TLS frames built.
//
// return: TSI_OK if either successfully created a TSI frame or copied the
//         |unprotected_data| into |buffer|. Returns corresponding TSI errors
//         otherwise.
tsi_result SslProtectorUnprotect(const unsigned char* protected_frames_bytes,
                                 SSL* ssl, BIO* network_io,
                                 size_t* protected_frames_bytes_size,
                                 unsigned char* unprotected_bytes,
                                 size_t* unprotected_bytes_size);

// Verifies that `crl` was signed by `issuer.
// return: true if valid, false otherwise.
bool VerifyCrlSignature(X509_CRL* crl, X509* issuer);

// Verifies the CRL issuer and certificate issuer name match.
// return: true if equal, false if not.
bool VerifyCrlCertIssuerNamesMatch(X509_CRL* crl, X509* cert);

// Verifies the certificate in question has the cRLSign bit present.
// return: true if cRLSign bit is present, false otherwise.
bool HasCrlSignBit(X509* cert);

// Gets a stable representation of the issuer name from an X509 certificate.
// return: a std::string of the DER encoding of the X509_NAME issuer name.
absl::StatusOr<std::string> IssuerFromCert(X509* cert);

// Gets a stable representation of the authority key identifier from an X509
// certificate.
// return: a std::string of the DER encoding of the AKID or a status on failure.
absl::StatusOr<std::string> AkidFromCertificate(X509* cert);

// Gets a stable representation of the authority key identifier from an X509
// crl.
// return: a std::string of the DER encoding of the AKID or a status on failure.
absl::StatusOr<std::string> AkidFromCrl(X509_CRL* crl);

// Returns a vector of X509 instances parsed from the given PEM-encoded
// certificate chain. Caller takes ownership of the X509 pointers in the output
// vector.
absl::StatusOr<std::vector<X509*>> ParsePemCertificateChain(
    absl::string_view cert_chain_pem);

// Returns an EVP_PKEY instance parsed from the non-empty PEM private key block
// in private_key_pem. Caller takes ownership of the EVP_PKEY pointer.
absl::StatusOr<EVP_PKEY*> ParsePemPrivateKey(absl::string_view private_key_pem);
}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_TSI_SSL_TRANSPORT_SECURITY_UTILS_H

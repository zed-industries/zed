/*
 *  Copyright 2018 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

// Generic interface for SSL Certificates, used in both the SSLAdapter
// for TLS TURN connections and the SSLStreamAdapter for DTLS Peer to Peer
// Connections for SRTP Key negotiation and SCTP encryption.

#ifndef RTC_BASE_SSL_CERTIFICATE_H_
#define RTC_BASE_SSL_CERTIFICATE_H_

#include <stddef.h>
#include <stdint.h>

#include <memory>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "rtc_base/buffer.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

struct RTC_EXPORT SSLCertificateStats {
  SSLCertificateStats(std::string&& fingerprint,
                      std::string&& fingerprint_algorithm,
                      std::string&& base64_certificate,
                      std::unique_ptr<SSLCertificateStats> issuer);
  ~SSLCertificateStats();
  std::string fingerprint;
  std::string fingerprint_algorithm;
  std::string base64_certificate;
  std::unique_ptr<SSLCertificateStats> issuer;

  std::unique_ptr<SSLCertificateStats> Copy() const;
};

// Abstract interface overridden by SSL library specific
// implementations.

// A somewhat opaque type used to encapsulate a certificate.
// Wraps the SSL library's notion of a certificate, with reference counting.
// The SSLCertificate object is pretty much immutable once created.
// (The OpenSSL implementation only does reference counting and
// possibly caching of intermediate results.)
class RTC_EXPORT SSLCertificate {
 public:
  // Parses and builds a certificate from a PEM encoded string.
  // Returns null on failure.
  // The length of the string representation of the certificate is
  // stored in *pem_length if it is non-null, and only if
  // parsing was successful.
  static std::unique_ptr<SSLCertificate> FromPEMString(
      absl::string_view pem_string);
  virtual ~SSLCertificate() = default;

  // Returns a new SSLCertificate object instance wrapping the same
  // underlying certificate, including its chain if present.
  virtual std::unique_ptr<SSLCertificate> Clone() const = 0;

  // Returns a PEM encoded string representation of the certificate.
  virtual std::string ToPEMString() const = 0;

  // Provides a DER encoded binary representation of the certificate.
  virtual void ToDER(Buffer* der_buffer) const = 0;

  // Gets the name of the digest algorithm that was used to compute this
  // certificate's signature.
  virtual bool GetSignatureDigestAlgorithm(std::string* algorithm) const = 0;

  // Compute the digest of the certificate given algorithm
  virtual bool ComputeDigest(absl::string_view algorithm,
                             Buffer& digest) const = 0;

  // Returns the time in seconds relative to epoch, 1970-01-01T00:00:00Z (UTC),
  // or -1 if an expiration time could not be retrieved.
  virtual int64_t CertificateExpirationTime() const = 0;

  // Gets information (fingerprint, etc.) about this certificate. This is used
  // for certificate stats, see
  // https://w3c.github.io/webrtc-stats/#certificatestats-dict*.
  std::unique_ptr<SSLCertificateStats> GetStats() const;
};

// SSLCertChain is a simple wrapper for a vector of SSLCertificates. It serves
// primarily to ensure proper memory management (especially deletion) of the
// SSLCertificate pointers.
class RTC_EXPORT SSLCertChain final {
 public:
  explicit SSLCertChain(std::unique_ptr<SSLCertificate> single_cert);
  explicit SSLCertChain(std::vector<std::unique_ptr<SSLCertificate>> certs);
  // Allow move semantics for the object.
  SSLCertChain(SSLCertChain&&);
  SSLCertChain& operator=(SSLCertChain&&);

  ~SSLCertChain();

  SSLCertChain(const SSLCertChain&) = delete;
  SSLCertChain& operator=(const SSLCertChain&) = delete;

  // Vector access methods.
  size_t GetSize() const { return certs_.size(); }

  // Returns a temporary reference, only valid until the chain is destroyed.
  const SSLCertificate& Get(size_t pos) const { return *(certs_[pos]); }

  // Returns a new SSLCertChain object instance wrapping the same underlying
  // certificate chain.
  std::unique_ptr<SSLCertChain> Clone() const;

  // Gets information (fingerprint, etc.) about this certificate chain. This is
  // used for certificate stats, see
  // https://w3c.github.io/webrtc-stats/#certificatestats-dict*.
  std::unique_ptr<SSLCertificateStats> GetStats() const;

 private:
  std::vector<std::unique_ptr<SSLCertificate>> certs_;
};

// SSLCertificateVerifier provides a simple interface to allow third parties to
// define their own certificate verification code. It is completely independent
// from the underlying SSL implementation.
class SSLCertificateVerifier {
 public:
  virtual ~SSLCertificateVerifier() = default;
  // Returns true if the certificate is valid, else false. It is up to the
  // implementer to define what a valid certificate looks like.
  virtual bool Verify(const SSLCertificate& certificate) = 0;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::SSLCertChain;
using ::webrtc::SSLCertificate;
using ::webrtc::SSLCertificateStats;
using ::webrtc::SSLCertificateVerifier;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // RTC_BASE_SSL_CERTIFICATE_H_

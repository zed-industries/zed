/*
 *  Copyright 2004 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

// Handling of certificates and keypairs for SSLStreamAdapter's peer mode.

#ifndef RTC_BASE_SSL_IDENTITY_H_
#define RTC_BASE_SSL_IDENTITY_H_

#include <stdint.h>

#include <ctime>
#include <memory>
#include <string>

#include "absl/strings/string_view.h"
#include "rtc_base/ssl_certificate.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

// KT_LAST is intended for vector declarations and loops over all key types;
// it does not represent any key type in itself.
// KT_DEFAULT is used as the default KeyType for KeyParams.
enum KeyType { KT_RSA, KT_ECDSA, KT_LAST, KT_DEFAULT = KT_ECDSA };

static const int kRsaDefaultModSize = 2048;
static const int kRsaDefaultExponent = 0x10001;  // = 2^16+1 = 65537
// TODO(bugs.webrtc.org/364338811): raise the bar to 2048 bits.
static const int kRsaMinModSize = 1024;
static const int kRsaMaxModSize = 8192;

// Certificate default validity lifetime.
static const int kDefaultCertificateLifetimeInSeconds =
    60 * 60 * 24 * 30;  // 30 days
// Certificate validity window.
// This is to compensate for slightly incorrect system clocks.
static const int kCertificateWindowInSeconds = -60 * 60 * 24;

struct RSAParams {
  unsigned int mod_size;
  unsigned int pub_exp;
};

enum ECCurve { EC_NIST_P256, /* EC_FANCY, */ EC_LAST };

class RTC_EXPORT KeyParams {
 public:
  // Generate a KeyParams object from a simple KeyType, using default params.
  explicit KeyParams(KeyType key_type = KT_DEFAULT);

  // Generate a a KeyParams for RSA with explicit parameters.
  static KeyParams RSA(int mod_size = kRsaDefaultModSize,
                       int pub_exp = kRsaDefaultExponent);

  // Generate a a KeyParams for ECDSA specifying the curve.
  static KeyParams ECDSA(ECCurve curve = EC_NIST_P256);

  // Check validity of a KeyParams object. Since the factory functions have
  // no way of returning errors, this function can be called after creation
  // to make sure the parameters are OK.
  bool IsValid() const;

  RSAParams rsa_params() const;

  ECCurve ec_curve() const;

  KeyType type() const { return type_; }

 private:
  KeyType type_;
  union {
    RSAParams rsa;
    ECCurve curve;
  } params_;
};

// Parameters for generating a certificate. If `common_name` is non-empty, it
// will be used for the certificate's subject and issuer name, otherwise a
// random string will be used.
struct SSLIdentityParams {
  std::string common_name;
  time_t not_before;  // Absolute time since epoch in seconds.
  time_t not_after;   // Absolute time since epoch in seconds.
  KeyParams key_params;
};

// Our identity in an SSL negotiation: a keypair and certificate (both
// with the same public key).
// This too is pretty much immutable once created.
class RTC_EXPORT SSLIdentity {
 public:
  // Generates an identity (keypair and self-signed certificate). If
  // `common_name` is non-empty, it will be used for the certificate's subject
  // and issuer name, otherwise a random string will be used. The key type and
  // parameters are defined in `key_param`. The certificate's lifetime in
  // seconds from the current time is defined in `certificate_lifetime`; it
  // should be a non-negative number.
  // Returns null on failure.
  // Caller is responsible for freeing the returned object.
  static std::unique_ptr<SSLIdentity> Create(absl::string_view common_name,
                                             const KeyParams& key_param,
                                             time_t certificate_lifetime);
  static std::unique_ptr<SSLIdentity> Create(absl::string_view common_name,
                                             const KeyParams& key_param);
  static std::unique_ptr<SSLIdentity> Create(absl::string_view common_name,
                                             KeyType key_type);

  // Allows fine-grained control over expiration time.
  static std::unique_ptr<SSLIdentity> CreateForTest(
      const SSLIdentityParams& params);

  // Construct an identity from a private key and a certificate.
  static std::unique_ptr<SSLIdentity> CreateFromPEMStrings(
      absl::string_view private_key,
      absl::string_view certificate);

  // Construct an identity from a private key and a certificate chain.
  static std::unique_ptr<SSLIdentity> CreateFromPEMChainStrings(
      absl::string_view private_key,
      absl::string_view certificate_chain);

  virtual ~SSLIdentity() {}

  // Returns a new SSLIdentity object instance wrapping the same
  // identity information.
  std::unique_ptr<SSLIdentity> Clone() const { return CloneInternal(); }

  // Returns a temporary reference to the end-entity (leaf) certificate.
  virtual const SSLCertificate& certificate() const = 0;
  // Returns a temporary reference to the entire certificate chain.
  virtual const SSLCertChain& cert_chain() const = 0;
  virtual std::string PrivateKeyToPEMString() const = 0;
  virtual std::string PublicKeyToPEMString() const = 0;

  // Helpers for parsing converting between PEM and DER format.
  static bool PemToDer(absl::string_view pem_type,
                       absl::string_view pem_string,
                       std::string* der);
  static std::string DerToPem(absl::string_view pem_type,
                              const unsigned char* data,
                              size_t length);

 protected:
  virtual std::unique_ptr<SSLIdentity> CloneInternal() const = 0;
};

bool operator==(const SSLIdentity& a, const SSLIdentity& b);
bool operator!=(const SSLIdentity& a, const SSLIdentity& b);

// Convert from ASN1 time as restricted by RFC 5280 to seconds from 1970-01-01
// 00.00 ("epoch").  If the ASN1 time cannot be read, return -1.  The data at
// `s` is not 0-terminated; its char count is defined by `length`.
int64_t ASN1TimeToSec(const unsigned char* s, size_t length, bool long_format);

extern const char kPemTypeCertificate[];
extern const char kPemTypeRsaPrivateKey[];
extern const char kPemTypeEcPrivateKey[];

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::ASN1TimeToSec;
using ::webrtc::EC_LAST;
using ::webrtc::EC_NIST_P256;
using ::webrtc::ECCurve;
using ::webrtc::kCertificateWindowInSeconds;
using ::webrtc::kDefaultCertificateLifetimeInSeconds;
using ::webrtc::KeyParams;
using ::webrtc::KeyType;
using ::webrtc::kPemTypeCertificate;
using ::webrtc::kPemTypeEcPrivateKey;
using ::webrtc::kPemTypeRsaPrivateKey;
using ::webrtc::kRsaDefaultExponent;
using ::webrtc::kRsaDefaultModSize;
using ::webrtc::kRsaMaxModSize;
using ::webrtc::kRsaMinModSize;
using ::webrtc::KT_DEFAULT;
using ::webrtc::KT_ECDSA;
using ::webrtc::KT_LAST;
using ::webrtc::KT_RSA;
using ::webrtc::RSAParams;
using ::webrtc::SSLIdentity;
using ::webrtc::SSLIdentityParams;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // RTC_BASE_SSL_IDENTITY_H_

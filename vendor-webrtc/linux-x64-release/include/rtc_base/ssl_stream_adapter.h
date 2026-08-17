/*
 *  Copyright 2004 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_SSL_STREAM_ADAPTER_H_
#define RTC_BASE_SSL_STREAM_ADAPTER_H_

#include <stddef.h>
#include <stdint.h>

#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "absl/functional/any_invocable.h"
#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "api/field_trials_view.h"
#include "rtc_base/buffer.h"
#include "rtc_base/ssl_certificate.h"
#include "rtc_base/ssl_identity.h"
#include "rtc_base/stream.h"

namespace webrtc {

// Constants for SSL profile.
constexpr int kTlsNullWithNullNull = 0;
constexpr int kSslCipherSuiteMaxValue = 0xFFFF;

// Constants for SRTP profiles.
constexpr int kSrtpInvalidCryptoSuite = 0;
constexpr int kSrtpAes128CmSha1_80 = 0x0001;
constexpr int kSrtpAes128CmSha1_32 = 0x0002;
constexpr int kSrtpAeadAes128Gcm = 0x0007;
constexpr int kSrtpAeadAes256Gcm = 0x0008;
constexpr int kSrtpCryptoSuiteMaxValue = 0xFFFF;

// Constants for SSL signature algorithms.
constexpr int kSslSignatureAlgorithmUnknown = 0;
constexpr int kSslSignatureAlgorithmMaxValue = 0xFFFF;

// Names of SRTP profiles listed above.
// 128-bit AES with 80-bit SHA-1 HMAC.
extern const char kCsAesCm128HmacSha1_80[];
// 128-bit AES with 32-bit SHA-1 HMAC.
extern const char kCsAesCm128HmacSha1_32[];
// 128-bit AES GCM with 16 byte AEAD auth tag.
extern const char kCsAeadAes128Gcm[];
// 256-bit AES GCM with 16 byte AEAD auth tag.
extern const char kCsAeadAes256Gcm[];

// Given the DTLS-SRTP protection profile ID, as defined in
// https://tools.ietf.org/html/rfc4568#section-6.2 , return the SRTP profile
// name, as defined in https://tools.ietf.org/html/rfc5764#section-4.1.2.
std::string SrtpCryptoSuiteToName(int crypto_suite);

// Get key length and salt length for given crypto suite. Returns true for
// valid suites, otherwise false.
bool GetSrtpKeyAndSaltLengths(int crypto_suite,
                              int* key_length,
                              int* salt_length);

// Returns true if the given crypto suite id uses a GCM cipher.
bool IsGcmCryptoSuite(int crypto_suite);

// SSLStreamAdapter : A StreamInterfaceAdapter that does SSL/TLS.
// After SSL has been started, the stream will only open on successful
// SSL verification of certificates, and the communication is
// encrypted of course.
//
// This class was written with SSLAdapter as a starting point. It
// offers a similar interface, with two differences: there is no
// support for a restartable SSL connection, and this class has a
// peer-to-peer mode.
//
// The SSL library requires initialization and cleanup. Static method
// for doing this are in SSLAdapter. They should possibly be moved out
// to a neutral class.

enum SSLRole { SSL_CLIENT, SSL_SERVER };
enum SSLMode { SSL_MODE_TLS, SSL_MODE_DTLS };

// TODO bugs.webrtc.org/40644300 remove unused legacy constants.
enum SSLProtocolVersion {
  SSL_PROTOCOL_NOT_GIVEN = -1,
  SSL_PROTOCOL_TLS_10 = 0,  // Deprecated and no longer supported.
  SSL_PROTOCOL_TLS_11 = 1,  // Deprecated and no longer supported.
  SSL_PROTOCOL_TLS_12 = 2,
  SSL_PROTOCOL_TLS_13 = 3,
  SSL_PROTOCOL_DTLS_10 = 1,  // Deprecated and no longer supported.
  SSL_PROTOCOL_DTLS_12 = SSL_PROTOCOL_TLS_12,
  SSL_PROTOCOL_DTLS_13 = SSL_PROTOCOL_TLS_13,
};

// Versions returned from BoringSSL.
const uint16_t kDtls10VersionBytes = 0xfeff;
const uint16_t kDtls12VersionBytes = 0xfefd;
const uint16_t kDtls13VersionBytes = 0xfefc;

enum class SSLPeerCertificateDigestError {
  NONE,
  UNKNOWN_ALGORITHM,
  INVALID_LENGTH,
  VERIFICATION_FAILED,
};

// Errors for Read -- in the high range so no conflict with OpenSSL.
enum { SSE_MSG_TRUNC = 0xff0001 };

// Used to send back UMA histogram value. Logged when Dtls handshake fails.
enum class SSLHandshakeError { UNKNOWN, INCOMPATIBLE_CIPHERSUITE, MAX_VALUE };

class SSLStreamAdapter : public StreamInterface {
 public:
  // Instantiate an SSLStreamAdapter wrapping the given stream,
  // (using the selected implementation for the platform).
  // Caller is responsible for freeing the returned object.
  static std::unique_ptr<SSLStreamAdapter> Create(
      std::unique_ptr<StreamInterface> stream,
      absl::AnyInvocable<void(webrtc::SSLHandshakeError)> handshake_error =
          nullptr,
      const FieldTrialsView* field_trials = nullptr);

  SSLStreamAdapter() = default;
  ~SSLStreamAdapter() override = default;

  // Specify our SSL identity: key and certificate. SSLStream takes ownership
  // of the SSLIdentity object and will free it when appropriate. Should be
  // called no more than once on a given SSLStream instance.
  virtual void SetIdentity(std::unique_ptr<SSLIdentity> identity) = 0;
  virtual SSLIdentity* GetIdentityForTesting() const = 0;

  // Call this to indicate that we are to play the server role (or client role,
  // if the default argument is replaced by SSL_CLIENT).
  // The default argument is for backward compatibility.
  // TODO(ekr@rtfm.com): rename this SetRole to reflect its new function
  virtual void SetServerRole(SSLRole role = SSL_SERVER) = 0;

  [[deprecated("Only DTLS is supported by the stream adapter")]] virtual void
  SetMode(SSLMode mode) = 0;

  // Set maximum supported protocol version. The highest version supported by
  // both ends will be used for the connection, i.e. if one party supports
  // DTLS 1.0 and the other DTLS 1.2, DTLS 1.0 will be used.
  // If requested version is not supported by underlying crypto library, the
  // next lower will be used.
  virtual void SetMaxProtocolVersion(SSLProtocolVersion version) = 0;

  // Set the initial retransmission timeout for DTLS messages. When the timeout
  // expires, the message gets retransmitted and the timeout is exponentially
  // increased.
  // This should only be called before StartSSL().
  virtual void SetInitialRetransmissionTimeout(int timeout_ms) = 0;

  // Set MTU to be used for next handshake flight.
  virtual void SetMTU(int mtu) = 0;

  // StartSSL starts negotiation with a peer, whose certificate is verified
  // using the certificate digest. Generally, SetIdentity() and possibly
  // SetServerRole() should have been called before this.
  // SetPeerCertificateDigest() must also be called. It may be called after
  // StartSSLWithPeer() but must be called before the underlying stream opens.
  //
  // Use of the stream prior to calling StartSSL will pass data in clear text.
  // Calling StartSSL causes SSL negotiation to begin as soon as possible: right
  // away if the underlying wrapped stream is already opened, or else as soon as
  // it opens.
  //
  // StartSSL returns a negative error code on failure. Returning 0 means
  // success so far, but negotiation is probably not complete and will continue
  // asynchronously. In that case, the exposed stream will open after
  // successful negotiation and verification, or an SE_CLOSE event will be
  // raised if negotiation fails.
  virtual int StartSSL() = 0;

  // Specify the digest of the certificate that our peer is expected to use.
  // Only this certificate will be accepted during SSL verification. The
  // certificate is assumed to have been obtained through some other secure
  // channel (such as the signaling channel). This must specify the terminal
  // certificate, not just a CA. SSLStream makes a copy of the digest value.
  //
  // Returns SSLPeerCertificateDigestError::NONE if successful.
  virtual SSLPeerCertificateDigestError SetPeerCertificateDigest(
      absl::string_view digest_alg,
      ArrayView<const uint8_t> digest_val) = 0;
  [[deprecated(
      "Use SetPeerCertificateDigest with ArrayView instead")]] virtual bool
  SetPeerCertificateDigest(absl::string_view digest_alg,
                           const unsigned char* digest_val,
                           size_t digest_len,
                           SSLPeerCertificateDigestError* error = nullptr);

  // Retrieves the peer's certificate chain including leaf certificate, if a
  // connection has been established.
  virtual std::unique_ptr<SSLCertChain> GetPeerSSLCertChain() const = 0;

  // Retrieves the IANA registration id of the cipher suite used for the
  // connection (e.g. 0x2F for "TLS_RSA_WITH_AES_128_CBC_SHA").
  virtual bool GetSslCipherSuite(int* cipher_suite) const = 0;
  // Returns the name of the cipher suite used for the DTLS transport,
  // as defined in the "Description" column of the IANA cipher suite registry.
  virtual std::optional<absl::string_view> GetTlsCipherSuiteName() const = 0;

  // Retrieves the enum value for SSL version.
  // Will return -1 until the version has been negotiated.
  [[deprecated("Use GetSslVersionBytes")]] virtual SSLProtocolVersion
  GetSslVersion() const = 0;
  // Retrieves the 2-byte version from the TLS protocol.
  // Will return false until the version has been negotiated.
  virtual bool GetSslVersionBytes(int* version) const = 0;

  // Key Exporter interface from RFC 5705
  virtual bool ExportSrtpKeyingMaterial(
      ZeroOnFreeBuffer<uint8_t>& keying_material) = 0;

  // Returns the signature algorithm or 0 if not applicable.
  virtual uint16_t GetPeerSignatureAlgorithm() const = 0;

  // DTLS-SRTP interface
  virtual bool SetDtlsSrtpCryptoSuites(
      const std::vector<int>& crypto_suites) = 0;
  virtual bool GetDtlsSrtpCryptoSuite(int* crypto_suite) const = 0;

  // Returns true if a TLS connection has been established.
  // The only difference between this and "GetState() == SE_OPEN" is that if
  // the peer certificate digest hasn't been verified, the state will still be
  // SS_OPENING but IsTlsConnected should return true.
  virtual bool IsTlsConnected() = 0;

  // Capabilities testing.
  // Used to have "DTLS supported", "DTLS-SRTP supported" etc. methods, but now
  // that's assumed.
  static bool IsBoringSsl();

  // Returns true iff the supplied cipher is deemed to be strong.
  // TODO(torbjorng): Consider removing the KeyType argument.
  static bool IsAcceptableCipher(int cipher, KeyType key_type);
  static bool IsAcceptableCipher(absl::string_view cipher, KeyType key_type);

  ////////////////////////////////////////////////////////////////////////////
  // Testing only member functions
  ////////////////////////////////////////////////////////////////////////////

  // Use our timeutils.h source of timing in BoringSSL, allowing us to test
  // using a fake clock.
  static void EnableTimeCallbackForTesting();

  // Return max DTLS SSLProtocolVersion supported by implementation.
  static SSLProtocolVersion GetMaxSupportedDTLSProtocolVersion();

  // Deprecated. Do not use this API outside of testing.
  // Do not set this to false outside of testing.
  void SetClientAuthEnabledForTesting(bool enabled) {
    client_auth_enabled_ = enabled;
  }

  // Deprecated. Do not use this API outside of testing.
  // Returns true by default, else false if explicitly set to disable client
  // authentication.
  bool GetClientAuthEnabled() const { return client_auth_enabled_; }

  // Return number of times DTLS retransmission has been triggered.
  // Used for testing (and maybe put into stats?).
  virtual int GetRetransmissionCount() const = 0;

  // Return the the ID of the group used by the adapters most recently
  // completed handshake, or 0 if not applicable (e.g. before the handshake).
  virtual uint16_t GetSslGroupIdForTesting() const = 0;

 private:
  // If true (default), the client is required to provide a certificate during
  // handshake. If no certificate is given, handshake fails. This applies to
  // server mode only.
  bool client_auth_enabled_ = true;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::GetSrtpKeyAndSaltLengths;
using ::webrtc::IsGcmCryptoSuite;
using ::webrtc::kCsAeadAes128Gcm;
using ::webrtc::kCsAeadAes256Gcm;
using ::webrtc::kCsAesCm128HmacSha1_32;
using ::webrtc::kCsAesCm128HmacSha1_80;
using ::webrtc::kDtls10VersionBytes;
using ::webrtc::kDtls12VersionBytes;
using ::webrtc::kDtls13VersionBytes;
using ::webrtc::kSrtpAeadAes128Gcm;
using ::webrtc::kSrtpAeadAes256Gcm;
using ::webrtc::kSrtpAes128CmSha1_32;
using ::webrtc::kSrtpAes128CmSha1_80;
using ::webrtc::kSrtpCryptoSuiteMaxValue;
using ::webrtc::kSrtpInvalidCryptoSuite;
using ::webrtc::kSslCipherSuiteMaxValue;
using ::webrtc::kSslSignatureAlgorithmMaxValue;
using ::webrtc::kSslSignatureAlgorithmUnknown;
using ::webrtc::kTlsNullWithNullNull;
using ::webrtc::SrtpCryptoSuiteToName;
using ::webrtc::SSE_MSG_TRUNC;
using ::webrtc::SSL_CLIENT;
using ::webrtc::SSL_MODE_DTLS;
using ::webrtc::SSL_MODE_TLS;
using ::webrtc::SSL_PROTOCOL_DTLS_10;
using ::webrtc::SSL_PROTOCOL_DTLS_12;
using ::webrtc::SSL_PROTOCOL_DTLS_13;
using ::webrtc::SSL_PROTOCOL_NOT_GIVEN;
using ::webrtc::SSL_PROTOCOL_TLS_10;
using ::webrtc::SSL_PROTOCOL_TLS_11;
using ::webrtc::SSL_PROTOCOL_TLS_12;
using ::webrtc::SSL_PROTOCOL_TLS_13;
using ::webrtc::SSL_SERVER;
using ::webrtc::SSLHandshakeError;
using ::webrtc::SSLMode;
using ::webrtc::SSLPeerCertificateDigestError;
using ::webrtc::SSLProtocolVersion;
using ::webrtc::SSLRole;
using ::webrtc::SSLStreamAdapter;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // RTC_BASE_SSL_STREAM_ADAPTER_H_

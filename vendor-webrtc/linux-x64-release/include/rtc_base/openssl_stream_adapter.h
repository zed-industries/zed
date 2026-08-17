/*
 *  Copyright 2004 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_OPENSSL_STREAM_ADAPTER_H_
#define RTC_BASE_OPENSSL_STREAM_ADAPTER_H_

#include <openssl/ossl_typ.h>
#include <stddef.h>
#include <stdint.h>

#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "absl/functional/any_invocable.h"
#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "rtc_base/buffer.h"
#include "rtc_base/ssl_certificate.h"
#ifdef OPENSSL_IS_BORINGSSL
#include "rtc_base/boringssl_identity.h"
#include "rtc_base/openssl.h"
#else
#include "rtc_base/openssl_identity.h"
#endif
#include "api/field_trials_view.h"
#include "api/task_queue/pending_task_safety_flag.h"
#include "rtc_base/ssl_identity.h"
#include "rtc_base/ssl_stream_adapter.h"
#include "rtc_base/stream.h"
#include "rtc_base/task_utils/repeating_task.h"
#include "rtc_base/thread.h"

namespace webrtc {

// This class was written with OpenSSLAdapter (a socket adapter) as a
// starting point. It has similar structure and functionality, but uses a
// "peer-to-peer" mode, verifying the peer's certificate using a digest
// sent over a secure signaling channel.
//
// Static methods to initialize and deinit the SSL library are in
// OpenSSLAdapter. These should probably be moved out to a neutral class.
//
// In a few cases I have factored out some OpenSSLAdapter code into static
// methods so it can be reused from this class. Eventually that code should
// probably be moved to a common support class. Unfortunately there remain a
// few duplicated sections of code. I have not done more restructuring because
// I did not want to affect existing code that uses OpenSSLAdapter.
//
// This class does not support the SSL connection restart feature present in
// OpenSSLAdapter. I am not entirely sure how the feature is useful and I am
// not convinced that it works properly.
//
// This implementation is careful to disallow data exchange after an SSL error,
// and it has an explicit SSL_CLOSED state. It should not be possible to send
// any data in clear after one of the StartSSL methods has been called.

// Look in ssl_stream_adapter.h for documentation of the methods.

///////////////////////////////////////////////////////////////////////////////

class OpenSSLStreamAdapter final : public SSLStreamAdapter {
 public:
  OpenSSLStreamAdapter(
      std::unique_ptr<StreamInterface> stream,
      absl::AnyInvocable<void(SSLHandshakeError)> handshake_error,
      const FieldTrialsView* field_trials = nullptr);
  ~OpenSSLStreamAdapter() override;

  void SetIdentity(std::unique_ptr<SSLIdentity> identity) override;
  SSLIdentity* GetIdentityForTesting() const override;

  // Default argument is for compatibility
  void SetServerRole(SSLRole role = webrtc::SSL_SERVER) override;
  SSLPeerCertificateDigestError SetPeerCertificateDigest(
      absl::string_view digest_alg,
      ArrayView<const uint8_t> digest_val) override;

  std::unique_ptr<SSLCertChain> GetPeerSSLCertChain() const override;

  // Goes from state SSL_NONE to either SSL_CONNECTING or SSL_WAIT, depending
  // on whether the underlying stream is already open or not.
  int StartSSL() override;
  [[deprecated]] void SetMode(SSLMode mode) override;
  void SetMaxProtocolVersion(SSLProtocolVersion version) override;
  void SetInitialRetransmissionTimeout(int timeout_ms) override;
  void SetMTU(int mtu) override;

  StreamResult Read(ArrayView<uint8_t> data, size_t& read, int& error) override;
  StreamResult Write(ArrayView<const uint8_t> data,
                     size_t& written,
                     int& error) override;
  void Close() override;
  StreamState GetState() const override;

  std::optional<absl::string_view> GetTlsCipherSuiteName() const override;

  bool GetSslCipherSuite(int* cipher) const override;
  [[deprecated("Use GetSslVersionBytes")]] SSLProtocolVersion GetSslVersion()
      const override;
  bool GetSslVersionBytes(int* version) const override;
  // Key Extractor interface
  bool ExportSrtpKeyingMaterial(
      ZeroOnFreeBuffer<uint8_t>& keying_material) override;

  uint16_t GetPeerSignatureAlgorithm() const override;

  // DTLS-SRTP interface
  bool SetDtlsSrtpCryptoSuites(const std::vector<int>& crypto_suites) override;
  bool GetDtlsSrtpCryptoSuite(int* crypto_suite) const override;

  bool IsTlsConnected() override;

  // Capabilities interfaces.
  static bool IsBoringSsl();

  static bool IsAcceptableCipher(int cipher, KeyType key_type);
  static bool IsAcceptableCipher(absl::string_view cipher, KeyType key_type);

  // Use our timeutils.h source of timing in BoringSSL, allowing us to test
  // using a fake clock.
  static void EnableTimeCallbackForTesting();

  // Return max DTLS SSLProtocolVersion supported by implementation.
  static SSLProtocolVersion GetMaxSupportedDTLSProtocolVersion();

  // Return number of times DTLS retransmission has been triggered.
  // Used for testing (and maybe put into stats?).
  int GetRetransmissionCount() const override { return retransmission_count_; }

  // Return the the ID of the group used by the adapters most recently
  // completed handshake, or 0 if not applicable (e.g. before the handshake).
  uint16_t GetSslGroupIdForTesting() const override;

 private:
  enum SSLState {
    // Before calling one of the StartSSL methods, data flows
    // in clear text.
    SSL_NONE,
    SSL_WAIT,        // waiting for the stream to open to start SSL negotiation
    SSL_CONNECTING,  // SSL negotiation in progress
    SSL_CONNECTED,   // SSL stream successfully established
    SSL_ERROR,       // some SSL error occurred, stream is closed
    SSL_CLOSED       // Clean close
  };

  void OnEvent(int events, int err);

  void PostEvent(int events, int err);
  void SetTimeout(int delay_ms);

  // The following three methods return 0 on success and a negative
  // error code on failure. The error code may be from OpenSSL or -1
  // on some other error cases, so it can't really be interpreted
  // unfortunately.

  // Prepare SSL library, state is SSL_CONNECTING.
  int BeginSSL();
  // Perform SSL negotiation steps.
  int ContinueSSL();

  // Error handler helper. signal is given as true for errors in
  // asynchronous contexts (when an error method was not returned
  // through some other method), and in that case an SE_CLOSE event is
  // raised on the stream with the specified error.
  // A 0 error means a graceful close, otherwise there is not really enough
  // context to interpret the error code.
  // `alert` indicates an alert description (one of the SSL_AD constants) to
  // send to the remote endpoint when closing the association. If 0, a normal
  // shutdown will be performed.
  void Error(absl::string_view context, int err, uint8_t alert, bool signal);
  void Cleanup(uint8_t alert);

  // Flush the input buffers by reading left bytes (for DTLS)
  void FlushInput(unsigned int left);

  // SSL library configuration
  SSL_CTX* SetupSSLContext();
  // Verify the peer certificate matches the signaled digest.
  bool VerifyPeerCertificate();

#ifdef OPENSSL_IS_BORINGSSL
  // SSL certificate verification callback. See SSL_CTX_set_custom_verify.
  static enum ssl_verify_result_t SSLVerifyCallback(SSL* ssl,
                                                    uint8_t* out_alert);
#else
  // SSL certificate verification callback. See
  // SSL_CTX_set_cert_verify_callback.
  static int SSLVerifyCallback(X509_STORE_CTX* store, void* arg);
#endif

  bool WaitingToVerifyPeerCertificate() const {
    return GetClientAuthEnabled() && !peer_certificate_verified_;
  }

  bool HasPeerCertificateDigest() const {
    return !peer_certificate_digest_algorithm_.empty() &&
           !peer_certificate_digest_value_.empty();
  }

  const std::unique_ptr<StreamInterface> stream_;
  absl::AnyInvocable<void(SSLHandshakeError)> handshake_error_;

  Thread* const owner_;
  ScopedTaskSafety task_safety_;
  RepeatingTaskHandle timeout_task_;

  SSLState state_;
  SSLRole role_;
  int ssl_error_code_;  // valid when state_ == SSL_ERROR or SSL_CLOSED
  // Whether the SSL negotiation is blocked on needing to read or
  // write to the wrapped stream.
  bool ssl_read_needs_write_;
  bool ssl_write_needs_read_;

  SSL* ssl_;
  SSL_CTX* ssl_ctx_;

  // Our key and certificate.
#ifdef OPENSSL_IS_BORINGSSL
  std::unique_ptr<BoringSSLIdentity> identity_;
#else
  std::unique_ptr<webrtc::OpenSSLIdentity> identity_;
#endif
  // The certificate chain that the peer presented. Initially null, until the
  // connection is established.
  std::unique_ptr<SSLCertChain> peer_cert_chain_;
  bool peer_certificate_verified_ = false;
  // The digest of the certificate that the peer must present.
  Buffer peer_certificate_digest_value_;
  std::string peer_certificate_digest_algorithm_;

  // The DtlsSrtp ciphers
  std::string srtp_ciphers_;

  // Do DTLS or not
  SSLMode ssl_mode_;

  // Max. allowed protocol version
  SSLProtocolVersion ssl_max_version_;

  // A 50-ms initial timeout ensures rapid setup on fast connections, but may
  // be too aggressive for low bandwidth links.
  int dtls_handshake_timeout_ms_ = 50;

  // MTU configured for dtls.
  int dtls_mtu_ = 1200;

  // 0 == Disabled
  // 1 == Max
  // 2 == Enabled (both min and max)
  const int force_dtls_13_ = 0;

  int retransmission_count_ = 0;

  // Experimental flag to enable Post-Quantum Cryptography TLS.
  const bool enable_dtls_pqc_ = false;
};

/////////////////////////////////////////////////////////////////////////////

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::OpenSSLStreamAdapter;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // RTC_BASE_OPENSSL_STREAM_ADAPTER_H_

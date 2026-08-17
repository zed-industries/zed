/*
 *  Copyright 2004 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_OPENSSL_ADAPTER_H_
#define RTC_BASE_OPENSSL_ADAPTER_H_

#include <openssl/ossl_typ.h>
#include <stddef.h>
#include <stdint.h>

#include <memory>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/task_queue/pending_task_safety_flag.h"
#include "rtc_base/buffer.h"
#include "rtc_base/openssl_stream_adapter.h"
#ifdef OPENSSL_IS_BORINGSSL
#include "rtc_base/boringssl_identity.h"
#else
#include "rtc_base/openssl_identity.h"
#endif
#include "rtc_base/openssl_session_cache.h"
#include "rtc_base/socket.h"
#include "rtc_base/socket_address.h"
#include "rtc_base/ssl_adapter.h"
#include "rtc_base/ssl_certificate.h"
#include "rtc_base/ssl_identity.h"
#include "rtc_base/ssl_stream_adapter.h"

namespace webrtc {

class OpenSSLAdapter final : public SSLAdapter {
 public:
  static bool InitializeSSL();
  static bool CleanupSSL();

  // Creating an OpenSSLAdapter requires a socket to bind to, an optional
  // session cache if you wish to improve performance by caching sessions for
  // hostnames you have previously connected to and an optional
  // SSLCertificateVerifier which can override any existing trusted roots to
  // validate a peer certificate. The cache and verifier are effectively
  // immutable after the the SSL connection starts.
  explicit OpenSSLAdapter(Socket* socket,
                          OpenSSLSessionCache* ssl_session_cache = nullptr,
                          SSLCertificateVerifier* ssl_cert_verifier = nullptr);
  ~OpenSSLAdapter() override;

  void SetIgnoreBadCert(bool ignore) override;
  void SetAlpnProtocols(const std::vector<std::string>& protos) override;
  void SetEllipticCurves(const std::vector<std::string>& curves) override;
  [[deprecated]] void SetMode(SSLMode mode) override;
  void SetCertVerifier(SSLCertificateVerifier* ssl_cert_verifier) override;
  void SetIdentity(std::unique_ptr<SSLIdentity> identity) override;
  void SetRole(SSLRole role) override;
  int StartSSL(absl::string_view hostname) override;
  int Send(const void* pv, size_t cb) override;
  int SendTo(const void* pv, size_t cb, const SocketAddress& addr) override;
  int Recv(void* pv, size_t cb, int64_t* timestamp) override;
  int RecvFrom(void* pv,
               size_t cb,
               SocketAddress* paddr,
               int64_t* timestamp) override;
  int Close() override;
  // Note that the socket returns ST_CONNECTING while SSL is being negotiated.
  ConnState GetState() const override;
  bool IsResumedSession() override;
  // Creates a new SSL_CTX object, configured for client-to-server usage
  // with SSLMode `mode`, and if `enable_cache` is true, with support for
  // storing successful sessions so that they can be later resumed.
  // OpenSSLAdapterFactory will call this method to create its own internal
  // SSL_CTX, and OpenSSLAdapter will also call this when used without a
  // factory.
  static SSL_CTX* CreateContext(SSLMode mode, bool enable_cache);

 protected:
  void OnConnectEvent(Socket* socket) override;
  void OnReadEvent(Socket* socket) override;
  void OnWriteEvent(Socket* socket) override;
  void OnCloseEvent(Socket* socket, int err) override;

 private:
  class EarlyExitCatcher {
   public:
    EarlyExitCatcher(OpenSSLAdapter& adapter_ptr);
    void disable();
    ~EarlyExitCatcher();

   private:
    bool disabled_ = false;
    OpenSSLAdapter& adapter_ptr_;
  };
  enum SSLState {
    SSL_NONE,
    SSL_WAIT,
    SSL_CONNECTING,
    SSL_CONNECTED,
    SSL_ERROR
  };

  int BeginSSL();
  int ContinueSSL();
  void Error(absl::string_view context, int err, bool signal = true);
  void Cleanup();
  void OnTimeout();

  // Return value and arguments have the same meanings as for Send; `error` is
  // an output parameter filled with the result of SSL_get_error.
  int DoSslWrite(const void* pv, size_t cb, int* error);
  bool SSLPostConnectionCheck(SSL* ssl, absl::string_view host);

  // Logs info about the state of the SSL connection.
  static void SSLInfoCallback(const SSL* ssl, int where, int ret);

#if defined(OPENSSL_IS_BORINGSSL) && \
    defined(WEBRTC_EXCLUDE_BUILT_IN_SSL_ROOT_CERTS)
  static enum ssl_verify_result_t SSLVerifyCallback(SSL* ssl,
                                                    uint8_t* out_alert);
  enum ssl_verify_result_t SSLVerifyInternal(SSL* ssl, uint8_t* out_alert);
#else
  static int SSLVerifyCallback(int ok, X509_STORE_CTX* store);
  // Call a custom verifier, if installed.
  // Returns 1 on success, `status_on_error` on error or verification failure.
  int SSLVerifyInternal(int status_on_error, SSL* ssl, X509_STORE_CTX* store);
#endif
  friend class OpenSSLStreamAdapter;  // for custom_verify_callback_;

  // If the SSL_CTX was created with `enable_cache` set to true, this callback
  // will be called when a SSL session has been successfully established,
  // to allow its SSL_SESSION* to be cached for later resumption.
  static int NewSSLSessionCallback(SSL* ssl, SSL_SESSION* session);

  // Optional SSL Shared session cache to improve performance.
  OpenSSLSessionCache* ssl_session_cache_ = nullptr;
  // Optional SSL Certificate verifier which can be set by a third party.
  SSLCertificateVerifier* ssl_cert_verifier_ = nullptr;
  // The current connection state of the (d)TLS connection.
  SSLState state_;

#ifdef OPENSSL_IS_BORINGSSL
  std::unique_ptr<BoringSSLIdentity> identity_;
#else
  std::unique_ptr<webrtc::OpenSSLIdentity> identity_;
#endif
  // Indicates whethere this is a client or a server.
  SSLRole role_;
  bool ssl_read_needs_write_;
  bool ssl_write_needs_read_;
  // This buffer is used if SSL_write fails with SSL_ERROR_WANT_WRITE, which
  // means we need to keep retrying with *the same exact data* until it
  // succeeds. Afterwards it will be cleared.
  Buffer pending_data_;
  SSL* ssl_;
  // Holds the SSL context, which may be shared if an session cache is provided.
  SSL_CTX* ssl_ctx_;
  // Hostname of server that is being connected, used for SNI.
  std::string ssl_host_name_;
  // Set the adapter to DTLS or TLS mode before creating the context.
  SSLMode ssl_mode_;
  // If true, the server certificate need not match the configured hostname.
  bool ignore_bad_cert_;
  // List of protocols to be used in the TLS ALPN extension.
  std::vector<std::string> alpn_protocols_;
  // List of elliptic curves to be used in the TLS elliptic curves extension.
  std::vector<std::string> elliptic_curves_;
  // Holds the result of the call to run of the ssl_cert_verify_->Verify()
  bool custom_cert_verifier_status_;
  // Flag to cancel pending timeout task.
  ScopedTaskSafety timer_;
};

// The OpenSSLAdapterFactory is responsbile for creating multiple new
// OpenSSLAdapters with a shared SSL_CTX and a shared SSL_SESSION cache. The
// SSL_SESSION cache allows existing SSL_SESSIONS to be reused instead of
// recreating them leading to a significant performance improvement.
class OpenSSLAdapterFactory : public SSLAdapterFactory {
 public:
  OpenSSLAdapterFactory();
  ~OpenSSLAdapterFactory() override;
  // Set the SSL Mode to use with this factory. This should only be set before
  // the first adapter is created with the factory. If it is called after it
  // will DCHECK.
  void SetMode(SSLMode mode) override;

  // Set a custom certificate verifier to be passed down to each instance
  // created with this factory. This should only ever be set before the first
  // call to the factory and cannot be changed after the fact.
  void SetCertVerifier(SSLCertificateVerifier* ssl_cert_verifier) override;

  void SetIdentity(std::unique_ptr<SSLIdentity> identity) override;

  // Choose whether the socket acts as a server socket or client socket.
  void SetRole(SSLRole role) override;

  // Methods that control server certificate verification, used in unit tests.
  // Do not call these methods in production code.
  void SetIgnoreBadCert(bool ignore) override;

  // Constructs a new socket using the shared OpenSSLSessionCache. This means
  // existing SSLSessions already in the cache will be reused instead of
  // re-created for improved performance.
  OpenSSLAdapter* CreateAdapter(Socket* socket) override;

 private:
  // Holds the SSLMode (DTLS,TLS) that will be used to set the session cache.
  SSLMode ssl_mode_ = webrtc::SSL_MODE_TLS;
  SSLRole ssl_role_ = webrtc::SSL_CLIENT;
  bool ignore_bad_cert_ = false;

  std::unique_ptr<SSLIdentity> identity_;

  // Holds a cache of existing SSL Sessions.
  std::unique_ptr<OpenSSLSessionCache> ssl_session_cache_;
  // Provides an optional custom callback for verifying SSL certificates, this
  // in currently only used for TURN/TLS connections.
  SSLCertificateVerifier* ssl_cert_verifier_ = nullptr;
  // TODO(benwright): Remove this when context is moved to OpenSSLCommon.
  // Hold a friend class to the OpenSSLAdapter to retrieve the context.
  friend class OpenSSLAdapter;
};

// The EarlyExitCatcher is responsible for calling OpenSSLAdapter::Cleanup on
// destruction. By doing this we have scoped cleanup which can be disabled if
// there were no errors, aka early exits.

std::string TransformAlpnProtocols(const std::vector<std::string>& protos);

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::OpenSSLAdapter;
using ::webrtc::OpenSSLAdapterFactory;
using ::webrtc::TransformAlpnProtocols;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // RTC_BASE_OPENSSL_ADAPTER_H_

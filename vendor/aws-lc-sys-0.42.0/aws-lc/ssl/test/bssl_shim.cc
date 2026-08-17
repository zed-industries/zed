// Copyright (c) 2014, Google Inc.
// SPDX-License-Identifier: ISC

#include <openssl/base.h>

#if !defined(OPENSSL_WINDOWS)
#include <arpa/inet.h>
#include <netinet/in.h>
#include <netinet/tcp.h>
#include <signal.h>
#include <sys/socket.h>
#include <sys/time.h>
#include <unistd.h>
#else
#include <io.h>
OPENSSL_MSVC_PRAGMA(warning(push, 3))
#include <winsock2.h>
#include <ws2tcpip.h>
OPENSSL_MSVC_PRAGMA(warning(pop))
#endif

#include <assert.h>
#include <errno.h>

#ifndef __STDC_FORMAT_MACROS
#define __STDC_FORMAT_MACROS
#endif
#include <inttypes.h>

#include <string.h>
#include <time.h>

#include <openssl/aead.h>
#include <openssl/bio.h>
#include <openssl/bytestring.h>
#include <openssl/cipher.h>
#include <openssl/crypto.h>
#include <openssl/digest.h>
#include <openssl/err.h>
#include <openssl/evp.h>
#include <openssl/hmac.h>
#include <openssl/nid.h>
#include <openssl/rand.h>
#include <openssl/ssl.h>
#include <openssl/x509.h>

#include <functional>
#include <memory>
#include <string>
#include <vector>

#include "../../crypto/internal.h"
#include "../../crypto/ube/vm_ube_detect.h"
#include "../internal.h"
#include "async_bio.h"
#include "handshake_util.h"
#include "mock_quic_transport.h"
#include "packeted_bio.h"
#include "settings_writer.h"
#include "ssl_transfer.h"
#include "test_config.h"
#include "test_state.h"


#if !defined(OPENSSL_WINDOWS)
using Socket = int;
#define INVALID_SOCKET (-1)

static int closesocket(int sock) { return close(sock); }
static void PrintSocketError(const char *func) { perror(func); }
#else
using Socket = SOCKET;

static void PrintSocketError(const char *func) {
  int error = WSAGetLastError();
  char *buffer;
  DWORD len = FormatMessageA(
      FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_ALLOCATE_BUFFER, 0, error, 0,
      reinterpret_cast<char *>(&buffer), 0, nullptr);
  std::string msg = "unknown error";
  if (len > 0) {
    msg.assign(buffer, len);
    while (!msg.empty() && (msg.back() == '\r' || msg.back() == '\n')) {
      msg.resize(msg.size() - 1);
    }
  }
  LocalFree(buffer);
  fprintf(stderr, "%s: %s (%d)\n", func, msg.c_str(), error);
}
#endif

class OwnedSocket {
 public:
  OwnedSocket() = default;
  explicit OwnedSocket(Socket sock) : sock_(sock) {}
  OwnedSocket(OwnedSocket &&other) { *this = std::move(other); }
  ~OwnedSocket() { reset(); }
  OwnedSocket &operator=(OwnedSocket &&other) {
    if (this != &other) {
      drain_on_close_ = other.drain_on_close_;
      reset(other.release());
    }
    return *this;
  }

  bool is_valid() const { return sock_ != INVALID_SOCKET; }
  void set_drain_on_close(bool drain) { drain_on_close_ = drain; }

  void reset(Socket sock = INVALID_SOCKET) {
    if (is_valid()) {
      if (drain_on_close_) {
#if defined(OPENSSL_WINDOWS)
        shutdown(sock_, SD_SEND);
#else
        shutdown(sock_, SHUT_WR);
#endif
        while (true) {
          char buf[1024];
          if (recv(sock_, buf, sizeof(buf), 0) <= 0) {
            break;
          }
        }
      }
      closesocket(sock_);
    }

    drain_on_close_ = false;
    sock_ = sock;
  }

  Socket get() const { return sock_; }

  Socket release() {
    Socket sock = sock_;
    sock_ = INVALID_SOCKET;
    drain_on_close_ = false;
    return sock;
  }

 private:
  Socket sock_ = INVALID_SOCKET;
  bool drain_on_close_ = false;
};

static int Usage(const char *program) {
  fprintf(stderr, "Usage: %s [flags...]\n", program);
  return 1;
}

template <typename T>
struct Free {
  void operator()(T *buf) { free(buf); }
};

// Connect returns a new socket connected to the runner, or -1 on error.
static OwnedSocket Connect(const TestConfig *config) {
  sockaddr_storage addr;
  socklen_t addr_len = 0;
  if (config->ipv6) {
    sockaddr_in6 sin6;
    OPENSSL_memset(&sin6, 0, sizeof(sin6));
    sin6.sin6_family = AF_INET6;
    sin6.sin6_port = htons(config->port);
    if (!inet_pton(AF_INET6, "::1", &sin6.sin6_addr)) {
      PrintSocketError("inet_pton");
      return OwnedSocket();
    }
    addr_len = sizeof(sin6);
    memcpy(&addr, &sin6, addr_len);
  } else {
    sockaddr_in sin;
    OPENSSL_memset(&sin, 0, sizeof(sin));
    sin.sin_family = AF_INET;
    sin.sin_port = htons(config->port);
    if (!inet_pton(AF_INET, "127.0.0.1", &sin.sin_addr)) {
      PrintSocketError("inet_pton");
      return OwnedSocket();
    }
    addr_len = sizeof(sin);
    memcpy(&addr, &sin, addr_len);
  }

  OwnedSocket sock(socket(addr.ss_family, SOCK_STREAM, 0));
  if (!sock.is_valid()) {
    PrintSocketError("socket");
    return OwnedSocket();
  }
  int nodelay = 1;
  if (setsockopt(sock.get(), IPPROTO_TCP, TCP_NODELAY,
                 reinterpret_cast<const char *>(&nodelay),
                 sizeof(nodelay)) != 0) {
    PrintSocketError("setsockopt");
    return OwnedSocket();
  }

  if (connect(sock.get(), reinterpret_cast<const sockaddr *>(&addr),
              addr_len) != 0) {
    PrintSocketError("connect");
    return OwnedSocket();
  }

  return sock;
}

static bool TransferSSL(const TestConfig *config, bssl::UniquePtr<SSL> *in,
                        SSL **ssl) {
  if (!config || !in || !ssl) {
    // No ssl transfer logic is executed.
    return true;
  }
  SSLTransfer sslTransfer;
  if (!sslTransfer.MarkOrReset(config, in)) {
    return false;
  }
  *ssl = in->get();
  return true;
}

// DoRead reads from |ssl|, resolving any asynchronous operations. It returns
// the result value of the final |SSL_read| call.
static int DoRead(bssl::UniquePtr<SSL> *in, uint8_t *out, size_t max_out) {
  SSL *ssl = in->get();
  const TestConfig *config = GetTestConfig(ssl);
  TestState *test_state = GetTestState(ssl);
  if (test_state->quic_transport) {
    return test_state->quic_transport->ReadApplicationData(out, max_out);
  }
  int ret;
  do {
    if (config->async) {
      // The DTLS retransmit logic silently ignores write failures. So the test
      // may progress, allow writes through synchronously. |SSL_read| may
      // trigger a retransmit, so disconnect the write quota.
      AsyncBioEnforceWriteQuota(test_state->async_bio, false);
    }
    ret = CheckIdempotentError("SSL_peek/SSL_read", ssl, [&]() -> int {
      return config->peek_then_read ? SSL_peek(ssl, out, max_out)
                                    : SSL_read(ssl, out, max_out);
    });
    if (config->async) {
      AsyncBioEnforceWriteQuota(test_state->async_bio, true);
    }

    // Run the exporter after each read. This is to test that the exporter fails
    // during a renegotiation.
    if (config->use_exporter_between_reads) {
      uint8_t buf;
      if (!SSL_export_keying_material(ssl, &buf, 1, NULL, 0, NULL, 0, 0)) {
        fprintf(stderr, "failed to export keying material\n");
        return -1;
      }
    }
  } while (RetryAsync(ssl, ret));

  if (!TransferSSL(config, in, &ssl)) {
    return false;
  }

  if (config->peek_then_read && ret > 0) {
    std::unique_ptr<uint8_t[]> buf(new uint8_t[static_cast<size_t>(ret)]);

    // SSL_peek should synchronously return the same data.
    int ret2 = SSL_peek(ssl, buf.get(), ret);
    if (ret2 != ret || OPENSSL_memcmp(buf.get(), out, ret) != 0) {
      fprintf(stderr, "First and second SSL_peek did not match.\n");
      return -1;
    }

    // Do transfer again to test SSL_peek.
    if (!TransferSSL(config, in, &ssl)) {
      return false;
    }

    // SSL_read should synchronously return the same data and consume it.
    ret2 = SSL_read(ssl, buf.get(), ret);
    if (ret2 != ret || OPENSSL_memcmp(buf.get(), out, ret) != 0) {
      fprintf(stderr, "SSL_peek and SSL_read did not match.\n");
      return -1;
    }
  }

  return ret;
}

// WriteAll writes |in_len| bytes from |in| to |ssl|, resolving any asynchronous
// operations. It returns the result of the final |SSL_write| call.
static int WriteAll(SSL *ssl, const void *in_, size_t in_len) {
  TestState *test_state = GetTestState(ssl);
  const uint8_t *in = reinterpret_cast<const uint8_t *>(in_);
  if (test_state->quic_transport) {
    if (!test_state->quic_transport->WriteApplicationData(in, in_len)) {
      return -1;
    }
    return in_len;
  }
  int ret;
  do {
    ret = SSL_write(ssl, in, in_len);
    if (ret > 0) {
      in += ret;
      in_len -= ret;
    }
  } while (RetryAsync(ssl, ret) || (ret > 0 && in_len > 0));
  return ret;
}

// DoShutdown calls |SSL_shutdown|, resolving any asynchronous operations. It
// returns the result of the final |SSL_shutdown| call.
static int DoShutdown(SSL *ssl) {
  int ret;
  do {
    ret = SSL_shutdown(ssl);
  } while (RetryAsync(ssl, ret));
  return ret;
}

// DoSendFatalAlert calls |SSL_send_fatal_alert|, resolving any asynchronous
// operations. It returns the result of the final |SSL_send_fatal_alert| call.
static int DoSendFatalAlert(SSL *ssl, uint8_t alert) {
  int ret;
  do {
    ret = SSL_send_fatal_alert(ssl, alert);
  } while (RetryAsync(ssl, ret));
  return ret;
}

static uint16_t GetProtocolVersion(const SSL *ssl) {
  uint16_t version = SSL_version(ssl);
  if (!SSL_is_dtls(ssl)) {
    return version;
  }
  return 0x0201 + ~version;
}

static bool CheckListContains(const char *type,
                              size_t (*list_func)(const char **, size_t),
                              const char *str) {
  std::vector<const char *> list(list_func(nullptr, 0));
  list_func(list.data(), list.size());
  for (const char *expected : list) {
    if (str == nullptr) {
      break;
    }
    if (strcmp(expected, str) == 0) {
      return true;
    }
  }
  fprintf(stderr, "Unexpected %s: %s\n", type, (str == nullptr) ? "<null>" : str);
  return false;
}

// CheckAuthProperties checks, after the initial handshake is completed or
// after a renegotiation, that authentication-related properties match |config|.
static bool CheckAuthProperties(SSL *ssl, bool is_resume,
                                const TestConfig *config) {
  if (!config->expect_ocsp_response.empty()) {
    const uint8_t *data;
    size_t len;
    SSL_get0_ocsp_response(ssl, &data, &len);
    if (config->expect_ocsp_response.size() != len ||
        OPENSSL_memcmp(config->expect_ocsp_response.data(), data, len) != 0) {
      fprintf(stderr, "OCSP response mismatch\n");
      return false;
    }
  }

  if (!config->expect_signed_cert_timestamps.empty()) {
    const uint8_t *data;
    size_t len;
    SSL_get0_signed_cert_timestamp_list(ssl, &data, &len);
    if (config->expect_signed_cert_timestamps.size() != len ||
        OPENSSL_memcmp(config->expect_signed_cert_timestamps.data(), data,
                       len) != 0) {
      fprintf(stderr, "SCT list mismatch\n");
      return false;
    }
  }

  if (config->expect_verify_result) {
    int expected_verify_result =
        config->verify_fail ? X509_V_ERR_APPLICATION_VERIFICATION : X509_V_OK;

    if (SSL_get_verify_result(ssl) != expected_verify_result) {
      fprintf(stderr, "Wrong certificate verification result\n");
      return false;
    }
  }

  if (!config->expect_peer_cert_file.empty()) {
    bssl::UniquePtr<X509> expect_leaf;
    bssl::UniquePtr<STACK_OF(X509)> expect_chain;
    if (!LoadCertificate(&expect_leaf, &expect_chain,
                         config->expect_peer_cert_file)) {
      return false;
    }

    // For historical reasons, clients report a chain with a leaf and servers
    // without.
    if (!config->is_server) {
      if (!sk_X509_insert(expect_chain.get(), expect_leaf.get(), 0)) {
        return false;
      }
      X509_up_ref(expect_leaf.get());  // sk_X509_insert takes ownership.
    }

    bssl::UniquePtr<X509> leaf(SSL_get_peer_certificate(ssl));
    STACK_OF(X509) *chain = SSL_get_peer_cert_chain(ssl);
    if (X509_cmp(leaf.get(), expect_leaf.get()) != 0) {
      fprintf(stderr, "Received a different leaf certificate than expected.\n");
      return false;
    }

    if (sk_X509_num(chain) != sk_X509_num(expect_chain.get())) {
      fprintf(stderr, "Received a chain of length %zu instead of %zu.\n",
              sk_X509_num(chain), sk_X509_num(expect_chain.get()));
      return false;
    }

    for (size_t i = 0; i < sk_X509_num(chain); i++) {
      if (X509_cmp(sk_X509_value(chain, i),
                   sk_X509_value(expect_chain.get(), i)) != 0) {
        fprintf(stderr, "Chain certificate %zu did not match.\n", i + 1);
        return false;
      }
    }
  }

  if (!!SSL_SESSION_has_peer_sha256(SSL_get_session(ssl)) !=
      config->expect_sha256_client_cert) {
    fprintf(stderr,
            "Unexpected SHA-256 client cert state: expected:%d is_resume:%d.\n",
            config->expect_sha256_client_cert, is_resume);
    return false;
  }

  if (config->expect_sha256_client_cert &&
      SSL_SESSION_get0_peer_certificates(SSL_get_session(ssl)) != nullptr) {
    fprintf(stderr, "Have both client cert and SHA-256 hash: is_resume:%d.\n",
            is_resume);
    return false;
  }

  const uint8_t *peer_sha256;
  size_t peer_sha256_len;
  SSL_SESSION_get0_peer_sha256(SSL_get_session(ssl), &peer_sha256,
                               &peer_sha256_len);
  if (SSL_SESSION_has_peer_sha256(SSL_get_session(ssl))) {
    if (peer_sha256_len != 32) {
      fprintf(stderr, "Peer SHA-256 hash had length %zu instead of 32\n",
              peer_sha256_len);
      return false;
    }
  } else {
    if (peer_sha256_len != 0) {
      fprintf(stderr, "Unexpected peer SHA-256 hash of length %zu\n",
              peer_sha256_len);
      return false;
    }
  }

  return true;
}

// CheckHandshakeProperties checks, immediately after |ssl| completes its
// initial handshake (or False Starts), whether all the properties are
// consistent with the test configuration and invariants.
static bool CheckHandshakeProperties(SSL *ssl, bool is_resume,
                                     const TestConfig *config) {
  if (!CheckAuthProperties(ssl, is_resume, config)) {
    return false;
  }

  if (SSL_get_current_cipher(ssl) == nullptr) {
    fprintf(stderr, "null cipher after handshake\n");
    return false;
  }

  if (config->expect_version != 0 &&
      SSL_version(ssl) != int{config->expect_version}) {
    uint16_t actual_version = static_cast<uint16_t>(SSL_version(ssl));
    fprintf(stderr, "want version %04x, got %04x\n", config->expect_version,
            actual_version);
    return false;
  }

  bool expect_resume =
      is_resume && (!config->expect_session_miss || SSL_in_early_data(ssl));
  if (!!SSL_session_reused(ssl) != expect_resume) {
    fprintf(stderr, "session unexpectedly was%s reused\n",
            SSL_session_reused(ssl) ? "" : " not");
    return false;
  }

  bool expect_handshake_done =
      (is_resume || !config->false_start) && !SSL_in_early_data(ssl);
  if (expect_handshake_done != GetTestState(ssl)->handshake_done) {
    fprintf(stderr, "handshake was%s completed\n",
            GetTestState(ssl)->handshake_done ? "" : " not");
    return false;
  }

  if (expect_handshake_done && !config->is_server) {
    bool expect_new_session =
        !config->expect_no_session &&
        (!SSL_session_reused(ssl) || config->expect_ticket_renewal) &&
        // Session tickets are sent post-handshake in TLS 1.3.
        GetProtocolVersion(ssl) < TLS1_3_VERSION;
    if (expect_new_session != GetTestState(ssl)->got_new_session) {
      fprintf(stderr,
              "new session was%s cached, but we expected the opposite\n",
              GetTestState(ssl)->got_new_session ? "" : " not");
      return false;
    }
  }

  if (!is_resume) {
    if (config->expect_session_id && !GetTestState(ssl)->got_new_session) {
      fprintf(stderr, "session was not cached on the server.\n");
      return false;
    }
    if (config->expect_no_session_id && GetTestState(ssl)->got_new_session) {
      fprintf(stderr, "session was unexpectedly cached on the server.\n");
      return false;
    }
  }

  // early_callback_called is updated in the handshaker, so we don't see it
  // here.
  if (!config->handoff && config->is_server &&
      !GetTestState(ssl)->early_callback_called) {
    fprintf(stderr, "early callback not called\n");
    return false;
  }

  if (!config->expect_server_name.empty()) {
    const char *server_name =
        SSL_get_servername(ssl, TLSEXT_NAMETYPE_host_name);
    if (server_name == nullptr || server_name != config->expect_server_name) {
      fprintf(stderr, "servername mismatch (got %s; want %s)\n", server_name,
              config->expect_server_name.c_str());
      return false;
    }
  }

  if (!config->expect_next_proto.empty()) {
    const uint8_t *next_proto;
    unsigned next_proto_len;
    SSL_get0_next_proto_negotiated(ssl, &next_proto, &next_proto_len);
    if (next_proto_len != config->expect_next_proto.size() ||
        OPENSSL_memcmp(next_proto, config->expect_next_proto.data(),
                       next_proto_len) != 0) {
      fprintf(stderr, "negotiated next proto mismatch\n");
      return false;
    }
  }

  // On the server, the protocol selected in the ALPN callback must be echoed
  // out of |SSL_get0_alpn_selected|. On the client, it should report what the
  // test expected.
  const std::string &expect_alpn =
      config->is_server ? config->select_alpn : config->expect_alpn;
  const uint8_t *alpn_proto;
  unsigned alpn_proto_len;
  SSL_get0_alpn_selected(ssl, &alpn_proto, &alpn_proto_len);
  if (alpn_proto_len != expect_alpn.size() ||
      OPENSSL_memcmp(alpn_proto, expect_alpn.data(), alpn_proto_len) != 0) {
    fprintf(stderr, "negotiated alpn proto mismatch\n");
    return false;
  }

  if (SSL_has_application_settings(ssl) !=
      (config->expect_peer_application_settings ? 1 : 0)) {
    fprintf(stderr,
            "connection %s application settings, but expected the opposite\n",
            SSL_has_application_settings(ssl) ? "has" : "does not have");
    return false;
  }
  std::string expect_settings = config->expect_peer_application_settings
                                    ? *config->expect_peer_application_settings
                                    : "";
  const uint8_t *peer_settings;
  size_t peer_settings_len;
  SSL_get0_peer_application_settings(ssl, &peer_settings, &peer_settings_len);
  if (expect_settings !=
      std::string(reinterpret_cast<const char *>(peer_settings),
                  peer_settings_len)) {
    fprintf(stderr, "peer application settings mismatch\n");
    return false;
  }

  if (!config->expect_quic_transport_params.empty() && expect_handshake_done) {
    const uint8_t *peer_params;
    size_t peer_params_len;
    SSL_get_peer_quic_transport_params(ssl, &peer_params, &peer_params_len);
    if (peer_params_len != config->expect_quic_transport_params.size() ||
        OPENSSL_memcmp(peer_params, config->expect_quic_transport_params.data(),
                       peer_params_len) != 0) {
      fprintf(stderr, "QUIC transport params mismatch\n");
      return false;
    }
  }

  if (!config->expect_channel_id.empty()) {
    uint8_t channel_id[64];
    if (!SSL_get_tls_channel_id(ssl, channel_id, sizeof(channel_id))) {
      fprintf(stderr, "no channel id negotiated\n");
      return false;
    }
    if (config->expect_channel_id.size() != 64 ||
        OPENSSL_memcmp(config->expect_channel_id.data(), channel_id, 64) != 0) {
      fprintf(stderr, "channel id mismatch\n");
      return false;
    }
  }

  if (config->expect_extended_master_secret && !SSL_get_extms_support(ssl)) {
    fprintf(stderr, "No EMS for connection when expected\n");
    return false;
  }

  if (config->expect_secure_renegotiation &&
      !SSL_get_secure_renegotiation_support(ssl)) {
    fprintf(stderr, "No secure renegotiation for connection when expected\n");
    return false;
  }

  if (config->expect_no_secure_renegotiation &&
      SSL_get_secure_renegotiation_support(ssl)) {
    fprintf(stderr,
            "Secure renegotiation unexpectedly negotiated for connection\n");
    return false;
  }

  if (config->expect_peer_signature_algorithm != 0 &&
      config->expect_peer_signature_algorithm !=
          SSL_get_peer_signature_algorithm(ssl)) {
    fprintf(stderr, "Peer signature algorithm was %04x, wanted %04x.\n",
            SSL_get_peer_signature_algorithm(ssl),
            config->expect_peer_signature_algorithm);
    return false;
  }

  if (config->expect_curve_id != 0) {
    uint16_t curve_id = SSL_get_curve_id(ssl);
    if (config->expect_curve_id != curve_id) {
      fprintf(stderr, "curve_id was %04x, wanted %04x\n", curve_id,
              config->expect_curve_id);
      return false;
    }
  }

  uint16_t cipher_id = SSL_CIPHER_get_protocol_id(SSL_get_current_cipher(ssl));
  if (!config->tls13_ciphersuites.empty() &&
      config->expect_cipher != cipher_id) {
    fprintf(stderr, "Cipher ID was %04x, wanted %04x\n", cipher_id,
            config->expect_cipher_aes);
    return false;
  }

  if (config->tls13_ciphersuites.empty() && config->expect_cipher_aes != 0 &&
      EVP_has_aes_hardware() && config->expect_cipher_aes != cipher_id) {
    fprintf(stderr, "Cipher ID was %04x, wanted %04x (has AES hardware)\n",
            cipher_id, config->expect_cipher_aes);
    return false;
  }

  if (config->tls13_ciphersuites.empty() && config->expect_cipher_no_aes != 0 &&
      !EVP_has_aes_hardware() && config->expect_cipher_no_aes != cipher_id) {
    fprintf(stderr, "Cipher ID was %04x, wanted %04x (no AES hardware)\n",
            cipher_id, config->expect_cipher_no_aes);
    return false;
  }

  if (config->expect_cipher != 0 && config->expect_cipher != cipher_id) {
    fprintf(stderr, "Cipher ID was %04x, wanted %04x\n", cipher_id,
            config->expect_cipher);
    return false;
  }

  // The early data status is only applicable after the handshake is confirmed.
  if (!SSL_in_early_data(ssl)) {
    if ((config->expect_accept_early_data && !SSL_early_data_accepted(ssl)) ||
        (config->expect_reject_early_data && SSL_early_data_accepted(ssl))) {
      fprintf(stderr,
              "Early data was%s accepted, but we expected the opposite\n",
              SSL_early_data_accepted(ssl) ? "" : " not");
      return false;
    }

    const char *early_data_reason =
        SSL_early_data_reason_string(SSL_get_early_data_reason(ssl));
    if (!config->expect_early_data_reason.empty() &&
        config->expect_early_data_reason != early_data_reason) {
      fprintf(stderr, "Early data reason was \"%s\", expected \"%s\"\n",
              early_data_reason, config->expect_early_data_reason.c_str());
      return false;
    }
  }

  if (!config->psk.empty()) {
    if (SSL_get_peer_cert_chain(ssl) != nullptr) {
      fprintf(stderr, "Received peer certificate on a PSK cipher.\n");
      return false;
    }
  } else if (!config->is_server || config->require_any_client_certificate) {
    if (SSL_get_peer_cert_chain(ssl) == nullptr) {
      fprintf(stderr, "Received no peer certificate but expected one.\n");
      return false;
    }
  }

  if (is_resume && config->expect_ticket_age_skew != 0 &&
      SSL_get_ticket_age_skew(ssl) != config->expect_ticket_age_skew) {
    fprintf(stderr, "Ticket age skew was %" PRId32 ", wanted %d\n",
            SSL_get_ticket_age_skew(ssl), config->expect_ticket_age_skew);
    return false;
  }

  if (config->expect_delegated_credential_used !=
      !!SSL_delegated_credential_used(ssl)) {
    fprintf(stderr,
            "Got %s delegated credential usage, but wanted opposite. \n",
            SSL_delegated_credential_used(ssl) ? "" : "no");
    return false;
  }

  if ((config->expect_hrr && !SSL_used_hello_retry_request(ssl)) ||
      (config->expect_no_hrr && SSL_used_hello_retry_request(ssl))) {
    fprintf(stderr, "Got %sHRR, but wanted opposite.\n",
            SSL_used_hello_retry_request(ssl) ? "" : "no ");
    return false;
  }

  if (config->expect_ech_accept != !!SSL_ech_accepted(ssl)) {
    fprintf(stderr, "ECH was %saccepted, but wanted opposite.\n",
            SSL_ech_accepted(ssl) ? "" : "not ");
    return false;
  }

  if (config->expect_key_usage_invalid != !!SSL_was_key_usage_invalid(ssl)) {
    fprintf(stderr, "X.509 key usage was %svalid, but wanted opposite.\n",
            SSL_was_key_usage_invalid(ssl) ? "in" : "");
    return false;
  }

  // Check all the selected parameters are covered by the string APIs.
  if (!CheckListContains("version", SSL_get_all_version_names,
                         SSL_get_version(ssl)) ||
      !CheckListContains(
          "cipher", SSL_get_all_standard_cipher_names,
          SSL_CIPHER_standard_name(SSL_get_current_cipher(ssl))) ||
      !CheckListContains("OpenSSL cipher name", SSL_get_all_cipher_names,
                         SSL_CIPHER_get_name(SSL_get_current_cipher(ssl))) ||
      (SSL_get_group_id(ssl) != 0 &&
       !CheckListContains("group", SSL_get_all_group_names,
                          SSL_get_group_name(SSL_get_group_id(ssl)))) ||
      (SSL_get_peer_signature_algorithm(ssl) != 0 &&
       !CheckListContains(
           "sigalg", SSL_get_all_signature_algorithm_names,
           SSL_get_signature_algorithm_name(
               SSL_get_peer_signature_algorithm(ssl), /*include_curve=*/0))) ||
      (SSL_get_peer_signature_algorithm(ssl) != 0 &&
       !CheckListContains(
           "sigalg with curve", SSL_get_all_signature_algorithm_names,
           SSL_get_signature_algorithm_name(
               SSL_get_peer_signature_algorithm(ssl), /*include_curve=*/1)))) {
    return false;
  }

  // Test that handshake hints correctly skipped the expected operations.
  if (config->handshake_hints && !config->allow_hint_mismatch) {
    const TestState *state = GetTestState(ssl);
    // If the private key operation is performed in the first roundtrip, a hint
    // match should have skipped it. This is ECDHE-based cipher suites in TLS
    // 1.2 and non-HRR handshakes in TLS 1.3.
    bool private_key_allowed;
    if (SSL_version(ssl) == TLS1_3_VERSION) {
      private_key_allowed = SSL_used_hello_retry_request(ssl);
    } else {
      private_key_allowed =
          SSL_CIPHER_get_kx_nid(SSL_get_current_cipher(ssl)) == NID_kx_rsa;
    }
    if (!private_key_allowed && state->used_private_key) {
      fprintf(
          stderr,
          "Performed private key operation, but hint should have skipped it\n");
      return false;
    }

    if (state->ticket_decrypt_done) {
      fprintf(stderr,
              "Performed ticket decryption, but hint should have skipped it\n");
      return false;
    }

    // TODO(davidben): Decide what we want to do with TLS 1.2 stateful
    // resumption.
  }
  return true;
}

static bool DoExchange(bssl::UniquePtr<SSL_SESSION> *out_session,
                       bssl::UniquePtr<SSL> *ssl_uniqueptr,
                       const TestConfig *config, bool is_resume, bool is_retry,
                       SettingsWriter *writer);

// DoConnection tests an SSL connection against the peer. On success, it returns
// true and sets |*out_session| to the negotiated SSL session. If the test is a
// resumption attempt, |is_resume| is true and |session| is the session from the
// previous exchange.
static bool DoConnection(bssl::UniquePtr<SSL_SESSION> *out_session,
                         SSL_CTX *ssl_ctx, const TestConfig *config,
                         const TestConfig *retry_config, bool is_resume,
                         SSL_SESSION *session, SettingsWriter *writer) {
  bssl::UniquePtr<SSL> ssl = config->NewSSL(
      ssl_ctx, session, std::unique_ptr<TestState>(new TestState));
  if (!ssl) {
    return false;
  }
  if (config->is_server) {
    SSL_set_accept_state(ssl.get());
  } else {
    SSL_set_connect_state(ssl.get());
  }
  if (config->handshake_hints) {
#if defined(HANDSHAKER_SUPPORTED)
    GetTestState(ssl.get())->get_handshake_hints_cb =
        [&](const SSL_CLIENT_HELLO *client_hello) {
          return GetHandshakeHint(ssl.get(), writer, is_resume, client_hello);
        };
#else
    fprintf(stderr, "The external handshaker can only be used on Linux\n");
    return false;
#endif
  }

  OwnedSocket sock = Connect(config);
  if (!sock.is_valid()) {
    return false;
  }

  // Half-close and drain the socket before releasing it. This seems to be
  // necessary for graceful shutdown on Windows. It will also avoid write
  // failures in the test runner.
  sock.set_drain_on_close(true);

  // Windows uses |SOCKET| for socket types, but OpenSSL's API requires casting
  // them to |int|.
  bssl::UniquePtr<BIO> bio(
      BIO_new_socket(static_cast<int>(sock.get()), BIO_NOCLOSE));
  if (!bio) {
    return false;
  }

  uint8_t shim_id[8];
  CRYPTO_store_u64_le(shim_id, config->shim_id);
  if (!BIO_write_all(bio.get(), shim_id, sizeof(shim_id))) {
    return false;
  }

  if (config->is_dtls) {
    bssl::UniquePtr<BIO> packeted = PacketedBioCreate(GetClock());
    if (!packeted) {
      return false;
    }
    GetTestState(ssl.get())->packeted_bio = packeted.get();
    BIO_push(packeted.get(), bio.release());
    bio = std::move(packeted);
  }
  if (config->async && !config->is_quic) {
    // Note async tests only affect callbacks in QUIC. The IO path does not
    // behave differently when synchronous or asynchronous our QUIC APIs.
    bssl::UniquePtr<BIO> async_scoped =
        config->is_dtls ? AsyncBioCreateDatagram() : AsyncBioCreate();
    if (!async_scoped) {
      return false;
    }
    BIO_push(async_scoped.get(), bio.release());
    GetTestState(ssl.get())->async_bio = async_scoped.get();
    bio = std::move(async_scoped);
  }
  if (config->is_quic) {
    GetTestState(ssl.get())->quic_transport.reset(
        new MockQuicTransport(std::move(bio), ssl.get()));
  } else {
    SSL_set_bio(ssl.get(), bio.get(), bio.get());
    bio.release();  // SSL_set_bio takes ownership.
  }

  bool ret = DoExchange(out_session, &ssl, config, is_resume, false, writer);
  if (!config->is_server && is_resume && config->expect_reject_early_data) {
    // We must have failed due to an early data rejection.
    if (ret) {
      fprintf(stderr, "0-RTT exchange unexpected succeeded.\n");
      return false;
    }
    if (SSL_get_error(ssl.get(), -1) != SSL_ERROR_EARLY_DATA_REJECTED) {
      fprintf(stderr,
              "SSL_get_error did not signal SSL_ERROR_EARLY_DATA_REJECTED.\n");
      return false;
    }

    // Before reseting, early state should still be available.
    if (!SSL_in_early_data(ssl.get()) ||
        !CheckHandshakeProperties(ssl.get(), is_resume, config)) {
      fprintf(stderr, "SSL_in_early_data returned false before reset.\n");
      return false;
    }

    // Client pre- and post-0-RTT reject states are considered logically
    // different connections with different test expections. Check that the test
    // did not mistakenly configure reason expectations on the wrong one.
    if (!config->expect_early_data_reason.empty()) {
      fprintf(stderr,
              "Test error: client reject -expect-early-data-reason flags "
              "should be configured with -on-retry, not -on-resume.\n");
      return false;
    }

    // Reset the connection and try again at 1-RTT.
    SSL_reset_early_data_reject(ssl.get());
    GetTestState(ssl.get())->cert_verified = false;

    // After reseting, the socket should report it is no longer in an early data
    // state.
    if (SSL_in_early_data(ssl.get())) {
      fprintf(stderr, "SSL_in_early_data returned true after reset.\n");
      return false;
    }

    if (!SetTestConfig(ssl.get(), retry_config)) {
      return false;
    }

    assert(!config->handoff);
    config = retry_config;
    ret = DoExchange(out_session, &ssl, retry_config, is_resume, true, writer);
  }

  // An ECH rejection appears as a failed connection. Note |ssl| may use a
  // different config on ECH rejection.
  if (config->expect_no_ech_retry_configs ||
      !config->expect_ech_retry_configs.empty()) {
    bssl::Span<const uint8_t> expected =
        config->expect_no_ech_retry_configs
            ? bssl::Span<const uint8_t>()
            : bssl::MakeConstSpan(reinterpret_cast<const uint8_t *>(
                                      config->expect_ech_retry_configs.data()),
                                  config->expect_ech_retry_configs.size());
    if (ret) {
      fprintf(stderr, "Expected ECH rejection, but connection succeeded.\n");
      return false;
    }
    uint32_t err = ERR_peek_error();
    if (SSL_get_error(ssl.get(), -1) != SSL_ERROR_SSL ||
        ERR_GET_LIB(err) != ERR_LIB_SSL ||
        ERR_GET_REASON(err) != SSL_R_ECH_REJECTED) {
      fprintf(stderr, "Expected ECH rejection, but connection succeeded.\n");
      return false;
    }
    const uint8_t *retry_configs;
    size_t retry_configs_len;
    SSL_get0_ech_retry_configs(ssl.get(), &retry_configs, &retry_configs_len);
    if (bssl::MakeConstSpan(retry_configs, retry_configs_len) != expected) {
      fprintf(stderr, "ECH retry configs did not match expectations.\n");
      // Clear the error queue. Otherwise |SSL_R_ECH_REJECTED| will be printed
      // to stderr and the test framework will think the test had the expected
      // expectations.
      ERR_clear_error();
      return false;
    }
  }

  if (!ret) {
    // Print the |SSL_get_error| code. Otherwise, some failures are silent and
    // hard to debug.
    int ssl_err = SSL_get_error(ssl.get(), -1);
    if (ssl_err != SSL_ERROR_NONE) {
      fprintf(stderr, "SSL error: %s\n", SSL_error_description(ssl_err));
      if (ssl_err == SSL_ERROR_SYSCALL) {
        int err = errno;
        fprintf(stderr, "Error occurred: errno = %d, description = %s\n", err, strerror(err));

      }
    }
    return false;
  }

  if (!GetTestState(ssl.get())->msg_callback_ok) {
    return false;
  }

  if (!config->expect_msg_callback.empty() &&
      GetTestState(ssl.get())->msg_callback_text !=
          config->expect_msg_callback) {
    fprintf(stderr, "Bad message callback trace. Wanted:\n%s\nGot:\n%s\n",
            config->expect_msg_callback.c_str(),
            GetTestState(ssl.get())->msg_callback_text.c_str());
    return false;
  }

  return true;
}

static bool DoExchange(bssl::UniquePtr<SSL_SESSION> *out_session,
                       bssl::UniquePtr<SSL> *ssl_uniqueptr,
                       const TestConfig *config, bool is_resume, bool is_retry,
                       SettingsWriter *writer) {
  int ret;
  SSL *ssl = ssl_uniqueptr->get();
  SSL_CTX *session_ctx = SSL_get_SSL_CTX(ssl);
  TestState *test_state = GetTestState(ssl);
  uint8_t tls_unique_before_transfer[16];
  size_t tls_unique_len_before_transfer = 0;

  if (!config->implicit_handshake) {
    if (config->handoff) {
#if defined(HANDSHAKER_SUPPORTED)
      if (!DoSplitHandshake(ssl_uniqueptr, writer, is_resume)) {
        return false;
      }
      ssl = ssl_uniqueptr->get();
      test_state = GetTestState(ssl);
#else
      fprintf(stderr, "The external handshaker can only be used on Linux\n");
      return false;
#endif
    }

    do {
      ret = CheckIdempotentError("SSL_do_handshake", ssl, [&]() -> int {
        return SSL_do_handshake(ssl);
      });
    } while (RetryAsync(ssl, ret));

    SSLTransfer sslTransfer;
    sslTransfer.MarkTest(config, ssl);
    // Fetch tls_unique_len_before_transfer before SSL transfer.
    if (config->do_ssl_transfer == 1 && sslTransfer.IsSupported(ssl)) {
      if (config->tls_unique) {
        if (!SSL_get_tls_unique(ssl, tls_unique_before_transfer,
                                &tls_unique_len_before_transfer,
                                sizeof(tls_unique_before_transfer))) {
          fprintf(stderr, "failed to get tls-unique before ssl transfer\n");
          return false;
        }
        if (tls_unique_len_before_transfer != 12) {
          fprintf(stderr, "expected 12 bytes of tls-unique but got %u",
                  static_cast<unsigned>(tls_unique_len_before_transfer));
          return false;
        }
      }
    }

    if (!sslTransfer.ResetSSL(config, ssl_uniqueptr)) {
      return false;
    }
    ssl = ssl_uniqueptr->get();

    if (config->forbid_renegotiation_after_handshake) {
      SSL_set_renegotiate_mode(ssl, ssl_renegotiate_never);
    }

    if (ret != 1 || !CheckHandshakeProperties(ssl, is_resume, config)) {
      return false;
    }

    CopySessions(session_ctx, SSL_get_SSL_CTX(ssl));

    if (is_resume && !is_retry && !config->is_server &&
        config->expect_no_offer_early_data && SSL_in_early_data(ssl)) {
      fprintf(stderr, "Client unexpectedly offered early data.\n");
      return false;
    }

    if (config->handshake_twice) {
      do {
        ret = SSL_do_handshake(ssl);
      } while (RetryAsync(ssl, ret));
      if (ret != 1) {
        return false;
      }
    }

    // Skip the |config->async| logic as this should be a no-op.
    if (config->no_op_extra_handshake && SSL_do_handshake(ssl) != 1) {
      fprintf(stderr, "Extra SSL_do_handshake was not a no-op.\n");
      return false;
    }

    if (config->early_write_after_message != 0) {
      if (!SSL_in_early_data(ssl) || config->is_server) {
        fprintf(stderr,
                "-early-write-after-message only works for 0-RTT connections "
                "on servers.\n");
        return false;
      }
      if (!config->shim_writes_first || !config->async) {
        fprintf(stderr,
                "-early-write-after-message requires -shim-writes-first and "
                "-async.\n");
        return false;
      }
      // Run the handshake until the specified message. Note that, if a
      // handshake record contains multiple messages, |SSL_do_handshake| usually
      // processes both atomically. The test must ensure there is a record
      // boundary after the desired message. Checking |last_message_received|
      // confirms this.
      do {
        ret = SSL_do_handshake(ssl);
      } while (test_state->last_message_received !=
                   config->early_write_after_message &&
               RetryAsync(ssl, ret));
      if (ret == 1) {
        fprintf(stderr, "Handshake unexpectedly succeeded.\n");
        return false;
      }
      if (test_state->last_message_received !=
          config->early_write_after_message) {
        // The handshake failed before we saw the target message. The generic
        // error-handling logic in the caller will print the error.
        return false;
      }
    }

    // Reset the state to assert later that the callback isn't called in
    // renegotations.
    test_state->got_new_session = false;
  }

  if (config->export_keying_material > 0) {
    std::vector<uint8_t> result(
        static_cast<size_t>(config->export_keying_material));
    if (!SSL_export_keying_material(
            ssl, result.data(), result.size(), config->export_label.data(),
            config->export_label.size(),
            reinterpret_cast<const uint8_t *>(config->export_context.data()),
            config->export_context.size(), config->use_export_context)) {
      fprintf(stderr, "failed to export keying material\n");
      return false;
    }
    if (WriteAll(ssl, result.data(), result.size()) < 0) {
      return false;
    }
  }

  if (config->export_traffic_secrets) {
    bssl::Span<const uint8_t> read_secret, write_secret;
    if (!SSL_get_traffic_secrets(ssl, &read_secret, &write_secret)) {
      fprintf(stderr, "failed to export traffic secrets\n");
      return false;
    }

    assert(read_secret.size() <= 0xffff);
    assert(write_secret.size() == read_secret.size());
    const uint16_t secret_len = read_secret.size();
    if (WriteAll(ssl, &secret_len, sizeof(secret_len)) < 0 ||
        WriteAll(ssl, read_secret.data(), read_secret.size()) < 0 ||
        WriteAll(ssl, write_secret.data(), write_secret.size()) < 0) {
      return false;
    }
  }

  if (config->tls_unique) {
    uint8_t tls_unique[16];
    size_t tls_unique_len;
    if (!SSL_get_tls_unique(ssl, tls_unique, &tls_unique_len,
                            sizeof(tls_unique))) {
      fprintf(stderr, "failed to get tls-unique\n");
      return false;
    }

    if (tls_unique_len != 12) {
      fprintf(stderr, "expected 12 bytes of tls-unique but got %u",
              static_cast<unsigned>(tls_unique_len));
      return false;
    }

    if (tls_unique_len_before_transfer == 12 &&
        !OPENSSL_memcmp(tls_unique, tls_unique_before_transfer, 12)) {
      fprintf(stderr,
              "tls_unique_before_transfer is different from tls_unique.");
      return false;
    }

    if (WriteAll(ssl, tls_unique, tls_unique_len) < 0) {
      return false;
    }
  }

  if (config->send_alert) {
    if (DoSendFatalAlert(ssl, SSL_AD_DECOMPRESSION_FAILURE) < 0) {
      return false;
    }
    return true;
  }

  if (config->write_different_record_sizes) {
    if (config->is_dtls) {
      fprintf(stderr, "write_different_record_sizes not supported for DTLS\n");
      return false;
    }
    // This mode writes a number of different record sizes in an attempt to
    // trip up the CBC record splitting code.
    static const size_t kBufLen = 32769;
    std::unique_ptr<uint8_t[]> buf(new uint8_t[kBufLen]);
    OPENSSL_memset(buf.get(), 0x42, kBufLen);
    static const size_t kRecordSizes[] = {
        0, 1, 255, 256, 257, 16383, 16384, 16385, 32767, 32768, 32769};
    for (size_t i = 0; i < OPENSSL_ARRAY_SIZE(kRecordSizes); i++) {
      const size_t len = kRecordSizes[i];
      if (len > kBufLen) {
        fprintf(stderr, "Bad kRecordSizes value.\n");
        return false;
      }
      if (WriteAll(ssl, buf.get(), len) < 0) {
        return false;
      }
    }
  } else {
    static const char kInitialWrite[] = "hello";
    bool pending_initial_write = false;
    if (config->read_with_unfinished_write) {
      if (!config->async) {
        fprintf(stderr, "-read-with-unfinished-write requires -async.\n");
        return false;
      }
      if (config->is_quic) {
        fprintf(stderr,
                "-read-with-unfinished-write is incompatible with QUIC.\n");
        return false;
      }

      // Let only one byte of the record through.
      AsyncBioAllowWrite(test_state->async_bio, 1);
      int write_ret = SSL_write(ssl, kInitialWrite, strlen(kInitialWrite));
      if (SSL_get_error(ssl, write_ret) != SSL_ERROR_WANT_WRITE) {
        fprintf(stderr, "Failed to leave unfinished write.\n");
        return false;
      }
      pending_initial_write = true;
    } else if (config->shim_writes_first) {
      if (WriteAll(ssl, kInitialWrite, strlen(kInitialWrite)) < 0) {
        return false;
      }
    }
    if (!config->shim_shuts_down) {
      for (;;) {
        // Read only 512 bytes at a time in TLS to ensure records may be
        // returned in multiple reads.
        size_t read_size = config->is_dtls ? 16384 : 512;
        if (config->read_size > 0) {
          read_size = config->read_size;
        }
        std::unique_ptr<uint8_t[]> buf(new uint8_t[read_size]);
        int n = DoRead(ssl_uniqueptr, buf.get(), read_size);
        ssl = ssl_uniqueptr->get();
        int err = SSL_get_error(ssl, n);
        if (err == SSL_ERROR_ZERO_RETURN ||
            (n == 0 && err == SSL_ERROR_SYSCALL)) {
          if (n != 0) {
            fprintf(stderr, "Invalid SSL_get_error output\n");
            return false;
          }
          // Stop on either clean or unclean shutdown.
          break;
        } else if (err != SSL_ERROR_NONE) {
          if (n > 0) {
            fprintf(stderr, "Invalid SSL_get_error output\n");
            return false;
          }
          return false;
        }
        // Successfully read data.
        if (n <= 0) {
          fprintf(stderr, "Invalid SSL_get_error output\n");
          return false;
        }

        if (!config->is_server && is_resume && !is_retry &&
            config->expect_reject_early_data) {
          fprintf(stderr,
                  "Unexpectedly received data instead of 0-RTT reject.\n");
          return false;
        }

        // After a successful read, with or without False Start, the handshake
        // must be complete unless we are doing early data.
        if (!test_state->handshake_done && !SSL_early_data_accepted(ssl)) {
          fprintf(stderr, "handshake was not completed after SSL_read\n");
          return false;
        }

        // Clear the initial write, if unfinished.
        if (pending_initial_write) {
          if (WriteAll(ssl, kInitialWrite, strlen(kInitialWrite)) < 0) {
            return false;
          }
          pending_initial_write = false;
        }

        if (config->key_update &&
            !SSL_key_update(ssl, SSL_KEY_UPDATE_NOT_REQUESTED)) {
          fprintf(stderr, "SSL_key_update failed.\n");
          return false;
        }

        for (int i = 0; i < n; i++) {
          buf[i] ^= 0xff;
        }
        if (WriteAll(ssl, buf.get(), n) < 0) {
          return false;
        }
      }
    }
  }

  if (config->read_ahead_buffer_size > 0) {
    SSL_set_read_ahead(ssl, 1);
    SSL_set_default_read_buffer_len(ssl, config->read_ahead_buffer_size);
  }

  if (!config->is_server && !config->false_start &&
      !config->implicit_handshake &&
      // Session tickets are sent post-handshake in TLS 1.3.
      GetProtocolVersion(ssl) < TLS1_3_VERSION && test_state->got_new_session) {
    fprintf(stderr, "new session was established after the handshake\n");
    return false;
  }

  if (GetProtocolVersion(ssl) >= TLS1_3_VERSION && !config->is_server) {
    bool expect_new_session =
        !config->expect_no_session && !config->shim_shuts_down;
    if (expect_new_session != test_state->got_new_session) {
      fprintf(stderr,
              "new session was%s cached, but we expected the opposite\n",
              test_state->got_new_session ? "" : " not");
      return false;
    }

    if (expect_new_session) {
      bool got_early_data = test_state->new_session->ticket_max_early_data != 0;
      if (config->expect_ticket_supports_early_data != got_early_data) {
        fprintf(stderr,
                "new session did%s support early data, but we expected the "
                "opposite\n",
                got_early_data ? "" : " not");
        return false;
      }
    }
  }

  if (out_session) {
    *out_session = std::move(test_state->new_session);
  }

  ret = DoShutdown(ssl);

  if (config->shim_shuts_down && config->check_close_notify) {
    // We initiate shutdown, so |SSL_shutdown| will return in two stages. First
    // it returns zero when our close_notify is sent, then one when the peer's
    // is received.
    if (ret != 0) {
      fprintf(stderr, "Unexpected SSL_shutdown result: %d != 0\n", ret);
      return false;
    }
    ret = DoShutdown(ssl);
  }

  if (ret != 1) {
    fprintf(stderr, "Unexpected SSL_shutdown result: %d != 1\n", ret);
    return false;
  }

  if (SSL_total_renegotiations(ssl) > 0) {
    if (!SSL_get_session(ssl)->not_resumable) {
      fprintf(stderr,
              "Renegotiations should never produce resumable sessions.\n");
      return false;
    }

    if (SSL_session_reused(ssl)) {
      fprintf(stderr, "Renegotiations should never resume sessions.\n");
      return false;
    }

    // Re-check authentication properties after a renegotiation. The reported
    // values should remain unchanged even if the server sent different SCT
    // lists.
    if (!CheckAuthProperties(ssl, is_resume, config)) {
      return false;
    }
  }

  if (SSL_total_renegotiations(ssl) != config->expect_total_renegotiations) {
    fprintf(stderr, "Expected %d renegotiations, got %d\n",
            config->expect_total_renegotiations, SSL_total_renegotiations(ssl));
    return false;
  }

  if (config->renegotiate_explicit &&
      SSL_total_renegotiations(ssl) != test_state->explicit_renegotiates) {
    fprintf(stderr, "Performed %d renegotiations, but triggered %d of them\n",
            SSL_total_renegotiations(ssl), test_state->explicit_renegotiates);
    return false;
  }

  if (SSL_clear_num_renegotiations(ssl) !=
          config->expect_total_renegotiations ||
      SSL_total_renegotiations(ssl) != 0) {
    fprintf(stderr, "Expected renegotiation count to be reset to 0, got %d\n",
            SSL_total_renegotiations(ssl));
    return false;
  }

  return true;
}

class StderrDelimiter {
 public:
  ~StderrDelimiter() { fprintf(stderr, "--- DONE ---\n"); }
};

int main(int argc, char **argv) {
#if defined(OPENSSL_LINUX) && defined(AWSLC_VM_UBE_TESTING)
  if (1 != HAZMAT_init_sysgenid_file()) {
    abort();
  }
#endif

  // To distinguish ASan's output from ours, add a trailing message to stderr.
  // Anything following this line will be considered an error.
  StderrDelimiter delimiter;

#if defined(OPENSSL_WINDOWS)
  // Initialize Winsock.
  WORD wsa_version = MAKEWORD(2, 2);
  WSADATA wsa_data;
  int wsa_err = WSAStartup(wsa_version, &wsa_data);
  if (wsa_err != 0) {
    fprintf(stderr, "WSAStartup failed: %d\n", wsa_err);
    return 1;
  }
  if (wsa_data.wVersion != wsa_version) {
    fprintf(stderr, "Didn't get expected version: %x\n", wsa_data.wVersion);
    return 1;
  }
#else
  signal(SIGPIPE, SIG_IGN);
#endif

  CRYPTO_library_init();

  TestConfig initial_config, resume_config, retry_config;
  if (!ParseConfig(argc - 1, argv + 1, /*is_shim=*/true, &initial_config,
                   &resume_config, &retry_config)) {
    return Usage(argv[0]);
  }

  if (initial_config.is_handshaker_supported) {
#if defined(HANDSHAKER_SUPPORTED)
    printf("Yes\n");
#else
    printf("No\n");
#endif
    return 0;
  }

  if (initial_config.wait_for_debugger) {
#if defined(OPENSSL_WINDOWS)
    fprintf(stderr, "-wait-for-debugger is not supported on Windows.\n");
    return 1;
#else
    // The debugger will resume the process.
    raise(SIGSTOP);
#endif
  }

  bssl::UniquePtr<SSL_CTX> ssl_ctx;

  bssl::UniquePtr<SSL_SESSION> session;
  for (int i = 0; i < initial_config.resume_count + 1; i++) {
    bool is_resume = i > 0;
    TestConfig *config = is_resume ? &resume_config : &initial_config;
    ssl_ctx = config->SetupCtx(ssl_ctx.get());
    if (!ssl_ctx) {
      ERR_print_errors_fp(stderr);
      return 1;
    }

    if (is_resume && !initial_config.is_server && !session) {
      fprintf(stderr, "No session to offer.\n");
      return 1;
    }

    bssl::UniquePtr<SSL_SESSION> offer_session = std::move(session);
    SettingsWriter writer;
    if (!writer.Init(i, config, offer_session.get())) {
      fprintf(stderr, "Error writing settings.\n");
      return 1;
    }
    bool ok = DoConnection(&session, ssl_ctx.get(), config, &retry_config,
                           is_resume, offer_session.get(), &writer);
    if (!writer.Commit()) {
      fprintf(stderr, "Error writing settings.\n");
      return 1;
    }
    if (!ok) {
      fprintf(stderr, "Connection %d failed.\n", i + 1);
      ERR_print_errors_fp(stderr);
      return 1;
    }

    if (config->resumption_delay != 0) {
      AdvanceClock(config->resumption_delay);
    }
  }

  return 0;
}

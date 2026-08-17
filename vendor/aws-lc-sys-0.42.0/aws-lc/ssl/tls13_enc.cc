// Copyright (c) 2016, Google Inc.
// SPDX-License-Identifier: ISC

#include <openssl/ssl.h>

#include <assert.h>
#include <string.h>

#include <algorithm>
#include <utility>

#include <openssl/aead.h>
#include <openssl/bytestring.h>
#include <openssl/digest.h>
#include <openssl/hkdf.h>
#include <openssl/hmac.h>
#include <openssl/kdf.h>
#include <openssl/mem.h>

#include "../crypto/internal.h"
#include "internal.h"


BSSL_NAMESPACE_BEGIN

static bool init_key_schedule(SSL_HANDSHAKE *hs, SSLTranscript *transcript,
                              uint16_t version, const SSL_CIPHER *cipher) {
  if (!transcript->InitHash(version, cipher)) {
    return false;
  }

  // Initialize the secret to the zero key.
  hs->ResizeSecrets(transcript->DigestLen());
  OPENSSL_memset(hs->secret().data(), 0, hs->secret().size());

  return true;
}

static bool hkdf_extract_to_secret(SSL_HANDSHAKE *hs,
                                   const SSLTranscript &transcript,
                                   Span<const uint8_t> in) {
  size_t len;
  if (!HKDF_extract(hs->secret().data(), &len, transcript.Digest(), in.data(),
                    in.size(), hs->secret().data(), hs->secret().size())) {
    return false;
  }
  assert(len == hs->secret().size());
  return true;
}

bool tls13_init_key_schedule(SSL_HANDSHAKE *hs, Span<const uint8_t> psk) {
  if (!init_key_schedule(hs, &hs->transcript, ssl_protocol_version(hs->ssl),
                         hs->new_cipher)) {
    return false;
  }

  // Handback includes the whole handshake transcript, so we cannot free the
  // transcript buffer in the handback case.
  if (!hs->handback) {
    hs->transcript.FreeBuffer();
  }
  return hkdf_extract_to_secret(hs, hs->transcript, psk);
}

bool tls13_init_early_key_schedule(SSL_HANDSHAKE *hs,
                                   const SSL_SESSION *session) {
  assert(!hs->ssl->server);
  // When offering ECH, early data is associated with ClientHelloInner, not
  // ClientHelloOuter.
  SSLTranscript *transcript =
      hs->selected_ech_config ? &hs->inner_transcript : &hs->transcript;
  return init_key_schedule(hs, transcript,
                           ssl_session_protocol_version(session),
                           session->cipher) &&
         hkdf_extract_to_secret(
             hs, *transcript,
             MakeConstSpan(session->secret, session->secret_length));
}

static Span<const char> label_to_span(const char *label) {
  return MakeConstSpan(label, strlen(label));
}

static bool hkdf_expand_label(Span<uint8_t> out, const EVP_MD *digest,
                              Span<const uint8_t> secret,
                              Span<const char> label,
                              Span<const uint8_t> hash) {
  return CRYPTO_tls13_hkdf_expand_label(
             out.data(), out.size(), digest, secret.data(), secret.size(),
             reinterpret_cast<const uint8_t *>(label.data()), label.size(),
             hash.data(), hash.size()) == 1;
}

static const char kTLS13LabelDerived[] = "derived";

bool tls13_advance_key_schedule(SSL_HANDSHAKE *hs, Span<const uint8_t> in) {
  uint8_t derive_context[EVP_MAX_MD_SIZE];
  unsigned derive_context_len;
  return EVP_Digest(nullptr, 0, derive_context, &derive_context_len,
                    hs->transcript.Digest(), nullptr) &&
         hkdf_expand_label(hs->secret(), hs->transcript.Digest(), hs->secret(),
                           label_to_span(kTLS13LabelDerived),
                           MakeConstSpan(derive_context, derive_context_len)) &&
         hkdf_extract_to_secret(hs, hs->transcript, in);
}

// derive_secret_with_transcript derives a secret of length |out.size()| and
// writes the result in |out| with the given label, the current base secret, and
// the state of |transcript|. It returns true on success and false on error.
static bool derive_secret_with_transcript(const SSL_HANDSHAKE *hs,
                                          Span<uint8_t> out,
                                          const SSLTranscript &transcript,
                                          Span<const char> label) {
  uint8_t context_hash[EVP_MAX_MD_SIZE];
  size_t context_hash_len;
  if (!transcript.GetHash(context_hash, &context_hash_len)) {
    return false;
  }

  return hkdf_expand_label(out, transcript.Digest(), hs->secret(), label,
                           MakeConstSpan(context_hash, context_hash_len));
}

static bool derive_secret(SSL_HANDSHAKE *hs, Span<uint8_t> out,
                          Span<const char> label) {
  return derive_secret_with_transcript(hs, out, hs->transcript, label);
}

bool tls13_set_traffic_key(SSL *ssl, enum ssl_encryption_level_t level,
                           enum evp_aead_direction_t direction,
                           const SSL_SESSION *session,
                           Span<const uint8_t> traffic_secret) {
  uint16_t version = ssl_session_protocol_version(session);
  UniquePtr<SSLAEADContext> traffic_aead;
  Span<const uint8_t> secret_for_quic;
  if (ssl->quic_method != nullptr) {
    // Install a placeholder SSLAEADContext so that SSL accessors work. The
    // encryption itself will be handled by the SSL_QUIC_METHOD.
    traffic_aead =
        SSLAEADContext::CreatePlaceholderForQUIC(version, session->cipher);
    secret_for_quic = traffic_secret;
  } else {
    // Look up cipher suite properties.
    const EVP_AEAD *aead;
    size_t discard;
    if (!ssl_cipher_get_evp_aead(&aead, &discard, &discard, session->cipher,
                                 version, SSL_is_dtls(ssl))) {
      return false;
    }

    const EVP_MD *digest = ssl_session_get_digest(session);

    // Derive the key.
    size_t key_len = EVP_AEAD_key_length(aead);
    uint8_t key_buf[EVP_AEAD_MAX_KEY_LENGTH];
    auto key = MakeSpan(key_buf, key_len);
    if (!hkdf_expand_label(key, digest, traffic_secret, label_to_span("key"),
                           {})) {
      return false;
    }

    // Derive the IV.
    size_t iv_len = EVP_AEAD_nonce_length(aead);
    uint8_t iv_buf[EVP_AEAD_MAX_NONCE_LENGTH];
    auto iv = MakeSpan(iv_buf, iv_len);
    if (!hkdf_expand_label(iv, digest, traffic_secret, label_to_span("iv"),
                           {})) {
      return false;
    }

    traffic_aead = SSLAEADContext::Create(direction, session->ssl_version,
                                          SSL_is_dtls(ssl), session->cipher,
                                          key, Span<const uint8_t>(), iv);
  }

  if (!traffic_aead) {
    return false;
  }

  if (traffic_secret.size() >
          OPENSSL_ARRAY_SIZE(ssl->s3->read_traffic_secret) ||
      traffic_secret.size() >
          OPENSSL_ARRAY_SIZE(ssl->s3->write_traffic_secret)) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
    return false;
  }

  if (direction == evp_aead_open) {
    if (!ssl->method->set_read_state(ssl, level, std::move(traffic_aead),
                                     secret_for_quic)) {
      return false;
    }
    OPENSSL_memmove(ssl->s3->read_traffic_secret, traffic_secret.data(),
                    traffic_secret.size());
    ssl->s3->read_traffic_secret_len = traffic_secret.size();
  } else {
    if (!ssl->method->set_write_state(ssl, level, std::move(traffic_aead),
                                      secret_for_quic)) {
      return false;
    }
    OPENSSL_memmove(ssl->s3->write_traffic_secret, traffic_secret.data(),
                    traffic_secret.size());
    ssl->s3->write_traffic_secret_len = traffic_secret.size();
  }

  return true;
}


static const char kTLS13LabelExporter[] = "exp master";

static const char kTLS13LabelClientEarlyTraffic[] = "c e traffic";
static const char kTLS13LabelClientHandshakeTraffic[] = "c hs traffic";
static const char kTLS13LabelServerHandshakeTraffic[] = "s hs traffic";
static const char kTLS13LabelClientApplicationTraffic[] = "c ap traffic";
static const char kTLS13LabelServerApplicationTraffic[] = "s ap traffic";

bool tls13_derive_early_secret(SSL_HANDSHAKE *hs) {
  SSL *const ssl = hs->ssl;
  // When offering ECH on the client, early data is associated with
  // ClientHelloInner, not ClientHelloOuter.
  const SSLTranscript &transcript = (!ssl->server && hs->selected_ech_config)
                                        ? hs->inner_transcript
                                        : hs->transcript;
  if (!derive_secret_with_transcript(
          hs, hs->early_traffic_secret(), transcript,
          label_to_span(kTLS13LabelClientEarlyTraffic)) ||
      !ssl_log_secret(ssl, "CLIENT_EARLY_TRAFFIC_SECRET",
                      hs->early_traffic_secret())) {
    return false;
  }
  return true;
}

bool tls13_derive_handshake_secrets(SSL_HANDSHAKE *hs) {
  SSL *const ssl = hs->ssl;
  if (!derive_secret(hs, hs->client_handshake_secret(),
                     label_to_span(kTLS13LabelClientHandshakeTraffic)) ||
      !ssl_log_secret(ssl, "CLIENT_HANDSHAKE_TRAFFIC_SECRET",
                      hs->client_handshake_secret()) ||
      !derive_secret(hs, hs->server_handshake_secret(),
                     label_to_span(kTLS13LabelServerHandshakeTraffic)) ||
      !ssl_log_secret(ssl, "SERVER_HANDSHAKE_TRAFFIC_SECRET",
                      hs->server_handshake_secret())) {
    return false;
  }

  return true;
}

bool tls13_derive_application_secrets(SSL_HANDSHAKE *hs) {
  SSL *const ssl = hs->ssl;
  ssl->s3->exporter_secret_len = hs->transcript.DigestLen();
  if (!derive_secret(hs, hs->client_traffic_secret_0(),
                     label_to_span(kTLS13LabelClientApplicationTraffic)) ||
      !ssl_log_secret(ssl, "CLIENT_TRAFFIC_SECRET_0",
                      hs->client_traffic_secret_0()) ||
      !derive_secret(hs, hs->server_traffic_secret_0(),
                     label_to_span(kTLS13LabelServerApplicationTraffic)) ||
      !ssl_log_secret(ssl, "SERVER_TRAFFIC_SECRET_0",
                      hs->server_traffic_secret_0()) ||
      !derive_secret(
          hs, MakeSpan(ssl->s3->exporter_secret, ssl->s3->exporter_secret_len),
          label_to_span(kTLS13LabelExporter)) ||
      !ssl_log_secret(ssl, "EXPORTER_SECRET",
                      MakeConstSpan(ssl->s3->exporter_secret,
                                    ssl->s3->exporter_secret_len))) {
    return false;
  }

  return true;
}

static const char kTLS13LabelApplicationTraffic[] = "traffic upd";

bool tls13_rotate_traffic_key(SSL *ssl, enum evp_aead_direction_t direction) {
  Span<uint8_t> secret;
  if (direction == evp_aead_open) {
    secret = MakeSpan(ssl->s3->read_traffic_secret,
                      ssl->s3->read_traffic_secret_len);
  } else {
    secret = MakeSpan(ssl->s3->write_traffic_secret,
                      ssl->s3->write_traffic_secret_len);
  }

  const SSL_SESSION *session = SSL_get_session(ssl);
  const EVP_MD *digest = ssl_session_get_digest(session);
  return hkdf_expand_label(secret, digest, secret,
                           label_to_span(kTLS13LabelApplicationTraffic), {}) &&
         tls13_set_traffic_key(ssl, ssl_encryption_application, direction,
                               session, secret);
}

static const char kTLS13LabelResumption[] = "res master";

bool tls13_derive_resumption_secret(SSL_HANDSHAKE *hs) {
  if (hs->transcript.DigestLen() > SSL_MAX_MASTER_KEY_LENGTH) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
    return false;
  }
  hs->new_session->secret_length = hs->transcript.DigestLen();
  return derive_secret(
      hs, MakeSpan(hs->new_session->secret, hs->new_session->secret_length),
      label_to_span(kTLS13LabelResumption));
}

static const char kTLS13LabelFinished[] = "finished";

// tls13_verify_data sets |out| to be the HMAC of |context| using a derived
// Finished key for both Finished messages and the PSK binder. |out| must have
// space available for |EVP_MAX_MD_SIZE| bytes.
static bool tls13_verify_data(uint8_t *out, size_t *out_len,
                              const EVP_MD *digest, uint16_t version,
                              Span<const uint8_t> secret,
                              Span<const uint8_t> context) {
  uint8_t key_buf[EVP_MAX_MD_SIZE];
  auto key = MakeSpan(key_buf, EVP_MD_size(digest));
  unsigned len;
  if (!hkdf_expand_label(key, digest, secret,
                         label_to_span(kTLS13LabelFinished), {}) ||
      HMAC(digest, key.data(), key.size(), context.data(), context.size(), out,
           &len) == nullptr) {
    return false;
  }
  *out_len = len;
  return true;
}

bool tls13_finished_mac(SSL_HANDSHAKE *hs, uint8_t *out, size_t *out_len,
                        bool is_server) {
  Span<const uint8_t> traffic_secret =
      is_server ? hs->server_handshake_secret() : hs->client_handshake_secret();

  uint8_t context_hash[EVP_MAX_MD_SIZE];
  size_t context_hash_len;
  if (!hs->transcript.GetHash(context_hash, &context_hash_len) ||
      !tls13_verify_data(out, out_len, hs->transcript.Digest(),
                         hs->ssl->version, traffic_secret,
                         MakeConstSpan(context_hash, context_hash_len))) {
    return false;
  }
  return true;
}

static const char kTLS13LabelResumptionPSK[] = "resumption";

bool tls13_derive_session_psk(SSL_SESSION *session, Span<const uint8_t> nonce) {
  const EVP_MD *digest = ssl_session_get_digest(session);
  // The session initially stores the resumption_master_secret, which we
  // override with the PSK.
  auto session_secret = MakeSpan(session->secret, session->secret_length);
  return hkdf_expand_label(session_secret, digest, session_secret,
                           label_to_span(kTLS13LabelResumptionPSK), nonce);
}

static const char kTLS13LabelExportKeying[] = "exporter";

bool tls13_export_keying_material(SSL *ssl, Span<uint8_t> out,
                                  Span<const uint8_t> secret,
                                  Span<const char> label,
                                  Span<const uint8_t> context) {
  if (secret.empty()) {
    assert(0);
    OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
    return false;
  }

  const EVP_MD *digest = ssl_session_get_digest(SSL_get_session(ssl));

  uint8_t hash_buf[EVP_MAX_MD_SIZE];
  uint8_t export_context_buf[EVP_MAX_MD_SIZE];
  unsigned hash_len;
  unsigned export_context_len;
  if (!EVP_Digest(context.data(), context.size(), hash_buf, &hash_len, digest,
                  nullptr) ||
      !EVP_Digest(nullptr, 0, export_context_buf, &export_context_len, digest,
                  nullptr)) {
    return false;
  }

  auto hash = MakeConstSpan(hash_buf, hash_len);
  auto export_context = MakeConstSpan(export_context_buf, export_context_len);
  uint8_t derived_secret_buf[EVP_MAX_MD_SIZE];
  auto derived_secret = MakeSpan(derived_secret_buf, EVP_MD_size(digest));
  return hkdf_expand_label(derived_secret, digest, secret, label,
                           export_context) &&
         hkdf_expand_label(out, digest, derived_secret,
                           label_to_span(kTLS13LabelExportKeying), hash);
}

static const char kTLS13LabelPSKBinder[] = "res binder";

static bool tls13_psk_binder(uint8_t *out, size_t *out_len,
                             const SSL_SESSION *session,
                             const SSLTranscript &transcript,
                             Span<const uint8_t> client_hello,
                             size_t binders_len) {
  const EVP_MD *digest = ssl_session_get_digest(session);

  // Compute the binder key.
  //
  // TODO(davidben): Ideally we wouldn't recompute early secret and the binder
  // key each time.
  uint8_t binder_context[EVP_MAX_MD_SIZE];
  unsigned binder_context_len;
  uint8_t early_secret[EVP_MAX_MD_SIZE] = {0};
  size_t early_secret_len;
  uint8_t binder_key_buf[EVP_MAX_MD_SIZE] = {0};
  auto binder_key = MakeSpan(binder_key_buf, EVP_MD_size(digest));
  if (!EVP_Digest(nullptr, 0, binder_context, &binder_context_len, digest,
                  nullptr) ||
      !HKDF_extract(early_secret, &early_secret_len, digest, session->secret,
                    session->secret_length, nullptr, 0) ||
      !hkdf_expand_label(binder_key, digest,
                         MakeConstSpan(early_secret, early_secret_len),
                         label_to_span(kTLS13LabelPSKBinder),
                         MakeConstSpan(binder_context, binder_context_len))) {
    return false;
  }

  // Hash the transcript and truncated ClientHello.
  if (client_hello.size() < binders_len) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
    return false;
  }
  auto truncated = client_hello.subspan(0, client_hello.size() - binders_len);
  uint8_t context[EVP_MAX_MD_SIZE];
  unsigned context_len;
  ScopedEVP_MD_CTX ctx;
  if (!transcript.CopyToHashContext(ctx.get(), digest) ||
      !EVP_DigestUpdate(ctx.get(), truncated.data(),
                        truncated.size()) ||
      !EVP_DigestFinal_ex(ctx.get(), context, &context_len)) {
    return false;
  }

  if (!tls13_verify_data(out, out_len, digest, session->ssl_version, binder_key,
                         MakeConstSpan(context, context_len))) {
    return false;
  }

  assert(*out_len == EVP_MD_size(digest));
  return true;
}

bool tls13_write_psk_binder(const SSL_HANDSHAKE *hs,
                            const SSLTranscript &transcript, Span<uint8_t> msg,
                            size_t *out_binder_len) {
  const SSL *const ssl = hs->ssl;
  const EVP_MD *digest = ssl_session_get_digest(ssl->session.get());
  const size_t hash_len = EVP_MD_size(digest);
  // We only offer one PSK, so the binders are a u16 and u8 length
  // prefix, followed by the binder. The caller is assumed to have constructed
  // |msg| with placeholder binders.
  const size_t binders_len = 3 + hash_len;
  uint8_t verify_data[EVP_MAX_MD_SIZE];
  size_t verify_data_len;
  if (!tls13_psk_binder(verify_data, &verify_data_len, ssl->session.get(),
                        transcript, msg, binders_len) ||
      verify_data_len != hash_len) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
    return false;
  }

  auto msg_binder = msg.last(verify_data_len);
  OPENSSL_memcpy(msg_binder.data(), verify_data, verify_data_len);
  if (out_binder_len != nullptr) {
    *out_binder_len = verify_data_len;
  }
  return true;
}

bool tls13_verify_psk_binder(const SSL_HANDSHAKE *hs,
                             const SSL_SESSION *session, const SSLMessage &msg,
                             CBS *binders) {
  uint8_t verify_data[EVP_MAX_MD_SIZE];
  size_t verify_data_len;
  CBS binder;
  // The binders are computed over |msg| with |binders| and its u16 length
  // prefix removed. The caller is assumed to have parsed |msg|, extracted
  // |binders|, and verified the PSK extension is last.
  if (!tls13_psk_binder(verify_data, &verify_data_len, session, hs->transcript,
                        msg.raw, 2 + CBS_len(binders)) ||
      // We only consider the first PSK, so compare against the first binder.
      !CBS_get_u8_length_prefixed(binders, &binder)) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
    return false;
  }

  bool binder_ok =
      CBS_len(&binder) == verify_data_len &&
      CRYPTO_memcmp(CBS_data(&binder), verify_data, verify_data_len) == 0;
#if defined(BORINGSSL_UNSAFE_FUZZER_MODE)
  binder_ok = true;
#endif
  if (!binder_ok) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_DIGEST_CHECK_FAILED);
    return false;
  }

  return true;
}

size_t ssl_ech_confirmation_signal_hello_offset(const SSL *ssl) {
  static_assert(ECH_CONFIRMATION_SIGNAL_LEN < SSL3_RANDOM_SIZE,
                "the confirmation signal is a suffix of the random");
  const size_t header_len =
      SSL_is_dtls(ssl) ? DTLS1_HM_HEADER_LENGTH : SSL3_HM_HEADER_LENGTH;
  return header_len + 2 /* version */ + SSL3_RANDOM_SIZE -
         ECH_CONFIRMATION_SIGNAL_LEN;
}

bool ssl_ech_accept_confirmation(const SSL_HANDSHAKE *hs, Span<uint8_t> out,
                                 Span<const uint8_t> client_random,
                                 const SSLTranscript &transcript, bool is_hrr,
                                 Span<const uint8_t> msg, size_t offset) {
  // See draft-ietf-tls-esni-13, sections 7.2 and 7.2.1.
  static const uint8_t kZeros[EVP_MAX_MD_SIZE] = {0};

  // We hash |msg|, with bytes from |offset| zeroed.
  if (msg.size() < offset + ECH_CONFIRMATION_SIGNAL_LEN) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
    return false;
  }

  auto before_zeros = msg.subspan(0, offset);
  auto after_zeros = msg.subspan(offset + ECH_CONFIRMATION_SIGNAL_LEN);
  uint8_t context[EVP_MAX_MD_SIZE];
  unsigned context_len;
  ScopedEVP_MD_CTX ctx;
  if (!transcript.CopyToHashContext(ctx.get(), transcript.Digest()) ||
      !EVP_DigestUpdate(ctx.get(), before_zeros.data(), before_zeros.size()) ||
      !EVP_DigestUpdate(ctx.get(), kZeros, ECH_CONFIRMATION_SIGNAL_LEN) ||
      !EVP_DigestUpdate(ctx.get(), after_zeros.data(), after_zeros.size()) ||
      !EVP_DigestFinal_ex(ctx.get(), context, &context_len)) {
    return false;
  }

  uint8_t secret[EVP_MAX_MD_SIZE];
  size_t secret_len;
  if (!HKDF_extract(secret, &secret_len, transcript.Digest(),
                    client_random.data(), client_random.size(), kZeros,
                    transcript.DigestLen())) {
    return false;
  }

  assert(out.size() == ECH_CONFIRMATION_SIGNAL_LEN);
  return hkdf_expand_label(out, transcript.Digest(),
                           MakeConstSpan(secret, secret_len),
                           is_hrr ? label_to_span("hrr ech accept confirmation")
                                  : label_to_span("ech accept confirmation"),
                           MakeConstSpan(context, context_len));
}

BSSL_NAMESPACE_END

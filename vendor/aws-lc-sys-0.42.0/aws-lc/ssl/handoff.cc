// Copyright (c) 2018, Google Inc.
// SPDX-License-Identifier: ISC

#include <openssl/ssl.h>

#include <openssl/bytestring.h>
#include <openssl/err.h>

#include "internal.h"


BSSL_NAMESPACE_BEGIN

constexpr int kHandoffVersion = 0;
constexpr int kHandbackVersion = 0;

static const CBS_ASN1_TAG kHandoffTagALPS = CBS_ASN1_CONTEXT_SPECIFIC | 0;

// early_data_t represents the state of early data in a more compact way than
// the 3 bits used by the implementation.
enum early_data_t {
  early_data_not_offered = 0,
  early_data_accepted = 1,
  early_data_rejected_hrr = 2,
  early_data_skipped = 3,

  early_data_max_value = early_data_skipped,
};

// serialize_features adds a description of features supported by this binary to
// |out|.  Returns true on success and false on error.
static bool serialize_features(CBB *out) {
  CBB ciphers;
  if (!CBB_add_asn1(out, &ciphers, CBS_ASN1_OCTETSTRING)) {
    return false;
  }
  Span<const SSL_CIPHER> all_ciphers = AllCiphers();
  for (const SSL_CIPHER& cipher : all_ciphers) {
    if (!CBB_add_u16(&ciphers, static_cast<uint16_t>(cipher.id))) {
      return false;
    }
  }
  CBB groups;
  if (!CBB_add_asn1(out, &groups, CBS_ASN1_OCTETSTRING)) {
    return false;
  }
  for (const NamedGroup& g : NamedGroups()) {
    if (!CBB_add_u16(&groups, g.group_id)) {
      return false;
    }
  }
  // ALPS is a draft protocol and may change over time. The handoff structure
  // contains a [0] IMPLICIT OCTET STRING OPTIONAL, containing a list of u16
  // ALPS versions that the binary supports. For now we name them by codepoint.
  // Once ALPS is finalized and past the support horizon, this field can be
  // removed.
  CBB alps;
  if (!CBB_add_asn1(out, &alps, kHandoffTagALPS) ||
      !CBB_add_u16(&alps, TLSEXT_TYPE_application_settings_old) ||
      !CBB_add_u16(&alps, TLSEXT_TYPE_application_settings)) {
    return false;
  }
  return CBB_flush(out);
}

bool SSL_serialize_handoff(const SSL *ssl, CBB *out,
                           SSL_CLIENT_HELLO *out_hello) {
  const SSL3_STATE *const s3 = ssl->s3;
  if (!ssl->server ||
      s3->hs == nullptr ||
      s3->rwstate != SSL_ERROR_HANDOFF) {
    return false;
  }

  CBB seq;
  SSLMessage msg;
  Span<const uint8_t> transcript = s3->hs->transcript.buffer();

  if (!CBB_add_asn1(out, &seq, CBS_ASN1_SEQUENCE) ||
      !CBB_add_asn1_uint64(&seq, kHandoffVersion) ||
      !CBB_add_asn1_octet_string(&seq, transcript.data(), transcript.size()) ||
      !CBB_add_asn1_octet_string(&seq,
                                 reinterpret_cast<uint8_t *>(s3->hs_buf->data),
                                 s3->hs_buf->length) ||
      !serialize_features(&seq) ||
      !CBB_flush(out) ||
      !ssl->method->get_message(ssl, &msg) ||
      !ssl_client_hello_init(ssl, out_hello, msg.body)) {
    return false;
  }

  return true;
}

bool SSL_decline_handoff(SSL *ssl) {
  const SSL3_STATE *const s3 = ssl->s3;
  if (!ssl->server ||
      s3->hs == nullptr ||
      s3->rwstate != SSL_ERROR_HANDOFF) {
    return false;
  }

  s3->hs->config->handoff = false;
  return true;
}

// apply_remote_features reads a list of supported features from |in| and
// (possibly) reconfigures |ssl| to disallow the negotation of features whose
// support has not been indicated.  (This prevents the the handshake from
// committing to features that are not supported on the handoff/handback side.)
static bool apply_remote_features(SSL *ssl, CBS *in) {
  CBS ciphers;
  if (!CBS_get_asn1(in, &ciphers, CBS_ASN1_OCTETSTRING)) {
    return false;
  }
  bssl::UniquePtr<STACK_OF(SSL_CIPHER)> supported(sk_SSL_CIPHER_new_null());
  if (!supported) {
    return false;
  }
  while (CBS_len(&ciphers)) {
    uint16_t id;
    if (!CBS_get_u16(&ciphers, &id)) {
      return false;
    }
    const SSL_CIPHER *cipher = SSL_get_cipher_by_value(id);
    if (!cipher) {
      continue;
    }
    if (!sk_SSL_CIPHER_push(supported.get(), cipher)) {
      return false;
    }
  }
  STACK_OF(SSL_CIPHER) *configured =
      ssl->config->cipher_list ? ssl->config->cipher_list->ciphers.get()
                               : ssl->ctx->cipher_list->ciphers.get();
  bssl::UniquePtr<STACK_OF(SSL_CIPHER)> unsupported(sk_SSL_CIPHER_new_null());
  if (!unsupported) {
    return false;
  }
  for (const SSL_CIPHER *configured_cipher : configured) {
    if (sk_SSL_CIPHER_find_awslc(supported.get(), nullptr, configured_cipher)) {
      continue;
    }
    if (!sk_SSL_CIPHER_push(unsupported.get(), configured_cipher)) {
      return false;
    }
  }
  if (sk_SSL_CIPHER_num(unsupported.get()) && !ssl->config->cipher_list) {
    ssl->config->cipher_list = bssl::MakeUnique<SSLCipherPreferenceList>();
    if (!ssl->config->cipher_list ||
        !ssl->config->cipher_list->Init(*ssl->ctx->cipher_list)) {
      return false;
    }
  }
  for (const SSL_CIPHER *unsupported_cipher : unsupported.get()) {
    ssl->config->cipher_list->Remove(unsupported_cipher);
  }
  if (sk_SSL_CIPHER_num(SSL_get_ciphers(ssl)) == 0) {
    return false;
  }

  CBS groups;
  if (!CBS_get_asn1(in, &groups, CBS_ASN1_OCTETSTRING)) {
    return false;
  }
  Array<uint16_t> supported_groups;
  if (!supported_groups.Init(CBS_len(&groups) / 2)) {
    return false;
  }
  size_t idx = 0;
  while (CBS_len(&groups)) {
    uint16_t group;
    if (!CBS_get_u16(&groups, &group)) {
      return false;
    }
    supported_groups[idx++] = group;
  }
  Span<const uint16_t> configured_groups =
      tls1_get_grouplist(ssl->s3->hs.get());
  Array<uint16_t> new_configured_groups;
  if (!new_configured_groups.Init(configured_groups.size())) {
    return false;
  }
  idx = 0;
  for (uint16_t configured_group : configured_groups) {
    bool ok = false;
    for (uint16_t supported_group : supported_groups) {
      if (supported_group == configured_group) {
        ok = true;
        break;
      }
    }
    if (ok) {
      new_configured_groups[idx++] = configured_group;
    }
  }
  if (idx == 0) {
    return false;
  }
  new_configured_groups.Shrink(idx);
  ssl->config->supported_group_list = std::move(new_configured_groups);

  CBS alps;
  CBS_init(&alps, nullptr, 0);
  if (!CBS_get_optional_asn1(in, &alps, /*out_present=*/nullptr,
                             kHandoffTagALPS)) {
    return false;
  }
  bool supports_alps = false;
  while (CBS_len(&alps) != 0) {
    uint16_t id;
    if (!CBS_get_u16(&alps, &id)) {
      return false;
    }
    // For now, we support two ALPS codepoints, so we need to extract both
    // codepoints, and then filter what the handshaker might try to send.
    if ((id == TLSEXT_TYPE_application_settings &&
         ssl->config->alps_use_new_codepoint) ||
        (id == TLSEXT_TYPE_application_settings_old &&
         !ssl->config->alps_use_new_codepoint)) {
      supports_alps = true;
      break;
    }
  }
  if (!supports_alps) {
    ssl->config->alps_configs.clear();
  }

  return true;
}

// uses_disallowed_feature returns true iff |ssl| enables a feature that
// disqualifies it for split handshakes.
static bool uses_disallowed_feature(const SSL *ssl) {
  return ssl->method->is_dtls || (ssl->config->cert && ssl->config->cert->dc) ||
         ssl->config->quic_transport_params.size() > 0 || ssl->ctx->ech_keys;
}

bool SSL_apply_handoff(SSL *ssl, Span<const uint8_t> handoff) {
  if (uses_disallowed_feature(ssl)) {
    return false;
  }

  CBS seq, handoff_cbs(handoff);
  uint64_t handoff_version;
  if (!CBS_get_asn1(&handoff_cbs, &seq, CBS_ASN1_SEQUENCE) ||
      !CBS_get_asn1_uint64(&seq, &handoff_version) ||
      handoff_version != kHandoffVersion) {
    return false;
  }

  CBS transcript, hs_buf;
  if (!CBS_get_asn1(&seq, &transcript, CBS_ASN1_OCTETSTRING) ||
      !CBS_get_asn1(&seq, &hs_buf, CBS_ASN1_OCTETSTRING) ||
      !apply_remote_features(ssl, &seq)) {
    return false;
  }

  SSL_set_accept_state(ssl);

  SSL3_STATE *const s3 = ssl->s3;
  s3->v2_hello_done = true;
  s3->has_message = true;

  s3->hs_buf.reset(BUF_MEM_new());
  if (!s3->hs_buf ||
      !BUF_MEM_append(s3->hs_buf.get(), CBS_data(&hs_buf), CBS_len(&hs_buf))) {
    return false;
  }

  if (CBS_len(&transcript) != 0) {
    s3->hs->transcript.Update(transcript);
    s3->is_v2_hello = true;
  }
  s3->hs->handback = true;

  return true;
}

bool SSL_serialize_handback(const SSL *ssl, CBB *out) {
  if (!ssl->server || uses_disallowed_feature(ssl)) {
    return false;
  }
  const SSL3_STATE *const s3 = ssl->s3;
  SSL_HANDSHAKE *const hs = s3->hs.get();
  handback_t type;
  switch (hs->state) {
    case state12_read_change_cipher_spec:
      type = handback_after_session_resumption;
      break;
    case state12_read_client_certificate:
      type = handback_after_ecdhe;
      break;
    case state12_finish_server_handshake:
      type = handback_after_handshake;
      break;
    case state12_tls13:
      if (hs->tls13_state != state13_send_half_rtt_ticket) {
        return false;
      }
      type = handback_tls13;
      break;
    default:
      return false;
  }

  size_t hostname_len = 0;
  if (s3->hostname) {
    hostname_len = strlen(s3->hostname.get());
  }

  Span<const uint8_t> transcript;
  if (type != handback_after_handshake) {
    transcript = s3->hs->transcript.buffer();
  }
  size_t write_iv_len = 0;
  const uint8_t *write_iv = nullptr;
  if ((type == handback_after_session_resumption ||
       type == handback_after_handshake) &&
      ssl->version == TLS1_VERSION &&
      SSL_CIPHER_is_block_cipher(s3->aead_write_ctx->cipher()) &&
      !s3->aead_write_ctx->GetIV(&write_iv, &write_iv_len)) {
    return false;
  }
  size_t read_iv_len = 0;
  const uint8_t *read_iv = nullptr;
  if (type == handback_after_handshake &&
      ssl->version == TLS1_VERSION &&
      SSL_CIPHER_is_block_cipher(s3->aead_read_ctx->cipher()) &&
      !s3->aead_read_ctx->GetIV(&read_iv, &read_iv_len)) {
      return false;
  }

  // TODO(mab): make sure everything is serialized.
  CBB seq, key_share;
  const SSL_SESSION *session;
  if (type == handback_tls13) {
    session = hs->new_session.get();
  } else {
    session = s3->session_reused ? ssl->session.get() : hs->new_session.get();
  }
  static const uint8_t kUnusedChannelID[64] = {0};
  if (!CBB_add_asn1(out, &seq, CBS_ASN1_SEQUENCE) ||
      !CBB_add_asn1_uint64(&seq, kHandbackVersion) ||
      !CBB_add_asn1_uint64(&seq, type) ||
      !CBB_add_asn1_octet_string(&seq, s3->read_sequence,
                                 sizeof(s3->read_sequence)) ||
      !CBB_add_asn1_octet_string(&seq, s3->write_sequence,
                                 sizeof(s3->write_sequence)) ||
      !CBB_add_asn1_octet_string(&seq, s3->server_random,
                                 sizeof(s3->server_random)) ||
      !CBB_add_asn1_octet_string(&seq, s3->client_random,
                                 sizeof(s3->client_random)) ||
      !CBB_add_asn1_octet_string(&seq, read_iv, read_iv_len) ||
      !CBB_add_asn1_octet_string(&seq, write_iv, write_iv_len) ||
      !CBB_add_asn1_bool(&seq, s3->session_reused) ||
      !CBB_add_asn1_bool(&seq, hs->channel_id_negotiated) ||
      !ssl_session_serialize(session, &seq) ||
      !CBB_add_asn1_octet_string(&seq, s3->next_proto_negotiated.data(),
                                 s3->next_proto_negotiated.size()) ||
      !CBB_add_asn1_octet_string(&seq, s3->alpn_selected.data(),
                                 s3->alpn_selected.size()) ||
      !CBB_add_asn1_octet_string(
          &seq, reinterpret_cast<uint8_t *>(s3->hostname.get()),
          hostname_len) ||
      !CBB_add_asn1_octet_string(&seq, kUnusedChannelID,
                                 sizeof(kUnusedChannelID)) ||
      // These two fields were historically |token_binding_negotiated| and
      // |negotiated_token_binding_param|.
      !CBB_add_asn1_bool(&seq, 0) ||
      !CBB_add_asn1_uint64(&seq, 0) ||
      !CBB_add_asn1_bool(&seq, s3->hs->next_proto_neg_seen) ||
      !CBB_add_asn1_bool(&seq, s3->hs->cert_request) ||
      !CBB_add_asn1_bool(&seq, s3->hs->extended_master_secret) ||
      !CBB_add_asn1_bool(&seq, s3->hs->ticket_expected) ||
      !CBB_add_asn1_uint64(&seq, SSL_CIPHER_get_id(s3->hs->new_cipher)) ||
      !CBB_add_asn1_octet_string(&seq, transcript.data(), transcript.size()) ||
      !CBB_add_asn1(&seq, &key_share, CBS_ASN1_SEQUENCE)) {
    return false;
  }
  if (type == handback_after_ecdhe) {
    CBB private_key;
    if (!CBB_add_asn1_uint64(&key_share, s3->hs->key_shares[0]->GroupID()) ||
        !CBB_add_asn1(&key_share, &private_key, CBS_ASN1_OCTETSTRING) ||
        !s3->hs->key_shares[0]->SerializePrivateKey(&private_key) ||
        !CBB_flush(&key_share)) {
      return false;
    }
  }
  if (type == handback_tls13) {
    early_data_t early_data;
    // Check early data invariants.
    if (ssl->enable_early_data ==
        (s3->early_data_reason == ssl_early_data_disabled)) {
      return false;
    }
    if (hs->early_data_offered) {
      if (s3->early_data_accepted && !s3->skip_early_data) {
        early_data = early_data_accepted;
      } else if (!s3->early_data_accepted && !s3->skip_early_data) {
        early_data = early_data_rejected_hrr;
      } else if (!s3->early_data_accepted && s3->skip_early_data) {
        early_data = early_data_skipped;
      } else {
        return false;
      }
    } else if (!s3->early_data_accepted && !s3->skip_early_data) {
      early_data = early_data_not_offered;
    } else {
      return false;
    }
    if (!CBB_add_asn1_octet_string(&seq, hs->client_traffic_secret_0().data(),
                                   hs->client_traffic_secret_0().size()) ||
        !CBB_add_asn1_octet_string(&seq, hs->server_traffic_secret_0().data(),
                                   hs->server_traffic_secret_0().size()) ||
        !CBB_add_asn1_octet_string(&seq, hs->client_handshake_secret().data(),
                                   hs->client_handshake_secret().size()) ||
        !CBB_add_asn1_octet_string(&seq, hs->server_handshake_secret().data(),
                                   hs->server_handshake_secret().size()) ||
        !CBB_add_asn1_octet_string(&seq, hs->secret().data(),
                                   hs->secret().size()) ||
        !CBB_add_asn1_octet_string(&seq, s3->exporter_secret,
                                   s3->exporter_secret_len) ||
        !CBB_add_asn1_bool(&seq, s3->used_hello_retry_request) ||
        !CBB_add_asn1_bool(&seq, hs->accept_psk_mode) ||
        !CBB_add_asn1_int64(&seq, s3->ticket_age_skew) ||
        !CBB_add_asn1_uint64(&seq, s3->early_data_reason) ||
        !CBB_add_asn1_uint64(&seq, early_data)) {
      return false;
    }
    if (early_data == early_data_accepted &&
        !CBB_add_asn1_octet_string(&seq, hs->early_traffic_secret().data(),
                                   hs->early_traffic_secret().size())) {
      return false;
    }

    if (session->has_application_settings) {
      uint16_t alps_codepoint = TLSEXT_TYPE_application_settings_old;
      if (hs->config->alps_use_new_codepoint) {
        alps_codepoint = TLSEXT_TYPE_application_settings;
      }
      if (!CBB_add_asn1_uint64(&seq, alps_codepoint)) {
        return false;
      }
    }
  }
  return CBB_flush(out);
}

static bool CopyExact(Span<uint8_t> out, const CBS *in) {
  if (CBS_len(in) != out.size()) {
    return false;
  }
  OPENSSL_memcpy(out.data(), CBS_data(in), out.size());
  return true;
}

bool SSL_apply_handback(SSL *ssl, Span<const uint8_t> handback) {
  if (ssl->do_handshake != nullptr ||
      ssl->method->is_dtls) {
    return false;
  }

  SSL3_STATE *const s3 = ssl->s3;
  uint64_t handback_version, unused_token_binding_param, cipher, type_u64,
           alps_codepoint;

  CBS seq, read_seq, write_seq, server_rand, client_rand, read_iv, write_iv,
      next_proto, alpn, hostname, unused_channel_id, transcript, key_share;
  int session_reused, channel_id_negotiated, cert_request,
      extended_master_secret, ticket_expected, unused_token_binding,
      next_proto_neg_seen;
  SSL_SESSION *session = nullptr;

  CBS handback_cbs(handback);
  if (!CBS_get_asn1(&handback_cbs, &seq, CBS_ASN1_SEQUENCE) ||
      !CBS_get_asn1_uint64(&seq, &handback_version) ||
      handback_version != kHandbackVersion ||
      !CBS_get_asn1_uint64(&seq, &type_u64) ||
      type_u64 > handback_max_value) {
    return false;
  }

  handback_t type = static_cast<handback_t>(type_u64);
  if (!CBS_get_asn1(&seq, &read_seq, CBS_ASN1_OCTETSTRING) ||
      CBS_len(&read_seq) != sizeof(s3->read_sequence) ||
      !CBS_get_asn1(&seq, &write_seq, CBS_ASN1_OCTETSTRING) ||
      CBS_len(&write_seq) != sizeof(s3->write_sequence) ||
      !CBS_get_asn1(&seq, &server_rand, CBS_ASN1_OCTETSTRING) ||
      CBS_len(&server_rand) != sizeof(s3->server_random) ||
      !CBS_copy_bytes(&server_rand, s3->server_random,
                      sizeof(s3->server_random)) ||
      !CBS_get_asn1(&seq, &client_rand, CBS_ASN1_OCTETSTRING) ||
      CBS_len(&client_rand) != sizeof(s3->client_random) ||
      !CBS_copy_bytes(&client_rand, s3->client_random,
                      sizeof(s3->client_random)) ||
      !CBS_get_asn1(&seq, &read_iv, CBS_ASN1_OCTETSTRING) ||
      !CBS_get_asn1(&seq, &write_iv, CBS_ASN1_OCTETSTRING) ||
      !CBS_get_asn1_bool(&seq, &session_reused) ||
      !CBS_get_asn1_bool(&seq, &channel_id_negotiated)) {
    return false;
  }

  s3->hs = ssl_handshake_new(ssl);
  if (!s3->hs) {
    return false;
  }
  SSL_HANDSHAKE *const hs = s3->hs.get();
  if (!session_reused || type == handback_tls13) {
    hs->new_session =
        SSL_SESSION_parse(&seq, ssl->ctx->x509_method, ssl->ctx->pool);
    session = hs->new_session.get();
  } else {
    ssl->session =
        SSL_SESSION_parse(&seq, ssl->ctx->x509_method, ssl->ctx->pool);
    session = ssl->session.get();
  }

  if (!session || !CBS_get_asn1(&seq, &next_proto, CBS_ASN1_OCTETSTRING) ||
      !CBS_get_asn1(&seq, &alpn, CBS_ASN1_OCTETSTRING) ||
      !CBS_get_asn1(&seq, &hostname, CBS_ASN1_OCTETSTRING) ||
      !CBS_get_asn1(&seq, &unused_channel_id, CBS_ASN1_OCTETSTRING) ||
      !CBS_get_asn1_bool(&seq, &unused_token_binding) ||
      !CBS_get_asn1_uint64(&seq, &unused_token_binding_param) ||
      !CBS_get_asn1_bool(&seq, &next_proto_neg_seen) ||
      !CBS_get_asn1_bool(&seq, &cert_request) ||
      !CBS_get_asn1_bool(&seq, &extended_master_secret) ||
      !CBS_get_asn1_bool(&seq, &ticket_expected) ||
      !CBS_get_asn1_uint64(&seq, &cipher)) {
    return false;
  }
  if ((hs->new_cipher =
           SSL_get_cipher_by_value(static_cast<uint16_t>(cipher))) == nullptr) {
    return false;
  }
  if (!CBS_get_asn1(&seq, &transcript, CBS_ASN1_OCTETSTRING) ||
      !CBS_get_asn1(&seq, &key_share, CBS_ASN1_SEQUENCE)) {
    return false;
  }
  CBS client_handshake_secret, server_handshake_secret, client_traffic_secret_0,
      server_traffic_secret_0, secret, exporter_secret, early_traffic_secret;
  if (type == handback_tls13) {
    int used_hello_retry_request, accept_psk_mode;
    uint64_t early_data, early_data_reason;
    int64_t ticket_age_skew;
    if (!CBS_get_asn1(&seq, &client_traffic_secret_0, CBS_ASN1_OCTETSTRING) ||
        !CBS_get_asn1(&seq, &server_traffic_secret_0, CBS_ASN1_OCTETSTRING) ||
        !CBS_get_asn1(&seq, &client_handshake_secret, CBS_ASN1_OCTETSTRING) ||
        !CBS_get_asn1(&seq, &server_handshake_secret, CBS_ASN1_OCTETSTRING) ||
        !CBS_get_asn1(&seq, &secret, CBS_ASN1_OCTETSTRING) ||
        !CBS_get_asn1(&seq, &exporter_secret, CBS_ASN1_OCTETSTRING) ||
        !CBS_get_asn1_bool(&seq, &used_hello_retry_request) ||
        !CBS_get_asn1_bool(&seq, &accept_psk_mode) ||
        !CBS_get_asn1_int64(&seq, &ticket_age_skew) ||
        !CBS_get_asn1_uint64(&seq, &early_data_reason) ||
        early_data_reason > ssl_early_data_reason_max_value ||
        !CBS_get_asn1_uint64(&seq, &early_data) ||
        early_data > early_data_max_value) {
      return false;
    }
    early_data_t early_data_type = static_cast<early_data_t>(early_data);
    if (early_data_type == early_data_accepted &&
        !CBS_get_asn1(&seq, &early_traffic_secret, CBS_ASN1_OCTETSTRING)) {
      return false;
    }

    if (session->has_application_settings) {
      // Making it optional to keep compatibility with older handshakers.
      // Older handshakers won't send the field.
      if (CBS_len(&seq) == 0) {
        hs->config->alps_use_new_codepoint = false;
      } else {
        if (!CBS_get_asn1_uint64(&seq, &alps_codepoint)) {
          return false;
        }

        if (alps_codepoint == TLSEXT_TYPE_application_settings) {
          hs->config->alps_use_new_codepoint = true;
        } else if (alps_codepoint == TLSEXT_TYPE_application_settings_old) {
          hs->config->alps_use_new_codepoint = false;
        } else {
          OPENSSL_PUT_ERROR(SSL, SSL_R_INVALID_ALPS_CODEPOINT);
          return false;
        }
      }
    }

    if (ticket_age_skew > std::numeric_limits<int32_t>::max() ||
        ticket_age_skew < std::numeric_limits<int32_t>::min()) {
      return false;
    }
    s3->ticket_age_skew = static_cast<int32_t>(ticket_age_skew);
    s3->used_hello_retry_request = used_hello_retry_request;
    hs->accept_psk_mode = accept_psk_mode;

    s3->early_data_reason =
        static_cast<ssl_early_data_reason_t>(early_data_reason);
    ssl->enable_early_data = s3->early_data_reason != ssl_early_data_disabled;
    s3->skip_early_data = false;
    s3->early_data_accepted = false;
    hs->early_data_offered = false;
    switch (early_data_type) {
      case early_data_not_offered:
        break;
      case early_data_accepted:
        s3->early_data_accepted = true;
        hs->early_data_offered = true;
        hs->can_early_write = true;
        hs->can_early_read = true;
        hs->in_early_data = true;
        break;
      case early_data_rejected_hrr:
        hs->early_data_offered = true;
        break;
      case early_data_skipped:
        s3->skip_early_data = true;
        hs->early_data_offered = true;
        break;
      default:
        return false;
    }
  } else {
    s3->early_data_reason = ssl_early_data_protocol_version;
  }

  ssl->version = session->ssl_version;
  ssl->verify_result = session->verify_result;
  s3->have_version = true;
  if (!ssl_method_supports_version(ssl->method, ssl->version) ||
      session->cipher != hs->new_cipher ||
      ssl_protocol_version(ssl) < SSL_CIPHER_get_min_version(session->cipher) ||
      SSL_CIPHER_get_max_version(session->cipher) < ssl_protocol_version(ssl)) {
    return false;
  }
  ssl->do_handshake = ssl_server_handshake;
  ssl->server = true;
  switch (type) {
    case handback_after_session_resumption:
      hs->state = state12_read_change_cipher_spec;
      if (!session_reused) {
        return false;
      }
      break;
    case handback_after_ecdhe:
      hs->state = state12_read_client_certificate;
      if (session_reused) {
        return false;
      }
      break;
    case handback_after_handshake:
      hs->state = state12_finish_server_handshake;
      break;
    case handback_tls13:
      hs->state = state12_tls13;
      hs->tls13_state = state13_send_half_rtt_ticket;
      break;
    default:
      return false;
  }
  s3->session_reused = session_reused;
  hs->channel_id_negotiated = channel_id_negotiated;
  if (!s3->next_proto_negotiated.CopyFrom(next_proto) ||
      !s3->alpn_selected.CopyFrom(alpn)) {
    return false;
  }

  const size_t hostname_len = CBS_len(&hostname);
  if (hostname_len == 0) {
    s3->hostname.reset();
  } else {
    char *hostname_str = nullptr;
    if (!CBS_strdup(&hostname, &hostname_str)) {
      return false;
    }
    s3->hostname.reset(hostname_str);
  }

  hs->next_proto_neg_seen = next_proto_neg_seen;
  hs->wait = ssl_hs_flush;
  hs->extended_master_secret = extended_master_secret;
  hs->ticket_expected = ticket_expected;
  s3->aead_write_ctx->SetVersionIfNullCipher(ssl->version);
  hs->cert_request = cert_request;

  if (type != handback_after_handshake &&
      (!hs->transcript.Init() ||
       !hs->transcript.InitHash(ssl_protocol_version(ssl), hs->new_cipher) ||
       !hs->transcript.Update(transcript))) {
    return false;
  }
  if (type == handback_tls13) {
    hs->ResizeSecrets(hs->transcript.DigestLen());
    if (!CopyExact(hs->client_traffic_secret_0(), &client_traffic_secret_0) ||
        !CopyExact(hs->server_traffic_secret_0(), &server_traffic_secret_0) ||
        !CopyExact(hs->client_handshake_secret(), &client_handshake_secret) ||
        !CopyExact(hs->server_handshake_secret(), &server_handshake_secret) ||
        !CopyExact(hs->secret(), &secret) ||
        !CopyExact({s3->exporter_secret, hs->transcript.DigestLen()},
                   &exporter_secret)) {
      return false;
    }
    s3->exporter_secret_len = CBS_len(&exporter_secret);

    if (s3->early_data_accepted &&
        !CopyExact(hs->early_traffic_secret(), &early_traffic_secret)) {
      return false;
    }
  }
  Array<uint8_t> key_block;
  switch (type) {
    case handback_after_session_resumption:
      // The write keys are installed after server Finished, but the client
      // keys must wait for ChangeCipherSpec.
      if (!tls1_configure_aead(ssl, evp_aead_seal, &key_block, session,
                               write_iv)) {
        return false;
      }
      break;
    case handback_after_ecdhe:
      // The premaster secret is not yet computed, so install no keys.
      break;
    case handback_after_handshake:
      // The handshake is complete, so both keys are installed.
      if (!tls1_configure_aead(ssl, evp_aead_seal, &key_block, session,
                               write_iv) ||
          !tls1_configure_aead(ssl, evp_aead_open, &key_block, session,
                               read_iv)) {
        return false;
      }
      break;
    case handback_tls13:
      // After server Finished, the application write keys are installed, but
      // none of the read keys. The read keys are installed in the state machine
      // immediately after processing handback.
      if (!tls13_set_traffic_key(ssl, ssl_encryption_application, evp_aead_seal,
                                 hs->new_session.get(),
                                 hs->server_traffic_secret_0())) {
        return false;
      }
      break;
  }
  if (!CopyExact({s3->read_sequence, sizeof(s3->read_sequence)}, &read_seq) ||
      !CopyExact({s3->write_sequence, sizeof(s3->write_sequence)},
                 &write_seq)) {
    return false;
  }
  if (type == handback_after_ecdhe) {
    uint64_t group_id;
    CBS private_key;
    if (!CBS_get_asn1_uint64(&key_share, &group_id) ||  //
        group_id > 0xffff ||
        !CBS_get_asn1(&key_share, &private_key, CBS_ASN1_OCTETSTRING)) {
      return false;
    }
    hs->key_shares[0] = SSLKeyShare::Create(group_id);
    if (!hs->key_shares[0] ||
        !hs->key_shares[0]->DeserializePrivateKey(&private_key)) {
      return false;
    }
  }
  return true;  // Trailing data allowed for extensibility.
}

BSSL_NAMESPACE_END

using namespace bssl;

int SSL_serialize_capabilities(const SSL *ssl, CBB *out) {
  CBB seq;
  if (!CBB_add_asn1(out, &seq, CBS_ASN1_SEQUENCE) ||
      !serialize_features(&seq) ||  //
      !CBB_flush(out)) {
    return 0;
  }

  return 1;
}

int SSL_request_handshake_hints(SSL *ssl, const uint8_t *client_hello,
                                size_t client_hello_len,
                                const uint8_t *capabilities,
                                size_t capabilities_len) {
  if (SSL_is_dtls(ssl)) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_SHOULD_NOT_HAVE_BEEN_CALLED);
    return 0;
  }

  CBS cbs, seq;
  CBS_init(&cbs, capabilities, capabilities_len);
  UniquePtr<SSL_HANDSHAKE_HINTS> hints = MakeUnique<SSL_HANDSHAKE_HINTS>();
  if (hints == nullptr ||
      !CBS_get_asn1(&cbs, &seq, CBS_ASN1_SEQUENCE) ||
      !apply_remote_features(ssl, &seq)) {
    return 0;
  }

  SSL3_STATE *const s3 = ssl->s3;
  s3->v2_hello_done = true;
  s3->has_message = true;

  Array<uint8_t> client_hello_msg;
  ScopedCBB client_hello_cbb;
  CBB client_hello_body;
  if (!ssl->method->init_message(ssl, client_hello_cbb.get(),
                                 &client_hello_body, SSL3_MT_CLIENT_HELLO) ||
      !CBB_add_bytes(&client_hello_body, client_hello, client_hello_len) ||
      !ssl->method->finish_message(ssl, client_hello_cbb.get(),
                                   &client_hello_msg)) {
    return 0;
  }

  s3->hs_buf.reset(BUF_MEM_new());
  if (!s3->hs_buf || !BUF_MEM_append(s3->hs_buf.get(), client_hello_msg.data(),
                                     client_hello_msg.size())) {
    return 0;
  }

  s3->hs->hints_requested = true;
  s3->hs->hints = std::move(hints);
  return 1;
}

// |SSL_HANDSHAKE_HINTS| is serialized as the following ASN.1 structure. We use
// implicit tagging to make it a little more compact.
//
// HandshakeHints ::= SEQUENCE {
//     serverRandomTLS13       [0] IMPLICIT OCTET STRING OPTIONAL,
//     keyShareHint            [1] IMPLICIT KeyShareHint OPTIONAL,
//     signatureHint           [2] IMPLICIT SignatureHint OPTIONAL,
//     -- At most one of decryptedPSKHint or ignorePSKHint may be present. It
//     -- corresponds to the first entry in pre_shared_keys. TLS 1.2 session
//     -- tickets use a separate hint, to ensure the caller does not apply the
//     -- hint to the wrong field.
//     decryptedPSKHint        [3] IMPLICIT OCTET STRING OPTIONAL,
//     ignorePSKHint           [4] IMPLICIT NULL OPTIONAL,
//     compressCertificateHint [5] IMPLICIT CompressCertificateHint OPTIONAL,
//     -- TLS 1.2 and 1.3 use different server random hints because one contains
//     -- a timestamp while the other doesn't. If the hint was generated
//     -- assuming TLS 1.3 but we actually negotiate TLS 1.2, mixing the two
//     -- will break this.
//     serverRandomTLS12       [6] IMPLICIT OCTET STRING OPTIONAL,
//     ecdheHint               [7] IMPLICIT ECDHEHint OPTIONAL
//     -- At most one of decryptedTicketHint or ignoreTicketHint may be present.
//     -- renewTicketHint requires decryptedTicketHint.
//     decryptedTicketHint     [8] IMPLICIT OCTET STRING OPTIONAL,
//     renewTicketHint         [9] IMPLICIT NULL OPTIONAL,
//     ignoreTicketHint       [10] IMPLICIT NULL OPTIONAL,
// }
//
// KeyShareHint ::= SEQUENCE {
//     groupId                 INTEGER,
//     publicKey               OCTET STRING,
//     secret                  OCTET STRING,
// }
//
// SignatureHint ::= SEQUENCE {
//     algorithm               INTEGER,
//     input                   OCTET STRING,
//     subjectPublicKeyInfo    OCTET STRING,
//     signature               OCTET STRING,
// }
//
// CompressCertificateHint ::= SEQUENCE {
//     algorithm               INTEGER,
//     input                   OCTET STRING,
//     compressed              OCTET STRING,
// }
//
// ECDHEHint ::= SEQUENCE {
//     groupId                 INTEGER,
//     publicKey               OCTET STRING,
//     privateKey              OCTET STRING,
// }

// HandshakeHints tags.
static const CBS_ASN1_TAG kServerRandomTLS13Tag =
    CBS_ASN1_CONTEXT_SPECIFIC | 0;
static const CBS_ASN1_TAG kKeyShareHintTag =
    CBS_ASN1_CONSTRUCTED | CBS_ASN1_CONTEXT_SPECIFIC | 1;
static const CBS_ASN1_TAG kSignatureHintTag =
    CBS_ASN1_CONSTRUCTED | CBS_ASN1_CONTEXT_SPECIFIC | 2;
static const CBS_ASN1_TAG kDecryptedPSKTag = CBS_ASN1_CONTEXT_SPECIFIC | 3;
static const CBS_ASN1_TAG kIgnorePSKTag = CBS_ASN1_CONTEXT_SPECIFIC | 4;
static const CBS_ASN1_TAG kCompressCertificateTag =
    CBS_ASN1_CONTEXT_SPECIFIC | 5;
static const CBS_ASN1_TAG kServerRandomTLS12Tag =
    CBS_ASN1_CONTEXT_SPECIFIC | 6;
static const CBS_ASN1_TAG kECDHEHintTag = CBS_ASN1_CONSTRUCTED | 7;
static const CBS_ASN1_TAG kDecryptedTicketTag = CBS_ASN1_CONTEXT_SPECIFIC | 8;
static const CBS_ASN1_TAG kRenewTicketTag = CBS_ASN1_CONTEXT_SPECIFIC | 9;
static const CBS_ASN1_TAG kIgnoreTicketTag = CBS_ASN1_CONTEXT_SPECIFIC | 10;

int SSL_serialize_handshake_hints(const SSL *ssl, CBB *out) {
  const SSL_HANDSHAKE *hs = ssl->s3->hs.get();
  if (!ssl->server || !hs->hints_requested) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_SHOULD_NOT_HAVE_BEEN_CALLED);
    return 0;
  }

  const SSL_HANDSHAKE_HINTS *hints = hs->hints.get();
  CBB seq, child;
  if (!CBB_add_asn1(out, &seq, CBS_ASN1_SEQUENCE)) {
    return 0;
  }

  if (!hints->server_random_tls13.empty()) {
    if (!CBB_add_asn1(&seq, &child, kServerRandomTLS13Tag) ||
        !CBB_add_bytes(&child, hints->server_random_tls13.data(),
                       hints->server_random_tls13.size())) {
      return 0;
    }
  }

  if (hints->key_share_group_id != 0 && !hints->key_share_public_key.empty() &&
      !hints->key_share_secret.empty()) {
    if (!CBB_add_asn1(&seq, &child, kKeyShareHintTag) ||
        !CBB_add_asn1_uint64(&child, hints->key_share_group_id) ||
        !CBB_add_asn1_octet_string(&child, hints->key_share_public_key.data(),
                                   hints->key_share_public_key.size()) ||
        !CBB_add_asn1_octet_string(&child, hints->key_share_secret.data(),
                                   hints->key_share_secret.size())) {
      return 0;
    }
  }

  if (hints->signature_algorithm != 0 && !hints->signature_input.empty() &&
      !hints->signature.empty()) {
    if (!CBB_add_asn1(&seq, &child, kSignatureHintTag) ||
        !CBB_add_asn1_uint64(&child, hints->signature_algorithm) ||
        !CBB_add_asn1_octet_string(&child, hints->signature_input.data(),
                                   hints->signature_input.size()) ||
        !CBB_add_asn1_octet_string(&child, hints->signature_spki.data(),
                                   hints->signature_spki.size()) ||
        !CBB_add_asn1_octet_string(&child, hints->signature.data(),
                                   hints->signature.size())) {
      return 0;
    }
  }

  if (!hints->decrypted_psk.empty()) {
    if (!CBB_add_asn1(&seq, &child, kDecryptedPSKTag) ||
        !CBB_add_bytes(&child, hints->decrypted_psk.data(),
                       hints->decrypted_psk.size())) {
      return 0;
    }
  }

  if (hints->ignore_psk &&  //
      !CBB_add_asn1(&seq, &child, kIgnorePSKTag)) {
    return 0;
  }

  if (hints->cert_compression_alg_id != 0 &&
      !hints->cert_compression_input.empty() &&
      !hints->cert_compression_output.empty()) {
    if (!CBB_add_asn1(&seq, &child, kCompressCertificateTag) ||
        !CBB_add_asn1_uint64(&child, hints->cert_compression_alg_id) ||
        !CBB_add_asn1_octet_string(&child, hints->cert_compression_input.data(),
                                   hints->cert_compression_input.size()) ||
        !CBB_add_asn1_octet_string(&child,
                                   hints->cert_compression_output.data(),
                                   hints->cert_compression_output.size())) {
      return 0;
    }
  }

  if (!hints->server_random_tls12.empty()) {
    if (!CBB_add_asn1(&seq, &child, kServerRandomTLS12Tag) ||
        !CBB_add_bytes(&child, hints->server_random_tls12.data(),
                       hints->server_random_tls12.size())) {
      return 0;
    }
  }

  if (hints->ecdhe_group_id != 0 && !hints->ecdhe_public_key.empty() &&
      !hints->ecdhe_private_key.empty()) {
    if (!CBB_add_asn1(&seq, &child, kECDHEHintTag) ||
        !CBB_add_asn1_uint64(&child, hints->ecdhe_group_id) ||
        !CBB_add_asn1_octet_string(&child, hints->ecdhe_public_key.data(),
                                   hints->ecdhe_public_key.size()) ||
        !CBB_add_asn1_octet_string(&child, hints->ecdhe_private_key.data(),
                                   hints->ecdhe_private_key.size())) {
      return 0;
    }
  }


  if (!hints->decrypted_ticket.empty()) {
    if (!CBB_add_asn1(&seq, &child, kDecryptedTicketTag) ||
        !CBB_add_bytes(&child, hints->decrypted_ticket.data(),
                       hints->decrypted_ticket.size())) {
      return 0;
    }
  }

  if (hints->renew_ticket &&  //
      !CBB_add_asn1(&seq, &child, kRenewTicketTag)) {
    return 0;
  }

  if (hints->ignore_ticket &&  //
      !CBB_add_asn1(&seq, &child, kIgnoreTicketTag)) {
    return 0;
  }

  return CBB_flush(out);
}

static bool get_optional_implicit_null(CBS *cbs, bool *out_present,
                                       CBS_ASN1_TAG tag) {
  CBS value;
  int present;
  if (!CBS_get_optional_asn1(cbs, &value, &present, tag) ||
      (present && CBS_len(&value) != 0)) {
    return false;
  }
  *out_present = present;
  return true;
}

int SSL_set_handshake_hints(SSL *ssl, const uint8_t *hints, size_t hints_len) {
  if (SSL_is_dtls(ssl)) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_SHOULD_NOT_HAVE_BEEN_CALLED);
    return 0;
  }

  UniquePtr<SSL_HANDSHAKE_HINTS> hints_obj = MakeUnique<SSL_HANDSHAKE_HINTS>();
  if (hints_obj == nullptr) {
    return 0;
  }

  CBS cbs, seq, server_random_tls13, key_share, signature_hint, psk,
      cert_compression, server_random_tls12, ecdhe, ticket;
  int has_server_random_tls13, has_key_share, has_signature_hint, has_psk,
      has_cert_compression, has_server_random_tls12, has_ecdhe, has_ticket;
  CBS_init(&cbs, hints, hints_len);
  if (!CBS_get_asn1(&cbs, &seq, CBS_ASN1_SEQUENCE) ||
      !CBS_get_optional_asn1(&seq, &server_random_tls13,
                             &has_server_random_tls13, kServerRandomTLS13Tag) ||
      !CBS_get_optional_asn1(&seq, &key_share, &has_key_share,
                             kKeyShareHintTag) ||
      !CBS_get_optional_asn1(&seq, &signature_hint, &has_signature_hint,
                             kSignatureHintTag) ||
      !CBS_get_optional_asn1(&seq, &psk, &has_psk, kDecryptedPSKTag) ||
      !get_optional_implicit_null(&seq, &hints_obj->ignore_psk,
                                  kIgnorePSKTag) ||
      !CBS_get_optional_asn1(&seq, &cert_compression, &has_cert_compression,
                             kCompressCertificateTag) ||
      !CBS_get_optional_asn1(&seq, &server_random_tls12,
                             &has_server_random_tls12, kServerRandomTLS12Tag) ||
      !CBS_get_optional_asn1(&seq, &ecdhe, &has_ecdhe, kECDHEHintTag) ||
      !CBS_get_optional_asn1(&seq, &ticket, &has_ticket, kDecryptedTicketTag) ||
      !get_optional_implicit_null(&seq, &hints_obj->renew_ticket,
                                  kRenewTicketTag) ||
      !get_optional_implicit_null(&seq, &hints_obj->ignore_ticket,
                                  kIgnoreTicketTag)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_COULD_NOT_PARSE_HINTS);
    return 0;
  }

  if (has_server_random_tls13 &&
      !hints_obj->server_random_tls13.CopyFrom(server_random_tls13)) {
    return 0;
  }

  if (has_key_share) {
    uint64_t group_id;
    CBS public_key, secret;
    if (!CBS_get_asn1_uint64(&key_share, &group_id) ||  //
        group_id == 0 || group_id > 0xffff ||
        !CBS_get_asn1(&key_share, &public_key, CBS_ASN1_OCTETSTRING) ||
        !hints_obj->key_share_public_key.CopyFrom(public_key) ||
        !CBS_get_asn1(&key_share, &secret, CBS_ASN1_OCTETSTRING) ||
        !hints_obj->key_share_secret.CopyFrom(secret)) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_COULD_NOT_PARSE_HINTS);
      return 0;
    }
    hints_obj->key_share_group_id = static_cast<uint16_t>(group_id);
  }

  if (has_signature_hint) {
    uint64_t sig_alg;
    CBS input, spki, signature;
    if (!CBS_get_asn1_uint64(&signature_hint, &sig_alg) ||  //
        sig_alg == 0 || sig_alg > 0xffff ||
        !CBS_get_asn1(&signature_hint, &input, CBS_ASN1_OCTETSTRING) ||
        !hints_obj->signature_input.CopyFrom(input) ||
        !CBS_get_asn1(&signature_hint, &spki, CBS_ASN1_OCTETSTRING) ||
        !hints_obj->signature_spki.CopyFrom(spki) ||
        !CBS_get_asn1(&signature_hint, &signature, CBS_ASN1_OCTETSTRING) ||
        !hints_obj->signature.CopyFrom(signature)) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_COULD_NOT_PARSE_HINTS);
      return 0;
    }
    hints_obj->signature_algorithm = static_cast<uint16_t>(sig_alg);
  }

  if (has_psk && !hints_obj->decrypted_psk.CopyFrom(psk)) {
    return 0;
  }
  if (has_psk && hints_obj->ignore_psk) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_COULD_NOT_PARSE_HINTS);
    return 0;
  }

  if (has_cert_compression) {
    uint64_t alg;
    CBS input, output;
    if (!CBS_get_asn1_uint64(&cert_compression, &alg) ||  //
        alg == 0 || alg > 0xffff ||
        !CBS_get_asn1(&cert_compression, &input, CBS_ASN1_OCTETSTRING) ||
        !hints_obj->cert_compression_input.CopyFrom(input) ||
        !CBS_get_asn1(&cert_compression, &output, CBS_ASN1_OCTETSTRING) ||
        !hints_obj->cert_compression_output.CopyFrom(output)) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_COULD_NOT_PARSE_HINTS);
      return 0;
    }
    hints_obj->cert_compression_alg_id = static_cast<uint16_t>(alg);
  }

  if (has_server_random_tls12 &&
      !hints_obj->server_random_tls12.CopyFrom(server_random_tls12)) {
    return 0;
  }

  if (has_ecdhe) {
    uint64_t group_id;
    CBS public_key, private_key;
    if (!CBS_get_asn1_uint64(&ecdhe, &group_id) ||  //
        group_id == 0 || group_id > 0xffff ||
        !CBS_get_asn1(&ecdhe, &public_key, CBS_ASN1_OCTETSTRING) ||
        !hints_obj->ecdhe_public_key.CopyFrom(public_key) ||
        !CBS_get_asn1(&ecdhe, &private_key, CBS_ASN1_OCTETSTRING) ||
        !hints_obj->ecdhe_private_key.CopyFrom(private_key)) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_COULD_NOT_PARSE_HINTS);
      return 0;
    }
    hints_obj->ecdhe_group_id = static_cast<uint16_t>(group_id);
  }

  if (has_ticket && !hints_obj->decrypted_ticket.CopyFrom(ticket)) {
    return 0;
  }
  if (has_ticket && hints_obj->ignore_ticket) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_COULD_NOT_PARSE_HINTS);
    return 0;
  }
  if (!has_ticket && hints_obj->renew_ticket) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_COULD_NOT_PARSE_HINTS);
    return 0;
  }

  ssl->s3->hs->hints = std::move(hints_obj);
  return 1;
}

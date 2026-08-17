// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include <openssl/ssl.h>

#include <openssl/bytestring.h>
#include <openssl/err.h>
#include <openssl/mem.h>

#include "internal.h"

BSSL_NAMESPACE_BEGIN

bool ssl_transfer_supported(const SSL *in) {
  if (in == NULL) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_UNSUPPORTED);
    return false;
  }

  // An SSL connection can't be serialized by current implementation under some
  // conditions 0) It's not server SSL. 1) It's a DTLS connection. 2) It uses
  // QUIC 3) Its SSL_SESSION isn't serializable. 4) Handshake hasn't finished
  // yet. 5) TLS version is not supported(currently, only TLS 1.2 & TLS 1.3 are
  // supported). 6) Write is not in clean state(|SSL_write| should finish the
  // |in| write, no pending writes). 7) A handshake message is not in a clean
  // state. 8) ssl shutdown state is not ssl_shutdown_none.
  if (!SSL_is_server(in) ||                               // (0)
      SSL_is_dtls(in) ||                                  // (1)
      in->quic_method != nullptr ||                       // (2)
      !in->s3 ||                                          // (3)
      !in->s3->established_session || SSL_in_init(in) ||  // (4)
      !(in->version == TLS1_2_VERSION ||
        in->version == TLS1_3_VERSION) ||      // (5)
      in->s3->unreported_bytes_written > 0 ||  // (6)
      in->s3->pending_write.size() > 0 ||
      in->s3->pending_flight_offset > 0 ||           // (7)
      in->s3->read_shutdown != ssl_shutdown_none ||  // (8)
      in->s3->write_shutdown != ssl_shutdown_none ||
      in->is_suspended_state) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_UNSUPPORTED);
    return false;
  }

  return true;
}

BSSL_NAMESPACE_END

using namespace bssl;

// SSL3_STATE_parse_octet_string gets an optional ASN.1 OCTET STRING explicitly
// tagged with |tag| from |cbs| and stows it in |*out|. It returns one on
// success, whether or not the element was found, and zero on decode error.
static bool SSL3_STATE_parse_octet_string(CBS *cbs, Array<uint8_t> *out,
                                          unsigned tag) {
  CBS value;
  if (!CBS_get_optional_asn1_octet_string(cbs, &value, NULL, tag)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL3_STATE);
    return false;
  }
  return out->CopyFrom(value);
}

// parse_optional_string gets an optional ASN.1 OCTET STRING explicitly
// tagged with |tag| from |cbs| and saves it in |*out|. If the element was not
// found, it sets |*out| to NULL. It returns one on success, whether or not the
// element was found, and zero on decode error.
static int parse_optional_string(CBS *cbs, UniquePtr<char> *out, unsigned tag,
                                 int reason) {
  CBS value;
  int present;
  if (!CBS_get_optional_asn1_octet_string(cbs, &value, &present, tag)) {
    OPENSSL_PUT_ERROR(SSL, reason);
    return 0;
  }
  if (present) {
    if (CBS_contains_zero_byte(&value)) {
      OPENSSL_PUT_ERROR(SSL, reason);
      return 0;
    }
    char *raw = nullptr;
    if (!CBS_strdup(&value, &raw)) {
      return 0;
    }
    out->reset(raw);
  } else {
    out->reset();
  }
  return 1;
}

// SSL3_STATE_get_optional_octet_string requires |dst| has |target_len|
// space to store the |tag| data from the |cbs|.
static bool SSL3_STATE_get_optional_octet_string(CBS *cbs, void *dst,
                                                 unsigned tag,
                                                 size_t target_len) {
  int present;
  CBS value;
  if (!CBS_get_optional_asn1_octet_string(cbs, &value, &present, tag)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL3_STATE);
    return false;
  }
  if (!present) {
    return true;
  }
  size_t data_len = CBS_len(&value);
  if (target_len > 0 && data_len != target_len) {
    return false;
  }
  OPENSSL_memcpy(dst, CBS_data(&value), data_len);
  return true;
}

// SSL3_STATE serialization.

enum SSL3_STATE_SERDE_VERSION {
  SSL3_STATE_SERDE_VERSION_ONE = 1,
  SSL3_STATE_SERDE_VERSION_TWO = 2,
  SSL3_STATE_SERDE_VERSION_THREE = 3
};

static const unsigned kS3EstablishedSessionTag =
    CBS_ASN1_CONSTRUCTED | CBS_ASN1_CONTEXT_SPECIFIC | 0;
static const unsigned kS3SessionReusedTag =
    CBS_ASN1_CONSTRUCTED | CBS_ASN1_CONTEXT_SPECIFIC | 1;
static const unsigned kS3HostNameTag =
    CBS_ASN1_CONSTRUCTED | CBS_ASN1_CONTEXT_SPECIFIC | 2;
static const unsigned kS3ALPNSelectedTag =
    CBS_ASN1_CONSTRUCTED | CBS_ASN1_CONTEXT_SPECIFIC | 3;
static const unsigned kS3NextProtoNegotiatedTag =
    CBS_ASN1_CONSTRUCTED | CBS_ASN1_CONTEXT_SPECIFIC | 4;
static const unsigned kS3ChannelIdValidTag =
    CBS_ASN1_CONSTRUCTED | CBS_ASN1_CONTEXT_SPECIFIC | 5;
static const unsigned kS3ChannelIdTag =
    CBS_ASN1_CONSTRUCTED | CBS_ASN1_CONTEXT_SPECIFIC | 6;
static const unsigned kS3SendConnectionBindingTag =
    CBS_ASN1_CONSTRUCTED | CBS_ASN1_CONTEXT_SPECIFIC | 7;
static const unsigned kS3PendingAppDataTag =
    CBS_ASN1_CONSTRUCTED | CBS_ASN1_CONTEXT_SPECIFIC | 8;
static const unsigned kS3ReadBufferTag =
    CBS_ASN1_CONSTRUCTED | CBS_ASN1_CONTEXT_SPECIFIC | 9;
static const unsigned kS3NotResumableTag =
    CBS_ASN1_CONSTRUCTED | CBS_ASN1_CONTEXT_SPECIFIC | 10;
static const unsigned kS3EarlyDataSkippedTag =
    CBS_ASN1_CONSTRUCTED | CBS_ASN1_CONTEXT_SPECIFIC | 11;
static const unsigned kS3DelegatedCredentialUsedTag =
    CBS_ASN1_CONSTRUCTED | CBS_ASN1_CONTEXT_SPECIFIC | 12;
static const unsigned kS3EarlyDataAcceptedTag =
    CBS_ASN1_CONSTRUCTED | CBS_ASN1_CONTEXT_SPECIFIC | 13;
static const unsigned kS3UsedHelloRetryRequestTag =
    CBS_ASN1_CONSTRUCTED | CBS_ASN1_CONTEXT_SPECIFIC | 14;
static const unsigned kS3TicketAgeSkewTag =
    CBS_ASN1_CONSTRUCTED | CBS_ASN1_CONTEXT_SPECIFIC | 15;
static const unsigned kS3WriteTrafficSecretTag =
    CBS_ASN1_CONSTRUCTED | CBS_ASN1_CONTEXT_SPECIFIC | 16;
static const unsigned kS3WriteTrafficSecretLenTag =
    CBS_ASN1_CONSTRUCTED | CBS_ASN1_CONTEXT_SPECIFIC | 17;
static const unsigned kS3ReadTrafficSecretTag =
    CBS_ASN1_CONSTRUCTED | CBS_ASN1_CONTEXT_SPECIFIC | 18;
static const unsigned kS3ReadTrafficSecretLenTag =
    CBS_ASN1_CONSTRUCTED | CBS_ASN1_CONTEXT_SPECIFIC | 19;
static const unsigned kS3ExporterSecretTag =
    CBS_ASN1_CONSTRUCTED | CBS_ASN1_CONTEXT_SPECIFIC | 20;
static const unsigned kS3ExporterSecretLenTag =
    CBS_ASN1_CONSTRUCTED | CBS_ASN1_CONTEXT_SPECIFIC | 21;
static const unsigned kS3HandshakeBufferTag =
    CBS_ASN1_CONSTRUCTED | CBS_ASN1_CONTEXT_SPECIFIC | 22;
static const unsigned kS3EchStatusTag =
    CBS_ASN1_CONSTRUCTED | CBS_ASN1_CONTEXT_SPECIFIC | 23;
static const unsigned kS3PendingHsDataTag =
    CBS_ASN1_CONSTRUCTED | CBS_ASN1_CONTEXT_SPECIFIC | 24;
static const unsigned kS3PendingFlightTag =
    CBS_ASN1_CONSTRUCTED | CBS_ASN1_CONTEXT_SPECIFIC | 25;
static const unsigned kS3AeadReadCtxTag =
    CBS_ASN1_CONSTRUCTED | CBS_ASN1_CONTEXT_SPECIFIC | 26;
static const unsigned kS3AeadWriteCtxTag =
    CBS_ASN1_CONSTRUCTED | CBS_ASN1_CONTEXT_SPECIFIC | 27;
static const unsigned kS3KeyUpdatePending =
    CBS_ASN1_CONSTRUCTED | CBS_ASN1_CONTEXT_SPECIFIC | 28;

static int serialize_buf_mem(bssl::UniquePtr<BUF_MEM> &buf, CBS_ASN1_TAG tag,
                             CBB &cbb) {
  CBB child;
  if (!CBB_add_asn1(&cbb, &child, tag) ||
      !CBB_add_asn1_octet_string(&child, reinterpret_cast<uint8_t *>(buf->data),
                                 buf->length) ||
      !CBB_flush(&cbb)) {
    return 0;
  }
  return 1;
}

static int deserialize_buf_mem(bssl::UniquePtr<BUF_MEM> &buf, int &present,
                               CBS_ASN1_TAG tag, CBS &cbs) {
  CBS child;

  if (!CBS_get_optional_asn1_octet_string(&cbs, &child, &present, tag)) {
    return 0;
  }

  buf.reset(BUF_MEM_new());
  if (!BUF_MEM_append(buf.get(), CBS_data(&child), CBS_len(&child))) {
    return 0;
  }

  return 1;
}

// *** EXPERIMENTAL — DO NOT USE WITHOUT CHECKING ***
// These SSL3_STATE serialization functions are developed to support SSL
// transfer.

// ssl3_state_to_bytes serializes |in| to bytes stored in |cbb|.
// It returns one on success and zero on failure.
//
// SSL3_STATE ASN.1 structure: see ssl_transfer_asn1.cc
static int SSL3_STATE_to_bytes(SSL3_STATE *in, uint16_t protocol_version,
                               CBB *cbb) {
  if (in == NULL || cbb == NULL) {
    return 0;
  }

  CBB s3, child;
  if (!CBB_add_asn1(cbb, &s3, CBS_ASN1_SEQUENCE) ||
        !CBB_add_asn1_uint64(&s3, SSL3_STATE_SERDE_VERSION_THREE) ||
      !CBB_add_asn1_octet_string(&s3, in->read_sequence, TLS_SEQ_NUM_SIZE) ||
      !CBB_add_asn1_octet_string(&s3, in->write_sequence, TLS_SEQ_NUM_SIZE) ||
      !CBB_add_asn1_octet_string(&s3, in->server_random, SSL3_RANDOM_SIZE) ||
      !CBB_add_asn1_octet_string(&s3, in->client_random, SSL3_RANDOM_SIZE) ||
      !CBB_add_asn1_octet_string(&s3, in->send_alert, SSL3_SEND_ALERT_SIZE) ||
      !CBB_add_asn1_int64(&s3, in->rwstate) ||
      !CBB_add_asn1_int64(&s3, in->early_data_reason) ||
      !CBB_add_asn1_octet_string(&s3, in->previous_client_finished,
                                 in->previous_client_finished_len) ||
      !CBB_add_asn1_uint64(&s3, in->previous_client_finished_len) ||
      !CBB_add_asn1_octet_string(&s3, in->previous_server_finished,
                                 in->previous_server_finished_len) ||
      !CBB_add_asn1_uint64(&s3, in->previous_server_finished_len) ||
      !CBB_add_asn1_uint64(&s3, in->empty_record_count) ||
      !CBB_add_asn1_uint64(&s3, in->warning_alert_count) ||
      !CBB_add_asn1_uint64(&s3, in->total_renegotiations)) {
    return 0;
  }

  if (!CBB_add_asn1(&s3, &child, kS3EstablishedSessionTag) ||
      !ssl_session_serialize(in->established_session.get(), &child)) {
    return 0;
  }

  if (in->session_reused) {
    if (!CBB_add_asn1(&s3, &child, kS3SessionReusedTag) ||
        !CBB_add_asn1_bool(&child, true)) {
      return 0;
    }
  }

  if (in->hostname) {
    if (!CBB_add_asn1(&s3, &child, kS3HostNameTag) ||
        !CBB_add_asn1_octet_string(&child,
                                   (const uint8_t *)(in->hostname.get()),
                                   strlen(in->hostname.get()))) {
      return 0;
    }
  }

  if (!in->alpn_selected.empty()) {
    if (!CBB_add_asn1(&s3, &child, kS3ALPNSelectedTag) ||
        !CBB_add_asn1_octet_string(&child, in->alpn_selected.data(),
                                   in->alpn_selected.size())) {
      return 0;
    }
  }

  if (!in->next_proto_negotiated.empty()) {
    if (!CBB_add_asn1(&s3, &child, kS3NextProtoNegotiatedTag) ||
        !CBB_add_asn1_octet_string(&child, in->next_proto_negotiated.data(),
                                   in->next_proto_negotiated.size())) {
      return 0;
    }
  }

  if (in->channel_id_valid) {
    if (!CBB_add_asn1(&s3, &child, kS3ChannelIdValidTag) ||
        !CBB_add_asn1_bool(&child, true)) {
      return 0;
    }
    if (!CBB_add_asn1(&s3, &child, kS3ChannelIdTag) ||
        !CBB_add_asn1_octet_string(&child, in->channel_id,
                                   SSL3_CHANNEL_ID_SIZE)) {
      return 0;
    }
  }

  if (in->send_connection_binding) {
    if (!CBB_add_asn1(&s3, &child, kS3SendConnectionBindingTag) ||
        !CBB_add_asn1_bool(&child, true)) {
      return 0;
    }
  }

  if (!in->pending_app_data.empty()) {
    if (!CBB_add_asn1(&s3, &child, kS3PendingAppDataTag) ||
        !in->read_buffer.SerializeBufferView(child, in->pending_app_data)) {
      return 0;
    }
  }

  if (!in->pending_app_data.empty() || !in->read_buffer.empty()) {
    if (!CBB_add_asn1(&s3, &child, kS3ReadBufferTag) ||
        !in->read_buffer.DoSerialization(child)) {
      return 0;
    }
  }
  // serialization of |not_resumable| is not added in |ssl_session_serialize|
  // because the function is used to serialize a session for resumption.
  if (in->established_session.get()->not_resumable) {
    if (!CBB_add_asn1(&s3, &child, kS3NotResumableTag) ||
        !CBB_add_asn1_bool(&child, true)) {
      return 0;
    }
  }

  // Version 2 and higher extensions starts below, all which are optional as they are
  // TLS 1.3 specific.
  if (protocol_version >= TLS1_3_VERSION) {
    if (!CBB_add_asn1(&s3, &child, kS3EarlyDataSkippedTag) ||
        !CBB_add_asn1_uint64(&child, in->early_data_skipped)) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_MALLOC_FAILURE);
      return 0;
    }

    if (!CBB_add_asn1(&s3, &child, kS3DelegatedCredentialUsedTag) ||
        !CBB_add_asn1_bool(&child, in->delegated_credential_used)) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_MALLOC_FAILURE);
      return 0;
    }

    if (!CBB_add_asn1(&s3, &child, kS3EarlyDataAcceptedTag) ||
        !CBB_add_asn1_bool(&child, in->early_data_accepted)) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_MALLOC_FAILURE);
      return 0;
    }

    if (!CBB_add_asn1(&s3, &child, kS3UsedHelloRetryRequestTag) ||
        !CBB_add_asn1_bool(&child, in->used_hello_retry_request)) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_MALLOC_FAILURE);
      return 0;
    }

    if (!CBB_add_asn1(&s3, &child, kS3TicketAgeSkewTag) ||
        !CBB_add_asn1_int64(&child, in->ticket_age_skew)) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_MALLOC_FAILURE);
      return 0;
    }

    if (!CBB_add_asn1(&s3, &child, kS3WriteTrafficSecretTag) ||
        !CBB_add_asn1_octet_string(&child, in->write_traffic_secret,
                                   in->write_traffic_secret_len)) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_MALLOC_FAILURE);
      return 0;
    }

    if (!CBB_add_asn1(&s3, &child, kS3WriteTrafficSecretLenTag) ||
        !CBB_add_asn1_uint64(&child, in->write_traffic_secret_len)) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_MALLOC_FAILURE);
      return 0;
    }

    if (!CBB_add_asn1(&s3, &child, kS3ReadTrafficSecretTag) ||
        !CBB_add_asn1_octet_string(&child, in->read_traffic_secret,
                                   in->read_traffic_secret_len)) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_MALLOC_FAILURE);
      return 0;
    }

    if (!CBB_add_asn1(&s3, &child, kS3ReadTrafficSecretLenTag) ||
        !CBB_add_asn1_uint64(&child, in->read_traffic_secret_len)) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_MALLOC_FAILURE);
      return 0;
    }

    if (!CBB_add_asn1(&s3, &child, kS3ExporterSecretTag) ||
        !CBB_add_asn1_octet_string(&child, in->exporter_secret,
                                   in->exporter_secret_len)) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_MALLOC_FAILURE);
      return 0;
    }

    if (!CBB_add_asn1(&s3, &child, kS3ExporterSecretLenTag) ||
        !CBB_add_asn1_uint64(&child, in->exporter_secret_len)) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_MALLOC_FAILURE);
      return 0;
    }

    if (in->hs_buf && in->hs_buf->length > 0) {
      if (!CBB_add_asn1(&s3, &child, kS3HandshakeBufferTag) ||
          !CBB_add_asn1_octet_string(
              &child, reinterpret_cast<uint8_t *>(in->hs_buf->data),
              in->hs_buf->length)) {
        OPENSSL_PUT_ERROR(SSL, ERR_R_MALLOC_FAILURE);
        return 0;
      }
    }

    if (in->ech_status) {
      if (!CBB_add_asn1(&s3, &child, kS3EchStatusTag) ||
          !CBB_add_asn1_uint64(&child, in->ech_status)) {
        OPENSSL_PUT_ERROR(SSL, ERR_R_MALLOC_FAILURE);
        return 0;
      }
    }

    if (in->pending_hs_data &&
        (in->pending_hs_data->length > 0 || in->pending_hs_data->max > 0)) {
      if (!serialize_buf_mem(in->pending_hs_data, kS3PendingHsDataTag, s3)) {
        OPENSSL_PUT_ERROR(SSL, ERR_R_MALLOC_FAILURE);
        return 0;
      }
    }

    if (in->pending_flight &&
        (in->pending_flight->length > 0 || in->pending_flight->max > 0)) {
      if (!serialize_buf_mem(in->pending_flight, kS3PendingFlightTag, s3)) {
        OPENSSL_PUT_ERROR(SSL, ERR_R_MALLOC_FAILURE);
        return 0;
      }
    }

    if (in->aead_read_ctx) {
      if (!CBB_add_asn1(&s3, &child, kS3AeadReadCtxTag) ||
          !in->aead_read_ctx.get()->SerializeState(&child)) {
        OPENSSL_PUT_ERROR(SSL, ERR_R_MALLOC_FAILURE);
        return 0;
      }
    }

    if (in->aead_write_ctx) {
      if (!CBB_add_asn1(&s3, &child, kS3AeadWriteCtxTag) ||
          !in->aead_write_ctx.get()->SerializeState(&child)) {
        OPENSSL_PUT_ERROR(SSL, ERR_R_MALLOC_FAILURE);
        return 0;
      }
    }

    if (!CBB_add_asn1(&s3, &child, kS3KeyUpdatePending) ||
        !CBB_add_asn1_int64(&child, in->key_update_pending)) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_MALLOC_FAILURE);
      return 0;
    }
  }

  return CBB_flush(cbb);
}

static int SSL3_STATE_parse_session(CBS *cbs, UniquePtr<SSL_SESSION> *out,
                                    const SSL_CTX *ctx) {
  CBS value;
  int present;
  if (!CBS_get_optional_asn1(cbs, &value, &present, kS3EstablishedSessionTag)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL3_STATE);
    return 0;
  }
  if (present) {
    UniquePtr<SSL_SESSION> ptr =
        SSL_SESSION_parse(&value, ctx->x509_method, ctx->pool);
    if (!ptr) {
      return 0;
    }
    out->reset(ptr.release());
    return 1;
  } else {
    // session should exist because ssl transfer only supports SSL completes
    // handshake.
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL3_STATE);
    out->reset();
    return 0;
  }
}

// SSL3_STATE_from_bytes recovers SSL3_STATE from |cbs|.
// |ssl| is used because |tls1_configure_aead| is used to recover
// |aead_read_ctx| and |aead_write_ctx|.
static int SSL3_STATE_from_bytes(SSL *ssl, CBS *cbs, const SSL_CTX *ctx) {
  // We expect the caller to have configured |ssl| with the protocol
  // version prior to calling us.
  uint16_t protocol_version;
  if (!ssl_protocol_version_from_wire(&protocol_version, ssl->version)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL3_STATE);
    return 0;
  }
  bool is_tls13 = protocol_version >= TLS1_3_VERSION;

  SSL3_STATE *out = ssl->s3;
  CBS s3, read_seq, write_seq, server_random, client_random, send_alert,
      pending_app_data, read_buffer;
  CBS previous_client_finished, previous_server_finished;
  int session_reused, channel_id_valid, send_connection_binding, not_resumable;
  uint64_t serde_version, early_data_reason, previous_client_finished_len,
      previous_server_finished_len;
  uint64_t empty_record_count, warning_alert_count, total_renegotiations;
  int64_t rwstate;
  int pending_app_data_present, read_buffer_present;
  if (!CBS_get_asn1(cbs, &s3, CBS_ASN1_SEQUENCE) ||
      !CBS_get_asn1_uint64(&s3, &serde_version) ||
      serde_version > SSL3_STATE_SERDE_VERSION_THREE ||
      (is_tls13 && serde_version < SSL3_STATE_SERDE_VERSION_TWO) ||
      !CBS_get_asn1(&s3, &read_seq, CBS_ASN1_OCTETSTRING) ||
      CBS_len(&read_seq) != TLS_SEQ_NUM_SIZE ||
      !CBS_get_asn1(&s3, &write_seq, CBS_ASN1_OCTETSTRING) ||
      CBS_len(&write_seq) != TLS_SEQ_NUM_SIZE ||
      !CBS_get_asn1(&s3, &server_random, CBS_ASN1_OCTETSTRING) ||
      CBS_len(&server_random) != SSL3_RANDOM_SIZE ||
      !CBS_get_asn1(&s3, &client_random, CBS_ASN1_OCTETSTRING) ||
      CBS_len(&client_random) != SSL3_RANDOM_SIZE ||
      !CBS_get_asn1(&s3, &send_alert, CBS_ASN1_OCTETSTRING) ||
      CBS_len(&send_alert) != SSL3_SEND_ALERT_SIZE ||
      !CBS_get_asn1_int64(&s3, &rwstate) ||
      !CBS_get_asn1_uint64(&s3, &early_data_reason) ||
      early_data_reason > ssl_early_data_reason_max_value ||
      !CBS_get_asn1(&s3, &previous_client_finished, CBS_ASN1_OCTETSTRING) ||
      !CBS_get_asn1_uint64(&s3, &previous_client_finished_len) ||
      previous_client_finished_len > PREV_FINISHED_MAX_SIZE ||
      (serde_version >= SSL3_STATE_SERDE_VERSION_THREE &&
       CBS_len(&previous_client_finished) != previous_client_finished_len) ||
      (serde_version < SSL3_STATE_SERDE_VERSION_THREE &&
       CBS_len(&previous_client_finished) > PREV_FINISHED_MAX_SIZE) ||
      !CBS_get_asn1(&s3, &previous_server_finished, CBS_ASN1_OCTETSTRING) ||
      !CBS_get_asn1_uint64(&s3, &previous_server_finished_len) ||
      previous_server_finished_len > PREV_FINISHED_MAX_SIZE ||
      (serde_version >= SSL3_STATE_SERDE_VERSION_THREE &&
       CBS_len(&previous_server_finished) != previous_server_finished_len) ||
      (serde_version < SSL3_STATE_SERDE_VERSION_THREE &&
       CBS_len(&previous_server_finished) > PREV_FINISHED_MAX_SIZE) ||
      !CBS_get_asn1_uint64(&s3, &empty_record_count) ||
      !CBS_get_asn1_uint64(&s3, &warning_alert_count) ||
      !CBS_get_asn1_uint64(&s3, &total_renegotiations) ||
      !SSL3_STATE_parse_session(&s3, &(out->established_session), ctx) ||
      !CBS_get_optional_asn1_bool(&s3, &session_reused, kS3SessionReusedTag,
                                  0 /* default to false */) ||
      !parse_optional_string(&s3, &(out->hostname), kS3HostNameTag,
                             SSL_R_SERIALIZATION_INVALID_SSL3_STATE) ||
      !SSL3_STATE_parse_octet_string(&s3, &(out->alpn_selected),
                                     kS3ALPNSelectedTag) ||
      !SSL3_STATE_parse_octet_string(&s3, &(out->next_proto_negotiated),
                                     kS3NextProtoNegotiatedTag) ||
      !CBS_get_optional_asn1_bool(&s3, &channel_id_valid, kS3ChannelIdValidTag,
                                  0 /* default to false */) ||
      !SSL3_STATE_get_optional_octet_string(
          &s3, out->channel_id, kS3ChannelIdTag, SSL3_CHANNEL_ID_SIZE) ||
      !CBS_get_optional_asn1_bool(&s3, &send_connection_binding,
                                  kS3SendConnectionBindingTag,
                                  0 /* default to false */) ||
      !CBS_get_optional_asn1(&s3, &pending_app_data, &pending_app_data_present,
                             kS3PendingAppDataTag) ||
      !CBS_get_optional_asn1(&s3, &read_buffer, &read_buffer_present,
                             kS3ReadBufferTag) ||
      !CBS_get_optional_asn1_bool(&s3, &not_resumable, kS3NotResumableTag,
                                  0 /* default to false */)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL3_STATE);
    return 0;
  }

  // We should have no more data at this point if we are deserializing v1
  // encoding.
  if (serde_version < SSL3_STATE_SERDE_VERSION_TWO && CBS_len(&s3) > 0) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL3_STATE);
    return 0;
  }

  uint64_t early_data_skipped;
  if (!CBS_get_optional_asn1_uint64(&s3, &early_data_skipped,
                                    kS3EarlyDataSkippedTag,
                                    0 /* default to 0 */) ||
      early_data_skipped > UINT16_MAX) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL3_STATE);
    return 0;
  }
  out->early_data_skipped = static_cast<uint16_t>(early_data_skipped);

  int delegated_credential_used;
  if (!CBS_get_optional_asn1_bool(&s3, &delegated_credential_used,
                                  kS3DelegatedCredentialUsedTag, 0)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL3_STATE);
    return 0;
  }
  out->delegated_credential_used = delegated_credential_used != 0;

  int early_data_accepted;
  if (!CBS_get_optional_asn1_bool(&s3, &early_data_accepted,
                                  kS3EarlyDataAcceptedTag, 0)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL3_STATE);
    return 0;
  }
  out->early_data_accepted = early_data_accepted != 0;

  int used_hello_retry_request;
  if (!CBS_get_optional_asn1_bool(&s3, &used_hello_retry_request,
                                  kS3UsedHelloRetryRequestTag, 0)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL3_STATE);
    return 0;
  }
  out->used_hello_retry_request = used_hello_retry_request != 0;

  int64_t ticket_age_skew;
  if (!CBS_get_optional_asn1_int64(&s3, &ticket_age_skew, kS3TicketAgeSkewTag,
                                   0) ||
      ticket_age_skew > INT32_MAX || ticket_age_skew < INT32_MIN) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL3_STATE);
    return 0;
  }
  out->ticket_age_skew = static_cast<int32_t>(ticket_age_skew);

  CBS write_traffic_secret;
  int write_traffic_secret_present;
  if (!CBS_get_optional_asn1_octet_string(&s3, &write_traffic_secret,
                                          &write_traffic_secret_present,
                                          kS3WriteTrafficSecretTag)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL3_STATE);
    return 0;
  }

  uint64_t write_traffic_secret_len;
  if (!CBS_get_optional_asn1_uint64(&s3, &write_traffic_secret_len,
                                    kS3WriteTrafficSecretLenTag, 0)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL3_STATE);
    return 0;
  }

  CBS read_traffic_secret;
  int read_traffic_secret_present;
  if (!CBS_get_optional_asn1_octet_string(&s3, &read_traffic_secret,
                                          &read_traffic_secret_present,
                                          kS3ReadTrafficSecretTag)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL3_STATE);
    return 0;
  }

  uint64_t read_traffic_secret_len;
  if (!CBS_get_optional_asn1_uint64(&s3, &read_traffic_secret_len,
                                    kS3ReadTrafficSecretLenTag, 0)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL3_STATE);
    return 0;
  }

  CBS exporter_secret;
  int exporter_secret_present;
  if (!CBS_get_optional_asn1_octet_string(&s3, &exporter_secret,
                                          &exporter_secret_present,
                                          kS3ExporterSecretTag)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL3_STATE);
    return 0;
  }

  uint64_t exporter_secret_len;
  if (!CBS_get_optional_asn1_uint64(&s3, &exporter_secret_len,
                                    kS3ExporterSecretLenTag, 0)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL3_STATE);
    return 0;
  }

  CBS hs_buf;
  int hs_buf_present;
  if (!CBS_get_optional_asn1_octet_string(&s3, &hs_buf, &hs_buf_present,
                                          kS3HandshakeBufferTag)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL3_STATE);
    return 0;
  }

  uint64_t ech_status;
  if (!CBS_get_optional_asn1_uint64(&s3, &ech_status, kS3EchStatusTag,
                                    ssl_ech_none)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL3_STATE);
    return 0;
  }

  bssl::UniquePtr<BUF_MEM> pending_hs_data;
  int pending_hs_data_present;

  if (!deserialize_buf_mem(pending_hs_data, pending_hs_data_present,
                           kS3PendingHsDataTag, s3)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL3_STATE);
    return 0;
  }

  bssl::UniquePtr<BUF_MEM> pending_flight;
  int pending_flight_present;
  if (!deserialize_buf_mem(pending_flight, pending_flight_present,
                           kS3PendingFlightTag, s3)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL3_STATE);
    return 0;
  }

  CBS aead_read_ctx;
  int aead_read_ctx_present;
  if (!CBS_get_optional_asn1(&s3, &aead_read_ctx, &aead_read_ctx_present,
                             kS3AeadReadCtxTag)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL3_STATE);
    return 0;
  }

  CBS aead_write_ctx;
  int aead_write_ctx_present;
  if (!CBS_get_optional_asn1(&s3, &aead_write_ctx, &aead_write_ctx_present,
                             kS3AeadWriteCtxTag)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL3_STATE);
    return 0;
  }

  int64_t key_update_pending = SSL_KEY_UPDATE_NONE;
  if (!CBS_get_optional_asn1_int64(&s3, &key_update_pending,
                                   kS3KeyUpdatePending, SSL_KEY_UPDATE_NONE)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL3_STATE);
    return 0;
  }

  if (is_tls13) {
    if (!write_traffic_secret_present || !write_traffic_secret_len ||
        write_traffic_secret_len > UINT8_MAX ||
        write_traffic_secret_len > SSL_MAX_MD_SIZE ||
        (serde_version >= SSL3_STATE_SERDE_VERSION_THREE &&
         CBS_len(&write_traffic_secret) != write_traffic_secret_len) ||
        (serde_version < SSL3_STATE_SERDE_VERSION_THREE &&
         CBS_len(&write_traffic_secret) != SSL_MAX_MD_SIZE) ||
        !read_traffic_secret_present || !read_traffic_secret_len ||
        read_traffic_secret_len > UINT8_MAX ||
        read_traffic_secret_len > SSL_MAX_MD_SIZE ||
        (serde_version >= SSL3_STATE_SERDE_VERSION_THREE &&
         CBS_len(&read_traffic_secret) != read_traffic_secret_len) ||
        (serde_version < SSL3_STATE_SERDE_VERSION_THREE &&
         CBS_len(&read_traffic_secret) != SSL_MAX_MD_SIZE) ||
        !exporter_secret_present || !exporter_secret_len ||
        exporter_secret_len > UINT8_MAX ||
        exporter_secret_len > SSL_MAX_MD_SIZE ||
        (serde_version >= SSL3_STATE_SERDE_VERSION_THREE &&
         CBS_len(&exporter_secret) != exporter_secret_len) ||
        (serde_version < SSL3_STATE_SERDE_VERSION_THREE &&
         CBS_len(&exporter_secret) != SSL_MAX_MD_SIZE) ||
        ech_status > ssl_ech_rejected || !aead_read_ctx_present ||
        !aead_write_ctx_present ||
        !(key_update_pending == SSL_KEY_UPDATE_NONE ||
          key_update_pending == SSL_KEY_UPDATE_NOT_REQUESTED ||
          key_update_pending == SSL_KEY_UPDATE_REQUESTED)) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL3_STATE);
      return 0;
    }

    OPENSSL_memcpy(out->exporter_secret, CBS_data(&exporter_secret),
                   exporter_secret_len);
    out->exporter_secret_len = exporter_secret_len;

    out->hs_buf.reset(BUF_MEM_new());
    if (hs_buf_present) {
      if (!BUF_MEM_append(out->hs_buf.get(), CBS_data(&hs_buf),
                          CBS_len(&hs_buf))) {
        OPENSSL_PUT_ERROR(SSL, ERR_R_MALLOC_FAILURE);
        return 0;
      }
    }

    out->ech_status = static_cast<ssl_ech_status_t>(ech_status);
  } else if (write_traffic_secret_present || write_traffic_secret_len ||
             read_traffic_secret_present || read_traffic_secret_len ||
             exporter_secret_present || exporter_secret_len || hs_buf_present ||
             ech_status || pending_hs_data_present || pending_flight_present) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL3_STATE);
    return 0;
  }

  if (read_buffer_present &&
      !out->read_buffer.DoDeserialization(read_buffer)) {
    return 0;
  }
  // If |pending_app_data_size| is not zero, it needs to point to |read_buffer|.
  if (pending_app_data_present) {
    if (!read_buffer_present || !out->read_buffer.DeserializeBufferView(
                                    pending_app_data, out->pending_app_data)) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL3_STATE);
      return 0;
    }
  }

  if (CBS_len(&s3)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL3_STATE);
    return 0;
  }

  // Initialize some states before call |tls1_configure_aead|.
  // Below comment is copied from |SSL_do_handshake|.
  // Destroy the handshake object if the handshake has completely finished.
  out->hs.reset();
  // have_version is true if the connection's final version is known. Otherwise
  // the version has not been negotiated yet.
  out->have_version = true;
  OPENSSL_memcpy(out->server_random, CBS_data(&server_random),
                 SSL3_RANDOM_SIZE);
  OPENSSL_memcpy(out->client_random, CBS_data(&client_random),
                 SSL3_RANDOM_SIZE);
  SSL_SESSION *session = out->established_session.get();
  if (is_tls13) {
    if (!tls13_set_traffic_key(ssl, ssl_encryption_application, evp_aead_seal,
                               session,
                               MakeSpan(CBS_data(&write_traffic_secret),
                                        write_traffic_secret_len))) {
      return 0;
    }
    if (!tls13_set_traffic_key(ssl, ssl_encryption_application, evp_aead_open,
                               session,
                               MakeSpan(CBS_data(&read_traffic_secret),
                                        read_traffic_secret_len))) {
      return 0;
    }

    // !! We set pending_hs_data and pending_hs_flight after as to avoid
    // tls13_set_traffic_key from trying to flush their contents if they are not
    // empty !!

    if (pending_hs_data_present) {
      out->pending_hs_data = std::move(pending_hs_data);
    } else {
      out->pending_hs_data.reset(BUF_MEM_new());
    }

    if (pending_flight_present) {
      out->pending_flight = std::move(pending_flight);
    } else {
      out->pending_flight.reset(BUF_MEM_new());
    }
  } else {
    // the impl of |SSL_serialize_handback|, which only fetch IV when it's
    // TLS 1.
    Array<uint8_t> key_block1, key_block2;
    if (!tls1_configure_aead(ssl, evp_aead_seal, &key_block1, session, {})) {
      return 0;
    }
    if (!tls1_configure_aead(ssl, evp_aead_open, &key_block2, session, {})) {
      return 0;
    }
  }
  if (aead_read_ctx_present &&
      !out->aead_read_ctx->DeserializeState(&aead_read_ctx)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL3_STATE);
    return 0;
  }
  if (aead_write_ctx_present &&
      !out->aead_write_ctx->DeserializeState(&aead_write_ctx)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL3_STATE);
    return 0;
  }
  // read_sequence & write_sequence must be set AFTER setting the traffic keys,
  // otherwise would be reset back to zero.
  OPENSSL_memcpy(out->read_sequence, CBS_data(&read_seq), TLS_SEQ_NUM_SIZE);
  OPENSSL_memcpy(out->write_sequence, CBS_data(&write_seq), TLS_SEQ_NUM_SIZE);
  OPENSSL_memcpy(out->send_alert, CBS_data(&send_alert), SSL3_SEND_ALERT_SIZE);
  OPENSSL_memcpy(out->previous_client_finished,
                 CBS_data(&previous_client_finished), previous_client_finished_len);
  OPENSSL_memcpy(out->previous_server_finished,
                 CBS_data(&previous_server_finished), previous_server_finished_len);
  out->early_data_reason =
      static_cast<ssl_early_data_reason_t>(early_data_reason);
  out->rwstate = rwstate;
  out->session_reused = !!session_reused;
  if (out->session_reused) {
    ssl->session = UpRef(out->established_session);
  }
  out->channel_id_valid = !!channel_id_valid;
  out->previous_client_finished_len = previous_client_finished_len;
  out->previous_server_finished_len = previous_server_finished_len;
  out->v2_hello_done = true;
  out->initial_handshake_complete = true;
  out->empty_record_count = empty_record_count;
  out->warning_alert_count = warning_alert_count;
  out->total_renegotiations = total_renegotiations;
  out->send_connection_binding = !!send_connection_binding;
  out->established_session.get()->not_resumable = !!not_resumable;
  out->key_update_pending = key_update_pending;
  return 1;
}

// SSL_CONFIG serialization.

static const unsigned kSSLConfigVersion = 2;

static const unsigned kSSLConfigOcspStaplingEnabledTag =
    CBS_ASN1_CONSTRUCTED | CBS_ASN1_CONTEXT_SPECIFIC | 0;

static const unsigned kSSLConfigJdk11WorkaroundTag =
    CBS_ASN1_CONSTRUCTED | CBS_ASN1_CONTEXT_SPECIFIC | 1;

static const unsigned kSSLConfigConfMaxVersionUseDefault =
    CBS_ASN1_CONSTRUCTED | CBS_ASN1_CONTEXT_SPECIFIC | 2;

static const unsigned kSSLConfigConfMinVersionUseDefault =
    CBS_ASN1_CONSTRUCTED | CBS_ASN1_CONTEXT_SPECIFIC | 3;

// *** EXPERIMENTAL — DO NOT USE WITHOUT CHECKING ***
// These SSL_CONFIG serialization functions are developed to support SSL
// transfer. Most fields of SSL_CONFIG are not used after handshake completes.
// It only encodes some fields needed by SSL_*_getter functions.

// SSL_CONFIG_to_bytes serializes |in| to bytes stored in |cbb|.
// It returns one on success and zero on failure.
//
// SSL_CONFIG ASN.1 structure: see ssl_transfer_asn1.cc
static int SSL_CONFIG_to_bytes(SSL_CONFIG *in, CBB *cbb) {
  if (in == NULL || cbb == NULL) {
    return 0;
  }

  CBB config, child;
  if (!CBB_add_asn1(cbb, &config, CBS_ASN1_SEQUENCE) ||
      !CBB_add_asn1_uint64(&config, kSSLConfigVersion) ||
      !CBB_add_asn1_uint64(&config, in->conf_max_version) ||
      !CBB_add_asn1_uint64(&config, in->conf_min_version)) {
    return 0;
  }

  if (in->ocsp_stapling_enabled) {
    if (!CBB_add_asn1(&config, &child, kSSLConfigOcspStaplingEnabledTag) ||
        !CBB_add_asn1_bool(&child, true)) {
      return 0;
    }
  }
  if (in->jdk11_workaround) {
    if (!CBB_add_asn1(&config, &child, kSSLConfigJdk11WorkaroundTag) ||
        !CBB_add_asn1_bool(&child, true)) {
      return 0;
    }
  }
  if (in->conf_max_version_use_default) {
    if (!CBB_add_asn1(&config, &child, kSSLConfigConfMaxVersionUseDefault) ||
        !CBB_add_asn1_bool(&child, true)) {
      return 0;
    }
  }
  if (in->conf_min_version_use_default) {
    if (!CBB_add_asn1(&config, &child, kSSLConfigConfMinVersionUseDefault) ||
        !CBB_add_asn1_bool(&child, true)) {
      return 0;
    }
  }
  return CBB_flush(cbb);
}

static int SSL_CONFIG_from_bytes(SSL_CONFIG *out, CBS *cbs) {
  CBS config;
  int ocsp_stapling_enabled, jdk11_workaround;
  int conf_max_version_use_default, conf_min_version_use_default;
  uint64_t version, conf_max_version, conf_min_version;
  if (!CBS_get_asn1(cbs, &config, CBS_ASN1_SEQUENCE) ||
      !CBS_get_asn1_uint64(&config, &version) || version > kSSLConfigVersion ||
      !CBS_get_asn1_uint64(&config, &conf_max_version) ||
      !CBS_get_asn1_uint64(&config, &conf_min_version) ||
      !CBS_get_optional_asn1_bool(&config, &ocsp_stapling_enabled,
                                  kSSLConfigOcspStaplingEnabledTag,
                                  0 /* default to false */) ||
      !CBS_get_optional_asn1_bool(&config, &jdk11_workaround,
                                  kSSLConfigJdk11WorkaroundTag,
                                  0 /* default to false */)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL_CONFIG);
    return 0;
  }

  bool is_atleast_v2 = (version >= 2);

  if (!is_atleast_v2 && CBS_len(&config) != 0) {
    // We are parsing version 1 of config format, but there is more data to
    // parse than expected. That is, the input is not v1 formatted.
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL_CONFIG);
    return 0;
  }

  if (!CBS_get_optional_asn1_bool(&config, &conf_max_version_use_default,
                                  kSSLConfigConfMaxVersionUseDefault,
                                  1 /* default to true */) ||
      !CBS_get_optional_asn1_bool(&config, &conf_min_version_use_default,
                                  kSSLConfigConfMinVersionUseDefault,
                                  1 /* default to true */) ||
      CBS_len(&config) != 0) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL_CONFIG);
    return 0;
  }

  out->conf_max_version = conf_max_version;
  out->conf_min_version = conf_min_version;
  out->conf_max_version_use_default = !!conf_max_version_use_default;
  out->conf_min_version_use_default = !!conf_min_version_use_default;
  out->ocsp_stapling_enabled = !!ocsp_stapling_enabled;
  out->jdk11_workaround = !!jdk11_workaround;
  // handoff will always be the normal state(false) after handshake completes.
  out->handoff = false;
  // shed_handshake_config will always be false if config can be encoded(not
  // sheded).
  out->shed_handshake_config = false;
  return 1;
}

// SSL serialization.

// Serialized SSL data version for forward compatibility
#define SSL_SERIAL_VERSION 1

static const unsigned kSSLQuietShutdownTag =
    CBS_ASN1_CONSTRUCTED | CBS_ASN1_CONTEXT_SPECIFIC | 0;
static const unsigned kSSLConfigTag =
    CBS_ASN1_CONSTRUCTED | CBS_ASN1_CONTEXT_SPECIFIC | 1;
static const unsigned kSSLVerifyResultTag =
    CBS_ASN1_CONSTRUCTED | CBS_ASN1_CONTEXT_SPECIFIC | 2;

// Parse serialized SSL connection binary
//
// SSL_to_bytes_full serializes |in| to bytes stored in |cbb|.
// It returns one on success and zero on failure.
//
// SSL ASN.1 structure: see ssl_transfer_asn1.cc
static int SSL_to_bytes_full(const SSL *in, CBB *cbb) {
  CBB ssl, child;

  if (!CBB_add_asn1(cbb, &ssl, CBS_ASN1_SEQUENCE) ||
      !CBB_add_asn1_uint64(&ssl, SSL_SERIAL_VERSION) ||
      //    FIXME add hash of SSL_CTX
      !CBB_add_asn1_uint64(&ssl, in->version) ||
      !CBB_add_asn1_uint64(&ssl, in->max_send_fragment) ||
      !SSL3_STATE_to_bytes(in->s3, ssl_protocol_version(in), &ssl) ||
      !CBB_add_asn1_uint64(&ssl, in->mode) ||
      !CBB_add_asn1_uint64(&ssl, in->options)) {
    return 0;
  }

  if (in->quiet_shutdown) {
    if (!CBB_add_asn1(&ssl, &child, kSSLQuietShutdownTag) ||
        !CBB_add_asn1_bool(&child, true)) {
      return 0;
    }
  }

  if (in->config) {
    if (!CBB_add_asn1(&ssl, &child, kSSLConfigTag) ||
        !SSL_CONFIG_to_bytes(in->config.get(), &child)) {
      return 0;
    }
  }

  if (!CBB_add_asn1(&ssl, &child, kSSLVerifyResultTag) ||
      !CBB_add_asn1_int64(&child, in->verify_result)) {
    return 0;
  }

  return CBB_flush(cbb);
}

static int SSL_parse(SSL *ssl, CBS *cbs, SSL_CTX *ctx) {
  CBS ssl_cbs, ssl_config;
  uint64_t ssl_serial_ver, version, max_send_fragment, mode, options;
  int64_t verify_result = X509_V_ERR_INVALID_CALL;
  int quiet_shutdown;
  int ssl_config_present = 0;

  if (!CBS_get_asn1(cbs, &ssl_cbs, CBS_ASN1_SEQUENCE) || CBS_len(cbs) != 0) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL);
    return 0;
  }

  if (!CBS_get_asn1_uint64(&ssl_cbs, &ssl_serial_ver) ||
      ssl_serial_ver > SSL_SERIAL_VERSION) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SERDE_VERSION);
    return 0;
  }

  //    FIXME add hash of SSL_CTX
  // This TODO is actually a part of SSL DER struct revisit.
  if (!CBS_get_asn1_uint64(&ssl_cbs, &version) ||
      !CBS_get_asn1_uint64(&ssl_cbs, &max_send_fragment)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL);
    return 0;
  }

  ssl->version = version;
  ssl->max_send_fragment = max_send_fragment;

  SSL_set_accept_state(ssl);

  // This is called separately to avoid overriding error code.
  // This function expects ssl->version to have been set, otherwise will error!
  if (!SSL3_STATE_from_bytes(ssl, &ssl_cbs, ssl->ctx.get())) {
    return 0;
  }

  if (!CBS_get_asn1_uint64(&ssl_cbs, &mode) ||
      !CBS_get_asn1_uint64(&ssl_cbs, &options)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL);
    return 0;
  }
  ssl->mode = mode;
  ssl->options = options;

  if (!CBS_get_optional_asn1_bool(&ssl_cbs, &quiet_shutdown,
                                  kSSLQuietShutdownTag,
                                  0 /* default to false */)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL);
    return 0;
  }
  ssl->quiet_shutdown = !!quiet_shutdown;

  if (!CBS_get_optional_asn1(&ssl_cbs, &ssl_config, &ssl_config_present,
                             kSSLConfigTag)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL);
    return 0;
  }

  if (ssl_config_present) {
    if (!SSL_CONFIG_from_bytes(ssl->config.get(), &ssl_config)) {
      return 0;
    }
  } else {
    ssl->config.reset();
  }

  if (!CBS_get_optional_asn1_int64(
          &ssl_cbs, &verify_result, kSSLVerifyResultTag,
          ssl->s3->established_session->verify_result)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL);
    return 0;
  }

  if (CBS_len(&ssl_cbs) != 0) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL);
    return 0;
  }

  ssl->verify_result = verify_result;
  ssl->is_suspended_state = false;

  return 1;
}

SSL *SSL_from_bytes(const uint8_t *in, size_t in_len, SSL_CTX *ctx) {
  if (!in || !in_len || !ctx) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_UNSUPPORTED);
    return NULL;
  }

  SSL *ssl = SSL_new(ctx);
  if (ssl == NULL) {
    return NULL;
  }
  UniquePtr<SSL> ret(ssl);

  CBS cbs, seq;
  CBS_init(&cbs, in, in_len);
  if (!CBS_get_asn1(&cbs, &seq, CBS_ASN1_SEQUENCE) || (CBS_len(&cbs) != 0)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL);
    return NULL;
  }

  // Restore SSL part
  if (!SSL_parse(ret.get(), &seq, ctx)) {
    return NULL;
  }

  return ret.release();
}

int SSL_to_bytes(const SSL *in, uint8_t **out_data, size_t *out_len) {
  if (!ssl_transfer_supported(in)) {
    return 0;
  }

  ScopedCBB cbb;

  CBB seq;
  if (!CBB_init(cbb.get(), 1024) ||
      !CBB_add_asn1(cbb.get(), &seq, CBS_ASN1_SEQUENCE) ||
      !SSL_to_bytes_full(in, &seq)) {
    return 0;
  }

  const_cast<SSL *>(in)->is_suspended_state = true;

  return CBB_finish(cbb.get(), out_data, out_len);
}

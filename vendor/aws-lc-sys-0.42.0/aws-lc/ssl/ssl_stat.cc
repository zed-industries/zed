// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com)
// Copyright 2005 Nokia. All rights reserved.
//
// The Contribution, originally written by Mika Kousa and Pasi Eronen of
// Nokia Corporation, consists of the "PSK" (Pre-Shared Key) ciphersuites
// support (see RFC 4279) to OpenSSL.
//
// SPDX-License-Identifier: Apache-2.0

#include <openssl/ssl.h>

#include <assert.h>

#include "internal.h"


const char *SSL_state_string_long(const SSL *ssl) {
  if (ssl->s3->hs == nullptr) {
    return "SSL negotiation finished successfully";
  }

  return ssl->server ? ssl_server_handshake_state(ssl->s3->hs.get())
                     : ssl_client_handshake_state(ssl->s3->hs.get());
}

const char *SSL_state_string(const SSL *ssl) {
  return SSL_state_string_long(ssl);
}

const char *SSL_alert_type_string_long(int value) {
  value &= 0x0f00;
  value >>= 8;
  if (value == SSL3_AL_WARNING) {
    return "warning";
  } else if (value == SSL3_AL_FATAL) {
    return "fatal";
  }

  return "unknown";
}

const char *SSL_alert_type_string(int value) {
  value &= 0x0f00;
  value >>= 8;
  if (value == SSL3_AL_WARNING) {
    return "W";
  } else if (value == SSL3_AL_FATAL) {
    return "F";
  }

  return "U";
}

const char *SSL_alert_desc_string(int value) {
  switch (value & 0xff) {
    case SSL3_AD_CLOSE_NOTIFY:
      return "CN";

    case SSL3_AD_UNEXPECTED_MESSAGE:
      return "UM";

    case SSL3_AD_BAD_RECORD_MAC:
      return "BM";

    case SSL3_AD_DECOMPRESSION_FAILURE:
      return "DF";

    case SSL3_AD_HANDSHAKE_FAILURE:
      return "HF";

    case SSL3_AD_NO_CERTIFICATE:
      return "NC";

    case SSL3_AD_BAD_CERTIFICATE:
      return "BC";

    case SSL3_AD_UNSUPPORTED_CERTIFICATE:
      return "UC";

    case SSL3_AD_CERTIFICATE_REVOKED:
      return "CR";

    case SSL3_AD_CERTIFICATE_EXPIRED:
      return "CE";

    case SSL3_AD_CERTIFICATE_UNKNOWN:
      return "CU";

    case SSL3_AD_ILLEGAL_PARAMETER:
      return "IP";

    case TLS1_AD_DECRYPTION_FAILED:
      return "DC";

    case TLS1_AD_RECORD_OVERFLOW:
      return "RO";

    case TLS1_AD_UNKNOWN_CA:
      return "CA";

    case TLS1_AD_ACCESS_DENIED:
      return "AD";

    case TLS1_AD_DECODE_ERROR:
      return "DE";

    case TLS1_AD_DECRYPT_ERROR:
      return "CY";

    case TLS1_AD_EXPORT_RESTRICTION:
      return "ER";

    case TLS1_AD_PROTOCOL_VERSION:
      return "PV";

    case TLS1_AD_INSUFFICIENT_SECURITY:
      return "IS";

    case TLS1_AD_INTERNAL_ERROR:
      return "IE";

    case TLS1_AD_USER_CANCELLED:
      return "US";

    case TLS1_AD_NO_RENEGOTIATION:
      return "NR";

    case TLS1_AD_UNSUPPORTED_EXTENSION:
      return "UE";

    case TLS1_AD_CERTIFICATE_UNOBTAINABLE:
      return "CO";

    case TLS1_AD_UNRECOGNIZED_NAME:
      return "UN";

    case TLS1_AD_BAD_CERTIFICATE_STATUS_RESPONSE:
      return "BR";

    case TLS1_AD_BAD_CERTIFICATE_HASH_VALUE:
      return "BH";

    case TLS1_AD_UNKNOWN_PSK_IDENTITY:
      return "UP";

    default:
      return "UK";
  }
}

const char *SSL_alert_desc_string_long(int value) {
  switch (value & 0xff) {
    case SSL3_AD_CLOSE_NOTIFY:
      return "close notify";

    case SSL3_AD_UNEXPECTED_MESSAGE:
      return "unexpected_message";

    case SSL3_AD_BAD_RECORD_MAC:
      return "bad record mac";

    case SSL3_AD_DECOMPRESSION_FAILURE:
      return "decompression failure";

    case SSL3_AD_HANDSHAKE_FAILURE:
      return "handshake failure";

    case SSL3_AD_NO_CERTIFICATE:
      return "no certificate";

    case SSL3_AD_BAD_CERTIFICATE:
      return "bad certificate";

    case SSL3_AD_UNSUPPORTED_CERTIFICATE:
      return "unsupported certificate";

    case SSL3_AD_CERTIFICATE_REVOKED:
      return "certificate revoked";

    case SSL3_AD_CERTIFICATE_EXPIRED:
      return "certificate expired";

    case SSL3_AD_CERTIFICATE_UNKNOWN:
      return "certificate unknown";

    case SSL3_AD_ILLEGAL_PARAMETER:
      return "illegal parameter";

    case TLS1_AD_DECRYPTION_FAILED:
      return "decryption failed";

    case TLS1_AD_RECORD_OVERFLOW:
      return "record overflow";

    case TLS1_AD_UNKNOWN_CA:
      return "unknown CA";

    case TLS1_AD_ACCESS_DENIED:
      return "access denied";

    case TLS1_AD_DECODE_ERROR:
      return "decode error";

    case TLS1_AD_DECRYPT_ERROR:
      return "decrypt error";

    case TLS1_AD_EXPORT_RESTRICTION:
      return "export restriction";

    case TLS1_AD_PROTOCOL_VERSION:
      return "protocol version";

    case TLS1_AD_INSUFFICIENT_SECURITY:
      return "insufficient security";

    case TLS1_AD_INTERNAL_ERROR:
      return "internal error";

    case SSL3_AD_INAPPROPRIATE_FALLBACK:
      return "inappropriate fallback";

    case TLS1_AD_USER_CANCELLED:
      return "user canceled";

    case TLS1_AD_NO_RENEGOTIATION:
      return "no renegotiation";

    case TLS1_AD_MISSING_EXTENSION:
      return "missing extension";

    case TLS1_AD_UNSUPPORTED_EXTENSION:
      return "unsupported extension";

    case TLS1_AD_CERTIFICATE_UNOBTAINABLE:
      return "certificate unobtainable";

    case TLS1_AD_UNRECOGNIZED_NAME:
      return "unrecognized name";

    case TLS1_AD_BAD_CERTIFICATE_STATUS_RESPONSE:
      return "bad certificate status response";

    case TLS1_AD_BAD_CERTIFICATE_HASH_VALUE:
      return "bad certificate hash value";

    case TLS1_AD_UNKNOWN_PSK_IDENTITY:
      return "unknown PSK identity";

    case TLS1_AD_CERTIFICATE_REQUIRED:
      return "certificate required";

    case TLS1_AD_NO_APPLICATION_PROTOCOL:
      return "no application protocol";

    case TLS1_AD_ECH_REQUIRED:
      return "ECH required";

    default:
      return "unknown";
  }
}

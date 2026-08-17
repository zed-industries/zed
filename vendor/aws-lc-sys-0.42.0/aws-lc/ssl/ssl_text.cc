// Copyright 1995-2022 The OpenSSL Project Authors. All Rights Reserved.
// Copyright 2005 Nokia. All rights reserved.
//
// SPDX-License-Identifier: Apache-2.0
//
// Modifications Copyright Amazon.com, Inc. or its affiliates.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include <openssl/ssl.h>

#include "internal.h"


int SSL_SESSION_print(BIO *bp, const SSL_SESSION *sess) {
  if (sess == nullptr) {
    return 0;
  }

  bool tls13 = (sess->ssl_version == TLS1_3_VERSION);
  if (BIO_puts(bp, "SSL-Session:\n") <= 0) {
    return 0;
  }
  if (BIO_printf(bp, "    Protocol  : %s\n", SSL_SESSION_get_version(sess)) <=
      0) {
    return 0;
  }

  if (sess->cipher != nullptr) {
    if (BIO_printf(bp, "    Cipher    : %s\n",
                   ((sess->cipher->name == nullptr) ? "unknown"
                                                    : sess->cipher->name)) <=
        0) {
      return 0;
    }
  }
  if (BIO_puts(bp, "    Session-ID: ") <= 0) {
    return 0;
  }
  for (size_t i = 0; i < sess->session_id_length; i++) {
    if (BIO_printf(bp, "%02X", sess->session_id[i]) <= 0) {
      return 0;
    }
  }
  if (BIO_puts(bp, "\n    Session-ID-ctx: ") <= 0) {
    return 0;
  }
  for (size_t i = 0; i < sess->sid_ctx_length; i++) {
    if (BIO_printf(bp, "%02X", sess->sid_ctx[i]) <= 0) {
      return 0;
    }
  }
  if (tls13) {
    if (BIO_puts(bp, "\n    Resumption PSK: ") <= 0) {
      return 0;
    }
  } else if (BIO_puts(bp, "\n    Master-Key: ") <= 0) {
    return 0;
  }
  for (size_t i = 0; i < sess->secret_length; i++) {
    if (BIO_printf(bp, "%02X", sess->secret[i]) <= 0) {
      return 0;
    }
  }
  if (BIO_puts(bp, "\n    PSK identity: ") <= 0) {
    return 0;
  }
  if (BIO_printf(
          bp, "%s",
          sess->psk_identity.get() ? sess->psk_identity.get() : "None") <= 0) {
    return 0;
  }

  if (sess->ticket_lifetime_hint) {
    if (BIO_printf(bp, "\n    TLS session ticket lifetime hint: %u (seconds)",
                   sess->ticket_lifetime_hint) <= 0) {
      return 0;
    }
  }
  if (!sess->ticket.empty()) {
    if (BIO_puts(bp, "\n    TLS session ticket:\n    ") <= 0) {
      return 0;
    }
    for (uint8_t i : sess->ticket) {
      if (BIO_printf(bp, "%02X", i) <= 0) {
        return 0;
      }
    }
  }

  if (sess->time != 0) {
    if (BIO_printf(bp, "\n    Start Time: %lld", (long long)sess->time) <= 0) {
      return 0;
    }
  }
  if (sess->timeout != 0) {
    if (BIO_printf(bp, "\n    Timeout   : %u (sec)", sess->timeout) <= 0) {
      return 0;
    }
  }
  if (BIO_puts(bp, "\n") <= 0) {
    return 0;
  }

  if (BIO_puts(bp, "    Verify return code: ") <= 0) {
    return 0;
  }
  if (BIO_printf(bp, "%ld (%s)\n", sess->verify_result,
                 X509_verify_cert_error_string(sess->verify_result)) <= 0) {
    return 0;
  }

  if (BIO_printf(bp, "    Extended master secret: %s\n",
                 sess->extended_master_secret ? "yes" : "no") <= 0) {
    return 0;
  }

  if (tls13) {
    if (BIO_printf(bp, "    Max Early Data: %u\n",
                   sess->ticket_max_early_data) <= 0) {
      return 0;
    }
  }

  return 1;
}


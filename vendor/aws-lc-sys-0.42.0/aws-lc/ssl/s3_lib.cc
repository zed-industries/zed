// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com)
// Copyright (c) 1998-2007 The OpenSSL Project.  All rights reserved.
// Copyright 2002 Sun Microsystems, Inc. ALL RIGHTS RESERVED.
// Copyright 2005 Nokia. All rights reserved.
//
// ECC cipher suite support in OpenSSL originally developed by
// SUN MICROSYSTEMS, INC., and contributed to the OpenSSL project.
//
// The Contribution, originally written by Mika Kousa and Pasi Eronen of
// Nokia Corporation, consists of the "PSK" (Pre-Shared Key) ciphersuites
// support (see RFC 4279) to OpenSSL.
//
// SPDX-License-Identifier: Apache-2.0

#include <openssl/ssl.h>

#include <assert.h>
#include <string.h>

#include <openssl/digest.h>
#include <openssl/err.h>
#include <openssl/md5.h>
#include <openssl/mem.h>
#include <openssl/nid.h>

#include "../crypto/internal.h"
#include "internal.h"


BSSL_NAMESPACE_BEGIN

SSL3_STATE::SSL3_STATE()
    : skip_early_data(false),
      have_version(false),
      v2_hello_done(false),
      is_v2_hello(false),
      has_message(false),
      initial_handshake_complete(false),
      session_reused(false),
      delegated_credential_used(false),
      send_connection_binding(false),
      channel_id_valid(false),
      early_data_accepted(false),
      alert_dispatch(false),
      renegotiate_pending(false),
      used_hello_retry_request(false),
      was_key_usage_invalid(false) {}

SSL3_STATE::~SSL3_STATE() {
  sk_X509_NAME_pop_free(cached_x509_peer_ca_names, X509_NAME_free);
}

bool tls_new(SSL *ssl) {
  UniquePtr<SSL3_STATE> s3 = MakeUnique<SSL3_STATE>();
  if (!s3) {
    return false;
  }

  s3->aead_read_ctx = SSLAEADContext::CreateNullCipher(SSL_is_dtls(ssl));
  s3->aead_write_ctx = SSLAEADContext::CreateNullCipher(SSL_is_dtls(ssl));
  s3->hs = ssl_handshake_new(ssl);
  if (!s3->aead_read_ctx || !s3->aead_write_ctx || !s3->hs) {
    return false;
  }

  ssl->s3 = s3.release();

  // Set the version to the highest supported version.
  //
  // TODO(davidben): Move this field into |s3|, have it store the normalized
  // protocol version, and implement this pre-negotiation quirk in |SSL_version|
  // at the API boundary rather than in internal state.
  ssl->version = TLS1_2_VERSION;
  return true;
}

void tls_free(SSL *ssl) {
  if (ssl == NULL || ssl->s3 == NULL) {
    return;
  }

  Delete(ssl->s3);
  ssl->s3 = NULL;
}

BSSL_NAMESPACE_END

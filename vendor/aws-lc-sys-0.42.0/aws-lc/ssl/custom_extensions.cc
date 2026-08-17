// Copyright (c) 2014, Google Inc.
// SPDX-License-Identifier: ISC

#include <openssl/ssl.h>

#include <assert.h>
#include <string.h>

#include <openssl/bytestring.h>
#include <openssl/err.h>
#include <openssl/mem.h>
#include <openssl/stack.h>

#include "internal.h"


BSSL_NAMESPACE_BEGIN

void SSL_CUSTOM_EXTENSION_free(SSL_CUSTOM_EXTENSION *custom_extension) {
  OPENSSL_free(custom_extension);
}

static const SSL_CUSTOM_EXTENSION *custom_ext_find(
    STACK_OF(SSL_CUSTOM_EXTENSION) *stack,
    unsigned *out_index, uint16_t value) {
  for (size_t i = 0; i < sk_SSL_CUSTOM_EXTENSION_num(stack); i++) {
    const SSL_CUSTOM_EXTENSION *ext = sk_SSL_CUSTOM_EXTENSION_value(stack, i);
    if (ext->value == value) {
      if (out_index != NULL) {
        *out_index = i;
      }
      return ext;
    }
  }

  return NULL;
}

// default_add_callback is used as the |add_callback| when the user doesn't
// provide one. For servers, it does nothing while, for clients, it causes an
// empty extension to be included.
static int default_add_callback(SSL *ssl, unsigned extension_value,
                                const uint8_t **out, size_t *out_len,
                                int *out_alert_value, void *add_arg) {
  if (ssl->server) {
    return 0;
  }
  *out_len = 0;
  return 1;
}

static int custom_ext_add_hello(SSL_HANDSHAKE *hs, CBB *extensions) {
  SSL *const ssl = hs->ssl;
  STACK_OF(SSL_CUSTOM_EXTENSION) *stack = ssl->ctx->client_custom_extensions;
  if (ssl->server) {
    stack = ssl->ctx->server_custom_extensions;
  }

  if (stack == NULL) {
    return 1;
  }

  for (size_t i = 0; i < sk_SSL_CUSTOM_EXTENSION_num(stack); i++) {
    const SSL_CUSTOM_EXTENSION *ext = sk_SSL_CUSTOM_EXTENSION_value(stack, i);

    if (ssl->server &&
        !(hs->custom_extensions.received & (1u << i))) {
      // Servers cannot echo extensions that the client didn't send.
      continue;
    }

    const uint8_t *contents;
    size_t contents_len;
    int alert = SSL_AD_DECODE_ERROR;
    CBB contents_cbb;

    switch (ext->add_callback(ssl, ext->value, &contents, &contents_len, &alert,
                              ext->add_arg)) {
      case 1:
        if (!CBB_add_u16(extensions, ext->value) ||
            !CBB_add_u16_length_prefixed(extensions, &contents_cbb) ||
            !CBB_add_bytes(&contents_cbb, contents, contents_len) ||
            !CBB_flush(extensions)) {
          OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
          ERR_add_error_dataf("extension %u", (unsigned) ext->value);
          if (ext->free_callback && 0 < contents_len) {
            ext->free_callback(ssl, ext->value, contents, ext->add_arg);
          }
          return 0;
        }

        if (ext->free_callback && 0 < contents_len) {
          ext->free_callback(ssl, ext->value, contents, ext->add_arg);
        }

        if (!ssl->server) {
          assert((hs->custom_extensions.sent & (1u << i)) == 0);
          hs->custom_extensions.sent |= (1u << i);
        }
        break;

      case 0:
        break;

      default:
        ssl_send_alert(ssl, SSL3_AL_FATAL, alert);
        OPENSSL_PUT_ERROR(SSL, SSL_R_CUSTOM_EXTENSION_ERROR);
        ERR_add_error_dataf("extension %u", (unsigned) ext->value);
        return 0;
    }
  }

  return 1;
}

int custom_ext_add_clienthello(SSL_HANDSHAKE *hs, CBB *extensions) {
  return custom_ext_add_hello(hs, extensions);
}

int custom_ext_parse_serverhello(SSL_HANDSHAKE *hs, int *out_alert,
                                 uint16_t value, const CBS *extension) {
  SSL *const ssl = hs->ssl;
  unsigned index;
  const SSL_CUSTOM_EXTENSION *ext =
      custom_ext_find(ssl->ctx->client_custom_extensions, &index, value);

  if (// Unknown extensions are not allowed in a ServerHello.
      ext == NULL ||
      // Also, if we didn't send the extension, that's also unacceptable.
      !(hs->custom_extensions.sent & (1u << index))) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_UNEXPECTED_EXTENSION);
    ERR_add_error_dataf("extension %u", (unsigned)value);
    *out_alert = SSL_AD_UNSUPPORTED_EXTENSION;
    return 0;
  }

  if (ext->parse_callback != NULL &&
      !ext->parse_callback(ssl, value, CBS_data(extension), CBS_len(extension),
                           out_alert, ext->parse_arg)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_CUSTOM_EXTENSION_ERROR);
    ERR_add_error_dataf("extension %u", (unsigned)ext->value);
    return 0;
  }

  return 1;
}

int custom_ext_parse_clienthello(SSL_HANDSHAKE *hs, int *out_alert,
                                 uint16_t value, const CBS *extension) {
  SSL *const ssl = hs->ssl;
  unsigned index;
  const SSL_CUSTOM_EXTENSION *ext =
      custom_ext_find(ssl->ctx->server_custom_extensions, &index, value);

  if (ext == NULL) {
    return 1;
  }

  assert((hs->custom_extensions.received & (1u << index)) == 0);
  hs->custom_extensions.received |= (1u << index);

  if (ext->parse_callback &&
      !ext->parse_callback(ssl, value, CBS_data(extension), CBS_len(extension),
                           out_alert, ext->parse_arg)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_CUSTOM_EXTENSION_ERROR);
    ERR_add_error_dataf("extension %u", (unsigned)ext->value);
    return 0;
  }

  return 1;
}

int custom_ext_add_serverhello(SSL_HANDSHAKE *hs, CBB *extensions) {
  return custom_ext_add_hello(hs, extensions);
}

// MAX_NUM_CUSTOM_EXTENSIONS is the maximum number of custom extensions that
// can be set on an |SSL_CTX|. It's determined by the size of the bitset used
// to track when an extension has been sent.
#define MAX_NUM_CUSTOM_EXTENSIONS \
  (sizeof(((SSL_HANDSHAKE *)NULL)->custom_extensions.sent) * 8)

static int custom_ext_append(STACK_OF(SSL_CUSTOM_EXTENSION) **stack,
                             unsigned extension_value,
                             SSL_custom_ext_add_cb add_cb,
                             SSL_custom_ext_free_cb free_cb, void *add_arg,
                             SSL_custom_ext_parse_cb parse_cb,
                             void *parse_arg) {
  if (add_cb == NULL ||
      0xffff < extension_value ||
      SSL_extension_supported(extension_value) ||
      // Specifying a free callback without an add callback is nonsensical
      // and an error.
      (*stack != NULL &&
       (MAX_NUM_CUSTOM_EXTENSIONS <= sk_SSL_CUSTOM_EXTENSION_num(*stack) ||
        custom_ext_find(*stack, NULL, extension_value) != NULL))) {
    return 0;
  }

  SSL_CUSTOM_EXTENSION *ext =
      (SSL_CUSTOM_EXTENSION *)OPENSSL_malloc(sizeof(SSL_CUSTOM_EXTENSION));
  if (ext == NULL) {
    return 0;
  }
  ext->add_callback = add_cb;
  ext->add_arg = add_arg;
  ext->free_callback = free_cb;
  ext->parse_callback = parse_cb;
  ext->parse_arg = parse_arg;
  ext->value = extension_value;

  if (*stack == NULL) {
    *stack = sk_SSL_CUSTOM_EXTENSION_new_null();
    if (*stack == NULL) {
      SSL_CUSTOM_EXTENSION_free(ext);
      return 0;
    }
  }

  if (!sk_SSL_CUSTOM_EXTENSION_push(*stack, ext)) {
    SSL_CUSTOM_EXTENSION_free(ext);
    if (sk_SSL_CUSTOM_EXTENSION_num(*stack) == 0) {
      sk_SSL_CUSTOM_EXTENSION_free(*stack);
      *stack = NULL;
    }
    return 0;
  }

  return 1;
}

BSSL_NAMESPACE_END

using namespace bssl;

int SSL_CTX_add_client_custom_ext(SSL_CTX *ctx, unsigned extension_value,
                                  SSL_custom_ext_add_cb add_cb,
                                  SSL_custom_ext_free_cb free_cb, void *add_arg,
                                  SSL_custom_ext_parse_cb parse_cb,
                                  void *parse_arg) {
  return custom_ext_append(&ctx->client_custom_extensions, extension_value,
                           add_cb ? add_cb : default_add_callback, free_cb,
                           add_arg, parse_cb, parse_arg);
}

int SSL_CTX_add_server_custom_ext(SSL_CTX *ctx, unsigned extension_value,
                                  SSL_custom_ext_add_cb add_cb,
                                  SSL_custom_ext_free_cb free_cb, void *add_arg,
                                  SSL_custom_ext_parse_cb parse_cb,
                                  void *parse_arg) {
  return custom_ext_append(&ctx->server_custom_extensions, extension_value,
                           add_cb ? add_cb : default_add_callback, free_cb,
                           add_arg, parse_cb, parse_arg);
}

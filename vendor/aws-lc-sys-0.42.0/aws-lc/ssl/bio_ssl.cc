// Copyright 1995-2016 The OpenSSL Project Authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/ssl.h>

#include <openssl/bio.h>

// We intentionally redefine |iovec| here without header guards or redefinition
// protection to prevent "bio.h" from inadvertently including system headers
// (like <sys/socket.h>) in consuming applications. This avoids potential
// conflicts where system headers might define types that interfere with the
// consumer's code. Consumers should ideally handle potential struct
// redefinitions themselves, but unfortunately most legacy codebases do not
// implement such checks, making this approach necessary for compatibility.
//
// See commit aws-lc@9db959e for more details.
//
// On WASI, iovec is defined via basic libc headers (included transitively
// through standard headers like <stdio.h>), so it's already defined before
// this file is compiled. Consumer code on WASI will have the same situation,
// so the protection goal is still achieved - there's no risk of a surprise
// redefinition conflict.
#if !defined(OPENSSL_WASM_WASI)
struct iovec {
  void* iov_base;
  size_t iov_len;
};
#endif

static SSL *get_ssl(BIO *bio) {
  return reinterpret_cast<SSL *>(bio->ptr);
}

static int ssl_read(BIO *bio, char *out, int outl) {
  SSL *ssl = get_ssl(bio);
  if (ssl == NULL) {
    return 0;
  }

  BIO_clear_retry_flags(bio);

  const int ret = SSL_read(ssl, out, outl);

  switch (SSL_get_error(ssl, ret)) {
    case SSL_ERROR_WANT_READ:
      BIO_set_retry_read(bio);
      break;

    case SSL_ERROR_WANT_WRITE:
      BIO_set_retry_write(bio);
      break;

    case SSL_ERROR_WANT_ACCEPT:
      BIO_set_retry_special(bio);
      BIO_set_retry_reason(bio, BIO_RR_ACCEPT);
      break;

    case SSL_ERROR_WANT_CONNECT:
      BIO_set_retry_special(bio);
      BIO_set_retry_reason(bio, BIO_RR_CONNECT);
      break;

    case SSL_ERROR_NONE:
    case SSL_ERROR_SYSCALL:
    case SSL_ERROR_SSL:
    case SSL_ERROR_ZERO_RETURN:
    default:
      break;
  }

  return ret;
}

static int ssl_write(BIO *bio, const char *out, int outl) {
  SSL *ssl = get_ssl(bio);
  if (ssl == NULL) {
    return 0;
  }

  BIO_clear_retry_flags(bio);

  const int ret = SSL_write(ssl, out, outl);

  switch (SSL_get_error(ssl, ret)) {
    case SSL_ERROR_WANT_WRITE:
      BIO_set_retry_write(bio);
      break;

    case SSL_ERROR_WANT_READ:
      BIO_set_retry_read(bio);
      break;

    case SSL_ERROR_WANT_CONNECT:
      BIO_set_retry_special(bio);
      BIO_set_retry_reason(bio, BIO_RR_CONNECT);
      break;

    case SSL_ERROR_NONE:
    case SSL_ERROR_SYSCALL:
    case SSL_ERROR_SSL:
    default:
      break;
  }

  return ret;
}

static long ssl_ctrl(BIO *bio, int cmd, long num, void *ptr) {
  SSL *ssl = get_ssl(bio);
  if (ssl == NULL && cmd != BIO_C_SET_SSL) {
    return 0;
  }

  switch (cmd) {
    case BIO_C_SET_SSL:
      if (ssl != NULL) {
        // OpenSSL allows reusing an SSL BIO with a different SSL object. We do
        // not support this.
        OPENSSL_PUT_ERROR(SSL, ERR_R_SHOULD_NOT_HAVE_BEEN_CALLED);
        return 0;
      }

      // Note this differs from upstream OpenSSL, which synchronizes
      // |bio->next_bio| with |ssl|'s rbio here, and on |BIO_CTRL_PUSH|. We call
      // into the corresponding |BIO| directly. (We can implement the upstream
      // behavior if it ends up necessary.)
      bio->shutdown = static_cast<int>(num);
      bio->ptr = ptr;
      bio->init = 1;
      return 1;

    case BIO_C_GET_SSL:
      if (ptr != nullptr) {
        auto sslp = static_cast<SSL **>(ptr);
        *sslp = ssl;
        return 1;
      }
      return 0;

    case BIO_CTRL_GET_CLOSE:
      return bio->shutdown;

    case BIO_CTRL_SET_CLOSE:
      bio->shutdown = static_cast<int>(num);
      return 1;

    case BIO_CTRL_WPENDING: {
      BIO *wbio = SSL_get_wbio(ssl);
      if (wbio == NULL) {
        return 0;
      }
      return BIO_ctrl(wbio, cmd, num, ptr);
    }

    case BIO_CTRL_PENDING:
      return SSL_pending(ssl);

    case BIO_CTRL_FLUSH: {
      BIO *wbio = SSL_get_wbio(ssl);
      if (wbio == NULL) {
        return 0;
      }
      BIO_clear_retry_flags(bio);
      long ret = BIO_ctrl(wbio, cmd, num, ptr);
      BIO_set_flags(bio, BIO_get_retry_flags(wbio));
      BIO_set_retry_reason(bio, BIO_get_retry_reason(wbio));
      return ret;
    }

    case BIO_CTRL_PUSH:
    case BIO_CTRL_POP:
    case BIO_CTRL_DUP:
      return -1;

    default: {
      BIO *rbio = SSL_get_rbio(ssl);
      if (rbio == NULL) {
        return 0;
      }
      return BIO_ctrl(rbio, cmd, num, ptr);
    }
  }
}

static int ssl_new(BIO *bio) {
  return 1;
}

static int ssl_free(BIO *bio) {
  SSL *ssl = get_ssl(bio);

  if (ssl == NULL) {
    return 1;
  }

  SSL_shutdown(ssl);
  if (bio->shutdown) {
    SSL_free(ssl);
  }
  bio->ptr = NULL;

  return 1;
}

static long ssl_callback_ctrl(BIO *bio, int cmd, bio_info_cb fp) {
  SSL *ssl = get_ssl(bio);
  if (ssl == NULL) {
    return 0;
  }

  switch (cmd) {
    case BIO_CTRL_SET_CALLBACK:
      return -1;

    default: {
      BIO *rbio = SSL_get_rbio(ssl);
      if (rbio == NULL) {
        return 0;
      }
      return BIO_callback_ctrl(rbio, cmd, fp);
    }
  }
}

static const BIO_METHOD ssl_method = {
    BIO_TYPE_SSL, "SSL",    ssl_write, ssl_read, NULL,
    NULL,         ssl_ctrl, ssl_new,   ssl_free, ssl_callback_ctrl,
};

const BIO_METHOD *BIO_f_ssl(void) { return &ssl_method; }

long BIO_set_ssl(BIO *bio, SSL *ssl, int take_owership) {
  return BIO_ctrl(bio, BIO_C_SET_SSL, take_owership, ssl);
}

long BIO_get_ssl(BIO *bio, SSL **ssl) {
  return BIO_ctrl(bio, BIO_C_GET_SSL, 0, ssl);
}

#if !defined(OPENSSL_NO_SOCK)
BIO *BIO_new_ssl_connect(SSL_CTX *ctx) {
  bssl::UniquePtr<BIO> con(BIO_new(BIO_s_connect()));
  bssl::UniquePtr<BIO> ssl(BIO_new_ssl(ctx, 1));
  if (!con || !ssl) {
    return nullptr;
  }
  bssl::UniquePtr<BIO> ret(BIO_push(ssl.get(), con.get()));
  if (!ret) {
    return nullptr;
  }

  con.release();
  ssl.release();
  return ret.release();
}
#endif  // !OPENSSL_NO_SOCK

BIO *BIO_new_ssl(SSL_CTX *ctx, int client) {
  bssl::UniquePtr<BIO> ret(BIO_new(BIO_f_ssl()));
  bssl::UniquePtr<SSL> ssl(SSL_new(ctx));

  if (!ret || !ssl) {
    return nullptr;
  }
  if (client) {
    SSL_set_connect_state(ssl.get());
  } else {
    SSL_set_accept_state(ssl.get());
  }

  if (BIO_set_ssl(ret.get(), ssl.get(), BIO_CLOSE) <= 0) {
    return nullptr;
  }
  ssl.release();  // Ownership transferred to BIO
  return ret.release();
}

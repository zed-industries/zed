// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com)
// Copyright (c) 1998-2007 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/ssl.h>

#include <errno.h>
#include <string.h>

#include <openssl/asn1.h>
#include <openssl/bio.h>
#include <openssl/err.h>
#include <openssl/mem.h>
#include <openssl/pem.h>
#include <openssl/stack.h>
#include <openssl/x509.h>

#include "internal.h"


static int xname_cmp(const X509_NAME *const *a, const X509_NAME *const *b) {
  return X509_NAME_cmp(*a, *b);
}

static int add_bio_cert_subjects_to_stack(STACK_OF(X509_NAME) *out, BIO *bio,
                                          bool allow_empty) {
  // This function historically sorted |out| after every addition and skipped
  // duplicates. This implementation preserves that behavior, but only sorts at
  // the end, to avoid a quadratic running time. Existing duplicates in |out|
  // are preserved, but do not introduce new duplicates.
  bssl::UniquePtr<STACK_OF(X509_NAME)> to_append(sk_X509_NAME_new(xname_cmp));
  if (to_append == nullptr) {
    return 0;
  }

  // Temporarily switch the comparison function for |out|.
  struct RestoreCmpFunc {
    ~RestoreCmpFunc() { sk_X509_NAME_set_cmp_func(stack, old_cmp); }
    STACK_OF(X509_NAME) *stack;
    int (*old_cmp)(const X509_NAME *const *, const X509_NAME *const *);
  };
  RestoreCmpFunc restore = {out, sk_X509_NAME_set_cmp_func(out, xname_cmp)};

  sk_X509_NAME_sort(out);
  bool first = true;
  for (;;) {
    bssl::UniquePtr<X509> x509(
        PEM_read_bio_X509(bio, nullptr, nullptr, nullptr));
    if (x509 == nullptr) {
      if (first && !allow_empty) {
        return 0;
      }
      // TODO(davidben): This ignores PEM syntax errors. It should only succeed
      // on |PEM_R_NO_START_LINE|.
      ERR_clear_error();
      break;
    }
    first = false;

    X509_NAME *subject = X509_get_subject_name(x509.get());
    // Skip if already present in |out|. Duplicates in |to_append| will be
    // handled separately.
    if (sk_X509_NAME_find_awslc(out, nullptr, subject)) {
      continue;
    }

    bssl::UniquePtr<X509_NAME> copy(X509_NAME_dup(subject));
    if (copy == nullptr ||
        !bssl::PushToStack(to_append.get(), std::move(copy))) {
      return 0;
    }
  }

  // Append |to_append| to |stack|, skipping any duplicates.
  sk_X509_NAME_sort(to_append.get());
  size_t num = sk_X509_NAME_num(to_append.get());
  for (size_t i = 0; i < num; i++) {
    bssl::UniquePtr<X509_NAME> name(sk_X509_NAME_value(to_append.get(), i));
    sk_X509_NAME_set(to_append.get(), i, nullptr);
    if (i + 1 < num &&
        X509_NAME_cmp(name.get(), sk_X509_NAME_value(to_append.get(), i + 1)) ==
            0) {
      continue;
    }
    if (!bssl::PushToStack(out, std::move(name))) {
      return 0;
    }
  }

  // Sort |out| one last time, to preserve the historical behavior of
  // maintaining the sorted list.
  sk_X509_NAME_sort(out);
  return 1;
}

int SSL_add_bio_cert_subjects_to_stack(STACK_OF(X509_NAME) *out, BIO *bio) {
  return add_bio_cert_subjects_to_stack(out, bio, /*allow_empty=*/true);
}

STACK_OF(X509_NAME) *SSL_load_client_CA_file(const char *file) {
  bssl::UniquePtr<BIO> in(BIO_new_file(file, "rb"));
  if (in == nullptr) {
    return nullptr;
  }
  bssl::UniquePtr<STACK_OF(X509_NAME)> ret(sk_X509_NAME_new_null());
  if (ret == nullptr ||  //
      !add_bio_cert_subjects_to_stack(ret.get(), in.get(),
                                      /*allow_empty=*/false)) {
    return nullptr;
  }
  return ret.release();
}

int SSL_add_file_cert_subjects_to_stack(STACK_OF(X509_NAME) *out,
                                        const char *file) {
  bssl::UniquePtr<BIO> in(BIO_new_file(file, "rb"));
  if (in == nullptr) {
    return 0;
  }
  return SSL_add_bio_cert_subjects_to_stack(out, in.get());
}

int SSL_use_certificate_file(SSL *ssl, const char *file, int type) {
  int reason_code;
  BIO *in;
  int ret = 0;
  X509 *x = NULL;

  in = BIO_new(BIO_s_file());
  if (in == NULL) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_BUF_LIB);
    goto end;
  }

  if (BIO_read_filename(in, file) <= 0) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_SYS_LIB);
    goto end;
  }

  if (type == SSL_FILETYPE_ASN1) {
    reason_code = ERR_R_ASN1_LIB;
    x = d2i_X509_bio(in, NULL);
  } else if (type == SSL_FILETYPE_PEM) {
    reason_code = ERR_R_PEM_LIB;
    x = PEM_read_bio_X509(in, NULL, ssl->ctx->default_passwd_callback,
                          ssl->ctx->default_passwd_callback_userdata);
  } else {
    OPENSSL_PUT_ERROR(SSL, SSL_R_BAD_SSL_FILETYPE);
    goto end;
  }

  if (x == NULL) {
    OPENSSL_PUT_ERROR(SSL, reason_code);
    goto end;
  }

  ret = SSL_use_certificate(ssl, x);

end:
  X509_free(x);
  BIO_free(in);

  return ret;
}

int SSL_use_RSAPrivateKey_file(SSL *ssl, const char *file, int type) {
  int reason_code, ret = 0;
  BIO *in;
  RSA *rsa = NULL;

  in = BIO_new(BIO_s_file());
  if (in == NULL) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_BUF_LIB);
    goto end;
  }

  if (BIO_read_filename(in, file) <= 0) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_SYS_LIB);
    goto end;
  }

  if (type == SSL_FILETYPE_ASN1) {
    reason_code = ERR_R_ASN1_LIB;
    rsa = d2i_RSAPrivateKey_bio(in, NULL);
  } else if (type == SSL_FILETYPE_PEM) {
    reason_code = ERR_R_PEM_LIB;
    rsa =
        PEM_read_bio_RSAPrivateKey(in, NULL, ssl->ctx->default_passwd_callback,
                                   ssl->ctx->default_passwd_callback_userdata);
  } else {
    OPENSSL_PUT_ERROR(SSL, SSL_R_BAD_SSL_FILETYPE);
    goto end;
  }

  if (rsa == NULL) {
    OPENSSL_PUT_ERROR(SSL, reason_code);
    goto end;
  }
  ret = SSL_use_RSAPrivateKey(ssl, rsa);
  RSA_free(rsa);

end:
  BIO_free(in);
  return ret;
}

int SSL_use_PrivateKey_file(SSL *ssl, const char *file, int type) {
  int reason_code, ret = 0;
  BIO *in;
  EVP_PKEY *pkey = NULL;

  in = BIO_new(BIO_s_file());
  if (in == NULL) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_BUF_LIB);
    goto end;
  }

  if (BIO_read_filename(in, file) <= 0) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_SYS_LIB);
    goto end;
  }

  if (type == SSL_FILETYPE_PEM) {
    reason_code = ERR_R_PEM_LIB;
    pkey = PEM_read_bio_PrivateKey(in, NULL, ssl->ctx->default_passwd_callback,
                                   ssl->ctx->default_passwd_callback_userdata);
  } else if (type == SSL_FILETYPE_ASN1) {
    reason_code = ERR_R_ASN1_LIB;
    pkey = d2i_PrivateKey_bio(in, NULL);
  } else {
    OPENSSL_PUT_ERROR(SSL, SSL_R_BAD_SSL_FILETYPE);
    goto end;
  }

  if (pkey == NULL) {
    OPENSSL_PUT_ERROR(SSL, reason_code);
    goto end;
  }
  ret = SSL_use_PrivateKey(ssl, pkey);
  EVP_PKEY_free(pkey);

end:
  BIO_free(in);
  return ret;
}

int SSL_CTX_use_certificate_file(SSL_CTX *ctx, const char *file, int type) {
  int reason_code;
  BIO *in;
  int ret = 0;
  X509 *x = NULL;

  in = BIO_new(BIO_s_file());
  if (in == NULL) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_BUF_LIB);
    goto end;
  }

  if (BIO_read_filename(in, file) <= 0) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_SYS_LIB);
    goto end;
  }

  if (type == SSL_FILETYPE_ASN1) {
    reason_code = ERR_R_ASN1_LIB;
    x = d2i_X509_bio(in, NULL);
  } else if (type == SSL_FILETYPE_PEM) {
    reason_code = ERR_R_PEM_LIB;
    x = PEM_read_bio_X509(in, NULL, ctx->default_passwd_callback,
                          ctx->default_passwd_callback_userdata);
  } else {
    OPENSSL_PUT_ERROR(SSL, SSL_R_BAD_SSL_FILETYPE);
    goto end;
  }

  if (x == NULL) {
    OPENSSL_PUT_ERROR(SSL, reason_code);
    goto end;
  }

  ret = SSL_CTX_use_certificate(ctx, x);

end:
  X509_free(x);
  BIO_free(in);
  return ret;
}

int SSL_CTX_use_RSAPrivateKey_file(SSL_CTX *ctx, const char *file, int type) {
  int reason_code, ret = 0;
  BIO *in;
  RSA *rsa = NULL;

  in = BIO_new(BIO_s_file());
  if (in == NULL) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_BUF_LIB);
    goto end;
  }

  if (BIO_read_filename(in, file) <= 0) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_SYS_LIB);
    goto end;
  }

  if (type == SSL_FILETYPE_ASN1) {
    reason_code = ERR_R_ASN1_LIB;
    rsa = d2i_RSAPrivateKey_bio(in, NULL);
  } else if (type == SSL_FILETYPE_PEM) {
    reason_code = ERR_R_PEM_LIB;
    rsa = PEM_read_bio_RSAPrivateKey(in, NULL, ctx->default_passwd_callback,
                                     ctx->default_passwd_callback_userdata);
  } else {
    OPENSSL_PUT_ERROR(SSL, SSL_R_BAD_SSL_FILETYPE);
    goto end;
  }

  if (rsa == NULL) {
    OPENSSL_PUT_ERROR(SSL, reason_code);
    goto end;
  }
  ret = SSL_CTX_use_RSAPrivateKey(ctx, rsa);
  RSA_free(rsa);

end:
  BIO_free(in);
  return ret;
}

int SSL_CTX_use_PrivateKey_file(SSL_CTX *ctx, const char *file, int type) {
  int reason_code, ret = 0;
  BIO *in;
  EVP_PKEY *pkey = NULL;

  in = BIO_new(BIO_s_file());
  if (in == NULL) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_BUF_LIB);
    goto end;
  }

  if (BIO_read_filename(in, file) <= 0) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_SYS_LIB);
    goto end;
  }

  if (type == SSL_FILETYPE_PEM) {
    reason_code = ERR_R_PEM_LIB;
    pkey = PEM_read_bio_PrivateKey(in, NULL, ctx->default_passwd_callback,
                                   ctx->default_passwd_callback_userdata);
  } else if (type == SSL_FILETYPE_ASN1) {
    reason_code = ERR_R_ASN1_LIB;
    pkey = d2i_PrivateKey_bio(in, NULL);
  } else {
    OPENSSL_PUT_ERROR(SSL, SSL_R_BAD_SSL_FILETYPE);
    goto end;
  }

  if (pkey == NULL) {
    OPENSSL_PUT_ERROR(SSL, reason_code);
    goto end;
  }
  ret = SSL_CTX_use_PrivateKey(ctx, pkey);
  EVP_PKEY_free(pkey);

end:
  BIO_free(in);
  return ret;
}

// Read a file that contains our certificate in "PEM" format, possibly followed
// by a sequence of CA certificates that should be sent to the peer in the
// Certificate message.
static int use_certificate_chain_file(SSL_CTX *ctx, SSL *ssl, const char *file) {
  if(ctx == nullptr && ssl == nullptr) {
    return 0;
  }

  int ret = 0;
  // |parent_ctx| is |ctx| unless |ssl| is defined.
  SSL_CTX *parent_ctx = ctx;
  if (ssl != nullptr) {
    parent_ctx = ssl->ctx.get();
  }
  ERR_clear_error();  // clear error stack for SSL_CTX_use_certificate()

  bssl::UniquePtr<BIO> in(BIO_new(BIO_s_file()));
  if (in == nullptr) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_BUF_LIB);
    return 0;
  }

  if (BIO_read_filename(in.get(), file) <= 0) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_SYS_LIB);
    return 0;
  }

  bssl::UniquePtr<X509> x509(PEM_read_bio_X509_AUX(
      in.get(), nullptr, parent_ctx->default_passwd_callback,
      parent_ctx->default_passwd_callback_userdata));
  if (x509 == nullptr) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_PEM_LIB);
    return 0;
  }

  if (ctx != nullptr) {
    ret = SSL_CTX_use_certificate(ctx, x509.get());
  } else {
    ret = SSL_use_certificate(ssl, x509.get());
  }

  if (ERR_peek_error() != 0) {
    ret = 0;  // Key/certificate mismatch doesn't imply ret==0 ...
  }

  if (ret) {
    // If we could set up our certificate, now proceed to the CA
    // certificates.
    X509 *ca;
    int temp_ret;
    uint32_t err;

    if (ctx != nullptr) {
      SSL_CTX_clear_chain_certs(ctx);
    } else {
      SSL_clear_chain_certs(ssl);
    }

    while ((ca = PEM_read_bio_X509(
                in.get(), nullptr, parent_ctx->default_passwd_callback,
                parent_ctx->default_passwd_callback_userdata)) != nullptr) {
      if (ctx != nullptr) {
        temp_ret = SSL_CTX_add0_chain_cert(ctx, ca);
      } else {
        temp_ret = SSL_add0_chain_cert(ssl, ca);
      }

      if (temp_ret == 0) {
        X509_free(ca);
        return 0;
      }
      // Note that we must not free |ca| if it was successfully added to the
      // chain (while we must free the main certificate, since its reference
      // count is increased by |SSL_CTX_use_certificate|).
    }

    // When the while loop ends, it's usually just EOF.
    err = ERR_peek_last_error();
    if (ERR_GET_LIB(err) == ERR_LIB_PEM &&
        ERR_GET_REASON(err) == PEM_R_NO_START_LINE) {
      ERR_clear_error();
    } else {
      ret = 0;  // some real error
    }
  }

  return ret;
}


int SSL_CTX_use_certificate_chain_file(SSL_CTX *ctx, const char *file) {
  if (ctx == nullptr || file == nullptr) {
    return 0;
  }
  return use_certificate_chain_file(ctx, nullptr, file);
}

int SSL_use_certificate_chain_file(SSL *ssl, const char *file) {
  if (ssl == nullptr || file == nullptr) {
    return 0;
  }
  return use_certificate_chain_file(nullptr, ssl, file);
}

void SSL_CTX_set_default_passwd_cb(SSL_CTX *ctx, pem_password_cb *cb) {
  ctx->default_passwd_callback = cb;
}

pem_password_cb *SSL_CTX_get_default_passwd_cb(const SSL_CTX *ctx) {
  return ctx->default_passwd_callback;
}

void SSL_CTX_set_default_passwd_cb_userdata(SSL_CTX *ctx, void *data) {
  ctx->default_passwd_callback_userdata = data;
}

void *SSL_CTX_get_default_passwd_cb_userdata(const SSL_CTX *ctx) {
  return ctx->default_passwd_callback_userdata;
}

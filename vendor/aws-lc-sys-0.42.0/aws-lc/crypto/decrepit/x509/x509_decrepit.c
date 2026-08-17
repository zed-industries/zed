// Copyright (c) 2016, Google Inc.
// SPDX-License-Identifier: ISC

#include <openssl/x509.h>

#include <assert.h>

#include <openssl/conf.h>


X509_EXTENSION *X509V3_EXT_conf_nid(LHASH_OF(CONF_VALUE) *conf,
                                    const X509V3_CTX *ctx, int ext_nid,
                                    const char *value) {
  assert(conf == NULL);
  return X509V3_EXT_nconf_nid(NULL, ctx, ext_nid, value);
}

X509_EXTENSION *X509V3_EXT_conf(LHASH_OF(CONF_VALUE) *conf, X509V3_CTX *ctx,
                                const char *name, const char *value) {
  assert(conf == NULL);
  return X509V3_EXT_nconf(NULL, ctx, name, value);
}

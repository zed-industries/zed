// Copyright (c) 2014, Google Inc.
// SPDX-License-Identifier: ISC

#include <openssl/engine.h>

#include <string.h>
#include <assert.h>

#include <openssl/ec_key.h>
#include <openssl/err.h>
#include <openssl/mem.h>
#include <openssl/rsa.h>
#include <openssl/thread.h>

#include "../internal.h"
#include "../crypto/fipsmodule/ec/internal.h"


struct engine_st {
  RSA_METHOD *rsa_method;
  EC_KEY_METHOD *eckey_method;
};

ENGINE *ENGINE_new(void) { return OPENSSL_zalloc(sizeof(ENGINE)); }

int ENGINE_free(ENGINE *engine) {
  OPENSSL_free(engine);
  return 1;
}

int ENGINE_set_RSA(ENGINE *engine, const RSA_METHOD *method) {
  if(!engine) {
    OPENSSL_PUT_ERROR(ENGINE, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  engine->rsa_method = (RSA_METHOD *)method;
  return 1;
}

const RSA_METHOD *ENGINE_get_RSA(const ENGINE *engine) {
  if(engine) {
    return engine->rsa_method;;
  }
  return NULL;
}

int ENGINE_set_EC(ENGINE *engine, const EC_KEY_METHOD *method) {
  if(!engine) {
    OPENSSL_PUT_ERROR(ENGINE, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  engine->eckey_method = (EC_KEY_METHOD *)method;
  return 1;
}

const EC_KEY_METHOD *ENGINE_get_EC(const ENGINE *engine) {
  if(engine) {
    return engine->eckey_method;
  }
  return NULL;
}

OPENSSL_DECLARE_ERROR_REASON(ENGINE, OPERATION_NOT_SUPPORTED)

void ENGINE_cleanup(void) {}

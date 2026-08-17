// Copyright (c) 2016, Google Inc.	
// SPDX-License-Identifier: ISC

#include <openssl/obj.h>

#include <assert.h>
#include <string.h>

#include <openssl/evp.h>

#include "../../internal.h"


struct wrapped_callback {
  void (*callback)(const OBJ_NAME *, void *arg);
  void *arg;
};

static void cipher_callback(const EVP_CIPHER *cipher, const char *name,
                            const char *unused, void *arg) {
  const struct wrapped_callback *wrapped = (struct wrapped_callback *)arg;
  OBJ_NAME obj_name;

  OPENSSL_memset(&obj_name, 0, sizeof(obj_name));
  obj_name.type = OBJ_NAME_TYPE_CIPHER_METH;
  obj_name.name = name;
  obj_name.data = (const char *)cipher;

  wrapped->callback(&obj_name, wrapped->arg);
}

static void md_callback(const EVP_MD *md, const char *name, const char *unused,
                        void *arg) {
  const struct wrapped_callback *wrapped = (struct wrapped_callback*) arg;
  OBJ_NAME obj_name;

  OPENSSL_memset(&obj_name, 0, sizeof(obj_name));
  obj_name.type = OBJ_NAME_TYPE_MD_METH;
  obj_name.name = name;
  obj_name.data = (const char *)md;

  wrapped->callback(&obj_name, wrapped->arg);
}	

void OBJ_NAME_do_all_sorted(int type,
                            void (*callback)(const OBJ_NAME *, void *arg),
                            void *arg) {
  struct wrapped_callback wrapped;
  wrapped.callback = callback;
  wrapped.arg = arg;

  if (type == OBJ_NAME_TYPE_CIPHER_METH) {
    EVP_CIPHER_do_all_sorted(cipher_callback, &wrapped);
  } else if (type == OBJ_NAME_TYPE_MD_METH) {
    EVP_MD_do_all_sorted(md_callback, &wrapped);
  } else {
    assert(0);
  }
}

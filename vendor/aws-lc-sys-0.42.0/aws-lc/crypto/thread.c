// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/thread.h>


int CRYPTO_num_locks(void) { return 1; }

void CRYPTO_set_locking_callback(void (*func)(int mode, int lock_num,
                                              const char *file, int line)) {}

void (*CRYPTO_get_locking_callback(void))(int mode, int lock_num,
                                          const char *file, int line) {
  return NULL;
}

void CRYPTO_set_add_lock_callback(int (*func)(int *num, int mount, int lock_num,
                                              const char *file, int line)) {}

const char *CRYPTO_get_lock_name(int lock_num) {
  return "No old-style OpenSSL locks anymore";
}

int CRYPTO_THREADID_set_callback(void (*func)(CRYPTO_THREADID *)) { return 1; }

void CRYPTO_THREADID_set_numeric(CRYPTO_THREADID *id, unsigned long val) {}

void CRYPTO_THREADID_set_pointer(CRYPTO_THREADID *id, void *ptr) {}

void CRYPTO_THREADID_current(CRYPTO_THREADID *id) {}

void CRYPTO_set_id_callback(unsigned long (*func)(void)) {}

void CRYPTO_set_dynlock_create_callback(struct CRYPTO_dynlock_value *(
    *dyn_create_function)(const char *file, int line)) {}

void CRYPTO_set_dynlock_lock_callback(void (*dyn_lock_function)(
    int mode, struct CRYPTO_dynlock_value *l, const char *file, int line)) {}

void CRYPTO_set_dynlock_destroy_callback(void (*dyn_destroy_function)(
    struct CRYPTO_dynlock_value *l, const char *file, int line)) {}

struct CRYPTO_dynlock_value *(*CRYPTO_get_dynlock_create_callback(void))(
    const char *file, int line) {
  return NULL;
}

void (*CRYPTO_get_dynlock_lock_callback(void))(int mode,
                                               struct CRYPTO_dynlock_value *l,
                                               const char *file, int line) {
  return NULL;
}

void (*CRYPTO_get_dynlock_destroy_callback(void))(
    struct CRYPTO_dynlock_value *l, const char *file, int line) {
  return NULL;
}

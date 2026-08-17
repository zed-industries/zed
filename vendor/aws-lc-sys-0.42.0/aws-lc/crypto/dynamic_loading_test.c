// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

// This test verifies that AWS-LC's shared library can safely be
// loaded/unloaded using `dlopen`/`dlclose`.
//
// Problem:
// When a thread terminates, its thread-local data is destructed via a call to
// our internal thread_local_destructor function. However, when our shared
// library is unloaded (via `dlclose`) prior to the thread's termination, it may
// result in a segmentation fault due to the destructor function no longer being
// available.
//
// Building:
// This binary should not be linked to `libcrypto` when built. Doing so would
// result in the `dlclose` being no-op and invalidating the test.
//
// The path to the shared library must be passed as a compiler macro
// `-DLIBCRYPTO_PATH=<<path to libcrypto.so>>` when built.

#include <stdio.h>

#ifdef LIBCRYPTO_PATH

#include <stdint.h>
#include <stdlib.h>
#include <dlfcn.h>
#include <pthread.h>

typedef void (*fp_lc_clear_error_t)(void);
typedef int (*fp_lc_tl_func_t)(void);
typedef int (*fp_rand_bytes_t)(uint8_t *buf, size_t len);

#define BUFFER_SIZE 16
#define TEST_ITERS 10
#define THREAD_COUNT 10

static void *cycle_thread_local_setup(void *lc_so) {
  uint8_t buffer[BUFFER_SIZE];
  fp_lc_clear_error_t lc_clear_error = dlsym(lc_so, "ERR_clear_error");
  fp_rand_bytes_t lc_rand_bytes = dlsym(lc_so, "RAND_bytes");
  fp_lc_tl_func_t lc_tl_clear = dlsym(lc_so, "AWSLC_thread_local_clear");

  for (int i = 0; i < TEST_ITERS; i++) {
    (*lc_clear_error)();
    if (1 != (*lc_rand_bytes)(buffer, BUFFER_SIZE)) {
      fprintf(stderr, "Call to RAND_bytes failed.");
      exit(1);
    }
    if (1 != (*lc_tl_clear)()) {
      fprintf(stderr, "Call to AWSLC_thread_local_clear failed.");
      exit(1);
    }
  }

  return NULL;
}

static void *load_unload(void *ctx) {
  const char* path = ctx;
  void *lc_so = dlopen(path, RTLD_NOW);
  fp_lc_tl_func_t lc_tl_shutdown = dlsym(lc_so, "AWSLC_thread_local_shutdown");

  pthread_t thread_id[THREAD_COUNT];
  for (int i = 0; i < THREAD_COUNT; i++) {
    if (pthread_create(&thread_id[i], NULL, cycle_thread_local_setup, lc_so)) {
      fprintf(stderr, "Call to pthread_create in load_unload failed.");
      exit(1);
    }
  }

  // Also cycle on the current thread
  cycle_thread_local_setup(lc_so);

  for (int i = 0; i < THREAD_COUNT; i++) {
    if (pthread_join(thread_id[i], NULL)) {
      fprintf(stderr, "Call to pthread_join in load_unload failed.");
      exit(1);
    }
  }

  if (1 != (*lc_tl_shutdown)()) {
    fprintf(stderr, "Call to AWSLC_thread_local_shutdown failed.");
    exit(1);
  }
  dlclose(lc_so);

  return NULL;
}


int main(int argc, char *argv[]) {
  pthread_t thread_id;
  if (pthread_create(&thread_id, NULL, load_unload, (void*)LIBCRYPTO_PATH)) {
    fprintf(stderr, "Call to pthread_create in main failed.");
    exit(1);
  }

  if (pthread_join(thread_id, NULL)) {
    fprintf(stderr, "Call to pthread_join in main failed.");
    exit(1);
  }

  fprintf(stdout, "PASS\n");
  fflush(stdout);
  return 0;
}
#else

int main(int argc, char **argv) {
  fprintf(stdout, "PASS\n"); // NOLINT(clang-analyzer-security.insecureAPI.DeprecatedOrUnsafeBufferHandling)
  fflush(stdout);
  return 0;
}

#endif // LIBCRYPTO_PATH

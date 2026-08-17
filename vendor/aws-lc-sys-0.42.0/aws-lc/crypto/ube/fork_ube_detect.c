// Copyright (c) 2020, Google Inc.
// SPDX-License-Identifier: ISC

#include <openssl/target.h>

// Methods to detect fork events aren't generally portable over our supported
// platforms. Fork detection is therefore an opt-in. Capture the opt-in logic
// below that categorizes a platform targets as either having
//  1) fork detection support,
//  2) forking doesn't exist, and
//  3) no fork detection support.
// For (1) we implement sufficient methods for detecting fork events. For (2),
// since forking is not a semantic that exists for the platform, we do not need
// to do anything. Except fake that fork detection is supported. For (3), we
// can't do anything. In this case randomness generation falls back to
// randomizing the state per-request.
#if defined(OPENSSL_LINUX)
  #define AWSLC_FORK_UBE_DETECTION_SUPPORTED
  #if !defined(_GNU_SOURCE)
    #define _GNU_SOURCE  // Needed for madvise() and MAP_ANONYMOUS.
  #endif
#elif defined(OPENSSL_FREEBSD) || defined(OPENSSL_OPENBSD) || defined(OPENSSL_NETBSD)
  #define AWSLC_FORK_UBE_DETECTION_SUPPORTED
  // FreeBSD requires POSIX compatibility off for its syscalls
  // (enables __BSD_VISIBLE). Without the below line, <sys/mman.h> cannot be
  // imported (it requires __BSD_VISIBLE).
  #undef _POSIX_C_SOURCE
#elif defined(OPENSSL_WINDOWS) || defined(OPENSSL_TRUSTY)
  #define AWSLC_PLATFORM_DOES_NOT_FORK
#endif

#include "fork_ube_detect.h"
#include "../internal.h"

static struct CRYPTO_STATIC_MUTEX ignore_testing_lock = CRYPTO_STATIC_MUTEX_INIT;
static int ignore_wipeonfork = 0;
static int ignore_inheritzero = 0;

#if defined(AWSLC_FORK_UBE_DETECTION_SUPPORTED)

#include <openssl/base.h>
#include <openssl/type_check.h>

#include "../internal.h"

#include <stdlib.h>

static CRYPTO_once_t fork_detect_ube_once = CRYPTO_ONCE_INIT;
static struct CRYPTO_STATIC_MUTEX fork_detect_ube_lock = CRYPTO_STATIC_MUTEX_INIT;

// This value (pointed to) is |volatile| because the value pointed to may be
// changed by external forces (i.e. the kernel wiping the page) thus the
// compiler must not assume that it has exclusive access to it.
static volatile char *fork_detect_addr = NULL;
static uint64_t fgn = 0;

static int ignore_all_fork_ube_detection(void) {

  CRYPTO_STATIC_MUTEX_lock_read(&ignore_testing_lock);
  if (ignore_wipeonfork == 1 &&
      ignore_inheritzero == 1) {
    CRYPTO_STATIC_MUTEX_unlock_read(&ignore_testing_lock);
    return 1;
  }
  CRYPTO_STATIC_MUTEX_unlock_read(&ignore_testing_lock);
  return 0;
}

#if defined(OPENSSL_LINUX)

#include <sys/mman.h>
#include <unistd.h>

#if defined(MADV_WIPEONFORK)
OPENSSL_STATIC_ASSERT(MADV_WIPEONFORK == 18, MADV_WIPEONFORK_is_not_18)
#else
#define MADV_WIPEONFORK 18
#endif

static int init_fork_detect_wipeonfork(void *addr, long page_size) {

  GUARD_PTR(addr);

  // Some versions of qemu (up to at least 5.0.0-rc4, see linux-user/syscall.c)
  // ignore |madvise| calls and just return zero (i.e. success). But we need to
  // know whether MADV_WIPEONFORK actually took effect. Therefore try an invalid
  // call to check that the implementation of |madvise| is actually rejecting
  // unknown |advice| values.
  if (madvise(addr, (size_t)page_size, -1) == 0 ||
      madvise(addr, (size_t)page_size, MADV_WIPEONFORK) != 0) {
    return 0;
  }

  return 1;
}

#else // defined(OPENSSL_LINUX)

static int init_fork_detect_wipeonfork(void *addr, long page_size) {
  return 0;
}

#endif // defined(OPENSSL_LINUX)


#if defined(OPENSSL_FREEBSD) || defined(OPENSSL_OPENBSD) || defined(OPENSSL_NETBSD)

#include <sys/mman.h>
#include <unistd.h>

// Sometimes (for example, on FreeBSD) MAP_INHERIT_ZERO is called INHERIT_ZERO
#if !defined(MAP_INHERIT_ZERO) && defined(INHERIT_ZERO)
    #define MAP_INHERIT_ZERO INHERIT_ZERO
#endif

static int init_fork_detect_inheritzero(void *addr, long page_size) {

  GUARD_PTR(addr);

  if (minherit(addr, page_size, MAP_INHERIT_ZERO) != 0) {
    return 0;
  }

  return 1;
}

#else // defined(OPENSSL_FREEBSD) || defined(OPENSSL_OPENBSD)

static int init_fork_detect_inheritzero(void *addr, long page_size) {
  return 0;
}

#endif // defined(OPENSSL_FREEBSD) || defined(OPENSSL_OPENBSD)

// We assume that a method in this function is sufficient to detect fork events.
// Returns 1 if a method is sufficient for fork detection successfully
// initializes. Otherwise returns 0.
static int init_fork_detect_methods_sufficient(void *addr, long page_size) {

  // No sufficient method found.
  int ret = 0;

  CRYPTO_STATIC_MUTEX_lock_read(&ignore_testing_lock);

  if (ignore_wipeonfork != 1 &&
      init_fork_detect_wipeonfork(addr, page_size) == 1) {
    ret = 1;
    goto out;
  }

  if (ignore_inheritzero != 1 &&
      init_fork_detect_inheritzero(addr, page_size) == 1) {
    ret = 1;
    goto out;
  }

out:
  CRYPTO_STATIC_MUTEX_unlock_read(&ignore_testing_lock);
  return ret;
}

// Best-effort attempt to initialize fork detection methods that provides
// defense-in-depth. These methods should not be relied on for fork detection.
// If initialization fails for one of these methods, just ignore it.
static void init_fork_detect_methods_best_effort(void *addr, long page_size) {
  // No methods yet. pthread_at_fork() would be a "best-effort" choice. It can
  // detect fork events through e.g. fork(). But miss fork events through
  // clone().
}

static void init_fork_detect(void) {

  void *addr = MAP_FAILED;
  long page_size = 0;

  if (ignore_all_fork_ube_detection() == 1) {
    return;
  }

  page_size = sysconf(_SC_PAGESIZE);
  if (page_size <= 0) {
    return;
  }

  addr = mmap(NULL, (size_t)page_size, PROT_READ | PROT_WRITE,
                    MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
  if (addr == MAP_FAILED) {
    return;
  }

  // First attempt to initialize a method that is sufficient to detect fork
  // events. If we can't initialize one such method, return without setting
  // |fork_detect_addr|. This means that fork detection is not available.
  if (init_fork_detect_methods_sufficient(addr, page_size) != 1) {
    // No reason to verify return value of munmap() since we can't use that
    // information for anything anyway.
    munmap(addr, (size_t) page_size);
    addr = NULL;
    return;
  }

  // Next, try to initialize any defense-in-depth fork detection methods that
  // might be available. Fail-open.
  init_fork_detect_methods_best_effort(addr, page_size);

  *((volatile char *) addr) = 1;
  fork_detect_addr = addr;
  fgn = 1;
}

uint64_t CRYPTO_get_fork_ube_generation(void) {
  // In a single-threaded process, there are obviously no races because there's
  // only a single mutator in the address space.
  //
  // In a multi-threaded environment, |CRYPTO_once| ensures that the flag byte
  // is initialised atomically, even if multiple threads enter this function
  // concurrently.
  //
  // In the limit, the kernel may clear e.g. WIPEONFORK pages while a
  // multi-threaded process is running. (For example, because a VM was cloned.)
  // Therefore a lock is used below to synchronize the potentially multiple
  // threads that may concurrently observe the cleared flag.
  //
  // One cannot convert this to thread-local values to avoid locking. See e.g.
  // https://github.com/aws/s2n-tls/issues/3107.
  CRYPTO_once(&fork_detect_ube_once, init_fork_detect);

  volatile char *const flag_ptr = fork_detect_addr;
  if (flag_ptr == NULL) {
    // Our kernel does not support fork detection.
    return 0;
  }

  struct CRYPTO_STATIC_MUTEX *const lock = &fork_detect_ube_lock;

  CRYPTO_STATIC_MUTEX_lock_read(lock);
  uint64_t current_fgn = fgn;
  if (*flag_ptr) {
    CRYPTO_STATIC_MUTEX_unlock_read(lock);
    return current_fgn;
  }

  CRYPTO_STATIC_MUTEX_unlock_read(lock);
  CRYPTO_STATIC_MUTEX_lock_write(lock);
  current_fgn = fgn;
  if (*flag_ptr == 0) {
    // A fork has occurred.
    *flag_ptr = 1;

    current_fgn++;
    if (current_fgn == 0) {
      current_fgn = 1;
    }
    fgn = current_fgn;
  }
  CRYPTO_STATIC_MUTEX_unlock_write(lock);

  return current_fgn;
}

#elif defined(AWSLC_PLATFORM_DOES_NOT_FORK)

// These platforms are guaranteed not to fork, and therefore do not require
// fork detection support. Returning a constant non zero value makes BoringSSL
// assume address space duplication is not a concern and adding entropy to
// every RAND_bytes call is not needed.
uint64_t CRYPTO_get_fork_ube_generation(void) { return 0xc0ffee; }

#else

// These platforms may fork, but we do not have a mitigation mechanism in
// place.  Returning a constant zero value makes BoringSSL assume that address
// space duplication could have occured on any call entropy must be added to
// every RAND_bytes call.
uint64_t CRYPTO_get_fork_ube_generation(void) { return 0; }

#endif // defined(AWSLC_FORK_UBE_DETECTION_SUPPORTED)

void CRYPTO_fork_detect_ignore_wipeonfork_FOR_TESTING(void) {
  CRYPTO_STATIC_MUTEX_lock_write(&ignore_testing_lock);
  ignore_wipeonfork = 1;
  CRYPTO_STATIC_MUTEX_unlock_write(&ignore_testing_lock);
}

void CRYPTO_fork_detect_ignore_inheritzero_FOR_TESTING(void) {
  CRYPTO_STATIC_MUTEX_lock_write(&ignore_testing_lock);
  ignore_inheritzero = 1;
  CRYPTO_STATIC_MUTEX_unlock_write(&ignore_testing_lock);
}

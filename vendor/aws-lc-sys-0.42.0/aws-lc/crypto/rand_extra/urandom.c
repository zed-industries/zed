// Copyright (c) 2014, Google Inc.
// SPDX-License-Identifier: ISC

#if !defined(_GNU_SOURCE)
#define _GNU_SOURCE  // needed for syscall() on Linux.
#endif

#include <openssl/rand.h>

#include "internal.h"

#if defined(OPENSSL_RAND_URANDOM)

#include <assert.h>
#include <errno.h>
#include <fcntl.h>
#include <stdio.h>
#include <string.h>
#include <time.h>
#include <unistd.h>

#if defined(OPENSSL_LINUX)
#if defined(AWS_LC_URANDOM_NEEDS_U32)
  // On old Linux OS: unknown type name '__u32' when include <linux/random.h>.
  // If '__u32' is predefined, redefine will cause compiler error.
  typedef unsigned int __u32;
#endif
#include <linux/random.h>
#include <sys/ioctl.h>
#include <sys/param.h>
#include <sys/syscall.h>

#if defined(OPENSSL_ANDROID)
#include <sys/system_properties.h>
#endif

// On non-Android Linux, delegate getauxval availability detection (including
// the /proc/self/auxv fallback for libcs that lack <sys/auxv.h>, such as older
// uclibc) to the shared helper. Android intentionally skips the AT_EXECFN
// debug lookup to avoid depending on getauxval in older NDK sysroots.
#if !defined(OPENSSL_ANDROID)
#include "../fipsmodule/cpucap/cpu_getauxval_linux.h"
#endif
#endif  // OPENSSL_LINUX

#include <openssl/thread.h>
#include <openssl/mem.h>

#include "getrandom_fillin.h"
#include "../internal.h"

#if defined(OPENSSL_MSAN)
void __msan_unpoison(void *, size_t);
#endif


#ifndef MIN
#define AWSLC_MIN(X,Y) (((X) < (Y)) ? (X) : (Y))
#else
#define AWSLC_MIN(X,Y) MIN(X,Y)
#endif

// One second in nanoseconds.
#define ONE_SECOND INT64_C(1000000000)
// 250 milliseconds in nanoseconds.
#define MILLISECONDS_250 INT64_C(250000000)
#define INITIAL_BACKOFF_DELAY 1

enum random_flavor_t {
  NOT_CHOSEN,
  USE_GETRANDOM,
  USE_DEV_URANDOM
};

// random_flavor determines the randomness function used. This is either
// getrandom or /dev/urandom. It's protected by |initialize_random_flavor_once|.
// If both getrandom and /dev/urandom are available, we prefer getrandom. The
// reasons are:
//  - getrandom has better blocking semantics than /dev/urandom; Former blocks
//    on initialization by default.
//  - getrandom doesn't require a file descriptor.
//  - Existence of vgetrandom can yield greater performance, see e.g.
//    https://www.phoronix.com/news/glibc-getrandom-vDSO-Merged.
static enum random_flavor_t random_flavor = NOT_CHOSEN;

enum random_state_t {
  STATE_NOT_READY,
  STATE_READY
};

// random_flavor_state is |STATE_READY| if the entropy pool of |random_flavor|
// has been (fully) initialized and |STATE_NOT_READY| otherwise. Initialized in
// in this context means that the implementation of |random_flavor| has decided
// that enough entropy/noise has been accumulated. It's protected by
// |ensure_random_state_is_initialized_once|.
//
// If |random_flavor_state| has been initialized, we assume it stays
// initialized forever. That is, we assume "entropy depletion" does not exist.
// See e.g. https://www.nccgroup.com/us/research-blog/on-linux-s-random-number-generation/
// for an exposé on entropy depletion.
//
// If a consumer requests a non-blocking read of randomness from
// |random_flavor|, the state |random_flavor_state| is not checked before
// returning randomness.
static enum random_state_t random_flavor_state = STATE_NOT_READY;

// urandom_fd is a file descriptor to /dev/urandom. It's protected by
// |initialize_random_flavor_once|.
static int urandom_fd = -1;

static CRYPTO_once_t initialize_random_flavor_once = CRYPTO_ONCE_INIT;
static CRYPTO_once_t ensure_random_state_is_initialized_once = CRYPTO_ONCE_INIT;

// getrandom_syscall_number_available returns 1 if the getrandom system call
// number is defined, otherwise 0.
static int getrandom_syscall_number_available(void) {
#if defined(USE_NR_getrandom)
  return 1;
#else
  return 0;
#endif
}

// do_backoff initiates exponential backoff. |backoff| holds the previous
// backoff delay. Initial backoff delay is |INITIAL_BACKOFF_DELAY|. This
// function will be called so rarely (if ever), that we keep it as a function
// call and don't care about attempting to inline it.
static void do_backoff(long *backoff) {

  // Exponential backoff.
  //
  // iteration          delay
  // ---------    -----------------
  //    1         10          nsec
  //    2         100         nsec
  //    3         1,000       nsec
  //    4         10,000      nsec
  //    5         100,000     nsec
  //    6         1,000,000   nsec
  //    7         10,000,000  nsec
  //    8         99,999,999  nsec
  //    9         99,999,999  nsec
  //    ...

  struct timespec sleep_time = {.tv_sec = 0, .tv_nsec = 0 };

  // Cap backoff at 99,999,999  nsec, which is the maximum value the nanoseconds
  // field in |timespec| can hold.
  *backoff = AWSLC_MIN((*backoff) * 10, ONE_SECOND - 1);
  // |nanosleep| can mutate |sleep_time|. Hence, we use |backoff| for state.
  sleep_time.tv_nsec = *backoff;

  nanosleep(&sleep_time, &sleep_time);
}

static ssize_t wrapper_dev_urandom(void *buf, size_t buf_len, int block) {

  ssize_t ret = 0;
  size_t retry_counter = 0;
  long backoff = INITIAL_BACKOFF_DELAY;

  do {
    ret = read(urandom_fd, buf, buf_len);
    if ((ret == -1) && (errno != EINTR)) {
      if (!block ||
          retry_counter >= MAX_BACKOFF_RETRIES) {
        break;
      }
      // We have observed extremely rare events in which a |read| on a
      // |urandom| fd failed with |errno| != |EINTR|. We regard this as an
      // intermittent error that is recoverable. Therefore, backoff to allow
      // recovery and to avoid creating a tight spinning loop.
      do_backoff(&backoff);
      retry_counter = retry_counter + 1;
    }
  } while (ret == -1);

  return ret;
}

static ssize_t wrapper_getrandom(void *buf, size_t buf_len, int block) {

  ssize_t ret = -1;
#if defined(USE_NR_getrandom)
  size_t retry_counter = 0;
  long backoff = INITIAL_BACKOFF_DELAY;

  do {
    ret = syscall(__NR_getrandom, buf, buf_len, block ? 0 : GRND_NONBLOCK);
    if ((ret == -1) && (errno != EINTR)) {
      // Don't block in non-block mode except if a signal handler interrupted
      // |getrandom|.
      if (!block ||
          (retry_counter >= MAX_BACKOFF_RETRIES)) {
        break;
      }

      // We have observed extremely rare events in which a |read| on a
      // |urandom| fd failed with |errno| != |EINTR|. |getrandom| uses |urandom|
      // under the covers. Assuming transitivity, |getrandom| is therefore also
      // subject to the same rare error events.
      do_backoff(&backoff);
      retry_counter = retry_counter + 1;
    }
  } while (ret == -1);

#if defined(OPENSSL_MSAN)
  if (ret > 0) {
    // MSAN doesn't recognise |syscall| and thus doesn't notice that we have
    // initialised the output buffer.
    __msan_unpoison(buf, ret);
  }
#endif  // OPENSSL_MSAN
#endif // defined(USE_NR_getrandom)

    return ret;
}

// init_try_getrandom_once tests whether getrandom is supported. Returns 1 if
// getrandom is supported, 0 otherwise.
static int init_try_getrandom_once(void) {

  if (!getrandom_syscall_number_available()) {
    return 0;
  }

  uint8_t dummy = 0;
  ssize_t getrandom_ret = wrapper_getrandom(&dummy, sizeof(dummy), 0);

  if (getrandom_ret == 1) {
    // We have getrandom and entropy pool is also fully initialized.
    random_flavor = USE_GETRANDOM;
    random_flavor_state = STATE_READY;
    return 1;
  } else if (getrandom_ret == -1 && errno == EAGAIN) {
    // We have getrandom, but the entropy pool has not been initialized yet.
    random_flavor = USE_GETRANDOM;
    return 1;
  } else if (getrandom_ret == -1 && errno == ENOSYS) {
    // Kernel doesn't support getrandom.
    return 0;
  } else {
    // Other errors are fatal.
    perror("getrandom");
    abort();
  }
}

// init_try_urandom_once attempts to initialize /dev/urandom. Returns 1 if
// successful, 0 otherwise.
static int init_try_urandom_once(void) {

  int ret = 0;
  int fd = 0;

  do {
    fd = open("/dev/urandom", O_RDONLY);
  } while (fd == -1 && errno == EINTR);

  if (fd < 0) {
    perror("failed to open /dev/urandom");
    goto out;
  }

  int flags = fcntl(fd, F_GETFD);
  if (flags == -1) {
    // Native Client doesn't implement |fcntl|.
    if (errno != ENOSYS) {
      perror("failed to get flags from urandom fd");
      goto out;
    }
  } else {
    flags |= FD_CLOEXEC;
    if (fcntl(fd, F_SETFD, flags) == -1) {
      perror("failed to set FD_CLOEXEC on urandom fd");
      goto out;
    }
  }

  random_flavor = USE_DEV_URANDOM;
  urandom_fd = fd;
  ret = 1;

out:
  return ret;
}

// init_once initializes the state of this module to values previously
// requested. Only function that mutates the chosen random function flavor.
static void init_random_flavor_once(void) {

  // First determine if we can use getrandom. We prefer getrandom.
  if (init_try_getrandom_once()) {
      return;
  }

  // getrandom is not available. Fall-back to /dev/urandom.
  if (init_try_urandom_once()) {
    return;
  }

  // This is a fatal error and there is no way to recover.
  abort();
}

static void ensure_getrandom_is_initialized(void) {

  if (random_flavor_state == STATE_READY) {
    // The entropy pool was already initialized in |init_try_getrandom_once|.
    return;
  }

  uint8_t dummy = 0;
  ssize_t getrandom_ret = wrapper_getrandom(&dummy, sizeof(dummy), 0);

  if (getrandom_ret == -1 && errno == EAGAIN) {
    // Attempt to get the path of the current process to aid in debugging when
    // something blocks.
    const char *current_process = "<unknown>";
#if defined(OPENSSL_HAS_GETAUXVAL)
    const unsigned long getauxval_ret = getauxval(AT_EXECFN);
    if (getauxval_ret != 0) {
      current_process = (const char *)getauxval_ret;
    }
#endif

    fprintf(stderr,
      "%s: getrandom indicates that the entropy pool has not been "
      "initialized. Rather than continue with poor entropy, this process "
      "will block until entropy is available.\n",
      current_process);

    getrandom_ret = wrapper_getrandom(&dummy, sizeof(dummy), 1);
  }

  if (getrandom_ret != 1) {
    perror("getrandom");
    abort();
  }

  random_flavor_state = STATE_READY;
}

static void ensure_dev_urandom_is_initialized(void) {

  // On platforms where urandom doesn't block at startup, we ensure that the
  // kernel has sufficient entropy before continuing. We do this via the
  // RNDGETENTCNT ioctl from <linux/random.h>, which is Linux-specific.
  //
  // On other URANDOM-path platforms (e.g. AIX) we have no portable way to
  // query the kernel entropy pool, so we skip this pre-check and proceed.
#if defined(OPENSSL_LINUX)
  for (;;) {
    int entropy_bits = 0;
    if (ioctl(urandom_fd, RNDGETENTCNT, &entropy_bits)) {
      fprintf(stderr,
        "RNDGETENTCNT on /dev/urandom failed. We cannot determine if the kernel "
        "has enough entropy and must abort.\n");
      abort();
    }

    static const int kBitsNeeded = 256;
    if (entropy_bits >= kBitsNeeded) {
      break;
    }

    fprintf(stderr,
      "RNDGETENTCNT on /dev/urandom indicates that the entropy pool does not "
      "have enough entropy. Rather than continue with poor entropy, this "
      "process will block until entropy is available.\n");

    struct timespec sleep_time = {.tv_sec = 0, .tv_nsec = MILLISECONDS_250 };
    nanosleep(&sleep_time, &sleep_time);
  }
#endif  // OPENSSL_LINUX

  random_flavor_state = STATE_READY;
}

// wait_for_entropy ensures the entropy pool has been initialized. If
// initialization haven't happened yet, it will block the process.
static void ensure_entropy_state_is_initd_once(void) {

  if (random_flavor == USE_GETRANDOM) {
    ensure_getrandom_is_initialized();
  } else if (random_flavor == USE_DEV_URANDOM) {
    ensure_dev_urandom_is_initialized();
  } else {
    // Indicates a state machine logical error which is fatal.
    abort();
  }
}

// fill_with_entropy writes |len| bytes of entropy into |out|. It returns one
// on success and zero on error. If |block| is one, this function will block
// until the entropy pool is initialized. Otherwise, this function may fail,
// setting |errno| to |EAGAIN| if the entropy pool has not yet been initialized.
static int fill_with_entropy(uint8_t *out, size_t len, int block, int seed) {

  if (len == 0) {
    return 1;
  }

  CRYPTO_once(&initialize_random_flavor_once, init_random_flavor_once);

  if (random_flavor == NOT_CHOSEN) {
    // This should never happen. It means there is a logical error somewhere.
    // Hard bail.
    abort();
  }

  if (block) {
    // "Entropy depletion" is not a thing. So, it's enough to ensure the
    // entropy pool is initialized once.
    CRYPTO_once(&ensure_random_state_is_initialized_once, ensure_entropy_state_is_initd_once);

    if (random_flavor_state == STATE_NOT_READY) {
      // This should never happen. It means there is a logical error somewhere.
      // Hard bail.
      abort();
    }
  }

  // Clear |errno| so it has defined value if |read| or |getrandom|
  // "successfully" returns zero.
  errno = 0;
  while (len > 0) {
    ssize_t ret = 0;

    if (random_flavor == USE_GETRANDOM) {
      ret = wrapper_getrandom(out, len, block);
    } else {
      ret = wrapper_dev_urandom(out, len, block);
    }

    if (ret <= 0) {
      return 0;
    }
    out += ret;
    len -= ret;
  }

  return 1;
}

// CRYPTO_sysrand puts |requested| random bytes into |out|.
void CRYPTO_sysrand(uint8_t *out, size_t requested) {
  if (!fill_with_entropy(out, requested, /*block=*/1, /*seed=*/0)) {
    perror("entropy fill failed");
    abort();
  }
}

int CRYPTO_sysrand_if_available(uint8_t *out, size_t requested) {
  if (fill_with_entropy(out, requested, /*block=*/0, /*seed=*/0)) {
    return 1;
  } else if (errno == EAGAIN) {
    OPENSSL_memset(out, 0, requested);
    return 0;
  } else {
    perror("opportunistic entropy fill failed");
    abort();
  }
}

#endif  // OPENSSL_RAND_URANDOM

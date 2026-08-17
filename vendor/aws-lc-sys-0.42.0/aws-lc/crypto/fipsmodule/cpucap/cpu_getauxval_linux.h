// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#ifndef OPENSSL_HEADER_CRYPTO_CPUCAP_GETAUXVAL_LINUX_H
#define OPENSSL_HEADER_CRYPTO_CPUCAP_GETAUXVAL_LINUX_H

#include <openssl/base.h>

// This header provides a unified interface for obtaining auxiliary vector
// values on Linux. When the system provides getauxval (glibc >= 2.16, musl,
// recent uclibc-ng), it is used directly via <sys/auxv.h>. Otherwise, a
// fallback reads /proc/self/auxv.
//
// After including this header (on Linux), OPENSSL_HAS_GETAUXVAL is defined
// and getauxval(type) is callable -- regardless of whether <sys/auxv.h> was
// available -- so call sites do not need to special-case the libc.

#if defined(OPENSSL_LINUX)

// Detect <sys/auxv.h> availability. Use __has_include as the primary
// mechanism since it directly answers the question. Fall back to
// library-specific heuristics for compilers that lack __has_include.
//
// Define OPENSSL_GETAUXVAL_FORCE_PROC_FALLBACK to skip detection and
// unconditionally use the /proc/self/auxv fallback. This is used in CI to
// exercise the fallback path on systems that do have <sys/auxv.h>.
#if !defined(OPENSSL_HAS_GETAUXVAL) && !defined(OPENSSL_GETAUXVAL_FORCE_PROC_FALLBACK)
#if defined(__has_include)
#if __has_include(<sys/auxv.h>)
#define OPENSSL_HAS_GETAUXVAL
#endif
#elif defined(__GLIBC_PREREQ)
#if __GLIBC_PREREQ(2, 16)
#define OPENSSL_HAS_GETAUXVAL
#endif
#elif !defined(__UCLIBC__)
// Non-glibc, non-uclibc libc (e.g. musl) — assume getauxval is available.
#define OPENSSL_HAS_GETAUXVAL
#endif
#endif  // !defined(OPENSSL_HAS_GETAUXVAL) && !defined(OPENSSL_GETAUXVAL_FORCE_PROC_FALLBACK)

#if defined(OPENSSL_HAS_GETAUXVAL)
#include <sys/auxv.h>
#else

// When <sys/auxv.h> is not available (e.g. older uclibc without getauxval),
// fall back to reading /proc/self/auxv. The file contains sequential pairs of
// unsigned long values: [type_0, value_0, type_1, value_1, ..., AT_NULL, 0].

#include <errno.h>
#include <fcntl.h>
#include <unistd.h>

// O_CLOEXEC was added in Linux 2.6.23 / glibc 2.7. Older toolchains (e.g.
// manylinux1 / CentOS 5 with glibc 2.5) do not define it. Fall back to 0;
// the fd is opened, read, and closed synchronously within this function
// (no intervening fork), so close-on-exec is not security-relevant here.
#if !defined(O_CLOEXEC)
#define O_CLOEXEC 0
#endif

// Auxiliary vector type constants from the Linux kernel ABI
// (include/uapi/linux/auxvec.h). The specific values used here are stable.
#if !defined(AT_NULL)
#define AT_NULL 0
#endif
#if !defined(AT_HWCAP)
#define AT_HWCAP 16
#endif
#if !defined(AT_HWCAP2)
#define AT_HWCAP2 26
#endif
#if !defined(AT_EXECFN)
#define AT_EXECFN 31
#endif

// getauxval returns the value of the auxiliary-vector entry of the given
// |type|, or 0 if no such entry exists or /proc/self/auxv is unavailable.
//
// Note: unlike glibc >= 2.19, this implementation does not set errno to
// ENOENT when |type| is not present. No caller in AWS-LC inspects errno
// after getauxval, so this difference is not observable.
//
// OPENSSL_UNUSED guards against -Wunused-function in translation units that
// include this header but do not call getauxval (none today, but cheap
// insurance against future refactors).
static OPENSSL_UNUSED unsigned long getauxval(unsigned long type) {
  const int saved_errno = errno;
  int fd;
  do {
    fd = open("/proc/self/auxv", O_RDONLY | O_CLOEXEC);
  } while (fd < 0 && errno == EINTR);
  if (fd < 0) {
    errno = saved_errno;
    return 0;
  }

  unsigned long result = 0;
  unsigned long pair[2];
  // Bound the loop defensively. Real auxv vectors have on the order of 30
  // entries; 128 is a comfortable upper bound that guarantees forward
  // progress even if /proc/self/auxv is malformed or missing its AT_NULL
  // terminator.
  for (int i = 0; i < 128; i++) {
    // read() on /proc/self/auxv may in principle return a short count; loop
    // until a full pair is read or we hit EOF/error.
    size_t filled = 0;
    while (filled < sizeof(pair)) {
      ssize_t n = read(fd, (char *)pair + filled, sizeof(pair) - filled);
      if (n > 0) {
        filled += (size_t)n;
      } else if (n < 0 && errno == EINTR) {
        continue;
      } else {
        // EOF or unrecoverable error. Whatever we've accumulated so far is
        // unusable; treat the lookup as "not found".
        goto done;
      }
    }
    if (pair[0] == AT_NULL) {
      // AT_NULL terminates the vector.
      break;
    }
    if (pair[0] == type) {
      result = pair[1];
      break;
    }
  }

done:
  close(fd);
  errno = saved_errno;
  return result;
}

// Expose OPENSSL_HAS_GETAUXVAL so that callers can uniformly gate on
// "getauxval is callable" without caring whether it came from the libc or
// the fallback above.
#define OPENSSL_HAS_GETAUXVAL

#endif  // OPENSSL_HAS_GETAUXVAL

#endif  // OPENSSL_LINUX

#endif  // OPENSSL_HEADER_CRYPTO_CPUCAP_GETAUXVAL_LINUX_H

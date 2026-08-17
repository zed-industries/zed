// Copyright (c) 2016, Google Inc.
// SPDX-License-Identifier: ISC

#include "internal.h"

#if defined(OPENSSL_ARM) && defined(OPENSSL_LINUX) && \
    !defined(OPENSSL_STATIC_ARMCAP)
#include <errno.h>
#include <fcntl.h>
#include <sys/types.h>
#include <unistd.h>

#include <openssl/arm_arch.h>
#include <openssl/mem.h>

#include "cpu_arm_linux.h"
#include "cpu_getauxval_linux.h"

static int open_eintr(const char *path, int flags) {
  int ret;
  do {
    ret = open(path, flags);
  } while (ret < 0 && errno == EINTR);
  return ret;
}

static ssize_t read_eintr(int fd, void *out, size_t len) {
  ssize_t ret;
  do {
    ret = read(fd, out, len);
  } while (ret < 0 && errno == EINTR);
  return ret;
}

// read_file opens |path| and reads until end-of-file. On success, it returns
// one and sets |*out_ptr| and |*out_len| to a newly-allocated buffer with the
// contents. Otherwise, it returns zero.
static int read_file(char **out_ptr, size_t *out_len, const char *path) {
  int fd = open_eintr(path, O_RDONLY);
  if (fd < 0) {
    return 0;
  }

  static const size_t kReadSize = 1024;
  int ret = 0;
  size_t cap = kReadSize, len = 0;
  char *buf = OPENSSL_malloc(cap);
  if (buf == NULL) {
    goto err;
  }

  for (;;) {
    if (cap - len < kReadSize) {
      size_t new_cap = cap * 2;
      if (new_cap < cap) {
        goto err;
      }
      char *new_buf = OPENSSL_realloc(buf, new_cap);
      if (new_buf == NULL) {
        goto err;
      }
      buf = new_buf;
      cap = new_cap;
    }

    ssize_t bytes_read = read_eintr(fd, buf + len, kReadSize);
    if (bytes_read < 0) {
      goto err;
    }
    if (bytes_read == 0) {
      break;
    }
    len += bytes_read;
  }

  *out_ptr = buf;
  *out_len = len;
  ret = 1;
  buf = NULL;

err:
  OPENSSL_free(buf);
  close(fd);
  return ret;
}

static int g_needs_hwcap2_workaround;

void OPENSSL_cpuid_setup(void) {
  // We ignore the return value of |read_file| and proceed with an empty
  // /proc/cpuinfo on error. If |getauxval| works, we will still detect
  // capabilities.
  char *cpuinfo_data = NULL;
  size_t cpuinfo_len = 0;
  read_file(&cpuinfo_data, &cpuinfo_len, "/proc/cpuinfo");
  STRING_PIECE cpuinfo;
  cpuinfo.data = cpuinfo_data;
  cpuinfo.len = cpuinfo_len;

  // Matching OpenSSL, only report other features if NEON is present.
  unsigned long hwcap = getauxval(AT_HWCAP);
  if (hwcap & HWCAP_NEON) {
    OPENSSL_armcap_P |= ARMV7_NEON;

    // Some ARMv8 Android devices don't expose AT_HWCAP2. Fall back to
    // /proc/cpuinfo. See https://crbug.com/boringssl/46. As of February 2021,
    // this is now rare (see Chrome's Net.NeedsHWCAP2Workaround metric), but AES
    // and PMULL extensions are very useful, so we still carry the workaround
    // for now.
#if defined(AT_HWCAP2)
    unsigned long hwcap2 = getauxval(AT_HWCAP2);
#else
    unsigned long hwcap2 = 0;
#endif
    if (hwcap2 == 0) {
      hwcap2 = crypto_get_arm_hwcap2_from_cpuinfo(&cpuinfo);
      g_needs_hwcap2_workaround = hwcap2 != 0;
    }

    if (hwcap2 & HWCAP2_AES) {
      OPENSSL_armcap_P |= ARMV8_AES;
    }
    if (hwcap2 & HWCAP2_PMULL) {
      OPENSSL_armcap_P |= ARMV8_PMULL;
    }
    if (hwcap2 & HWCAP2_SHA1) {
      OPENSSL_armcap_P |= ARMV8_SHA1;
    }
    if (hwcap2 & HWCAP2_SHA2) {
      OPENSSL_armcap_P |= ARMV8_SHA256;
    }
  }

  OPENSSL_free(cpuinfo_data);

  OPENSSL_cpucap_initialized = 1;
}

int CRYPTO_has_broken_NEON(void) { return 0; }

int CRYPTO_needs_hwcap2_workaround(void) { return g_needs_hwcap2_workaround; }

#endif  // OPENSSL_ARM && OPENSSL_LINUX && !OPENSSL_STATIC_ARMCAP

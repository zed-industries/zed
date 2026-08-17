// Copyright 2018 The BoringSSL Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef OPENSSL_HEADER_CRYPTO_CPU_ARM_LINUX_H
#define OPENSSL_HEADER_CRYPTO_CPU_ARM_LINUX_H

#include <openssl/base.h>

#include <string.h>

#include "internal.h"

#if defined(__cplusplus)
extern "C" {
#endif


// The cpuinfo parser lives in a header file so it may be accessible from
// cross-platform fuzzers without adding code to those platforms normally.

#define CRYPTO_HWCAP_NEON (1 << 12)

// See /usr/include/asm/hwcap.h on an ARM installation for the source of
// these values.
// We add the prefix "CRYPTO_" to the definitions so as not to collide with
// some versions of glibc (>= 2.41) that expose them through <sys/auxv.h>.
#define CRYPTO_HWCAP2_AES (1 << 0)
#define CRYPTO_HWCAP2_PMULL (1 << 1)
#define CRYPTO_HWCAP2_SHA1 (1 << 2)
#define CRYPTO_HWCAP2_SHA2 (1 << 3)

typedef struct {
  const char *data;
  size_t len;
} STRING_PIECE;

static int STRING_PIECE_equals(const STRING_PIECE *a, const char *b) {
  size_t b_len = strlen(b);
  return a->len == b_len && OPENSSL_memcmp(a->data, b, b_len) == 0;
}

// STRING_PIECE_split finds the first occurence of |sep| in |in| and, if found,
// sets |*out_left| and |*out_right| to |in| split before and after it. It
// returns one if |sep| was found and zero otherwise.
static int STRING_PIECE_split(STRING_PIECE *out_left, STRING_PIECE *out_right,
                              const STRING_PIECE *in, char sep) {
  const char *p = (const char *)OPENSSL_memchr(in->data, sep, in->len);
  if (p == NULL) {
    return 0;
  }
  // |out_left| or |out_right| may alias |in|, so make a copy.
  STRING_PIECE in_copy = *in;
  out_left->data = in_copy.data;
  out_left->len = p - in_copy.data;
  out_right->data = in_copy.data + out_left->len + 1;
  out_right->len = in_copy.len - out_left->len - 1;
  return 1;
}

// STRING_PIECE_get_delimited reads a |sep|-delimited entry from |s|, writing it
// to |out| and updating |s| to point beyond it. It returns one on success and
// zero if |s| is empty. If |s| is has no copies of |sep| and is non-empty, it
// reads the entire string to |out|.
static int STRING_PIECE_get_delimited(STRING_PIECE *s, STRING_PIECE *out, char sep) {
  if (s->len == 0) {
    return 0;
  }
  if (!STRING_PIECE_split(out, s, s, sep)) {
    // |s| had no instances of |sep|. Return the entire string.
    *out = *s;
    s->data += s->len;
    s->len = 0;
  }
  return 1;
}

// STRING_PIECE_trim removes leading and trailing whitespace from |s|.
static void STRING_PIECE_trim(STRING_PIECE *s) {
  while (s->len != 0 && (s->data[0] == ' ' || s->data[0] == '\t')) {
    s->data++;
    s->len--;
  }
  while (s->len != 0 &&
         (s->data[s->len - 1] == ' ' || s->data[s->len - 1] == '\t')) {
    s->len--;
  }
}

// extract_cpuinfo_field extracts a /proc/cpuinfo field named |field| from
// |in|. If found, it sets |*out| to the value and returns one. Otherwise, it
// returns zero.
static int extract_cpuinfo_field(STRING_PIECE *out, const STRING_PIECE *in,
                                 const char *field) {
  // Process |in| one line at a time.
  STRING_PIECE remaining = *in, line;
  while (STRING_PIECE_get_delimited(&remaining, &line, '\n')) {
    STRING_PIECE key, value;
    if (!STRING_PIECE_split(&key, &value, &line, ':')) {
      continue;
    }
    STRING_PIECE_trim(&key);
    if (STRING_PIECE_equals(&key, field)) {
      STRING_PIECE_trim(&value);
      *out = value;
      return 1;
    }
  }

  return 0;
}

// has_list_item treats |list| as a space-separated list of items and returns
// one if |item| is contained in |list| and zero otherwise.
static int has_list_item(const STRING_PIECE *list, const char *item) {
  STRING_PIECE remaining = *list, feature;
  while (STRING_PIECE_get_delimited(&remaining, &feature, ' ')) {
    if (STRING_PIECE_equals(&feature, item)) {
      return 1;
    }
  }
  return 0;
}

// crypto_get_arm_hwcap2_from_cpuinfo returns an equivalent ARM |AT_HWCAP2|
// value from |cpuinfo|.
static unsigned long crypto_get_arm_hwcap2_from_cpuinfo(
    const STRING_PIECE *cpuinfo) {
  STRING_PIECE features;
  if (!extract_cpuinfo_field(&features, cpuinfo, "Features")) {
    return 0;
  }

  unsigned long ret = 0;
  if (has_list_item(&features, "aes")) {
    ret |= CRYPTO_HWCAP2_AES;
  }
  if (has_list_item(&features, "pmull")) {
    ret |= CRYPTO_HWCAP2_PMULL;
  }
  if (has_list_item(&features, "sha1")) {
    ret |= CRYPTO_HWCAP2_SHA1;
  }
  if (has_list_item(&features, "sha2")) {
    ret |= CRYPTO_HWCAP2_SHA2;
  }
  return ret;
}


#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_CRYPTO_CPU_ARM_LINUX_H

#ifndef __HASHFUNCTIONS_H__
#define __HASHFUNCTIONS_H__

#include <stdio.h>
#include <string.h>
#include <gtest/gtest.h>
#include "../sha1.h"
#include "crt_util_safe_x.h"

static void ToHashStr (char* dst, const unsigned char* src, size_t src_len) {
  for (size_t i = 0; i < src_len; ++i) {
    WelsSnprintf (&dst[i * 2], 3, "%.2x", src[i]);
  }
  dst[src_len * 2] = '\0';
}

inline void CompareHash (const unsigned char* digest, const char* hashStr) {
  char hashStrCmp[SHA_DIGEST_LENGTH * 2 + 1];
  ToHashStr (hashStrCmp, digest, SHA_DIGEST_LENGTH);
  EXPECT_STREQ (hashStr, hashStrCmp);
}

inline void CompareHashAnyOf (const unsigned char* digest, const char* const hashStr[], size_t nHashStr) {
  char hashStrCmp[SHA_DIGEST_LENGTH * 2 + 1];
  ToHashStr (hashStrCmp, digest, SHA_DIGEST_LENGTH);
  for (size_t i = 0; i < nHashStr && hashStr[i]; ++i) {
    if (0 == strcmp (hashStr[i], hashStrCmp))
      return;
  }
  // No match found. Compare to first hash so as to produce a grepable failure.
  EXPECT_STREQ (hashStr[0], hashStrCmp);
}

#endif //__HASHFUNCTIONS_H__

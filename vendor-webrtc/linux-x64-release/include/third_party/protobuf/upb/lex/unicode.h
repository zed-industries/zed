// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef UPB_LEX_UNICODE_H_
#define UPB_LEX_UNICODE_H_

#include <stdint.h>

// Must be last.
#include "upb/port/def.inc"

#ifdef __cplusplus
extern "C" {
#endif

// Returns true iff a codepoint is the value for a high surrogate.
UPB_INLINE bool upb_Unicode_IsHigh(uint32_t cp) {
  return (cp >= 0xd800 && cp <= 0xdbff);
}

// Returns true iff a codepoint is the value for a low surrogate.
UPB_INLINE bool upb_Unicode_IsLow(uint32_t cp) {
  return (cp >= 0xdc00 && cp <= 0xdfff);
}

// Returns the high 16-bit surrogate value for a supplementary codepoint.
// Does not sanity-check the input.
UPB_INLINE uint16_t upb_Unicode_ToHigh(uint32_t cp) {
  return (cp >> 10) + 0xd7c0;
}

// Returns the low 16-bit surrogate value for a supplementary codepoint.
// Does not sanity-check the input.
UPB_INLINE uint16_t upb_Unicode_ToLow(uint32_t cp) {
  return (cp & 0x3ff) | 0xdc00;
}

// Returns the 32-bit value corresponding to a pair of 16-bit surrogates.
// Does not sanity-check the input.
UPB_INLINE uint32_t upb_Unicode_FromPair(uint32_t high, uint32_t low) {
  return ((high & 0x3ff) << 10) + (low & 0x3ff) + 0x10000;
}

// Outputs a codepoint as UTF8.
// Returns the number of bytes written (1-4 on success, 0 on error).
// Does not sanity-check the input. Specifically does not check for surrogates.
int upb_Unicode_ToUTF8(uint32_t cp, char* out);

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif /* UPB_LEX_UNICODE_H_ */

/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil; tab-width: 4 -*- */
/* vi: set expandtab shiftwidth=4 tabstop=4: */

/**
 * \file
 * <PRE>
 * High performance base64 encoder / decoder
 * Version 1.3 -- 17-Mar-2006
 *
 * Copyright &copy; 2005, 2006, Nick Galbreath -- nickg [at] modp [dot] com
 * All rights reserved.
 *
 * http://modp.com/release/base64
 *
 * Released under bsd license.  See modp_b64.c for details.
 * </pre>
 *
 * The default implementation is the standard b64 encoding with padding.
 * It's easy to change this to use "URL safe" characters and to remove
 * padding.  See the modp_b64.c source code for details.
 *
 */

#ifndef MODP_B64
#define MODP_B64

#include <limits.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Encode a raw binary string into base 64.
 * src contains the bytes
 * len contains the number of bytes in the src
 * dest should be allocated by the caller to contain
 *   at least modp_b64_encode_len(len) bytes (see below)
 *   This will contain the (non-null terminated) b64 bytes.
 * returns length of the destination string.
 *
 * Example
 *
 * \code
 * char* src = ...;
 * int srclen = ...; //the length of number of bytes in src
 * char* dest = (char*) malloc(modp_b64_encode_len);
 * int len = modp_b64_encode_data(dest, src, sourcelen);
 * if (len == -1) {
 *   printf("Error\n");
 * } else {
 *   printf("b64 = %s\n", dest);
 * }
 * \endcode
 *
 */
size_t modp_b64_encode_data(char* dest, const char* str, size_t len);

/**
 * Same as modp_b64_encode_data, but additionally sets a null terminator at the
 * end of `dest` (i.e. at dest[output_size]).
 * Like modp_b64_encode_data, returns the length of the destination string (i.e.
 * not counting the null terminator).
 *
 * TODO(csharrison): Consider removing this once all callers migrate to
 * modp_b64_encode_data.
 */
size_t modp_b64_encode(char* dest, const char* str, size_t len);

/**
 * Decode a base64 encoded string
 *
 * src should contain exactly len bytes of b64 characters.
 *     if src contains -any- non-base characters (such as white
 *     space, -1 is returned.
 *
 * dest should be allocated by the caller to contain at least
 *    len * 3 / 4 bytes.
 *
 * Returns the length (strlen) of the output, or -1 if unable to
 * decode
 *
 * \code
 * char* src = ...;
 * int srclen = ...; // or if you don't know use strlen(src)
 * char* dest = (char*) malloc(modp_b64_decode_len(srclen));
 * int len = modp_b64_decode(dest, src, sourcelen);
 * if (len == -1) { error }
 * \endcode
 */
enum class ModpDecodePolicy {
  // src length must be divisible by 4, with a max of 2 pad chars.
  kStrict,

  // Matches the infra spec: https://infra.spec.whatwg.org/#forgiving-base64
  // _except_ for ignoring whitespace (Step 1).
  kForgiving,

  // src length % 4 must not equal 1, after stripping all pad chars.
  // Accepts any number of pad chars.
  kNoPaddingValidation,
};
size_t modp_b64_decode(
    char* dest,
    const char* src,
    size_t len,
    ModpDecodePolicy policy = ModpDecodePolicy::kStrict);

/**
 * The maximum input that can be passed into modp_b64_encode{_data}.
 * Lengths beyond this will overflow modp_b64_encode_len.
 *
 * This works because modp_b64_encode_len(A) computes:
 *     ceiling[max_len / 3] * 4 + 1
 *   = ceiling[floor[(SIZE_MAX-1)/4]*3 / 3] * 4 + 1
 *   = floor[(SIZE_MAX-1)/4] * 4 + 1
 *  <= SIZE_MAX-1 + 1
 *   = SIZE_MAX
 *
 * Note: technically modp_b64_encode_data can take one extra byte, but for
 * simplicity the bound is shared between the two functions.
 */
#define MODP_B64_MAX_INPUT_LEN ((SIZE_MAX - 1) / 4 * 3)

/**
 * Given a source string of length len, this returns the amount of
 * memory the destination string should have, for modp_b64_encode_data and
 * modp_b64_encode, respectively.
 *
 * remember, this is integer math
 * 3 bytes turn into 4 chars
 * ceiling[len / 3] * 4
 *
 *
 * WARNING: These expressions will overflow if the A is above
 * MODP_B64_MAX_INPUT_LEN. The caller must check this bound first.
 */
#define modp_b64_encode_data_len(A) ((A + 2) / 3 * 4)
#define modp_b64_encode_len(A) (modp_b64_encode_data_len(A) + 1)

/**
 * Given a base64 string of length len,
 *   this returns the amount of memory required for output string
 *  It maybe be more than the actual number of bytes written.
 * NOTE: remember this is integer math
 * this allocates a bit more memory than traditional versions of b64
 * decode  4 chars turn into 3 bytes
 * floor[len * 3/4] + 2
 */
#define modp_b64_decode_len(A) (A / 4 * 3 + 2)

#define MODP_B64_ERROR ((size_t)-1)

#ifdef __cplusplus
}
#endif /* __cplusplus */

#endif /* MODP_B64 */

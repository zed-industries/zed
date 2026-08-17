// Copyright 1995-2016 The OpenSSL Project Authors. All Rights Reserved.
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

#ifndef OPENSSL_HEADER_BASE64_H
#define OPENSSL_HEADER_BASE64_H

#include <openssl/base.h>   // IWYU pragma: export

#if defined(__cplusplus)
extern "C" {
#endif


// base64 functions.
//
// For historical reasons, these functions have the EVP_ prefix but just do
// base64 encoding and decoding. Note that BoringSSL is a cryptography library,
// so these functions are implemented with side channel protections, at a
// performance cost. For other base64 uses, use a general-purpose base64
// implementation.


// Encoding

// EVP_EncodeBlock encodes |src_len| bytes from |src| and writes the
// result to |dst| with a trailing NUL. It returns the number of bytes
// written, not including this trailing NUL.
OPENSSL_EXPORT size_t EVP_EncodeBlock(uint8_t *dst, const uint8_t *src,
                                      size_t src_len);

// EVP_EncodedLength sets |*out_len| to the number of bytes that will be needed
// to call |EVP_EncodeBlock| on an input of length |len|. This includes the
// final NUL that |EVP_EncodeBlock| writes. It returns one on success or zero
// on error.
OPENSSL_EXPORT int EVP_EncodedLength(size_t *out_len, size_t len);


// Decoding

// EVP_DecodedLength sets |*out_len| to the maximum number of bytes that will
// be needed to call |EVP_DecodeBase64| on an input of length |len|. It returns
// one on success or zero if |len| is not a valid length for a base64-encoded
// string.
OPENSSL_EXPORT int EVP_DecodedLength(size_t *out_len, size_t len);

// EVP_DecodeBase64 decodes |in_len| bytes from base64 and writes
// |*out_len| bytes to |out|. |max_out| is the size of the output
// buffer. If it is not enough for the maximum output size, the
// operation fails. It returns one on success or zero on error.
OPENSSL_EXPORT int EVP_DecodeBase64(uint8_t *out, size_t *out_len,
                                    size_t max_out, const uint8_t *in,
                                    size_t in_len);


// Deprecated functions.
//
// OpenSSL provides a streaming base64 implementation, however its behavior is
// very specific to PEM. It is also very lenient of invalid input. Use of any of
// these functions is thus deprecated.

// EVP_ENCODE_CTX_new returns a newly-allocated |EVP_ENCODE_CTX| or NULL on
// error. The caller must release the result with |EVP_ENCODE_CTX_free|  when
// done.
OPENSSL_EXPORT EVP_ENCODE_CTX *EVP_ENCODE_CTX_new(void);

// EVP_ENCODE_CTX_free releases memory associated with |ctx|.
OPENSSL_EXPORT void EVP_ENCODE_CTX_free(EVP_ENCODE_CTX *ctx);

// EVP_EncodeInit initialises |*ctx|, which is typically stack
// allocated, for an encoding operation.
//
// NOTE: The encoding operation breaks its output with newlines every
// 64 characters of output (48 characters of input). Use
// EVP_EncodeBlock to encode raw base64.
OPENSSL_EXPORT void EVP_EncodeInit(EVP_ENCODE_CTX *ctx);

// EVP_EncodeUpdate encodes |in_len| bytes from |in| and writes an encoded
// version of them to |out| and sets |*out_len| to the number of bytes written.
// Some state may be contained in |ctx| so |EVP_EncodeFinal| must be used to
// flush it before using the encoded data.
OPENSSL_EXPORT void EVP_EncodeUpdate(EVP_ENCODE_CTX *ctx, uint8_t *out,
                                     int *out_len, const uint8_t *in,
                                     size_t in_len);

// EVP_EncodeFinal flushes any remaining output bytes from |ctx| to |out| and
// sets |*out_len| to the number of bytes written.
OPENSSL_EXPORT void EVP_EncodeFinal(EVP_ENCODE_CTX *ctx, uint8_t *out,
                                    int *out_len);

// EVP_DecodeInit initialises |*ctx|, which is typically stack allocated, for
// a decoding operation.
//
// TODO(davidben): This isn't a straight-up base64 decode either. Document
// and/or fix exactly what's going on here; maximum line length and such.
OPENSSL_EXPORT void EVP_DecodeInit(EVP_ENCODE_CTX *ctx);

// EVP_DecodeUpdate decodes |in_len| bytes from |in| and writes the decoded
// data to |out| and sets |*out_len| to the number of bytes written. Some state
// may be contained in |ctx| so |EVP_DecodeFinal| must be used to flush it
// before using the encoded data.
//
// It returns -1 on error, one if a full line of input was processed and zero
// if the line was short (i.e. it was the last line).
OPENSSL_EXPORT int EVP_DecodeUpdate(EVP_ENCODE_CTX *ctx, uint8_t *out,
                                    int *out_len, const uint8_t *in,
                                    size_t in_len);

// EVP_DecodeFinal flushes any remaining output bytes from |ctx| to |out| and
// sets |*out_len| to the number of bytes written. It returns one on success
// and minus one on error.
OPENSSL_EXPORT int EVP_DecodeFinal(EVP_ENCODE_CTX *ctx, uint8_t *out,
                                   int *out_len);

// EVP_DecodeBlock encodes |src_len| bytes from |src| and writes the result to
// |dst|. It returns the number of bytes written or -1 on error.
//
// WARNING: EVP_DecodeBlock's return value does not take padding into
// account. It also strips leading whitespace and trailing
// whitespace and minuses.
OPENSSL_EXPORT int EVP_DecodeBlock(uint8_t *dst, const uint8_t *src,
                                   size_t src_len);


struct evp_encode_ctx_st {
  // data_used indicates the number of bytes of |data| that are valid. When
  // encoding, |data| will be filled and encoded as a lump. When decoding, only
  // the first four bytes of |data| will be used.
  unsigned data_used;
  uint8_t data[48];

  // eof_seen indicates that the end of the base64 data has been seen when
  // decoding. Only whitespace can follow.
  char eof_seen;

  // error_encountered indicates that invalid base64 data was found. This will
  // cause all future calls to fail.
  char error_encountered;
};


#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_BASE64_H

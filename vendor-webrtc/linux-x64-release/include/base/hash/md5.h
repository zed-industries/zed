// Copyright 2011 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_HASH_MD5_H_
#define BASE_HASH_MD5_H_

#include <string>
#include <string_view>

#include "base/base_export.h"
#include "base/containers/span.h"
#include "build/build_config.h"

#if BUILDFLAG(IS_NACL)
#include "base/hash/md5_nacl.h"
#else
#include "base/hash/md5_boringssl.h"
#endif

// MD5 stands for Message Digest algorithm 5.
//
// DANGER DANGER DANGER:
// MD5 is extremely obsolete and it is trivial for a malicious party to find MD5
// collisions. Do not use MD5 for any security-related purposes whatsoever, and
// especially do not use MD5 to validate that files or other data have not been
// modified maliciously. This entire interface is obsolete and you should either
// use a non-cryptographic hash (which will be much faster) or a cryptographic
// hash (which will be collision-resistant against adversarial inputs). If you
// believe you need to add a new use of MD5, consult a member of
// //CRYPTO_OWNERS.
//
// NEW USES OF THIS API ARE FORBIDDEN FOR ANY PURPOSE. INSTEAD, YOU MUST USE
// //crypto/obsolete/md5.h.

// These functions perform MD5 operations. The simplest call is MD5Sum() to
// generate the MD5 sum of the given data.
//
// You can also compute the MD5 sum of data incrementally by making multiple
// calls to MD5Update():
//   MD5Context ctx; // intermediate MD5 data: do not use
//   MD5Init(&ctx);
//   MD5Update(&ctx, data1, length1);
//   MD5Update(&ctx, data2, length2);
//   ...
//
//   MD5Digest digest; // the result of the computation
//   MD5Final(&digest, &ctx);
//
// You can call MD5DigestToBase16() to generate a string of the digest.

namespace base {

// Initializes the given MD5 context structure for subsequent calls to
// MD5Update().
BASE_EXPORT void MD5Init(MD5Context* context);

// For the given buffer of |data| as a std::string_view or span, updates the
// given MD5 context with the sum of the data. You can call this any number of
// times during the computation, except that MD5Init() must have been called
// first.
BASE_EXPORT void MD5Update(MD5Context* context, std::string_view data);
BASE_EXPORT void MD5Update(MD5Context* context, base::span<const uint8_t> data);

// Finalizes the MD5 operation and fills the buffer with the digest.
BASE_EXPORT void MD5Final(MD5Digest* digest, MD5Context* context);

// Converts a digest into human-readable hexadecimal.
BASE_EXPORT std::string MD5DigestToBase16(const MD5Digest& digest);

// Computes the MD5 sum of the given `data`.
// The 'digest' structure will be filled with the result.
BASE_EXPORT void MD5Sum(base::span<const uint8_t> data, MD5Digest* digest);

// Returns the MD5 (in hexadecimal) of a string.
BASE_EXPORT std::string MD5String(std::string_view str);

}  // namespace base

#endif  // BASE_HASH_MD5_H_

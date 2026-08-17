// Copyright 2011 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_HASH_SHA1_H_
#define BASE_HASH_SHA1_H_

#include <stddef.h>

#include <array>
#include <string>
#include <string_view>

#include "base/base_export.h"
#include "base/compiler_specific.h"
#include "base/containers/span.h"
#include "build/build_config.h"
#if BUILDFLAG(IS_NACL)
#include "base/hash/sha1_nacl.h"
#else
#include "base/hash/sha1_boringssl.h"
#endif

namespace base {

enum { kSHA1Length = 20 };  // Length in bytes of a SHA-1 hash.

// The output of an SHA-1 operation.
using SHA1Digest = std::array<uint8_t, kSHA1Length>;

// These functions perform SHA-1 operations.
// Computes the SHA-1 hash of the input |data| and returns the full hash.
BASE_EXPORT SHA1Digest SHA1Hash(span<const uint8_t> data);
// Computes the SHA-1 hash of the input string |str| and returns the full
// hash.
BASE_EXPORT std::string SHA1HashString(std::string_view str);

// These functions allow streaming SHA-1 operations.
BASE_EXPORT void SHA1Init(SHA1Context& context);
BASE_EXPORT void SHA1Update(std::string_view data, SHA1Context& context);
BASE_EXPORT void SHA1Final(SHA1Context& context, SHA1Digest& digest);
}  // namespace base

#endif  // BASE_HASH_SHA1_H_

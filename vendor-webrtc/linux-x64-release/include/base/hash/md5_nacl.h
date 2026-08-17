// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_HASH_MD5_NACL_H_
#define BASE_HASH_MD5_NACL_H_

#include <stdint.h>

#include <array>

namespace base {

// The output of an MD5 operation.
struct MD5Digest {
  std::array<uint8_t, 16> a;
};

// Used for storing intermediate data during an MD5 computation. Callers
// should not access the data.
typedef char MD5Context[88];

}  // namespace base

#endif  // BASE_HASH_MD5_NACL_H_

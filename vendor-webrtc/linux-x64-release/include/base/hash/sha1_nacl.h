// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_HASH_SHA1_NACL_H_
#define BASE_HASH_SHA1_NACL_H_

#include <stdint.h>

namespace base {

// Used for storing intermediate data during an SHA1 computation. Callers
// should not access the data.
class SHA1Context {
 public:
  void Init();
  void Update(const void* data, size_t nbytes);
  void Final();
  const unsigned char* GetDigest() const;

 private:
  void Pad();
  void Process();

  uint32_t A, B, C, D, E;

  uint32_t H[5];

  union {
    uint32_t W[80];
    uint8_t M[64];
  };

  uint32_t cursor;
  uint64_t l;
};

}  // namespace base

#endif  // BASE_HASH_SHA1_NACL_H_

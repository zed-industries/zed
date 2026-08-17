// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_IPC_FUZZER_MUTATE_RAND_UTIL_H_
#define TOOLS_IPC_FUZZER_MUTATE_RAND_UTIL_H_

#include <stddef.h>
#include <stdint.h>
#include <random>

namespace ipc_fuzzer {

extern std::mt19937* g_mersenne_twister;

void InitRand();

inline uint32_t RandU32() {
  return (*g_mersenne_twister)();
}

inline uint64_t RandU64() {
  return (static_cast<uint64_t>(RandU32()) << 32) | RandU32();
}

inline double RandDouble() {
  uint64_t rand_u64 = RandU64();
  return *reinterpret_cast<double*>(&rand_u64);
}

inline uint32_t RandInRange(uint32_t range) {
  return RandU32() % range;
}

inline bool RandEvent(uint32_t frequency) {
  return RandInRange(frequency) == 0;
}

inline size_t RandElementCount() {
  return RandU32() % 10;
}

}  // namespace ipc_fuzzer

#endif  // TOOLS_IPC_FUZZER_MUTATE_RAND_UTIL_H_

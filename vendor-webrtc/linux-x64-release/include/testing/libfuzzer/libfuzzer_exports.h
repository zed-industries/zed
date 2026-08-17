// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TESTING_LIBFUZZER_LIBFUZZER_EXPORTS_H_
#define TESTING_LIBFUZZER_LIBFUZZER_EXPORTS_H_

#include "build/build_config.h"

// On macOS, the linker may strip symbols for functions that are not reachable
// by the program entrypoint. Several libFuzzer functions are resolved via
// dlsym at runtime and therefore may be dead-stripped as a result. Including
// this header in the fuzzer's implementation file will ensure that all the
// symbols are kept and exported.

#if BUILDFLAG(IS_MAC)
#define EXPORT_FUZZER_FUNCTION \
  __attribute__((used)) __attribute__((visibility("default")))
#else
#define EXPORT_FUZZER_FUNCTION
#endif

extern "C" {

EXPORT_FUZZER_FUNCTION int LLVMFuzzerInitialize(int* argc, char*** argv);
EXPORT_FUZZER_FUNCTION int LLVMFuzzerTestOneInput(const uint8_t* data,
                                                  size_t size);
EXPORT_FUZZER_FUNCTION size_t LLVMFuzzerCustomMutator(uint8_t* data,
                                                      size_t size,
                                                      size_t max_size,
                                                      unsigned int seed);
EXPORT_FUZZER_FUNCTION size_t LLVMFuzzerCustomCrossOver(const uint8_t* data1,
                                                        size_t size1,
                                                        const uint8_t* data2,
                                                        size_t size2,
                                                        uint8_t* out,
                                                        size_t max_out_size,
                                                        unsigned int seed);
EXPORT_FUZZER_FUNCTION size_t LLVMFuzzerMutate(uint8_t* data,
                                               size_t size,
                                               size_t max_size);

}  // extern "C"

#undef EXPORT_FUZZER_FUNCTION

#endif  // TESTING_LIBFUZZER_LIBFUZZER_EXPORTS_H_

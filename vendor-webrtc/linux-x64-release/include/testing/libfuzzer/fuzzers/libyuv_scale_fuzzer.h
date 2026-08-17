// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TESTING_LIBFUZZER_FUZZERS_LIBYUV_SCALE_FUZZER_H_
#define TESTING_LIBFUZZER_FUZZERS_LIBYUV_SCALE_FUZZER_H_

#include <string>

void Scale(bool is420,
           int src_width,
           int src_height,
           int dst_width,
           int dst_height,
           int filter_num,
           std::string seed_str);
#endif  // TESTING_LIBFUZZER_FUZZERS_LIBYUV_SCALE_FUZZER_H_

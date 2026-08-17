/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef LOGGING_RTC_EVENT_LOG_ENCODER_DELTA_ENCODING_H_
#define LOGGING_RTC_EVENT_LOG_ENCODER_DELTA_ENCODING_H_

#include <stddef.h>
#include <stdint.h>

#include <optional>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"

namespace webrtc {

// Encode `values` as a sequence of deltas following on `base` and return it.
// If all of the values were equal to the base, an empty string will be
// returned; this is a valid encoding of that edge case.
// `base` is not guaranteed to be written into `output`, and must therefore
// be provided separately to the decoder.
// This function never fails.
// TODO(eladalon): Split into optional and non-optional variants (efficiency).
std::string EncodeDeltas(std::optional<uint64_t> base,
                         const std::vector<std::optional<uint64_t>>& values);

// EncodeDeltas() and DecodeDeltas() are inverse operations;
// invoking DecodeDeltas() over the output of EncodeDeltas(), will return
// the input originally given to EncodeDeltas().
// `num_of_deltas` must be greater than zero. If input is not a valid encoding
// of `num_of_deltas` elements based on `base`, the function returns an empty
// vector, which signals an error.
// TODO(eladalon): Split into optional and non-optional variants (efficiency).
std::vector<std::optional<uint64_t>> DecodeDeltas(absl::string_view input,
                                                  std::optional<uint64_t> base,
                                                  size_t num_of_deltas);

}  // namespace webrtc

#endif  // LOGGING_RTC_EVENT_LOG_ENCODER_DELTA_ENCODING_H_

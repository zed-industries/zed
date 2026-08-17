/*
 *  Copyright (c) 2025 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_BASE64_H_
#define RTC_BASE_BASE64_H_

#include <cstdint>
#include <optional>
#include <string>

#include "absl/strings/escaping.h"
#include "absl/strings/string_view.h"
#include "api/array_view.h"

namespace webrtc {

inline std::string Base64Encode(ArrayView<const uint8_t> data) {
  return absl::Base64Escape(absl::string_view(
      reinterpret_cast<const char*>(data.data()), data.size()));
}

inline std::string Base64Encode(absl::string_view data) {
  return absl::Base64Escape(data);
}

enum class Base64DecodeOptions {
  kStrict,
  // Matches https://infra.spec.whatwg.org/#forgiving-base64-decode.
  kForgiving,
};

// Returns the decoded data if successful, or std::nullopt if the decoding
// failed.
std::optional<std::string> Base64Decode(
    absl::string_view data,
    Base64DecodeOptions options = Base64DecodeOptions::kStrict);

}  // namespace webrtc

#endif  // RTC_BASE_BASE64_H_

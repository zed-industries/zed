/*
 *  Copyright (c) 2023 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef LOGGING_RTC_EVENT_LOG_ENCODER_OPTIONAL_BLOB_ENCODING_H_
#define LOGGING_RTC_EVENT_LOG_ENCODER_OPTIONAL_BLOB_ENCODING_H_

#include <stddef.h>

#include <optional>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"

namespace webrtc {

// Encode a sequence of optional strings, whose length is not known to be
// discernable from the blob itself (i.e. without being transmitted OOB),
// in a way that would allow us to separate them again on the decoding side.
// EncodeOptionalBlobs() may not fail but may return an empty string
std::string EncodeOptionalBlobs(
    const std::vector<std::optional<std::string>>& blobs);

// Calling DecodeOptionalBlobs() on an empty string, or with `num_of_blobs` set
// to 0, is an error. DecodeOptionalBlobs() returns an empty vector if it fails,
// which can happen if `encoded_blobs` is corrupted.
std::vector<std::optional<std::string>> DecodeOptionalBlobs(
    absl::string_view encoded_blobs,
    size_t num_of_blobs);

}  // namespace webrtc

#endif  // LOGGING_RTC_EVENT_LOG_ENCODER_OPTIONAL_BLOB_ENCODING_H_

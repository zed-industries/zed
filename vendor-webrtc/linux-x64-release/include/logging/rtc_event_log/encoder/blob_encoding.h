/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef LOGGING_RTC_EVENT_LOG_ENCODER_BLOB_ENCODING_H_
#define LOGGING_RTC_EVENT_LOG_ENCODER_BLOB_ENCODING_H_

#include <stddef.h>

#include <string>
#include <vector>

#include "absl/strings/string_view.h"

namespace webrtc {

// Encode/decode a sequence of strings, whose length is not known to be
// discernable from the blob itself (i.e. without being transmitted OOB),
// in a way that would allow us to separate them again on the decoding side.
// The number of blobs is assumed to be transmitted OOB. For example, if
// multiple sequences of different blobs are sent, but all sequences contain
// the same number of blobs, it is beneficial to not encode the number of blobs.
//
// EncodeBlobs() must be given a non-empty vector. The blobs themselves may
// be equal to "", though.
// EncodeBlobs() may not fail.
// EncodeBlobs() never returns the empty string.
//
// Calling DecodeBlobs() on an empty string, or with `num_of_blobs` set to 0,
// is an error.
// DecodeBlobs() returns an empty vector if it fails, e.g. due to a mismatch
// between `num_of_blobs` and `encoded_blobs`, which can happen if
// `encoded_blobs` is corrupted.
// When successful, DecodeBlobs() returns a vector of string_view objects,
// which refer to the original input (`encoded_blobs`), and therefore may
// not outlive it.
//
// Note that the returned std::string might have been reserved for significantly
// more memory than it ends up using. If the caller to EncodeBlobs() intends
// to store the result long-term, they should consider shrink_to_fit()-ing it.
std::string EncodeBlobs(const std::vector<std::string>& blobs);
std::vector<absl::string_view> DecodeBlobs(absl::string_view encoded_blobs,
                                           size_t num_of_blobs);

}  // namespace webrtc

#endif  // LOGGING_RTC_EVENT_LOG_ENCODER_BLOB_ENCODING_H_

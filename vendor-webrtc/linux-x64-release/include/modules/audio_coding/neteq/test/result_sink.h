/*
 *  Copyright (c) 2011 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_NETEQ_TEST_RESULT_SINK_H_
#define MODULES_AUDIO_CODING_NETEQ_TEST_RESULT_SINK_H_

#include <cstdio>
#include <memory>

#include "absl/strings/string_view.h"
#include "api/neteq/neteq.h"
#include "rtc_base/message_digest.h"

namespace webrtc {

class ResultSink {
 public:
  explicit ResultSink(absl::string_view output_file);
  ~ResultSink();

  template <typename T>
  void AddResult(const T* test_results, size_t length);

  void AddResult(const NetEqNetworkStatistics& stats);

  void VerifyChecksum(absl::string_view ref_check_sum);

 private:
  FILE* output_fp_;
  std::unique_ptr<MessageDigest> digest_;
};

template <typename T>
void ResultSink::AddResult(const T* test_results, size_t length) {
  if (output_fp_) {
    ASSERT_EQ(length, fwrite(test_results, sizeof(T), length, output_fp_));
  }
  digest_->Update(test_results, sizeof(T) * length);
}

}  // namespace webrtc
#endif  // MODULES_AUDIO_CODING_NETEQ_TEST_RESULT_SINK_H_

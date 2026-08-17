/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CODING_CHAIN_DIFF_CALCULATOR_H_
#define MODULES_VIDEO_CODING_CHAIN_DIFF_CALCULATOR_H_

#include <stdint.h>

#include <optional>
#include <vector>

#include "absl/container/inlined_vector.h"

namespace webrtc {

// This class is thread compatible.
class ChainDiffCalculator {
 public:
  ChainDiffCalculator() = default;
  ChainDiffCalculator(const ChainDiffCalculator&) = default;
  ChainDiffCalculator& operator=(const ChainDiffCalculator&) = default;

  // Restarts chains, i.e. for position where chains[i] == true next chain_diff
  // will be 0. Saves chains.size() as number of chains in the stream.
  void Reset(const std::vector<bool>& chains);

  // Returns chain diffs based on flags if frame is part of the chain.
  absl::InlinedVector<int, 4> From(int64_t frame_id,
                                   const std::vector<bool>& chains);

 private:
  absl::InlinedVector<int, 4> ChainDiffs(int64_t frame_id) const;

  absl::InlinedVector<std::optional<int64_t>, 4> last_frame_in_chain_;
};

}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_CHAIN_DIFF_CALCULATOR_H_

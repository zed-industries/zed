/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_RTP_RTCP_SOURCE_ACTIVE_DECODE_TARGETS_HELPER_H_
#define MODULES_RTP_RTCP_SOURCE_ACTIVE_DECODE_TARGETS_HELPER_H_

#include <stdint.h>

#include <bitset>
#include <optional>

#include "api/array_view.h"

namespace webrtc {

// Helper class that decides when active_decode_target_bitmask should be written
// into the dependency descriptor rtp header extension.
// See: https://aomediacodec.github.io/av1-rtp-spec/#a44-switching
// This class is thread-compatible
class ActiveDecodeTargetsHelper {
 public:
  ActiveDecodeTargetsHelper() = default;
  ActiveDecodeTargetsHelper(const ActiveDecodeTargetsHelper&) = delete;
  ActiveDecodeTargetsHelper& operator=(const ActiveDecodeTargetsHelper&) =
      delete;
  ~ActiveDecodeTargetsHelper() = default;

  // Decides if active decode target bitmask should be attached to the frame
  // that is about to be sent.
  void OnFrame(ArrayView<const int> decode_target_protected_by_chain,
               std::bitset<32> active_decode_targets,
               bool is_keyframe,
               int64_t frame_id,
               ArrayView<const int> chain_diffs);

  // Returns active decode target to attach to the dependency descriptor.
  std::optional<uint32_t> ActiveDecodeTargetsBitmask() const {
    if (unsent_on_chain_.none())
      return std::nullopt;
    return last_active_decode_targets_.to_ulong();
  }

  std::bitset<32> ActiveChainsBitmask() const { return last_active_chains_; }

 private:
  // `unsent_on_chain_[i]` indicates last active decode
  // target bitmask wasn't attached to a packet on the chain with id `i`.
  std::bitset<32> unsent_on_chain_ = 0;
  std::bitset<32> last_active_decode_targets_ = 0;
  std::bitset<32> last_active_chains_ = 0;
  int64_t last_frame_id_ = 0;
};

}  // namespace webrtc

#endif  // MODULES_RTP_RTCP_SOURCE_ACTIVE_DECODE_TARGETS_HELPER_H_

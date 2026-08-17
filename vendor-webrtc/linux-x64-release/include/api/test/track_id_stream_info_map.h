/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_TEST_TRACK_ID_STREAM_INFO_MAP_H_
#define API_TEST_TRACK_ID_STREAM_INFO_MAP_H_

#include <string>

#include "absl/strings/string_view.h"

namespace webrtc {
namespace webrtc_pc_e2e {

// Instances of `TrackIdStreamInfoMap` provide bookkeeping capabilities that
// are useful to associate stats reports track_ids to the remote stream info.
class TrackIdStreamInfoMap {
 public:
  struct StreamInfo {
    std::string receiver_peer;
    std::string stream_label;
    std::string sync_group;
  };

  virtual ~TrackIdStreamInfoMap() = default;

  // These methods must be called on the same thread where
  // StatsObserverInterface::OnStatsReports is invoked.

  // Precondition: `track_id` must be already mapped to stream info.
  virtual StreamInfo GetStreamInfoFromTrackId(
      absl::string_view track_id) const = 0;
};

}  // namespace webrtc_pc_e2e
}  // namespace webrtc

#endif  // API_TEST_TRACK_ID_STREAM_INFO_MAP_H_

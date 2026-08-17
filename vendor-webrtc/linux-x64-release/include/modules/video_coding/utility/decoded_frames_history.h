/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CODING_UTILITY_DECODED_FRAMES_HISTORY_H_
#define MODULES_VIDEO_CODING_UTILITY_DECODED_FRAMES_HISTORY_H_

#include <stdint.h>

#include <cstddef>
#include <optional>
#include <vector>


namespace webrtc {
namespace video_coding {

class DecodedFramesHistory {
 public:
  // window_size - how much frames back to the past are actually remembered.
  explicit DecodedFramesHistory(size_t window_size);
  ~DecodedFramesHistory();
  // Called for each decoded frame. Assumes frame id's are non-decreasing.
  void InsertDecoded(int64_t frame_id, uint32_t timestamp);
  // Query if the following (frame_id, spatial_id) pair was inserted before.
  // Should be at most less by window_size-1 than the last inserted frame id.
  bool WasDecoded(int64_t frame_id) const;

  void Clear();

  std::optional<int64_t> GetLastDecodedFrameId() const;
  std::optional<uint32_t> GetLastDecodedFrameTimestamp() const;

 private:
  int FrameIdToIndex(int64_t frame_id) const;

  std::vector<bool> buffer_;
  std::optional<int64_t> last_frame_id_;
  std::optional<int64_t> last_decoded_frame_;
  std::optional<uint32_t> last_decoded_frame_timestamp_;
};

}  // namespace video_coding
}  // namespace webrtc
#endif  // MODULES_VIDEO_CODING_UTILITY_DECODED_FRAMES_HISTORY_H_

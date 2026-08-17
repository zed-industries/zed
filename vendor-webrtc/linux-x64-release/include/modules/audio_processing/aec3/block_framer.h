/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AEC3_BLOCK_FRAMER_H_
#define MODULES_AUDIO_PROCESSING_AEC3_BLOCK_FRAMER_H_

#include <vector>

#include "api/array_view.h"
#include "modules/audio_processing/aec3/aec3_common.h"
#include "modules/audio_processing/aec3/block.h"

namespace webrtc {

// Class for producing frames consisting of 2 subframes of 80 samples each
// from 64 sample blocks. The class is designed to work together with the
// FrameBlocker class which performs the reverse conversion. Used together with
// that, this class produces output frames are the same rate as frames are
// received by the FrameBlocker class. Note that the internal buffers will
// overrun if any other rate of packets insertion is used.
class BlockFramer {
 public:
  BlockFramer(size_t num_bands, size_t num_channels);
  ~BlockFramer();
  BlockFramer(const BlockFramer&) = delete;
  BlockFramer& operator=(const BlockFramer&) = delete;

  // Adds a 64 sample block into the data that will form the next output frame.
  void InsertBlock(const Block& block);
  // Adds a 64 sample block and extracts an 80 sample subframe.
  void InsertBlockAndExtractSubFrame(
      const Block& block,
      std::vector<std::vector<ArrayView<float>>>* sub_frame);

 private:
  const size_t num_bands_;
  const size_t num_channels_;
  std::vector<std::vector<std::vector<float>>> buffer_;
};
}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AEC3_BLOCK_FRAMER_H_

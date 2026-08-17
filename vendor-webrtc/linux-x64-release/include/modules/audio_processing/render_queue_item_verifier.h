/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_RENDER_QUEUE_ITEM_VERIFIER_H_
#define MODULES_AUDIO_PROCESSING_RENDER_QUEUE_ITEM_VERIFIER_H_

#include <vector>

namespace webrtc {

// Functor to use when supplying a verifier function for the queue item
// verifcation.
template <typename T>
class RenderQueueItemVerifier {
 public:
  explicit RenderQueueItemVerifier(size_t minimum_capacity)
      : minimum_capacity_(minimum_capacity) {}

  bool operator()(const std::vector<T>& v) const {
    return v.capacity() >= minimum_capacity_;
  }

 private:
  size_t minimum_capacity_;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_RENDER_QUEUE_ITEM_VERIFIER_H__

/*
 *  Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_DESKTOP_CAPTURE_SCREEN_CAPTURE_FRAME_QUEUE_H_
#define MODULES_DESKTOP_CAPTURE_SCREEN_CAPTURE_FRAME_QUEUE_H_

#include <memory>

namespace webrtc {

// Represents a queue of reusable video frames. Provides access to the 'current'
// frame - the frame that the caller is working with at the moment, and to the
// 'previous' frame - the predecessor of the current frame swapped by
// MoveToNextFrame() call, if any.
//
// The caller is expected to (re)allocate frames if current_frame() returns
// NULL. The caller can mark all frames in the queue for reallocation (when,
// say, frame dimensions change). The queue records which frames need updating
// which the caller can query.
//
// Frame consumer is expected to never hold more than kQueueLength frames
// created by this function and it should release the earliest one before trying
// to capture a new frame (i.e. before MoveToNextFrame() is called).
template <typename FrameType>
class ScreenCaptureFrameQueue {
 public:
  ScreenCaptureFrameQueue() = default;
  ~ScreenCaptureFrameQueue() = default;

  ScreenCaptureFrameQueue(const ScreenCaptureFrameQueue&) = delete;
  ScreenCaptureFrameQueue& operator=(const ScreenCaptureFrameQueue&) = delete;

  // Moves to the next frame in the queue, moving the 'current' frame to become
  // the 'previous' one.
  void MoveToNextFrame() { current_ = (current_ + 1) % kQueueLength; }

  // Replaces the current frame with a new one allocated by the caller. The
  // existing frame (if any) is destroyed. Takes ownership of `frame`.
  void ReplaceCurrentFrame(std::unique_ptr<FrameType> frame) {
    frames_[current_] = std::move(frame);
  }

  // Marks all frames obsolete and resets the previous frame pointer. No
  // frames are freed though as the caller can still access them.
  void Reset() {
    for (int i = 0; i < kQueueLength; i++) {
      frames_[i].reset();
    }
    current_ = 0;
  }

  FrameType* current_frame() const { return frames_[current_].get(); }

  FrameType* previous_frame() const {
    return frames_[(current_ + kQueueLength - 1) % kQueueLength].get();
  }

 private:
  // Index of the current frame.
  int current_ = 0;

  static const int kQueueLength = 2;
  std::unique_ptr<FrameType> frames_[kQueueLength];
};

}  // namespace webrtc

#endif  // MODULES_DESKTOP_CAPTURE_SCREEN_CAPTURE_FRAME_QUEUE_H_

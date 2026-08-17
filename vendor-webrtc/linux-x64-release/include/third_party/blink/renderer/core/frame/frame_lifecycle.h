// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_FRAME_LIFECYCLE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_FRAME_LIFECYCLE_H_

namespace blink {

class FrameLifecycle {
 public:
  enum State {
    kAttached,
    kDetaching,
    kDetached,
  };

  FrameLifecycle();
  FrameLifecycle(const FrameLifecycle&) = delete;
  FrameLifecycle& operator=(const FrameLifecycle&) = delete;

  State GetState() const { return state_; }
  void AdvanceTo(State);

 private:
  State state_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_FRAME_LIFECYCLE_H_

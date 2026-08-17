// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_LAYOUT_OBJECT_COUNTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_LAYOUT_OBJECT_COUNTER_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// Used by LocalFrameView and FirstMeaningfulPaintDetector to keep track of
// the number of layout objects created in the frame.
class LayoutObjectCounter {
  DISALLOW_NEW();

 public:
  void Reset() { count_ = 0; }
  void Increment() { count_++; }
  unsigned Count() const { return count_; }

 private:
  unsigned count_ = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_LAYOUT_OBJECT_COUNTER_H_

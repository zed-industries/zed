// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_OPENED_FRAME_TRACKER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_OPENED_FRAME_TRACKER_H_

#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class Frame;

// Small helper class to track the set of frames that a Frame has opened.
// Due to layering restrictions, we need to hide the implementation, since
// public/web/ cannot depend on wtf/.
class OpenedFrameTracker {
  DISALLOW_NEW();

 public:
  OpenedFrameTracker();
  OpenedFrameTracker(const OpenedFrameTracker&) = delete;
  OpenedFrameTracker& operator=(const OpenedFrameTracker&) = delete;
  ~OpenedFrameTracker();
  void Trace(Visitor*) const;

  bool IsEmpty() const;
  void Add(Frame*);
  void Remove(Frame*);

  // Helper used when swapping a frame into the frame tree: this updates the
  // opener for opened frames to point to the new frame being swapped in.
  void TransferTo(Frame*) const;

  // Explicitly break opener references from opened frames when removing
  // a frame from the DOM, rather than relying on weak fields + GC to
  // non-deterministically clear them later.
  void Dispose();

 private:
  HeapHashSet<Member<Frame>> opened_frames_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_OPENED_FRAME_TRACKER_H_

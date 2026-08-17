// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_FRAME_VIEW_AUTO_SIZE_INFO_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_FRAME_VIEW_AUTO_SIZE_INFO_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "ui/gfx/geometry/size.h"

namespace blink {

class LocalFrameView;

class FrameViewAutoSizeInfo final
    : public GarbageCollected<FrameViewAutoSizeInfo> {
 public:
  explicit FrameViewAutoSizeInfo(LocalFrameView*);
  FrameViewAutoSizeInfo(const FrameViewAutoSizeInfo&) = delete;
  FrameViewAutoSizeInfo& operator=(const FrameViewAutoSizeInfo&) = delete;

  void ConfigureAutoSizeMode(const gfx::Size& min_size,
                             const gfx::Size& max_size);
  // Returns true if the LocalFrameView was resized.
  bool AutoSizeIfNeeded();
  void Clear();

  void Trace(Visitor*) const;

 private:
  Member<LocalFrameView> frame_view_;

  // The lower bound on the size when autosizing.
  gfx::Size min_auto_size_;
  // The upper bound on the size when autosizing.
  gfx::Size max_auto_size_;

  bool in_auto_size_;
  // True if autosize has been run since m_shouldAutoSize was set.
  bool did_run_autosize_;
  // The number of autosize passes that have been made since the last call to
  // Clear();
  bool running_first_autosize_ = false;
  uint32_t num_passes_ = 0u;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_FRAME_VIEW_AUTO_SIZE_INFO_H_

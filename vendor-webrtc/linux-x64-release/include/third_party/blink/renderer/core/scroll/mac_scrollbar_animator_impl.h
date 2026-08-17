// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SCROLL_MAC_SCROLLBAR_ANIMATOR_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SCROLL_MAC_SCROLLBAR_ANIMATOR_IMPL_H_

#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/renderer/core/scroll/mac_scrollbar_animator.h"
#include "third_party/blink/renderer/core/scroll/scroll_animator_base.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/scheduler/public/post_cancellable_task.h"
#include "third_party/blink/renderer/platform/timer.h"
#include "ui/gfx/geometry/point_f.h"
#include "ui/gfx/geometry/rect.h"
#include "ui/gfx/geometry/size_f.h"
#include "ui/native_theme/scrollbar_animator_mac.h"

namespace blink {

// Implementation of the ui::OverlayScrollbarAnimatorMac::Client interface to
// talk to a Scrollbar instance.
class CORE_EXPORT MacScrollbarImplV2
    : public ui::OverlayScrollbarAnimatorMac::Client,
      public MacScrollbar {
 public:
  MacScrollbarImplV2(Scrollbar& scrollbar,
                     scoped_refptr<base::SingleThreadTaskRunner> task_runner);
  ~MacScrollbarImplV2() override;

  // Return true if `this` is the animator for `scrollbar`.
  bool IsAnimatorFor(Scrollbar& scrollbar) const;

  // Function to call upon interaction with this scrollbar.
  void MouseDidEnter();
  void MouseDidExit();
  void DidScroll();

  // MacScrollbar:
  void SetEnabled(bool) final {}
  float GetKnobAlpha() final;
  float GetTrackAlpha() final;
  int GetTrackBoxWidth() final;

  // ui::OverlayScrollbarAnimatorMac::Client:
  bool IsMouseInScrollbarFrameRect() const override;
  void SetHidden(bool hidden) override;
  void SetThumbNeedsDisplay() override;
  void SetTrackNeedsDisplay() override;

 private:
  std::unique_ptr<ui::OverlayScrollbarAnimatorMac> overlay_animator_;
  Persistent<Scrollbar> scrollbar_;
};

// A non-Cocoa-based implementation of the MacScrollbarAnimator interface.
class CORE_EXPORT MacScrollbarAnimatorV2 : public MacScrollbarAnimator {
 public:
  MacScrollbarAnimatorV2(ScrollableArea*);
  virtual ~MacScrollbarAnimatorV2();

  // MacScrollbarAnimator:
  void Trace(Visitor* visitor) const final {
    MacScrollbarAnimator::Trace(visitor);
  }
  void MouseEnteredScrollbar(Scrollbar&) const final;
  void MouseExitedScrollbar(Scrollbar&) const final;
  void DidAddVerticalScrollbar(Scrollbar&) final;
  void WillRemoveVerticalScrollbar(Scrollbar&) final;
  void DidAddHorizontalScrollbar(Scrollbar&) final;
  void WillRemoveHorizontalScrollbar(Scrollbar&) final;
  void DidChangeUserVisibleScrollOffset(const ScrollOffset&) final;
  void Dispose() final;

 private:
  std::unique_ptr<MacScrollbarImplV2> horizontal_scrollbar_;
  std::unique_ptr<MacScrollbarImplV2> vertical_scrollbar_;
  scoped_refptr<base::SingleThreadTaskRunner> task_runner_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SCROLL_MAC_SCROLLBAR_ANIMATOR_IMPL_H_

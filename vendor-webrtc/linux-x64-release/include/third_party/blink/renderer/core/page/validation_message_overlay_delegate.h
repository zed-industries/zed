// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_VALIDATION_MESSAGE_OVERLAY_DELEGATE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_VALIDATION_MESSAGE_OVERLAY_DELEGATE_H_

#include "base/time/time.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/frame/frame_overlay.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/text/text_direction.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class ChromeClient;
class Element;
class LocalFrameView;
class Page;

// A ValidationMessageOverlayDelegate is responsible for rendering a form
// validation message bubble.
//
// Lifetime: An instance is created by a ValidationMessageClientImpl when a
// bubble is shown, and deleted when the bubble is closed.
//
// Ownership: A FrameOverlay instance owns a ValidationMessageOverlayDelegate.
class CORE_EXPORT ValidationMessageOverlayDelegate
    : public FrameOverlay::Delegate {
 public:
  ValidationMessageOverlayDelegate(Page& main_page,
                                   const Element& anchor,
                                   const String& message,
                                   TextDirection message_dir,
                                   const String& sub_message,
                                   TextDirection sub_message_dir);
  ~ValidationMessageOverlayDelegate() override;

  void CreatePage(const FrameOverlay&);

  // FrameOverlay::Delegate implementation.
  void PaintFrameOverlay(const FrameOverlay&,
                         GraphicsContext&,
                         const gfx::Size& view_size) const override;
  void ServiceScriptedAnimations(base::TimeTicks) override;

  void StartToHide();
  bool IsHiding() const;

  void UpdateFrameViewState(const FrameOverlay&);

  Page* GetPageForTesting() const { return page_; }

 private:
  LocalFrameView& FrameView() const;
  void WriteDocument(SegmentedBuffer&);
  Element& GetElementById(const AtomicString&) const;
  void AdjustBubblePosition(const gfx::Rect& view_rect);

  // An internal Page and a ChromeClient for it.
  Persistent<Page> page_;
  Persistent<ChromeClient> chrome_client_;

  // TODO(crbug.com/334963179): Remove bubble_size_ when the
  // ValidationBubbleNoForcedLayout flag is removed.
  gfx::Size bubble_size_;

  // A page which triggered this validation message.
  Persistent<Page> main_page_;

  Persistent<const Element> anchor_;
  String message_;
  String sub_message_;
  TextDirection message_dir_;
  TextDirection sub_message_dir_;

  // Used by CreatePage() to determine if this has been deleted in the middle of
  // the function.
  bool* destroyed_ptr_ = nullptr;
};

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_VALIDATION_MESSAGE_OVERLAY_DELEGATE_H_

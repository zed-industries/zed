// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_ELEMENT_BASE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_ELEMENT_BASE_H_

#include "third_party/blink/renderer/core/dom/element.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"

namespace gfx {
class Size;
}  // namespace gfx

namespace blink {

class Element;
class HTMLElement;
class HTMLMediaElement;
class MediaControlsImpl;

// MediaControlElementBase is the base class for all the media control elements.
// It is sub-classed by MediaControlInputElement and MediaControlDivElement
// which are then used by the final implementations.
class MODULES_EXPORT MediaControlElementBase : public GarbageCollectedMixin {
 public:
  // These hold the state about whether this control should be shown if
  // space permits.  These will also show / hide as needed.
  virtual void SetIsWanted(bool);
  bool IsWanted() const;

  // Tell us whether we fit or not.  This will hide / show the control as
  // needed, also.
  void SetDoesFit(bool);
  bool DoesFit() const;

  // Similar to SetIsWanted() for the overflow element associated to the current
  // element. Will be a no-op if the element does not have an associated
  // overflow element.
  virtual void SetOverflowElementIsWanted(bool) = 0;

  // Called when recording the display state of the media control element should
  // happen. It will be a no-op if the element isn't displayed in the controls.
  virtual void MaybeRecordDisplayed() = 0;

  // By default, media controls elements are not added to the overflow menu.
  // Controls that can be added to the overflow menu should override this
  // function and return true.
  virtual bool HasOverflowButton() const;

  // Get the size of the element in pixels or the default if we cannot get the
  // size because the element has not been laid out yet.
  virtual gfx::Size GetSizeOrDefault() const = 0;

  // Whether the element has been disabled via the HTML disabled attribute.
  virtual bool IsDisabled() const = 0;

  void Trace(Visitor*) const override;

 protected:
  MediaControlElementBase(MediaControlsImpl&,
                          HTMLElement*);

  // Hide or show based on our fits / wanted state.  We want to show
  // if and only if we're wanted and we fit.
  virtual void UpdateShownState();

  MediaControlsImpl& GetMediaControls() const;

  HTMLMediaElement& MediaElement() const;

 private:
  Member<MediaControlsImpl> media_controls_;
  Member<HTMLElement> element_;
  bool is_wanted_ : 1;
  bool does_fit_ : 1;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_ELEMENT_BASE_H_

// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_MEDIA_MEDIA_CONTROLS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_MEDIA_MEDIA_CONTROLS_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class HTMLDivElement;
class HTMLMediaElement;
class LayoutObject;

// The CSS class names for the different sizing thresholds.
const char kMediaControlsSizingSmallCSSClass[] = "sizing-small";
const char kMediaControlsSizingMediumCSSClass[] = "sizing-medium";
const char kMediaControlsSizingLargeCSSClass[] = "sizing-large";

// The different media controls sizing classes.
enum class MediaControlsSizingClass { kSmall, kMedium, kLarge };

// MediaControls is an interface to abstract the HTMLMediaElement controls. The
// implementation will be used using a Factory (see below).
class CORE_EXPORT MediaControls : public GarbageCollectedMixin {
 public:
  // Get the |MediaControlsSizingClass| based on the width of the element in
  // pixels.
  static MediaControlsSizingClass GetSizingClass(int width);

  // Get the CSS class name from the |MediaControlsSizingClass|.
  static AtomicString GetSizingCSSClass(MediaControlsSizingClass sizing_class);

  MediaControls(HTMLMediaElement&);
  virtual ~MediaControls() = default;

  HTMLMediaElement& MediaElement() const;

  // Enables showing of the controls - only shows if appropriate.
  virtual void MaybeShow() = 0;
  // Disables showing of the controls - immediately hides.
  virtual void Hide() = 0;

  virtual void Reset() = 0;

  // Notify the media controls that the controlsList attribute has changed.
  virtual void OnControlsListUpdated() = 0;

  // TODO(mlamouri): this is temporary to notify the controls that an
  // HTMLTrackElement failed to load because there is no web exposed way to
  // be notified on the TextTrack object. See https://crbug.com/669977
  virtual void OnTrackElementFailedToLoad() = 0;

  // TODO(mlamouri): this method should be moved away from the interface to
  // become an implementation detail.
  virtual void NetworkStateChanged() = 0;

  // Returns the layout object for the part of the controls that should be
  // used for overlap checking during text track layout. May be null.
  // TODO(mlamouri): required by VttCueLayoutAlgorithm.
  virtual LayoutObject* PanelLayoutObject() = 0;
  virtual LayoutObject* TimelineLayoutObject() = 0;
  virtual LayoutObject* ButtonPanelLayoutObject() = 0;
  // Returns the layout object of the media controls container. Maybe null.
  // TODO(mlamouri): required by VttCueLayoutAlgorithm.
  virtual LayoutObject* ContainerLayoutObject() = 0;

  // Used for web tests to disable some animations.
  virtual void SetTestMode(bool) = 0;

  // TODO: the following are required by other parts of the media controls
  // implementation and could be removed when the full implementation has moved
  // to modules.
  virtual HTMLDivElement* PanelElement() = 0;
  virtual void OnMediaControlsEnabledChange() = 0;

  void Trace(Visitor*) const override;

 private:
  Member<HTMLMediaElement> media_element_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_MEDIA_MEDIA_CONTROLS_H_

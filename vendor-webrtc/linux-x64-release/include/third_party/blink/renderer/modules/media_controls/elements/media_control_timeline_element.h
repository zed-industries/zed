// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_TIMELINE_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_TIMELINE_ELEMENT_H_

#include <optional>

#include "base/time/time.h"
#include "third_party/blink/renderer/modules/media_controls/elements/media_control_slider_element.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/timer.h"

namespace blink {

class Event;
class HTMLDivElement;
class MediaControlsImpl;

class MediaControlTimelineElement : public MediaControlSliderElement {
 public:
  MODULES_EXPORT explicit MediaControlTimelineElement(MediaControlsImpl&);

  // MediaControlInputElement overrides.
  bool WillRespondToMouseClickEvents() override;

  // FIXME: An "earliest possible position" will be needed once that concept
  // is supported by HTMLMediaElement, see https://crbug.com/137275
  void SetPosition(double, bool suppress_aria = false);
  void SetDuration(double);

  void OnMediaKeyboardEvent(Event* event) { DefaultEventHandler(*event); }

  void OnMediaPlaying();
  void OnMediaStoppedPlaying();
  void OnProgress();

  void RenderBarSegments();

  // Inform the timeline that the Media Controls have been shown or hidden.
  void OnControlsShown();
  void OnControlsHidden();

  void Trace(Visitor*) const override;

 protected:
  const char* GetNameForHistograms() const override;

 private:
  // Struct used to track the current live time.
  struct LiveAnchorTime {
    base::TimeTicks clock_time_;
    double media_time_ = 0;
  };

  void DefaultEventHandler(Event&) override;
  bool KeepEventInNode(const Event&) const override;

  void RenderTimelineTimerFired(TimerBase*);
  void MaybeUpdateTimelineInterval();

  // Checks if we can begin or end a scrubbing event. If the event is a pointer
  // event then it needs to start and end with valid pointer events. If the
  // event is a pointer event followed by a touch event then it can only be
  // ended when the touch has ended.
  bool BeginScrubbingEvent(Event&);
  bool EndScrubbingEvent(Event&);

  void UpdateAria();

  bool is_touching_ = false;

  bool controls_hidden_ = false;

  bool is_scrubbing_ = false;

  bool is_live_ = false;

  std::optional<LiveAnchorTime> live_anchor_time_;

  HeapTaskRunnerTimer<MediaControlTimelineElement> render_timeline_timer_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_TIMELINE_ELEMENT_H_

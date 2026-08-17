/*
 * Copyright (C) 2011 Google Inc.  All rights reserved.
 * Copyright (C) 2012, 2013 Apple Inc.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_TEXT_TRACK_CUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_TEXT_TRACK_CUE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/html/html_div_element.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class HTMLMediaElement;
class TextTrack;

class CORE_EXPORT TextTrackCue : public EventTarget {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static const AtomicString& CueShadowPseudoId() {
    DEFINE_STATIC_LOCAL(const AtomicString, cue, ("cue"));
    return cue;
  }

  ~TextTrackCue() override = default;

  TextTrack* track() const;
  void SetTrack(TextTrack*);

  Node* Owner() const;

  const AtomicString& id() const { return id_; }
  void setId(const AtomicString&);

  double startTime() const { return start_time_; }
  void setStartTime(double);

  double endTime() const { return end_time_; }
  void setEndTime(double);

  bool pauseOnExit() const { return pause_on_exit_; }
  void setPauseOnExit(bool);

  unsigned CueIndex();
  void UpdateCueIndex(unsigned cue_index) { cue_index_ = cue_index; }
  void InvalidateCueIndex();

  bool IsActive() const { return is_active_; }
  void SetIsActive(bool active) { is_active_ = active; }

  // Updates the display tree and appends it to container if it has not
  // already been added.
  virtual void UpdateDisplay(HTMLDivElement& container) = 0;

  // Called when entering or exiting the cue on the timeline in cue_timeline.cc
  // (cf. the 'enter' and 'exit' events). Handles enter and exit event behavior
  // for spoken cues.
  virtual void OnEnter(HTMLMediaElement& video) = 0;
  virtual void OnExit(HTMLMediaElement& video) = 0;

  // Marks the nodes of the display tree as past or future relative to
  // movieTime. If |updateDisplay| has not been called there is no display
  // tree and nothing is done.
  virtual void UpdatePastAndFutureNodes(double movie_time) = 0;

  // Returns the first timestamp value greater than the given time at which an
  // inter-cue update occurs, if such a timestamp exists.
  virtual std::optional<double> GetNextIntraCueTime(
      double movie_time) const = 0;

  // FIXME: Refactor to eliminate removeDisplayTree(). https://crbug.com/322434
  enum RemovalNotification { kDontNotifyRegion, kNotifyRegion };
  virtual void RemoveDisplayTree(RemovalNotification = kNotifyRegion) = 0;

  const AtomicString& InterfaceName() const override;

#ifndef NDEBUG
  virtual String ToString() const = 0;
#endif

  DEFINE_ATTRIBUTE_EVENT_LISTENER(enter, kEnter)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(exit, kExit)

  void Trace(Visitor*) const override;

 protected:
  TextTrackCue(double start, double end);

  enum CueMutationAffectsOrder {
    kCueMutationDoesNotAffectOrder,
    kCueMutationAffectsOrder
  };
  void CueWillChange();
  virtual void CueDidChange(
      CueMutationAffectsOrder = kCueMutationDoesNotAffectOrder);
  DispatchEventResult DispatchEventInternal(Event&) override;

 private:
  AtomicString id_;
  double start_time_;
  double end_time_;

  Member<TextTrack> track_;

  unsigned cue_index_;

  bool is_active_ : 1;
  bool pause_on_exit_ : 1;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_TEXT_TRACK_CUE_H_

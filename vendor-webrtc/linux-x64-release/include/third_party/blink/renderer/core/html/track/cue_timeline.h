// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_CUE_TIMELINE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_CUE_TIMELINE_H_

#include <optional>

#include "base/types/pass_key.h"
#include "third_party/blink/renderer/core/html/track/text_track_cue.h"
#include "third_party/blink/renderer/core/html/track/vtt/vtt_cue.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/pod_interval_tree.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class HTMLMediaElement;
class TextTrackCueList;

// TODO(Oilpan): This needs to be PODIntervalTree<double, Member<TextTrackCue>>.
// However, it is not easy to move PODIntervalTree to the heap (for a
// C++-template reason) so we leave it as a raw pointer at the moment. This is
// safe because CueTimeline and TextTrackCue are guaranteed to die at the same
// time when the owner HTMLMediaElement dies. Thus the raw TextTrackCue* cannot
// become stale pointers.
typedef WTF::PODIntervalTree<double, TextTrackCue*> CueIntervalTree;
typedef CueIntervalTree::IntervalType CueInterval;
typedef Vector<CueInterval> CueList;

// This class manages the timeline and rendering updates of cues associated
// with TextTracks. Owned by a HTMLMediaElement.
class CueTimeline final : public GarbageCollected<CueTimeline> {
 public:
  class IgnoreUpdateScope {
    STACK_ALLOCATED();
    friend CueTimeline;

   public:
    ~IgnoreUpdateScope() {
      DCHECK(cue_timeline_);
      cue_timeline_->EndIgnoreUpdateScope(base::PassKey<IgnoreUpdateScope>(),
                                          *this);
    }

   private:
    explicit IgnoreUpdateScope(CueTimeline& cue_timeline)
        : cue_timeline_(&cue_timeline) {}

    CueTimeline* cue_timeline_;
  };

  explicit CueTimeline(HTMLMediaElement&);

  void AddCues(TextTrack*, const TextTrackCueList*);
  void AddCue(TextTrack*, TextTrackCue*);
  void RemoveCues(TextTrack*, const TextTrackCueList*);
  void RemoveCue(TextTrack*, TextTrackCue*);
  void HideCues(TextTrack*, const TextTrackCueList*);

  const CueList& CurrentlyActiveCues() const { return currently_active_cues_; }

  // Methods called by |HTMLMediaElement| which affect cue playback.
  void InvokeTimeMarchesOn();
  void OnPause();
  void OnPlaybackRateUpdated();
  void OnReadyStateReset();

  bool InsideIgnoreUpdateScope() const { return ignore_update_ > 0; }
  IgnoreUpdateScope BeginIgnoreUpdateScope();
  void EndIgnoreUpdateScope(base::PassKey<IgnoreUpdateScope>,
                            IgnoreUpdateScope const& scope);

  void DidMoveToNewDocument(Document& old_document);

  void Trace(Visitor*) const;

 private:
  HTMLMediaElement& MediaElement() const { return *media_element_; }

  void AddCueInternal(TextTrackCue*);
  void RemoveCueInternal(TextTrackCue*);
  void TimeMarchesOn();
  void UpdateActiveCuePastAndFutureNodes();

  void CueEventTimerFired(TimerBase*);
  void SetCueEventTimer();
  void CancelCueEventTimer();

  void CueTimestampEventTimerFired(TimerBase*);
  void SetCueTimestampEventTimer();
  void CancelCueTimestampEventTimer();

  Member<HTMLMediaElement> media_element_;

  CueIntervalTree cue_tree_;

  CueList currently_active_cues_;
  double last_update_time_;

  // Timer data for cue events (start, end)
  std::optional<double> next_cue_event_;
  HeapTaskRunnerTimer<CueTimeline> cue_event_timer_;

  // Timer data for cue timestamps
  // https://w3c.github.io/webvtt/#ref-for-webvtt-timestamp-6
  HeapTaskRunnerTimer<CueTimeline> cue_timestamp_event_timer_;

  int ignore_update_;
  bool update_requested_while_ignoring_;
};

}  // namespace blink

namespace WTF {
#ifndef NDEBUG
// Template specializations required by PodIntervalTree in debug mode.
template <>
struct ValueToString<blink::TextTrackCue*> {
  static String ToString(blink::TextTrackCue* const& cue) {
    return cue->ToString();
  }
};
#endif
}

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_CUE_TIMELINE_H_

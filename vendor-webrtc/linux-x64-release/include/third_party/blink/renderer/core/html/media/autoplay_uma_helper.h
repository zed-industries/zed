// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_MEDIA_AUTOPLAY_UMA_HELPER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_MEDIA_AUTOPLAY_UMA_HELPER_H_

#include "base/time/time.h"
#include "third_party/blink/public/platform/web_media_player_client.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/events/native_event_listener.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"

namespace blink {

// These values are used for histograms. Do not reorder.
enum class AutoplaySource {
  // Autoplay comes from HTMLMediaElement `autoplay` attribute.
  kAttribute = 0,
  // Autoplay comes from `play()` method.
  kMethod = 1,
  // Both sources are used.
  kDualSource = 2,
  kMaxValue = kDualSource,
};

// These values are used for histograms. Do not reorder.
enum class AutoplayUnmuteActionStatus {
  kFailure = 0,
  kSuccess = 1,
  kMaxValue = kSuccess,
};

class Document;
class HTMLMediaElement;
class IntersectionObserver;
class IntersectionObserverEntry;

class CORE_EXPORT AutoplayUmaHelper : public NativeEventListener,
                                      public ExecutionContextLifecycleObserver {
 public:
  explicit AutoplayUmaHelper(HTMLMediaElement*);
  ~AutoplayUmaHelper() override;

  void ContextDestroyed() override;

  void OnAutoplayInitiated(AutoplaySource);

  void RecordAutoplayUnmuteStatus(AutoplayUnmuteActionStatus);

  void VideoWillBeDrawnToCanvas();
  void DidMoveToNewDocument(Document& old_document);

  bool IsVisible() const { return is_visible_; }

  bool HasSource() const { return !sources_.empty(); }

  void Invoke(ExecutionContext*, Event*) override;

  void Trace(Visitor*) const override;

 private:
  friend class MockAutoplayUmaHelper;

  // Called when source is initialized and loading starts.
  void OnLoadStarted();

  void HandlePlayingEvent();
  void HandlePauseEvent();
  virtual void HandleContextDestroyed();  // Make virtual for testing.

  void MaybeUnregisterContextDestroyedObserver();
  void MaybeUnregisterMediaElementPauseListener();

  void MaybeStartRecordingMutedVideoPlayMethodBecomeVisible();
  void MaybeStopRecordingMutedVideoPlayMethodBecomeVisible(bool is_visible);

  void MaybeStartRecordingMutedVideoOffscreenDuration();
  void MaybeStopRecordingMutedVideoOffscreenDuration();

  void OnIntersectionChangedForMutedVideoOffscreenDuration(
      const HeapVector<Member<IntersectionObserverEntry>>& entries);
  void OnIntersectionChangedForMutedVideoPlayMethodBecomeVisible(
      const HeapVector<Member<IntersectionObserverEntry>>& entries);

  bool ShouldListenToContextDestroyed() const;

  // The autoplay sources.
  HashSet<AutoplaySource> sources_;

  // |sources_| can be either contain 0, 1, or 2 distinct values. When
  // |sources_.size() == 2|, that indicates there are dual sources responsible
  // for autoplay.
  static constexpr size_t kDualSourceSize = 2;

  // The media element this UMA helper is attached to. |element| owns |this|.
  Member<HTMLMediaElement> element_;

  // The observer is used to observe whether a muted video autoplaying by play()
  // method become visible at some point.
  // The UMA is pending for recording as long as this observer is non-null.
  Member<IntersectionObserver> muted_video_play_method_intersection_observer_;

  // -----------------------------------------------------------------------
  // Variables used for recording the duration of autoplay muted video playing
  // offscreen.  The variables are valid when
  // |autoplayOffscrenVisibilityObserver| is non-null.
  // The recording stops whenever the playback pauses or the page is unloaded.

  // The starting time of autoplaying muted video.
  base::TimeTicks muted_video_autoplay_offscreen_start_time_;

  // The duration an autoplaying muted video has been in offscreen.
  base::TimeDelta muted_video_autoplay_offscreen_duration_;

  // Whether an autoplaying muted video is visible.
  bool is_visible_;

  // The observer is used to observer an autoplaying muted video changing it's
  // visibility, which is used for offscreen duration UMA.  The UMA is pending
  // for recording as long as this observer is non-null.
  Member<IntersectionObserver>
      muted_video_offscreen_duration_intersection_observer_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_MEDIA_AUTOPLAY_UMA_HELPER_H_

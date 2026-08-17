// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_PAGE_CONTENT_TO_VISIBLE_TIME_REPORTER_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_PAGE_CONTENT_TO_VISIBLE_TIME_REPORTER_H_

#include "base/functional/callback.h"
#include "base/memory/weak_ptr.h"
#include "base/time/time.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/mojom/widget/record_content_to_visible_time_request.mojom.h"

namespace viz {
struct FrameTimingDetails;
}

namespace blink {

// Generates UMA metric to track the duration of tab switching from when the
// active tab is changed until the frame presentation time. The metric will be
// separated into two whether the tab switch has saved frames or not.
class BLINK_COMMON_EXPORT ContentToVisibleTimeReporter {
 public:
  // Matches the TabSwitchResult2 enum in enums.xml.
  enum class TabSwitchResult {
    // A frame was successfully presented after a tab switch.
    kSuccess = 0,
    // Tab was hidden before a frame was presented after a tab switch.
    kIncomplete = 1,
    // TabWasShown called twice for a frame without TabWasHidden between. Treat
    // the first TabWasShown as an incomplete tab switch.
    kMissedTabHide = 2,
    kMaxValue = kMissedTabHide,
  };

  ContentToVisibleTimeReporter();
  ContentToVisibleTimeReporter(const ContentToVisibleTimeReporter&) = delete;
  ContentToVisibleTimeReporter& operator=(const ContentToVisibleTimeReporter&) =
      delete;
  ~ContentToVisibleTimeReporter();

  using SuccessfulPresentationTimeCallback = base::OnceCallback<void(
      const viz::FrameTimingDetails& frame_timing_details)>;

  // Invoked when the tab associated with this recorder is shown. Returns a
  // callback to invoke the next time a frame is presented for this tab.
  SuccessfulPresentationTimeCallback TabWasShown(
      bool has_saved_frames,
      mojom::RecordContentToVisibleTimeRequestPtr start_state);

  SuccessfulPresentationTimeCallback TabWasShown(
      bool has_saved_frames,
      base::TimeTicks event_start_time,
      bool destination_is_loaded,
      bool show_reason_tab_switching,
      bool show_reason_bfcache_restore);

  // Called when the device is unfolded and the activity is recreated. Returns
  // a callback to invoke the next time a frame is presented.
  SuccessfulPresentationTimeCallback GetCallbackForNextFrameAfterUnfold(
      base::TimeTicks begin_timestamp);

  // Indicates that the tab associated with this recorder was hidden. If no
  // frame was presented since the last tab switch, failure is reported to UMA.
  void TabWasHidden();

 private:
  // Records histograms and trace events for the current tab switch.
  void RecordHistogramsAndTraceEvents(TabSwitchResult tab_switch_result,
                                      bool show_reason_tab_switching,
                                      bool show_reason_bfcache_restore,
                                      base::TimeTicks presentation_timestamp);

  void RecordHistogramsAndTraceEventsWithFrameTimingDetails(
      TabSwitchResult tab_switch_result,
      bool show_reason_tab_switching,
      bool show_reason_bfcache_restore,
      const viz::FrameTimingDetails& frame_timing_details);

  // Saves the given `state` and `has_saved_frames`, and invalidates all
  // existing callbacks that might reference the old state.
  void OverwriteTabSwitchStartState(
      mojom::RecordContentToVisibleTimeRequestPtr state,
      bool has_saved_frames);

  // Clears state and invalidates all existing callbacks that might reference
  // the old state.
  void ResetTabSwitchStartState() {
    OverwriteTabSwitchStartState(nullptr, false);
  }

  // Whether there was a saved frame for the last tab switch.
  bool has_saved_frames_;

  // The information about the last tab switch request, or nullptr if there is
  // no incomplete tab switch.
  mojom::RecordContentToVisibleTimeRequestPtr tab_switch_start_state_;

  base::WeakPtrFactory<ContentToVisibleTimeReporter> weak_ptr_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_PAGE_CONTENT_TO_VISIBLE_TIME_REPORTER_H_

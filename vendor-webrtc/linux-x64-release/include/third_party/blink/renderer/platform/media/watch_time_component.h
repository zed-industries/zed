// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_WATCH_TIME_COMPONENT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_WATCH_TIME_COMPONENT_H_

#include <vector>

#include "base/functional/callback.h"
#include "base/memory/raw_ptr.h"
#include "base/time/time.h"
#include "media/base/timestamp_constants.h"
#include "media/base/watch_time_keys.h"
#include "media/mojo/mojom/watch_time_recorder.mojom.h"
#include "third_party/blink/renderer/platform/allow_discouraged_type.h"

namespace blink {

// Every input used to calculate watch time functions the same way, so we use a
// common WatchTimeComponent class to avoid lots of copy/paste and enforce rigor
// in the reporter. Components are not thread-safe.
//
// E.g., each component does something like flip pending value, record timestamp
// of that value change, wait for next reporting cycle, finalize the elapsed
// time, flip the actual value, and then start recording from that previous
// finalize time. They may also clear the pending value flip if the value
// changes back to the previous value.
template <typename T>
class WatchTimeComponent {
 public:
  // Callback used to convert |current_value_| into a WatchTimeKey which will be
  // given to WatchTimeRecorder::RecordWatchTime().
  using ValueToKeyCB = base::RepeatingCallback<media::WatchTimeKey(T value)>;

  // Mirror of WatchTimeReporter::GetMediaTimeCB to avoid circular dependency.
  using GetMediaTimeCB = base::RepeatingCallback<base::TimeDelta(void)>;

  // |initial_value| is the starting value for |current_value_| and
  // |pending_value_|.
  //
  // |keys_to_finalize| is the list of keys which should be finalized.
  //
  // |value_to_key_cb| is optional, if unspecified every time RecordWatchTime()
  // is called, |keys_to_finalize| will also be treated as the list of keys to
  // record watch time too.
  //
  // See WatchTimeReporter constructor for |get_media_time_cb| and |recorder|.
  WatchTimeComponent(T initial_value,
                     std::vector<media::WatchTimeKey> keys_to_finalize,
                     ValueToKeyCB value_to_key_cb,
                     GetMediaTimeCB get_media_time_cb,
                     media::mojom::WatchTimeRecorder* recorder);
  WatchTimeComponent(const WatchTimeComponent&) = delete;
  WatchTimeComponent& operator=(const WatchTimeComponent&) = delete;
  ~WatchTimeComponent();

  // Called when the main WatchTimeReporter timer is started. Reinitializes
  // tracking variables and sets |start_timestamp_|. May be called at any time.
  void OnReportingStarted(base::TimeDelta start_timestamp);

  // Called when the primary value tracked by this component changes but the
  // change shouldn't take effect until the next Finalize() call.
  //
  // |pending_value_| is set to |new_value| when different than |current_value_|
  // and a finalize is marked at the current media time. If the |current_value_|
  // is unchanged any pending finalize is cleared.
  void SetPendingValue(T new_value);

  // Called when the primary value tracked by this component changes and the
  // change should take effect immediately. This is typically only called when
  // the watch time timer is not running.
  void SetCurrentValue(T new_value);

  // If there's no pending finalize, records the amount of watch time which has
  // elapsed between |current_timestamp| and |start_timestamp_| by calling into
  // mojom::WatchTimeRecorder::RecordWatchTime(). The key to be recorded to is
  // determined by the |value_to_key_cb_|; or if none is present, all keys in
  // |keys_to_finalize_| are recorded to.
  //
  // If there's a pending finalize it records the delta between |end_timestamp_|
  // and |start_timestamp_| if |end_timestamp_| < |current_timestamp|. Does not
  // complete any pending finalize. May be called multiple times even if a
  // finalize is pending.
  void RecordWatchTime(base::TimeDelta current_timestamp);

  // Completes any pending finalize. Which means setting |current_value_| to
  // |pending_value_| and setting |start_timestamp_| to |end_timestamp_| so that
  // reporting may continue on a new key if desired. Adds all keys that should
  // be finalized to |keys_to_finalize|.
  //
  // Callers must call mojom::WatchTimeRecorder::FinalizeWatchTime() for the
  // resulting keys in order to actually complete the finalize. We rely on the
  // calling class to perform the actual finalization since it may desire to
  // batch a set of keys into one finalize call to the recorder.
  //
  // E.g., some components may stop reporting upon Finalize() while others want
  // to report to a new key for all watch time going forward.
  void Finalize(std::vector<media::WatchTimeKey>* keys_to_finalize);

  // Returns true if Finalize() should be called.
  bool NeedsFinalize() const;

  // Returns the current value for |end_timestamp_|.
  base::TimeDelta end_timestamp() const { return end_timestamp_; }

  T current_value_for_testing() const { return current_value_; }

 private:
  // Initialized during construction. See constructor for details.
  const std::vector<media::WatchTimeKey> keys_to_finalize_
      ALLOW_DISCOURAGED_TYPE("TODO(crbug.com/40760651): Pending migration");
  const ValueToKeyCB value_to_key_cb_;
  const GetMediaTimeCB get_media_time_cb_;
  const raw_ptr<media::mojom::WatchTimeRecorder> recorder_;

  // The current value which will be used to select keys for reporting WatchTime
  // during the next RecordWatchTime() call.
  T current_value_;

  // A pending value which will be used to set |current_value_| once Finalize()
  // has been called.
  T pending_value_;

  // The starting and ending timestamps used for reporting watch time. The end
  // timestamp may be kNoTimestamp if reporting is ongoing.
  base::TimeDelta start_timestamp_;
  base::TimeDelta end_timestamp_ = media::kNoTimestamp;

  // The last media timestamp seen by RecordWatchTime().
  base::TimeDelta last_timestamp_ = media::kNoTimestamp;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_WATCH_TIME_COMPONENT_H_

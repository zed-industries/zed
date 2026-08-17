/*
 *  Copyright (c) 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef CALL_BITRATE_ALLOCATOR_H_
#define CALL_BITRATE_ALLOCATOR_H_

#include <stdint.h>

#include <optional>
#include <vector>

#include "api/call/bitrate_allocation.h"
#include "api/field_trials_view.h"
#include "api/sequence_checker.h"
#include "api/transport/network_types.h"
#include "api/units/data_rate.h"
#include "rtc_base/system/no_unique_address.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {

class Clock;

// Used by all send streams with adaptive bitrate, to get the currently
// allocated bitrate for the send stream. The current network properties are
// given at the same time, to let the send stream decide about possible loss
// protection.
class BitrateAllocatorObserver {
 public:
  // Returns the amount of protection used by the BitrateAllocatorObserver
  // implementation, as bitrate in bps.
  virtual uint32_t OnBitrateUpdated(BitrateAllocationUpdate update) = 0;
  // Returns the bitrate consumed (vs allocated) by BitrateAllocatorObserver
  virtual std::optional<DataRate> GetUsedRate() const = 0;

 protected:
  virtual ~BitrateAllocatorObserver() {}
};

// Struct describing parameters for how a media stream should get bitrate
// allocated to it.

enum class TrackRateElasticity {
  kCanContributeUnusedRate,
  kCanConsumeExtraRate,
  kCanContributeAndConsume
};

struct MediaStreamAllocationConfig {
  // Minimum bitrate supported by track. 0 equals no min bitrate.
  uint32_t min_bitrate_bps;
  // Maximum bitrate supported by track. 0 equals no max bitrate.
  uint32_t max_bitrate_bps;
  uint32_t pad_up_bitrate_bps;
  int64_t priority_bitrate_bps;
  // True means track may not be paused by allocating 0 bitrate will allocate at
  // least `min_bitrate_bps` for this observer, even if the BWE is too low,
  // false will allocate 0 to the observer if BWE doesn't allow
  // `min_bitrate_bps`.
  bool enforce_min_bitrate;
  // The amount of bitrate allocated to this observer relative to all other
  // observers. If an observer has twice the bitrate_priority of other
  // observers, it should be allocated twice the bitrate above its min.
  double bitrate_priority;
  std::optional<TrackRateElasticity> rate_elasticity;
};

// Interface used for mocking
class BitrateAllocatorInterface {
 public:
  virtual void AddObserver(BitrateAllocatorObserver* observer,
                           MediaStreamAllocationConfig config) = 0;
  virtual void RemoveObserver(BitrateAllocatorObserver* observer) = 0;
  virtual int GetStartBitrate(BitrateAllocatorObserver* observer) const = 0;

 protected:
  virtual ~BitrateAllocatorInterface() = default;
};

namespace bitrate_allocator_impl {
struct AllocatableTrack {
  AllocatableTrack(BitrateAllocatorObserver* observer,
                   MediaStreamAllocationConfig allocation_config)
      : observer(observer),
        config(allocation_config),
        allocated_bitrate_bps(-1),
        media_ratio(1.0) {}
  BitrateAllocatorObserver* observer;
  MediaStreamAllocationConfig config;
  int64_t allocated_bitrate_bps;
  std::optional<DataRate> last_used_bitrate;
  double media_ratio;  // Part of the total bitrate used for media [0.0, 1.0].

  uint32_t LastAllocatedBitrate() const;
  // The minimum bitrate required by this observer, including
  // enable-hysteresis if the observer is in a paused state.
  uint32_t MinBitrateWithHysteresis() const;
};
}  // namespace bitrate_allocator_impl

// Usage: this class will register multiple RtcpBitrateObserver's one at each
// RTCP module. It will aggregate the results and run one bandwidth estimation
// and push the result to the encoders via BitrateAllocatorObserver(s).
class BitrateAllocator : public BitrateAllocatorInterface {
 public:
  // Used to get notified when send stream limits such as the minimum send
  // bitrate and max padding bitrate is changed.
  class LimitObserver {
   public:
    virtual void OnAllocationLimitsChanged(BitrateAllocationLimits limits) = 0;

   protected:
    virtual ~LimitObserver() = default;
  };

  // `upper_elastic_rate_limit` specifies the rate ceiling an observer can
  // reach when unused bits are added. A value of zero disables borrowing of
  // unused rates.
  BitrateAllocator(LimitObserver* limit_observer,
                   DataRate upper_elastic_rate_limit);
  ~BitrateAllocator() override;

  void UpdateStartRate(uint32_t start_rate_bps);

  // Allocate target_bitrate across the registered BitrateAllocatorObservers.
  void OnNetworkEstimateChanged(TargetTransferRate msg);

  // Set the configuration used by the bandwidth management.
  // `observer` updates bitrates if already in use.
  // `config` is the configuration to use for allocation.
  // Note that `observer`->OnBitrateUpdated() will be called
  // within the scope of this method with the current rtt, fraction_loss and
  // available bitrate and that the bitrate in OnBitrateUpdated will be zero if
  // the `observer` is currently not allowed to send data.
  void AddObserver(BitrateAllocatorObserver* observer,
                   MediaStreamAllocationConfig config) override;

  // Checks and recomputes bitrate allocation if necessary (when an
  // elastic/audio bitrate increases significantly). Returns whether there is an
  // active contributing and active consuming stream.
  bool RecomputeAllocationIfNeeded();

  // Removes a previously added observer, but will not trigger a new bitrate
  // allocation.
  void RemoveObserver(BitrateAllocatorObserver* observer) override;

  // Returns initial bitrate allocated for `observer`. If `observer` is not in
  // the list of added observers, a best guess is returned.
  int GetStartBitrate(BitrateAllocatorObserver* observer) const override;

 private:
  using AllocatableTrack = bitrate_allocator_impl::AllocatableTrack;

  // Calculates the minimum requested send bitrate and max padding bitrate and
  // calls LimitObserver::OnAllocationLimitsChanged.
  void UpdateAllocationLimits() RTC_RUN_ON(&sequenced_checker_);

  // Allow packets to be transmitted in up to 2 times max video bitrate if the
  // bandwidth estimate allows it.
  // TODO(bugs.webrtc.org/8541): May be worth to refactor to keep this logic in
  // video send stream.
  static uint8_t GetTransmissionMaxBitrateMultiplier();

  RTC_NO_UNIQUE_ADDRESS SequenceChecker sequenced_checker_;
  LimitObserver* const limit_observer_ RTC_GUARDED_BY(&sequenced_checker_);
  // Stored in a list to keep track of the insertion order.
  std::vector<AllocatableTrack> allocatable_tracks_
      RTC_GUARDED_BY(&sequenced_checker_);
  uint32_t last_target_bps_ RTC_GUARDED_BY(&sequenced_checker_);
  uint32_t last_stable_target_bps_ RTC_GUARDED_BY(&sequenced_checker_);
  uint32_t last_non_zero_bitrate_bps_ RTC_GUARDED_BY(&sequenced_checker_);
  uint8_t last_fraction_loss_ RTC_GUARDED_BY(&sequenced_checker_);
  int64_t last_rtt_ RTC_GUARDED_BY(&sequenced_checker_);
  int64_t last_bwe_period_ms_ RTC_GUARDED_BY(&sequenced_checker_);
  // Number of mute events based on too low BWE, not network up/down.
  int num_pause_events_ RTC_GUARDED_BY(&sequenced_checker_);
  int64_t last_bwe_log_time_ RTC_GUARDED_BY(&sequenced_checker_);
  BitrateAllocationLimits current_limits_ RTC_GUARDED_BY(&sequenced_checker_);
  const DataRate upper_elastic_rate_limit_ RTC_GUARDED_BY(&sequenced_checker_);
};

// TODO(b/350555527): Remove after experiment
DataRate GetElasticRateAllocationFieldTrialParameter(
    const FieldTrialsView& field_trials);

}  // namespace webrtc
#endif  // CALL_BITRATE_ALLOCATOR_H_

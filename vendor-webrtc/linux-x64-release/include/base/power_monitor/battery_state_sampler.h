// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_POWER_MONITOR_BATTERY_STATE_SAMPLER_H_
#define BASE_POWER_MONITOR_BATTERY_STATE_SAMPLER_H_

#include <memory>
#include <optional>
#include <vector>

#include "base/base_export.h"
#include "base/observer_list.h"
#include "base/observer_list_types.h"
#include "base/power_monitor/battery_level_provider.h"
#include "base/power_monitor/power_monitor_buildflags.h"
#include "base/power_monitor/sampling_event_source.h"
#include "base/sequence_checker.h"

namespace base {

// Periodically samples the battery and notifies its observers.
class BASE_EXPORT BatteryStateSampler {
 public:
  class Observer : public base::CheckedObserver {
   public:
    // Note: The first sample taken by the BatteryStateSampler may be out of
    // date (i.e. represent the battery state at an earlier time). Observers
    // that want to ignore those stale samples should ignore the first call to
    // OnBatteryStateSampled.
    virtual void OnBatteryStateSampled(
        const std::optional<BatteryLevelProvider::BatteryState>&
            battery_state) = 0;
  };

  // Creates a BatteryStateSampler and initializes the global instance. Will
  // DCHECK if an instance already exists.
  BatteryStateSampler(
      std::unique_ptr<SamplingEventSource> sampling_event_source =
          CreateSamplingEventSource(),
      std::unique_ptr<BatteryLevelProvider> battery_level_provider =
          BatteryLevelProvider::Create());
  ~BatteryStateSampler();

  // Returns the unique instance, or nullptr on platforms without a
  // `BatteryLevelProvider` implementation.
  static BatteryStateSampler* Get();

  // Adds/removes an observer. `OnBatteryStateSampled` will be immediately
  // invoked upon adding an observer if an existing sample exists already.
  void AddObserver(Observer* observer);
  void RemoveObserver(Observer* observer);

  // Shuts down this instance but doesn't destroy it. This allows it to remain
  // alive for its observers to deregister as they are destroyed without causing
  // use-after-frees, but it won't serve any samples after this is called.
  // Invoked in `PostMainMessageLoopRun`.
  void Shutdown();

  // Creates, installs, and returns an instance of the sampler for testing.
  // This is meant to be used in browser tests before browser init to control
  // the sampler's behavior in the tests.
  static std::unique_ptr<base::BatteryStateSampler> CreateInstanceForTesting(
      std::unique_ptr<SamplingEventSource> sampling_event_source,
      std::unique_ptr<BatteryLevelProvider> battery_level_provider);

  // Returns true if a sampler has been created using `CreateInstanceForTesting`
  static bool HasTestingInstance();

  // Returns the expected time between each sample. The actual time can vary
  // slighly, but a large discrepancy to this value indicates that the sample
  // is not to be trusted. For example, the machine might have gone to sleep,
  // skewing the data.
  base::TimeDelta GetSampleInterval();

 private:
  // Returns a platform specific SamplingEventSource.
  static std::unique_ptr<SamplingEventSource> CreateSamplingEventSource();

  // Called when the first battery sampled is obtained. Notifies current
  // observers as they are waiting on the cached battery state.
  void OnInitialBatteryStateSampled(
      const std::optional<BatteryLevelProvider::BatteryState>& battery_state);

  // Triggers the sampling of the battery state.
  void OnSamplingEvent();

  // Notifies observers of the sampled battery state.
  void OnBatteryStateSampled(
      const std::optional<BatteryLevelProvider::BatteryState>& battery_state);

  std::unique_ptr<SamplingEventSource> sampling_event_source_
      GUARDED_BY_CONTEXT(sequence_checker_);

  std::unique_ptr<BatteryLevelProvider> battery_level_provider_
      GUARDED_BY_CONTEXT(sequence_checker_);

  base::ObserverList<Observer> observer_list_
      GUARDED_BY_CONTEXT(sequence_checker_);

  // Indicates if |last_battery_state_| contains an actual sample. Note: a
  // separate bool is used to avoid nested optionals.
  bool has_last_battery_state_ GUARDED_BY_CONTEXT(sequence_checker_) = false;

  // The value of the last sample taken.
  std::optional<BatteryLevelProvider::BatteryState> last_battery_state_
      GUARDED_BY_CONTEXT(sequence_checker_);

  SEQUENCE_CHECKER(sequence_checker_);
};

}  // namespace base

#endif  // BASE_POWER_MONITOR_BATTERY_STATE_SAMPLER_H_

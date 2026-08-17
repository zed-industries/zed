// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_MAC_POWER_POWER_SAMPLER_BATTERY_SAMPLER_H_
#define TOOLS_MAC_POWER_POWER_SAMPLER_BATTERY_SAMPLER_H_

#include <stdint.h>

#include <cstdint>
#include <memory>
#include <optional>

#include "base/mac/scoped_ioobject.h"
#include "tools/mac/power/power_sampler/sampler.h"

namespace power_sampler {

// The battery sampler provides data retrieved from the IOPMPowerSource. The
// |GetSample| method is ideally invoked on change notification from the
// IOPMPowerSource in order to sample new power data immediately.
//
// The sampler provides battery voltage, as well as the "max capacity" and
// "current capacity" of the battery.
//
// This sampler also provides an average power consumption estimate when
// possible, which is when the net "current capacity" of the battery has changed
// from a previous sample. This may not occur every time GetSample is invoked,
// as the "capacity" data is in integral units of 1mAh, which represents a fair
// bit of energy. For a three-cell LiIon battery with a nominal voltage of
// 3*3.7V or 11.1V, a single mAh consumed over the period of one minute
// represents (1mAh * 11.1V)/60s or ~0.67W average power consumption. The M1
// MacBooks, in particular, have been observed to consume much lower power than
// this when the backlight is turned down, and so it may take multiple sampling
// intervals for a capacity change of 1mAh to accrue. To estimate the average
// power consumption, this class uses simple linear interpolation over the
// interval in question, e.g. the assumption is that the battery voltage changes
// linearly from the start to the end of the interval.
//
// This sampler assumes that the battery capacity reported by macOS has changed
// just before the first call to GetSample(). If it isn't the case, the computed
// average power consumption will be incorrect. For example:
//  t = -2 minute : current capacity = 51, max capacity = 60
//  t = -1 minute : current capacity = 50, max capacity = 60
//  t =  0 minute : current capacity = 50, max capacity = 60 -> GetSample
//  t =  1 minute : current capacity = 49, max capacity = 60 -> GetSample
// The 2nd call to GetSample() will assume that current capacity decreased by 1
// over a 1 minute interval, but it really decreased by 1 over a 2 minutes
// interval.
class BatterySampler : public Sampler {
 public:
  static constexpr char kSamplerName[] = "battery";

  ~BatterySampler() override;

  // Creates and initializes a new sampler, if possible.
  // Returns nullptr on failure.
  static std::unique_ptr<BatterySampler> Create();

  // Sampler implementation.
  std::string GetName() override;
  DatumNameUnits GetDatumNameUnits() override;
  Sample GetSample(base::TimeTicks sample_time) override;

 protected:
  // Exposed for testing only.
  struct BatteryData {
    bool external_connected;
    int64_t voltage_mv;
    int64_t current_capacity_mah;
    int64_t max_capacity_mah;
    int64_t update_time_seconds_since_epoch;
  };
  using MaybeGetBatteryDataFn =
      std::optional<BatteryData> (*)(io_service_t power_source);
  using GetSecondsSinceEpochFn = int64_t (*)();

  // TODO(siggi): It'd be possible to test the data extraction part of this
  //     function by splitting it in two and passing it a dictionary to
  //     dissect.
  static std::optional<BatteryData> MaybeGetBatteryData(
      io_service_t power_source);

  struct AvgConsumption {
    double watts;
    int64_t mah;
  };

  // Yields average for |prev_data|, |new_data| and |duration| if the current
  // capacity has changed between |prev_data| and |new_data|.
  static std::optional<AvgConsumption> MaybeComputeAvgConsumption(
      base::TimeDelta duration,
      const BatteryData& prev_data,
      const BatteryData& new_data);

  static std::unique_ptr<BatterySampler> CreateImpl(
      MaybeGetBatteryDataFn maybe_get_battery_data_fn,
      GetSecondsSinceEpochFn get_seconds_since_epoch_fn,
      base::mac::ScopedIOObject<io_service_t> power_source);

  BatterySampler(MaybeGetBatteryDataFn maybe_get_battery_data_fn,
                 GetSecondsSinceEpochFn get_seconds_since_epoch_fn,
                 base::mac::ScopedIOObject<io_service_t> power_source,
                 BatteryData initial_battery_data);

 private:
  void StoreBatteryData(base::TimeTicks sample_time,
                        const BatteryData& battery_data);

  const MaybeGetBatteryDataFn maybe_get_battery_data_fn_;
  const GetSecondsSinceEpochFn get_seconds_since_epoch_fn_;
  const base::mac::ScopedIOObject<io_service_t> power_source_;

  // To compute the average power consumed between non-identical
  // "current capacity" samples, keep track of the voltage, max capacity and
  // current capacity last seen, as well as the time the current capacity last
  // changed.
  // Note that the capacity of a battery is load-dependent, and the capacity
  // estimate provided by macOS takes this into account. To see what this looks
  // like, take a look at the data sheet for e.g. any lithium ion battery, and
  // see how the datasheet will specify multiple discharge curves at different
  // "C" discharge levels.
  //
  // This means that the reported max capacity of the battery may change
  // drastically on load changes, whether downwards on load increase, or
  // upwards on load decrease.
  // It's been observed that whenever the reported max capacity of the battery
  // changes, the same delta is also applied to the reported current capacity
  // value. Hence, by subtracting the max capacity change from the current
  // capacity change, it's possible to keep track of the actual current
  // consumption.
  base::TimeTicks prev_battery_sample_time_ = base::TimeTicks::Min();
  std::optional<BatteryData> prev_battery_data_;
};

}  // namespace power_sampler

#endif  // TOOLS_MAC_POWER_POWER_SAMPLER_BATTERY_SAMPLER_H_

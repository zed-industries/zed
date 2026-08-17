// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_MAC_POWER_POWER_SAMPLER_MAIN_DISPLAY_SAMPLER_H_
#define TOOLS_MAC_POWER_POWER_SAMPLER_MAIN_DISPLAY_SAMPLER_H_

#include <CoreGraphics/CoreGraphics.h>

#include <memory>
#include <optional>

#include "tools/mac/power/power_sampler/sampler.h"

namespace power_sampler {

// Samples the backlight level of the main display, if possible.
// Note that this sampler pretty much assumes that the computer under test has a
// single, built-in backlit display.
// Note also that this samples the set level of the backlight, which doesn't
// necessarily mean the display is lit at all. The sleeping flag will however
// reflect this.
class MainDisplaySampler : public Sampler {
 public:
  static constexpr char kSamplerName[] = "main_display";

  ~MainDisplaySampler() override;

  // Creates and initializes a new sampler, if possible.
  // Returns nullptr on failure.
  static std::unique_ptr<MainDisplaySampler> Create();

  // Sampler implementation.
  std::string GetName() override;
  DatumNameUnits GetDatumNameUnits() override;
  Sample GetSample(base::TimeTicks sample_time) override;

 protected:
  // Virtual for testing.
  virtual std::optional<float> GetDisplayBrightness();
  virtual bool GetIsDisplaySleeping();

  MainDisplaySampler(CGDirectDisplayID main_display);

 private:
  const CGDirectDisplayID main_display_;
};

}  // namespace power_sampler

#endif  // TOOLS_MAC_POWER_POWER_SAMPLER_MAIN_DISPLAY_SAMPLER_H_

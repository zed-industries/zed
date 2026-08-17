// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_PRIVACY_BUDGET_IDENTIFIABILITY_SAMPLE_TEST_UTILS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_PRIVACY_BUDGET_IDENTIFIABILITY_SAMPLE_TEST_UTILS_H_

#include "base/component_export.h"
#include "base/memory/raw_ptr.h"
#include "third_party/blink/public/common/privacy_budget/identifiability_study_settings_provider.h"

namespace blink {

struct CallCounts {
  bool response_for_is_meta_experiment_active = false;
  bool response_for_is_active = false;
  bool response_for_is_anything_blocked = false;
  bool response_for_is_allowed = false;

  int count_of_is_meta_experiment_active = 0;
  int count_of_is_active = 0;
  int count_of_is_any_type_or_surface_blocked = 0;
  int count_of_is_surface_allowed = 0;
  int count_of_is_type_allowed = 0;
};

class COMPONENT_EXPORT(PRIVACY_BUDGET_TEST_SUPPORT)
    CountingSettingsProvider final
    : public IdentifiabilityStudySettingsProvider {
 public:
  explicit CountingSettingsProvider(CallCounts* state) : state_(state) {}

  bool IsMetaExperimentActive() const override;

  bool IsActive() const override;

  bool IsAnyTypeOrSurfaceBlocked() const override;

  bool IsSurfaceAllowed(IdentifiableSurface surface) const override;

  bool IsTypeAllowed(IdentifiableSurface::Type type) const override;

 private:
  raw_ptr<CallCounts> state_ = nullptr;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_PRIVACY_BUDGET_IDENTIFIABILITY_SAMPLE_TEST_UTILS_H_

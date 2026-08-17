// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_PRIVACY_BUDGET_IDENTIFIABILITY_STUDY_SETTINGS_PROVIDER_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_PRIVACY_BUDGET_IDENTIFIABILITY_STUDY_SETTINGS_PROVIDER_H_

#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/privacy_budget/identifiable_surface.h"

namespace blink {

class BLINK_COMMON_EXPORT IdentifiabilityStudySettingsProvider {
 public:
  virtual ~IdentifiabilityStudySettingsProvider();

  // Returns true if the meta experiment of the identifiability study is active
  // (meaning that this client is selected for collecting meta surfaces). For
  // any specific instance of IdentifiabilityStudySettings, this answer cannot
  // change. It will only be queried once.
  virtual bool IsMetaExperimentActive() const = 0;

  // Returns true if the identifiability study is active. For any specific
  // instance of IdentifiabilityStudySettings, this answer cannot change. It
  // will only be queried once.
  virtual bool IsActive() const = 0;

  // Returns true if any specific surface or type is blocked. Otherwise it is
  // assumed that neither IsSurfaceBlocked() nor IsTypeBlocked() will ever
  // return true for anything.
  //
  // Only meaningful if IsActive() returns true.
  virtual bool IsAnyTypeOrSurfaceBlocked() const = 0;

  // Returns true if the given surface should be sampled.
  //
  // If IsActive() is false, this method will not be called.
  virtual bool IsSurfaceAllowed(IdentifiableSurface surface) const = 0;

  // Returns true if the given surface type should be sampled.
  //
  // If IsActive() is false, this method will not be called.
  virtual bool IsTypeAllowed(IdentifiableSurface::Type type) const = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_PRIVACY_BUDGET_IDENTIFIABILITY_STUDY_SETTINGS_PROVIDER_H_

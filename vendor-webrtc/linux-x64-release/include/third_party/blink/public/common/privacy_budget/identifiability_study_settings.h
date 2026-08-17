// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_PRIVACY_BUDGET_IDENTIFIABILITY_STUDY_SETTINGS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_PRIVACY_BUDGET_IDENTIFIABILITY_STUDY_SETTINGS_H_

#include <memory>

#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/privacy_budget/identifiability_study_settings_provider.h"
#include "third_party/blink/public/common/privacy_budget/identifiable_surface.h"
#include "third_party/blink/public/mojom/use_counter/metrics/web_feature.mojom-forward.h"

namespace blink {

// Determines whether the identifiability study is active and if so whether a
// given surface or surface type should be sampled.
//
// This class can used from multiple threads and does not require
// synchronization.
//
// See documentation on individual methods for notes on thread safety.
//
// Guidelines for when and how to use it can be found in:
//
//     //docs/privacy_budget/privacy_budget_instrumentation.md#gating
//
class BLINK_COMMON_EXPORT IdentifiabilityStudySettings {
 public:
  // Constructs a default IdentifiabilityStudySettings instance. By default the
  // settings instance acts as if the study is disabled, and implicitly as if
  // all surfaces and types are blocked.
  IdentifiabilityStudySettings();

  // Constructs a IdentifiabilityStudySettings instance which reflects the state
  // specified by |provider|.
  explicit IdentifiabilityStudySettings(
      std::unique_ptr<IdentifiabilityStudySettingsProvider> provider);

  ~IdentifiabilityStudySettings();

  // Get a pointer to an instance of IdentifiabilityStudySettings for the
  // process.
  //
  // This method and the returned object is safe to use from any thread and is
  // never destroyed.
  //
  // On the browser process, the returned instance is authoritative. On all
  // other processes the returned instance should be considered advisory. It's
  // only meant as an optimization to avoid calculating things unnecessarily.
  static const IdentifiabilityStudySettings* Get();

  // Initialize the process-wide settings instance with the specified settings
  // provider. Should only be called once per process and only from the main
  // thread.
  //
  // For testing, you can use ResetStateForTesting().
  static void SetGlobalProvider(
      std::unique_ptr<IdentifiabilityStudySettingsProvider> provider);

  // Returns true if the study is active for this client. Once if it returns
  // true, it doesn't return false at any point after. The converse is not true.
  // Note that metrics might still need to be sampled (because of tracing) even
  // if this returns `false`. Use one of the `ShouldSample...` methods below for
  // deciding whether a surface needs to be sampled.
  bool IsActive() const;

  // Returns true if |surface| should be sampled.
  bool ShouldSampleSurface(IdentifiableSurface surface) const;

  // Returns true if |type| should be sampled.
  bool ShouldSampleType(IdentifiableSurface::Type type) const;

  // Convenience method for determining whether the surface constructable from
  // the type (|kWebFeature|) and the |feature| is allowed. See
  // ShouldSampleSurface for more detail.
  bool ShouldSampleWebFeature(mojom::WebFeature feature) const;

  // Only used for testing. Resets internal state and violates API contracts
  // made above about the lifetime of IdentifiabilityStudySettings*.
  static void ResetStateForTesting();

  IdentifiabilityStudySettings(IdentifiabilityStudySettings&&) = delete;
  IdentifiabilityStudySettings(const IdentifiabilityStudySettings&) = delete;
  IdentifiabilityStudySettings& operator=(const IdentifiabilityStudySettings&) =
      delete;

 private:
  // If this returns `false`, then nothing should be sampled.
  bool ShouldSampleAnything() const;

  const std::unique_ptr<IdentifiabilityStudySettingsProvider> provider_;
  const bool is_enabled_ = false;
  const bool is_any_surface_or_type_blocked_ = false;
  const bool is_meta_experiment_active_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_PRIVACY_BUDGET_IDENTIFIABILITY_STUDY_SETTINGS_H_

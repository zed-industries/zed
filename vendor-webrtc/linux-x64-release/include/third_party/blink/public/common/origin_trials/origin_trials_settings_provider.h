// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_ORIGIN_TRIALS_ORIGIN_TRIALS_SETTINGS_PROVIDER_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_ORIGIN_TRIALS_ORIGIN_TRIALS_SETTINGS_PROVIDER_H_

#include "base/no_destructor.h"
#include "base/synchronization/lock.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/mojom/origin_trials/origin_trials_settings.mojom.h"

namespace blink {

// When OriginTrialPolicyImpl is constructed, it will use this class to get the
// settings. The settings were previously set via SetSettings during browser or
// renderer process startup.
class BLINK_COMMON_EXPORT OriginTrialsSettingsProvider {
 public:
  OriginTrialsSettingsProvider(const OriginTrialsSettingsProvider&) = delete;
  OriginTrialsSettingsProvider& operator=(const OriginTrialsSettingsProvider&) =
      delete;
  ~OriginTrialsSettingsProvider();

  // Get the single instance of this class.
  static OriginTrialsSettingsProvider* Get();

  // Sets the settings. The caller is responsible for ensuring that this only
  // called once for the process.
  void SetSettings(blink::mojom::OriginTrialsSettingsPtr);

  // Returns a copy of the settings stored in |settings_|.
  blink::mojom::OriginTrialsSettingsPtr GetSettings();

 protected:
  OriginTrialsSettingsProvider();
  friend class base::NoDestructor<OriginTrialsSettingsProvider>;

 private:
  // Synchronizes access to |settings_|. Only needed during start up.
  base::Lock settings_lock_;
  blink::mojom::OriginTrialsSettingsPtr settings_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_ORIGIN_TRIALS_ORIGIN_TRIALS_SETTINGS_PROVIDER_H_

// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FEATURE_CONTEXT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FEATURE_CONTEXT_H_

#include "third_party/blink/public/mojom/origin_trials/origin_trial_feature.mojom-blink-forward.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

class RuntimeFeatureStateOverrideContext;

// A pure virtual interface for checking the availability of origin trial
// features in a context as well as whether a feature's state has been
// overridden.
class PLATFORM_EXPORT FeatureContext {
 public:
  virtual bool FeatureEnabled(mojom::blink::OriginTrialFeature) const = 0;
  virtual RuntimeFeatureStateOverrideContext*
  GetRuntimeFeatureStateOverrideContext() const = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FEATURE_CONTEXT_H_

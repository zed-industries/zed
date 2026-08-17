/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_RUNTIME_FEATURES_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_RUNTIME_FEATURES_H_

#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_runtime_features_base.h"
#include "third_party/blink/public/platform/web_string.h"

#include <string>

namespace blink {

// This class is used to enable runtime features of Blink.
// Stable features are enabled by default.
class BLINK_PLATFORM_EXPORT WebRuntimeFeatures : public WebRuntimeFeaturesBase {
 public:
  // Enable or disable features with status=experimental listed in
  // renderer/platform/runtime_enabled_features.json5.
  static void EnableExperimentalFeatures(bool);

  // Enable or disable features with status=test listed in
  // renderer/platform/runtime_enabled_features.json5.
  static void EnableTestOnlyFeatures(bool);

  // Enable or disable features with non-empty origin_trial_feature_name in
  // renderer/platform/runtime_enabled_features.json5.
  static void EnableOriginTrialControlledFeatures(bool);

  // Enables or disables a feature by its string identifier from
  // renderer/platform/runtime_enabled_features.json5.
  // Note: We use std::string instead of WebString because this API can
  // be called before blink::Initalize(). We can't create WebString objects
  // before blink::Initialize().
  static void EnableFeatureFromString(const std::string& name, bool enable);

  // Update runtime features status from blink::features features status.
  static void UpdateStatusFromBaseFeatures();

  static void EnableOverlayScrollbars(bool);
  static void EnableFluentScrollbars(bool);
  static void EnableFluentOverlayScrollbars(bool);

  WebRuntimeFeatures() = delete;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_RUNTIME_FEATURES_H_
